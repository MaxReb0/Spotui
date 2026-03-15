use rspotify::model::{CurrentlyPlayingContext, SearchResult};

#[derive(Debug, PartialEq)]
pub enum SpotifyEvent {
    CurrentlyPlaying(Option<CurrentlyPlayingContext>),
    SearchResults(SearchResult),
}
