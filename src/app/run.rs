use async_openai::config::OpenAIConfig;
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame, text::Text};
use serde_json::{Value, json};
use std::sync::Mutex;
use std::{env, sync::Arc};

use crate::tools::{handle_bash, handle_read_file, handle_write_file};
use crate::util::Role;
use crate::{app::state::AppState, log};

pub struct App {
    state: AppState,
    exit_code: Option<i32>,
}

impl App {
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<i32> {
        // while self.exit_code.is_none() {
        //     terminal.draw(|f| self.render(f))?;
        // }

        let app_state = &self.state;

        let mut messages = app_state.messages.lock().unwrap().clone();

        // Initial request, ask worker thread to fetch OpenAI API response
        log!("[main] sending initial request");
        app_state.message_sender.send(messages.clone()).unwrap();

        let response_rx = &app_state.message_receiver;

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
                    }
                    .unwrap_or_else(|e| {
                        log!("Error handling tool call {}: {}", name, e);
                        json!({
                            "role": "tool",
                            "tool_call_id": tool_call_id,
                            "name": name,
                            "content": format!("Error handling tool call {}: {}", name, e)
                        })
                    });
                    messages.push(tool_response);
                }
                app_state.message_sender.send(messages.clone()).unwrap();
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
        //
        // // Ok(())
        //
        // loop {
        //     terminal.draw(render)?;
        //     if crossterm::event::read()?.is_key_press() {
        //         break Ok(());
        //     }
        // }
        self.render_dummy_widget(&mut terminal.get_frame());
        Ok(0)
    }
}

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }

    fn render_dummy_widget(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }
}

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

impl App {
    pub fn setup() -> Self {
        let args = Args::parse();

        let base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
            log!("OPENROUTER_API_KEY is not set");
            std::process::exit(1);
        });

        use std::sync::mpsc::{self, Receiver, Sender};

        let config = OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(api_key);

        let (request_tx, request_rx): (
            Sender<Vec<serde_json::Value>>,
            Receiver<Vec<serde_json::Value>>,
        ) = mpsc::channel();
        let (response_tx, response_rx): (Sender<Value>, Receiver<Value>) = mpsc::channel();

        let app_state = {
            AppState {
                messages: Arc::new(Mutex::new(
                    [json!({
                        "role": Role::user,
                        "content": args.prompt
                    })]
                    .to_vec(),
                )),
                message_sender: request_tx,
                message_receiver: response_rx,
                config,
            }
        };

        app_state.listen_for_messages(request_rx, response_tx.clone());

        App {
            state: app_state,
            exit_code: None,
        }
    }
}
