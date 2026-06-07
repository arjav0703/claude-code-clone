use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

use crate::util::{Model, Role, ToolSpec};

mod tools;
mod util;

use tools::{handle_bash, handle_read_file, handle_write_file};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let mut messages = [json!({
        "role": Role::user,
        "content": args.prompt
    })]
    .to_vec();

    loop {
        eprintln!(
            "entering loop with messages: {}",
            serde_json::to_string_pretty(&messages)?
        );
        let response: Value = client
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
            .await?;
        eprintln!("request fulfilled");

        eprintln!(
            "received response: {}",
            serde_json::to_string_pretty(&response)?
        );

        if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
            messages.push(json!({
                "role": Role::assistant,
                "content": response["choices"][0]["message"]["content"].clone(),
                "tool_calls": tool_calls
            }));

            for tool_call in tool_calls {
                eprintln!(
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
                        eprintln!("Unknown toolcall name: {}", other);
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
        } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            eprintln!("no toolcall");
            eprintln!("assistant response content: {}", content);
            messages.push(json!({
                "role": Role::assistant,
                "content": content
            }));
            println!("{}", content);
            break;
        } else {
            eprintln!("Unexpected response format: {}", response);
            break;
        }
    }

    Ok(())
}
