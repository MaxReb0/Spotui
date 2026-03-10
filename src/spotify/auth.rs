use std::collections::HashSet;

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::OAuthClient, scopes};

use crate::trace_dbg;

pub async fn auth() -> Result<AuthCodePkceSpotify> {
    // This reads the RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET from env, unless specified to
    // use .env file.
    trace_dbg!("Beginning rspotify authentication");
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(get_scopes()).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    // Configure and create the location for saving the token.
    let project_dirs = ProjectDirs::from("", "", "spotui").unwrap();
    let cache_path = project_dirs.config_dir().join("token.json");

    std::fs::create_dir_all(project_dirs.config_dir())?;
    spotify.config.cache_path = cache_path;
    spotify.config.token_cached = true;

    let url = spotify.get_authorize_url(None).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();
    trace_dbg!("Authentication with rspotify successful");

    Ok(spotify)
}

// Simple function for getting necessary scopes for spotify API.
fn get_scopes() -> HashSet<String> {
    scopes!(
        "user-read-playback-state",
        "user-read-private",
        "user-read-email"
    )
}
