mod app;
mod log;
mod tools;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    ratatui::run(app::app)?;
    Ok(())
}
