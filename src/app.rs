mod run;
mod setup;
mod state;
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

use async_openai::config::OpenAIConfig;
use serde_json::Value;
mod render;

use ratatui_textarea::TextArea;
pub struct App<'a> {
    state: AppState,
    exit_code: Option<i32>,
    text_area: TextArea<'a>,
}

pub struct AppState {
    pub messages: Arc<Mutex<Vec<Value>>>,
    pub message_sender: Sender<Vec<Value>>,
    pub message_receiver: Receiver<Value>,
    pub config: OpenAIConfig,
}
