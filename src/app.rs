use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use ratatui::{DefaultTerminal, Frame, crossterm};
use serde_json::{Value, json};
use std::{
    env::{self},
    process,
};

use crate::{
    log,
    tools::{handle_bash, handle_read_file, handle_write_file},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}
use crate::util::{Model, Role, ToolSpec};

pub fn app(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        log!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread;

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let (request_tx, request_rx): (
        Sender<Vec<serde_json::Value>>,
        Receiver<Vec<serde_json::Value>>,
    ) = mpsc::channel();
    let (response_tx, response_rx): (Sender<Value>, Receiver<Value>) = mpsc::channel();

    let config_clone = config.clone();
    thread::spawn(move || {
        log!("[worker thread] started");
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = Client::with_config(config_clone);

        while let Ok(mut messages) = request_rx.recv() {
            log!(
                "[worker thread] received request: {}",
                serde_json::to_string_pretty(&messages).unwrap()
            );
            let value = client.clone();
            let response: Result<Value, async_openai::error::OpenAIError> =
                rt.block_on(async move {
                    log!("[worker thread][tokio] creating and sending OpenAI chat request");
                    let res = value
                        .chat()
                        .create_byot(json!({
                            "messages": messages,
                            "model": Model::from_env().name,
                            "tools": [
                                json!({
                                    "type": "function",
                                    "function": ToolSpec::read_file()
                                }),
                                json!({
                                    "type": "function",
                                    "function": ToolSpec::write_file()
                                }),
                                json!({
                                    "type": "function",
                                    "function": ToolSpec::bash()
                                })
                            ]
                        }))
                        .await;
                    log!("[worker thread][tokio] OpenAI chat returned");
                    res
                });
            log!(
                "[worker thread] completed OpenAI call, result variant: {}",
                if response.is_ok() { "Ok" } else { "Err" }
            );
            // On error, return an error Value
            let resp_val = match response {
                Ok(ref r) => {
                    log!(
                        "[worker thread] AI response: {}",
                        serde_json::to_string_pretty(r).unwrap()
                    );
                    r.clone()
                }
                Err(ref e) => {
                    log!("[worker thread] OpenAI error: {e}");
                    json!({"error": format!("{e}")})
                }
            };
            if let Err(e) = response_tx.send(resp_val) {
                log!("[worker thread] FAILED to send response to main thread: {e}");
            } else {
                log!("[worker thread] sent result to main thread");
            }
        }
    });

    let mut messages = [json!({
        "role": Role::user,
        "content": args.prompt
    })]
    .to_vec();

    // Initial request, ask worker thread to fetch OpenAI API response
    log!("[main] sending initial request");
    request_tx.send(messages.clone()).unwrap();

    loop {
        log!("[main] waiting for response from worker thread...");
        let response = match response_rx.recv() {
            Ok(r) => r,
            Err(e) => {
                log!("[main] FAILED to receive response from worker: {e}");
                break;
            }
        };
        log!(
            "[main] received response: {}",
            serde_json::to_string_pretty(&response).unwrap()
        );

        if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
            messages.push(json!({
                "role": Role::assistant,
                "content": response["choices"][0]["message"]["content"].clone(),
                "tool_calls": tool_calls
            }));

            for tool_call in tool_calls {
                log!(
                    "processing tool call: {}",
                    serde_json::to_string_pretty(tool_call)?
                );
                let name = tool_call["function"]["name"].as_str().unwrap();
                let arguments: Value =
                    serde_json::from_str(tool_call["function"]["arguments"].as_str().unwrap())?;
                let tool_call_id = tool_call["id"].as_str().unwrap();

                let tool_response = match name {
                    "ReadFile" => handle_read_file(&arguments, tool_call_id, name),
                    "WriteFile" => handle_write_file(&arguments, tool_call_id, name),
                    "Bash" => handle_bash(&arguments, tool_call_id, name),
                    other => {
                        log!("Unknown toolcall name: {}", other);
                        Ok(json!({
                            "role": "tool",
                            "tool_call_id": tool_call_id,
                            "name": name,
                            "content": format!("Unknown toolcall: {}", other)
                        }))
                    }
                }?;
                messages.push(tool_response);
            }
            request_tx.send(messages.clone()).unwrap();
        } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            log!("no toolcall");
            log!("assistant response content: {}", content);
            messages.push(json!({
                "role": Role::assistant,
                "content": content
            }));
            log!("{}", content);
            break;
        } else {
            log!("Unexpected response format: {}", response);
            break;
        }
    }

    // Ok(())

    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
