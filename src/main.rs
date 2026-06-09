use crate::app::App;

mod app;
mod log;
mod tools;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let terminal = ratatui::init();

    let mut app = App::setup();
    let exit_code = app.run(terminal).await?;

    ratatui::restore();

    std::process::exit(exit_code);
}
