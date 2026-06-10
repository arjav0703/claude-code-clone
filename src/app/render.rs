use ratatui::{Frame, text::Text};

use super::App;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_messages(frame);
    }

    pub fn render_dummy_widget(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }

    pub fn render_input_area(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(&ratatui_textarea::TextArea::default(), area);
    }

    pub fn render_messages(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let messages = self.state.messages.lock().unwrap();
        let text = messages
            .iter()
            .map(|msg| {
                let role = msg["role"].as_str().unwrap_or("unknown");
                let content = msg["content"].as_str().unwrap_or("");
                format!("{}: {}", role, content)
            })
            .collect::<Vec<_>>()
            .join("\n");
        frame.render_widget(Text::from(text), area);
    }
}
