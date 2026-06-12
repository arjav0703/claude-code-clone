use super::App;
use crate::{
    app::{ActiveArea, AppState, Model, Settings, SettingsField},
    log,
};
use async_openai::config::{Config, OpenAIConfig};
use ratatui_textarea::TextArea;
use secrecy::ExposeSecret;
use serde_json::Value;
use std::{
    env,
    sync::{Arc, Mutex},
};

impl App<'_> {
    pub fn setup() -> Self {
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

        let model = Model {
            name: "openrouter/owl-alpha".to_string(),
        };

        let settings_state = Settings {
            base_url_textarea: TextArea::from([config.api_base().to_string()]),
            api_key_textarea: TextArea::from([config.api_key().expose_secret().to_string()]),
            model_textarea: TextArea::from([model.name.clone()]),
            selected_field: SettingsField::Model,
        };

        let state = {
            AppState {
                messages: Arc::new(Mutex::new([].to_vec())),
                message_sender: request_tx,
                message_receiver: response_rx,
                config,
                settings_state,
                model,
            }
        };

        state.listen_for_messages(request_rx, response_tx.clone());

        let text_area = TextArea::default();

        App {
            state,
            exit_code: None,
            text_area,
            active_area: ActiveArea::UserInput,
            logs_scroll: 0,
        }
    }
}
