use async_trait::async_trait;
use color_eyre::eyre::Result;
use rspotify::{
    AuthCodePkceSpotify,
    model::{CurrentlyPlayingContext, SearchResult},
    prelude::{BaseClient, OAuthClient},
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct SpotifyClient {
    username: Option<String>,
    spotify_web_client: AuthCodePkceSpotify,
}

impl SpotifyClient {
    pub fn new(spotify_web_client: AuthCodePkceSpotify) -> Self {
        Self {
            username: None,
            spotify_web_client,
        }
    }

    pub async fn set_username(&mut self) -> Result<()> {
        let user = self.spotify_web_client.current_user().await?;
        self.username = Some(user.display_name.unwrap_or(user.id.to_string()));
        Ok(())
    }

    pub async fn search(&mut self, query: &str) -> Result<SearchResult> {
        // TODO: Add usage for this.
        Ok(self
            .spotify_web_client
            .search(
                query,
                rspotify::model::SearchType::Track,
                None,
                None,
                None,
                None,
            )
            .await?)
    }
}

#[async_trait]
pub trait SpotifyApi: Send + Sync + Debug {
    fn username(&self) -> Option<&str>;
    async fn current_playback(&self) -> Result<Option<CurrentlyPlayingContext>>;
}

#[async_trait]
impl SpotifyApi for SpotifyClient {
    fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    async fn current_playback(&self) -> Result<Option<CurrentlyPlayingContext>> {
        Ok(self.spotify_web_client.current_user_playing_item().await?)
    }
}
