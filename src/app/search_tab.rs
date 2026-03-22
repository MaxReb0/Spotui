use rspotify::model::SearchType;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq)]
pub enum SearchTab {
    Tracks,
    Albums,
    Artist,
    Shows,
    Playlists,
}

impl SearchTab {
    pub fn search_type(self) -> SearchType {
        match self {
            SearchTab::Tracks => SearchType::Track,
            SearchTab::Albums => SearchType::Album,
            SearchTab::Artist => SearchType::Artist,
            SearchTab::Shows => SearchType::Show,
            SearchTab::Playlists => SearchType::Playlist,
        }
    }
}
