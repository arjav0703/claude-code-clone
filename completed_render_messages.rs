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

    if text.is_empty() {
        let empty_message = Paragraph::new("No messages yet...")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        frame.render_widget(empty_message, area);
    } else {
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }
}
