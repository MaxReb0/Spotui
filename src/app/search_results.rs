use color_eyre::eyre::Result;
use ratatui::widgets::Row;
use rspotify::model::{
    FullArtist, FullTrack, SearchResult, SearchType, SimplifiedAlbum, SimplifiedPlaylist,
    SimplifiedShow,
};

#[derive(Clone, Debug)]
pub struct SearchTypeResults<T: Clone> {
    pub items: Vec<T>,
    pub next_offset: u32,
    pub last_page_was_full: bool,
}

impl<T: Clone> SearchTypeResults<T> {
    pub fn from_first_page(items: Vec<T>) -> Self {
        let next_offset = items.len() as u32;
        let last_page_was_full = items.len() == 10;
        Self {
            items,
            next_offset,
            last_page_was_full,
        }
    }

    pub fn append_page(&mut self, new_items: Vec<T>) {
        self.last_page_was_full = new_items.len() == 10;
        self.next_offset += new_items.len() as u32;
        self.items.extend(new_items);
    }

    pub fn has_more(&self) -> bool {
        self.last_page_was_full
    }
}

#[derive(Clone, Debug, Default)]
pub struct SearchResults {
    pub tracks: Option<SearchTypeResults<FullTrack>>,
    pub albums: Option<SearchTypeResults<SimplifiedAlbum>>,
    pub artists: Option<SearchTypeResults<FullArtist>>,
    pub shows: Option<SearchTypeResults<SimplifiedShow>>,
    pub playlists: Option<SearchTypeResults<SimplifiedPlaylist>>,
}

impl SearchResults {
    //TODO: Add some documentation here.
    pub fn from_pages(pages: impl IntoIterator<Item = Result<SearchResult>>) -> Self {
        let mut results = SearchResults::default();
        for sr in pages.into_iter().flatten() {
            match sr {
                SearchResult::Tracks(page) => match results.tracks {
                    None => results.tracks = Some(SearchTypeResults::from_first_page(page.items)),
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Albums(page) => match results.albums {
                    None => results.albums = Some(SearchTypeResults::from_first_page(page.items)),
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Artists(page) => match results.artists {
                    None => results.artists = Some(SearchTypeResults::from_first_page(page.items)),
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Shows(page) => match results.shows {
                    None => results.shows = Some(SearchTypeResults::from_first_page(page.items)),
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Playlists(page) => match results.playlists {
                    None => {
                        results.playlists = Some(SearchTypeResults::from_first_page(page.items))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                _ => {}
            }
        }
        results
    }

    pub fn append_page(&mut self, sr: SearchResult) {
        match sr {
            SearchResult::Tracks(page) => {
                if let Some(ref mut r) = self.tracks {
                    r.append_page(page.items);
                }
            }
            SearchResult::Albums(page) => {
                if let Some(ref mut r) = self.albums {
                    r.append_page(page.items);
                }
            }
            SearchResult::Artists(page) => {
                if let Some(ref mut r) = self.artists {
                    r.append_page(page.items);
                }
            }
            SearchResult::Shows(page) => {
                if let Some(ref mut r) = self.shows {
                    r.append_page(page.items);
                }
            }
            SearchResult::Playlists(page) => {
                if let Some(ref mut r) = self.playlists {
                    r.append_page(page.items);
                }
            }
            _ => {}
        }
    }

    pub fn active(&self, search_type: SearchType) -> Option<ActiveResults<'_>> {
        match search_type {
            SearchType::Track => self.tracks.as_ref().map(ActiveResults::Tracks),
            SearchType::Album => self.albums.as_ref().map(ActiveResults::Albums),
            SearchType::Artist => self.artists.as_ref().map(ActiveResults::Artist),
            SearchType::Show => self.shows.as_ref().map(ActiveResults::Shows),
            SearchType::Playlist => self.playlists.as_ref().map(ActiveResults::Playlists),
            _ => None,
        }
    }
}

pub enum ActiveResults<'a> {
    Tracks(&'a SearchTypeResults<FullTrack>),
    Albums(&'a SearchTypeResults<SimplifiedAlbum>),
    Artist(&'a SearchTypeResults<FullArtist>),
    Shows(&'a SearchTypeResults<SimplifiedShow>),
    Playlists(&'a SearchTypeResults<SimplifiedPlaylist>),
}

impl<'a> ActiveResults<'a> {
    pub fn len(&self) -> usize {
        match self {
            Self::Tracks(r) => r.items.len(),
            Self::Albums(r) => r.items.len(),
            Self::Artist(r) => r.items.len(),
            Self::Shows(r) => r.items.len(),
            Self::Playlists(r) => r.items.len(),
        }
    }

    pub fn has_more(&self) -> bool {
        match self {
            Self::Tracks(r) => r.has_more(),
            Self::Albums(r) => r.has_more(),
            Self::Artist(r) => r.has_more(),
            Self::Shows(r) => r.has_more(),
            Self::Playlists(r) => r.has_more(),
        }
    }

    pub fn next_offset(&self) -> u32 {
        match self {
            Self::Tracks(r) => r.next_offset,
            Self::Albums(r) => r.next_offset,
            Self::Artist(r) => r.next_offset,
            Self::Shows(r) => r.next_offset,
            Self::Playlists(r) => r.next_offset,
        }
    }
    // TODO: This is a placeholder row creation. Will probably need to be fleshed out more.
    pub fn headers(&self) -> Row<'static> {
        match self {
            Self::Tracks(_) => Row::new(vec!["Track", "Artist", "Album"]),
            Self::Albums(_) => Row::new(vec!["Album", "Artist"]),
            Self::Artist(_) => Row::new(vec!["Artist", "Genres"]),
            Self::Shows(_) => Row::new(vec!["Show"]),
            Self::Playlists(_) => Row::new(vec!["Playlist"]),
        }
    }

    pub fn row_at(&self, idx: usize) -> Row<'_> {
        match self {
            Self::Tracks(r) => {
                let t = &r.items[idx];
                let artist = t.artists.first().map(|a| a.name.as_str()).unwrap_or("");
                Row::new(vec![
                    t.name.clone(),
                    artist.to_string(),
                    t.album.name.clone(),
                ])
            }
            // TODO: Need to implement this!
            Self::Albums(r) => {
                let a = &r.items[idx];
                let artist = a.artists.first().map(|art| art.name.as_str()).unwrap_or("");
                Row::new(vec![a.name.clone(), artist.to_string()])
            }
            Self::Artist(r) => {
                let a = &r.items[idx];
                let name = a.name.to_string();
                Row::new(vec![name, a.genres.join(", ")])
            }
            Self::Shows(r) => {
                let s = &r.items[idx];
                Row::new(vec![s.name.clone()])
            }
            Self::Playlists(r) => {
                let p = &r.items[idx];
                Row::new(vec![p.name.clone()])
            }
        }
    }
}
