use color_eyre::eyre::Result;

mod app;
mod logging;
mod spotify;

use app::App;
use logging::initialize_logging;
use spotify::auth::auth;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    initialize_logging()?;
    tracing::debug!("Spotui has started!");
    auth().await?;
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)?;
    tracing::debug!("Spotui has finished!");
    ratatui::restore();
    Ok(())
}
