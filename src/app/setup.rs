use async_openai::config::OpenAIConfig;
use ratatui_textarea::TextArea;
use serde_json::Value;
use std::{
    env,
    sync::{Arc, Mutex},
};

use super::App;
use crate::{
    app::{ActiveArea, AppState},
    log,
};

impl App<'_> {
    pub fn setup() -> Self {
        let mut exit_code = None;

        let base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
            log!("OPENROUTER_API_KEY is not set");
            // exit_code = Some(1);
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

        let state = {
            AppState {
                messages: Arc::new(Mutex::new([].to_vec())),
                message_sender: request_tx,
                message_receiver: response_rx,
                config,
            }
        };

        state.listen_for_messages(request_rx, response_tx.clone());

        let text_area = TextArea::default();

        App {
            state,
            exit_code,
            text_area,
            active_area: ActiveArea::UserInput,
            logs_scroll: 0,
        }
    }
}
