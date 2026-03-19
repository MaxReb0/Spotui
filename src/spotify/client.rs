use async_trait::async_trait;
use color_eyre::eyre::Result;
use rspotify::{
    AuthCodePkceSpotify,
    model::{CurrentlyPlayingContext, SearchResult, SearchType},
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
}

#[async_trait]
pub trait SpotifyApi: Send + Sync + Debug {
    fn username(&self) -> Option<&str>;
    async fn current_playback(&self) -> Result<Option<CurrentlyPlayingContext>>;
    async fn search(
        &self,
        search_type: SearchType,
        offset: u32,
        query: &str,
    ) -> Result<SearchResult>;
}

#[async_trait]
impl SpotifyApi for SpotifyClient {
    fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    async fn current_playback(&self) -> Result<Option<CurrentlyPlayingContext>> {
        Ok(self.spotify_web_client.current_user_playing_item().await?)
    }

    async fn search(
        &self,
        search_type: SearchType,
        offset: u32,
        query: &str,
    ) -> Result<SearchResult> {
        Ok(self
            .spotify_web_client
            .search(query, search_type, None, None, Some(10), Some(offset))
            .await?)
    }
}
