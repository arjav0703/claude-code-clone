use std::fs;

use super::App;
use crate::app::{ActiveArea, SettingsField};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Borders, Clear, List, Paragraph};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Stylize,
    text::Text,
    widgets::Block,
};
use tui_big_text::{BigText, PixelSize};

impl App<'_> {
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();

        let main_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(area);

        self.render_history_list(frame, main_layout[1]);

        let chat_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(7)],
        )
        .split(main_layout[0]);

        self.render_messages(frame, chat_layout[0]);
        self.render_input_area(frame, chat_layout[1]);

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
                Text::from("CTRL + Q to quit \nALT + ENTER to send message \nCTRL + . to open settings \nCTRL + L to view logs\nCTRL + H to access chat history\nCTRL + N to create a new chat").white(),
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

    fn render_settings_popup(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.state.settings_state.model_textarea.set_block(
            Block::default()
                .title(" Model ")
                .borders(Borders::ALL)
                .border_style(self.get_active_settings_style(SettingsField::Model)),
        );

        self.state.settings_state.base_url_textarea.set_block(
            Block::default()
                .title(" API URL ")
                .borders(Borders::ALL)
                .border_style(self.get_active_settings_style(SettingsField::BaseUrl)),
        );

        self.state.settings_state.api_key_textarea.set_block(
            Block::default()
                .title(" API Key ")
                .borders(Borders::ALL)
                .border_style(self.get_active_settings_style(SettingsField::ApiKey)),
        );

        let popup_block = Block::default()
            .title(" Settings ")
            .borders(Borders::ALL)
            .border_style(Style::new().magenta());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block.clone(), area);

        let inner = popup_block.inner(area);

        let horizontal = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ],
        );

        let centered = horizontal.split(inner)[1];

        let vertical = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ],
        );

        let chunks = vertical.split(centered);

        // frame.render_widget(api_key_display, chunks[0]);
        frame.render_widget(&self.state.settings_state.model_textarea, chunks[0]);
        frame.render_widget(&self.state.settings_state.base_url_textarea, chunks[1]);

        frame.render_widget(&self.state.settings_state.api_key_textarea, chunks[2]);
    }

    fn get_active_settings_style(&self, field: SettingsField) -> Style {
        if self.state.settings_state.selected_field == field {
            Style::new().magenta()
        } else {
            Style::new()
        }
    }

    fn render_history_list(&mut self, frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
        let title = Line::from_iter([
            Span::from("Chat History").bold(),
            Span::from("(Press arrow keys to navigate, enter to select conversation)")
                .italic()
                .dim(),
        ]);

        let history_list = self.get_history_list();
        let list = List::new(history_list)
            .block(
                Block::bordered()
                    .title(title.clone())
                    .style(Style::new().red()),
            )
            .highlight_style(Style::new().yellow().bold())
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.state.history_list_state);
    }
}
