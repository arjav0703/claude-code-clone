use ratatui::crossterm::{
    self,
    event::{self, KeyCode, KeyModifiers},
};
use serde_json::json;

use crate::{
    app::{ActiveArea, App},
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

            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if matches!(self.active_area, ActiveArea::LogsPopup) {
                    self.active_area = ActiveArea::UserInput;
                } else {
                    self.active_area = ActiveArea::LogsPopup;
                }
            }

            KeyCode::Char('q') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.exit_code = Some(0);
            }
            _ => {
                self.text_area.input(key);
            }
        }
    }
}
