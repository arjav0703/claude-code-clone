use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

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
        "role": "user",
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
                "model": get_model(),
                "tools": [
                    {
                    "type": "function",
                    "function": {
                        "name": "ReadFile",
                        "description": "Read the contents of a file",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                    "type": "string",
                                    "description": "Path to the file to read"
                                }
                            },
                            "required": ["file_path"]
                        }
                    }
                    },
                    {
                    "type": "function",
                    "function": {
                        "name": "WriteFile",
                        "description": "Write content to a file",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                "type": "string",
                                "description": "The path of the file to write to"
                                },
                                "content": {
                                "type": "string",
                                "description": "The content to write to the file"
                                }
                            },
                            "required": ["file_path", "content"],

                        }
                    }
                    }
                ]
            }))
            .await?;

        eprintln!(
            "received response: {}",
            serde_json::to_string_pretty(&response)?
        );

        if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
            messages.push(json!({
                "role": "assistant",
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

                if name == "ReadFile" {
                    let file_path = arguments["file_path"].as_str().unwrap();
                    let contents = std::fs::read_to_string(file_path)?;
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": tool_call_id,
                        "name": name,
                        "content": contents
                    }));
                }

                if name == "WriteFile" {
                    let file_path = arguments["file_path"].as_str().unwrap();
                    let content = arguments["content"].as_str().unwrap();
                    std::fs::write(file_path, content)?;
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": tool_call_id,
                        "name": name,
                        "content": format!("Wrote to file {}", file_path)
                    }));
                }
            }
        } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            eprintln!("no toolcall");
            eprintln!("assistant response content: {}", content);
            messages.push(json!({
                "role": "assistant",
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

fn get_model() -> String {
    env::var("OPENROUTER_MODEL").unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string())
}
