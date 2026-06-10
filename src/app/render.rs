use super::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
    text::Text,
};
impl App<'_> {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(3)],
        )
        .split(area);

        self.render_messages(frame, layout[0]);
        self.render_input_area(frame, layout[1]);
    }

    pub fn render_dummy_widget(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        frame.render_widget(Text::from("hllo"), area);
    }

    pub fn render_input_area(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        frame.render_widget(self.text_area.widget(), area);
    }

    pub fn render_messages(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
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
