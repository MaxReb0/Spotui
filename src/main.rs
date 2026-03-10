use color_eyre::eyre::Result;

mod app;
mod logging;
mod spotify;

use app::App;
use logging::initialize_logging;
use spotify::auth::auth;
use std::sync::Arc;

use crate::spotify::client::SpotifyClient;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    initialize_logging()?;
    tracing::debug!("Spotui has started!");

    let spotify_web_client = auth().await?;
    let mut spotify_client = SpotifyClient::new(spotify_web_client);
    spotify_client.set_username().await?;
    let mut app = App::new(Arc::new(spotify_client));

    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal).await;

    tracing::debug!("Spotui has finished!");
    ratatui::restore();
    app_result
}
