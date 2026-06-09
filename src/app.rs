use async_openai::config::OpenAIConfig;
use clap::Parser;
use ratatui::{DefaultTerminal, Frame, crossterm};
use serde_json::{Value, json};
use std::{
    env::{self},
    process,
    sync::{Arc, Mutex},
};

mod run;
mod state;
use state::AppState;

use crate::{
    log,
    tools::{handle_bash, handle_read_file, handle_write_file},
};

pub use run::App;

use crate::util::Role;

pub fn app(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
