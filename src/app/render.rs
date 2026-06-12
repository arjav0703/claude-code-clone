use std::fs;

use super::App;
use crate::app::ActiveArea;
use async_openai::config::Config;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Stylize,
    text::Text,
    widgets::Block,
};
use secrecy::ExposeSecret;
use tui_big_text::{BigText, PixelSize};

impl App<'_> {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(7)],
        )
        .split(area);

        self.render_messages(frame, main_layout[0]);
        self.render_input_area(frame, main_layout[1]);

        match self.active_area {
            ActiveArea::LogsPopup => {
                let popup_layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ],
                )
                .split(area);
                self.render_logs_popup(frame, popup_layout[1]);
            }
            ActiveArea::SettingsPopup => {
                let popup_layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ],
                )
                .split(area);
                self.render_settings_popup(frame, popup_layout[1]);
            }
            _ => {}
        }
    }

    fn render_input_area(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        let _ = &self.text_area.set_block(
            Block::bordered()
                .fg(ratatui::style::Color::Yellow)
                .not_underlined()
                .title("Input"),
        );

        frame.render_widget(&self.text_area, area);
    }

    fn render_logs_popup(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        let logs = fs::read_to_string("log.txt")
            .unwrap_or_else(|_| "No logs available.".to_string())
            .split("\n")
            .map(|line| Line::from(line.to_string()))
            .collect::<Vec<Line>>();

        let paragraph = Paragraph::new(logs)
            .block(
                Block::bordered()
                    .title("Logs")
                    .fg(ratatui::style::Color::Blue)
                    .bg(ratatui::style::Color::Black),
            )
            .scroll((self.logs_scroll, 0));

        let layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ],
        );

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, layout.split(area)[1]);
    }

    fn render_messages(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        let messages = self.state.messages.lock().unwrap();

        let mut lines = Vec::new();

        for msg in messages.iter() {
            let line = Line::from(Vec::from([
                Span::styled(
                    format!("{}: ", msg["role"].as_str().unwrap_or_default()),
                    Style::new().blue(),
                ),
                Span::styled(
                    msg["content"].as_str().unwrap_or("unknown"),
                    Style::new().white(),
                ),
            ]));
            lines.push(line);
        }

        if lines.is_empty() {
            let big_text = BigText::builder()
                .pixel_size(PixelSize::Full)
                .style(Style::new().blue())
                .lines(vec![
                    "Start".red().into(),
                    "Typing...".white().into(),
                    "~~~~~".into(),
                ])
                .build();

            let layout = Layout::new(
                Direction::Vertical,
                [Constraint::Min(0), Constraint::Min(0)],
            )
            .split(area);
            frame.render_widget(big_text, layout[0]);
            frame.render_widget(
                Text::from("CTRL+Q to quit \nALT + ENTER to send message \nCTRL + . to open settings \nCTRL + L to view logs").white(),
                layout[1],
            );
        } else {
            let paragraph = Paragraph::new(lines).block(
                Block::bordered()
                    .title("Messages")
                    .fg(ratatui::style::Color::Green),
            );
            frame.render_widget(paragraph, area);
        }
    }

    fn render_settings_popup(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        let api_key = self.state.config.api_key().expose_secret().to_string();
        let api_key_display = if api_key.is_empty() {
            "API Key not set".to_string()
        } else {
            format!(
                "API key: {}{}",
                "*".repeat(api_key.len() - 5),
                &api_key[api_key.len() - 5..]
            )
        };

        let url = self.state.config.api_base().to_string();

        let paragraph = Paragraph::new(format!("{api_key_display} \n URL: {url}")).block(
            Block::bordered()
                .title("Settings")
                .fg(ratatui::style::Color::Magenta)
                .bg(ratatui::style::Color::Black),
        );

        let layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ],
        );

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, layout.split(area)[1]);
    }
}
