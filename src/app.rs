use async_openai::config::OpenAIConfig;
use clap::Parser;
use serde_json::{Value, json};
use std::{
    env,
    sync::{Arc, Mutex},
};

mod run;
mod state;
use state::AppState;
mod render;

pub struct App {
    state: AppState,
    exit_code: Option<i32>,
}

use crate::{log, util::Role};
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

impl App {
    pub fn setup() -> Self {
        let mut exit_code = None;
        let args = Args::parse();

        let base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
            log!("OPENROUTER_API_KEY is not set");
            exit_code = Some(1);
            "".to_string()
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
