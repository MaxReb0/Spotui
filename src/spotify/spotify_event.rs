use rspotify::model::CurrentlyPlayingContext;

#[derive(Debug, PartialEq)]
pub enum SpotifyEvent {
    CurrentlyPlaying(Option<CurrentlyPlayingContext>),
}
