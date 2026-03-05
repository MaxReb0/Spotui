use color_eyre::eyre::Result;
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, prelude::OAuthClient, scopes};

pub async fn auth() -> Result<()> {
    // They say I could use any logger here, not sure what to do for that.

    // This reads the RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET from env, unless specified to
    // use .env file.
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = spotify.get_authorize_url(None).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();

    let history = spotify.current_playback(None, None::<Vec<_>>).await;
    println!("Response: {history:?}");

    // let prev_token = spotify.token.lock().await.unwrap();
    // let spotify = AuthCodePkceSpotify::new(creds, oauth)
    Ok(())
}
