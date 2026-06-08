mod app;
mod tools;
mod util;

use ratatui::{DefaultTerminal, Frame};
use tools::{handle_bash, handle_read_file, handle_write_file};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    ratatui::run(app::app)?;
    Ok(())
}
