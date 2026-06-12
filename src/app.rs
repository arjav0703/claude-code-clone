mod event;
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

pub struct Model {
    pub name: String,
}

pub struct App<'a> {
    state: AppState,
    exit_code: Option<i32>,
    text_area: TextArea<'a>,
    active_area: ActiveArea,
    logs_scroll: u16,
}

pub struct AppState {
    pub model: Model,
    pub settings_state: Settings<'static>,
    pub messages: Arc<Mutex<Vec<Value>>>,
    pub message_sender: Sender<Vec<Value>>,
    pub message_receiver: Receiver<Value>,
    pub config: OpenAIConfig,
}

#[derive(PartialEq)]
pub enum ActiveArea {
    UserInput,
    LogsPopup,
    SettingsPopup,
}

pub struct Settings<'a> {
    selected_field: SettingsField,
    api_key_textarea: TextArea<'a>,
    model_textarea: TextArea<'a>,
    base_url_textarea: TextArea<'a>,
}

enum SettingsField {
    ApiKey,
    Model,
    BaseUrl,
}
