use rspotify::model::CurrentlyPlayingContext;

use crate::app::search_results::SearchResults;
use rspotify::model::SearchResult;

#[derive(Debug)]
pub enum SpotifyEvent {
    CurrentlyPlaying(Option<CurrentlyPlayingContext>),
    NewSearchResults(SearchResults),
    AppendSearchResults(SearchResult),
}
