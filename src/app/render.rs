use ratatui::{Frame, text::Text};

use super::App;

impl App {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }

    pub fn render_dummy_widget(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }
}
