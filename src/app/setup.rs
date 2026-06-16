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
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

impl App<'_> {
    pub fn setup() -> Self {
        let settings = config::Config::builder()
            // .add_source(config::Environment::default())
            .add_source(config::File::with_name("config.toml").required(false))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap_or_default();

        let base_url: String = settings
            .get("OPENROUTER_API_BASE")
            .cloned()
            .unwrap_or_else(|| {
                log!("OPENROUTER_API_BASE is not set, using default");
                "https://openrouter.ai/api/v1".to_string()
            });

        let api_key: String = settings
            .get("OPENROUTER_API_KEY")
            .cloned()
            .unwrap_or_else(|| {
                log!("OPENROUTER_API_KEY is not set, using default");
                env::var("OPENROUTER_API_KEY").unwrap_or_default()
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
            name: settings.get("MODEL_NAME").cloned().unwrap_or_else(|| {
                log!("MODEL_NAME is not set, using default");
                "openrouter/owl-alpha".to_string()
            }),
        };

        let settings_state = Settings {
            base_url_textarea: TextArea::from([config.api_base().to_string()]),
            api_key_textarea: TextArea::from([config.api_key().expose_secret().to_string()]),
            model_textarea: TextArea::from([model.name.clone()]),
            selected_field: SettingsField::Model,
        };

        let chat_id = format!("chat_{}", small_uid::SmallUid::new());

        let state = {
            AppState {
                messages: Arc::new(Mutex::new([].to_vec())),
                message_sender: request_tx,
                message_receiver: response_rx,
                config: Arc::new(Mutex::new(config)),
                settings_state,
                model,
                chat_id,
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
