use rspotify::model::CurrentlyPlayingContext;

use crate::app::search_results::SearchResults;

#[derive(Debug)]
pub enum SpotifyEvent {
    CurrentlyPlaying(Option<CurrentlyPlayingContext>),
    NewSearchResults(SearchResults),
}
