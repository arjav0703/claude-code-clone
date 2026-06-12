use async_openai::config::OpenAIConfig;
use ratatui::crossterm::{
    self,
    event::{self, KeyCode, KeyModifiers},
};
use serde_json::json;

use crate::{
    app::{ActiveArea, App, Model, SettingsField},
    log,
    util::Role,
};

impl App<'_> {
    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if self.active_area == ActiveArea::LogsPopup {
            match key.code {
                KeyCode::Up => {
                    self.logs_scroll = self.logs_scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.logs_scroll = self.logs_scroll.saturating_add(1);
                }
                _ => {}
            }
        }

        if self.active_area == ActiveArea::SettingsPopup {
            if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
                self.update_openrouter_config();
                return;
            }
            match self.state.settings_state.selected_field {
                SettingsField::ApiKey => match key.code {
                    KeyCode::Tab => {
                        self.state.settings_state.selected_field = SettingsField::Model;
                    }
                    _ => {
                        self.state.settings_state.api_key_textarea.input(key);
                    }
                },
                SettingsField::Model => match key.code {
                    KeyCode::Tab => {
                        self.state.settings_state.selected_field = SettingsField::BaseUrl;
                    }
                    _ => {
                        self.state.settings_state.model_textarea.input(key);
                    }
                },
                SettingsField::BaseUrl => match key.code {
                    KeyCode::Tab => {
                        self.state.settings_state.selected_field = SettingsField::ApiKey;
                    }
                    _ => {
                        self.state.settings_state.base_url_textarea.input(key);
                    }
                },
            }
        }

        match key.code {
            KeyCode::Enter if key.modifiers == KeyModifiers::ALT => {
                let app_state = &self.state;
                let mut msgs = app_state.messages.lock().unwrap();
                msgs.push(json!({
                    "role": Role::user,
                    "content": self.text_area.lines().join("\n")
                }));
                app_state.message_sender.send(msgs.clone()).unwrap();
                self.text_area.delete_line_by_end();
                self.text_area.delete_line_by_head();
                self.text_area.clear();
            }

            KeyCode::Char('.') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_active_area(ActiveArea::SettingsPopup);
            }

            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_active_area(ActiveArea::LogsPopup);
            }

            KeyCode::Char('q') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.exit_code = Some(0);
            }
            _ if self.active_area == ActiveArea::UserInput => {
                self.text_area.input(key);
            }
            _ => {}
        }
    }

    fn toggle_active_area(&mut self, area: ActiveArea) {
        if self.active_area == area {
            self.active_area = ActiveArea::UserInput;
        } else {
            self.active_area = area;
        }
    }

    fn update_openrouter_config(&mut self) {
        let base_url = self.state.settings_state.base_url_textarea.lines().join("");
        let api_key = self.state.settings_state.api_key_textarea.lines().join("");
        let model = self.state.settings_state.model_textarea.lines().join("");

        self.state.model = Model {
            name: model.clone(),
        };

        log!(
            "Updating OpenRouter config: base_url={}, api_key={}",
            base_url,
            api_key,
        );

        *self.state.config.lock().unwrap() = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);
    }
}
