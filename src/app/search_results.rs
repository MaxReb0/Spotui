use color_eyre::eyre::Result;
use rspotify::model::{
    FullArtist, FullTrack, SearchResult, SearchType, SimplifiedAlbum, SimplifiedPlaylist,
    SimplifiedShow,
};

#[derive(Clone, Debug)]
pub struct SearchTypeResults<T: Clone> {
    pub items: Vec<T>,
    pub total: u32,
    pub next_offset: u32,
}

impl<T: Clone> SearchTypeResults<T> {
    pub fn from_first_page(items: Vec<T>, total: u32) -> Self {
        let next_offset = items.len() as u32;
        Self {
            items,
            total,
            next_offset,
        }
    }

    pub fn append_page(&mut self, new_items: Vec<T>) {
        self.next_offset += new_items.len() as u32;
        self.items.extend(new_items);
    }

    pub fn has_more(&self) -> bool {
        (self.items.len() as u32) < self.total
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
                    None => {
                        results.tracks =
                            Some(SearchTypeResults::from_first_page(page.items, page.total))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Albums(page) => match results.albums {
                    None => {
                        results.albums =
                            Some(SearchTypeResults::from_first_page(page.items, page.total))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Artists(page) => match results.artists {
                    None => {
                        results.artists =
                            Some(SearchTypeResults::from_first_page(page.items, page.total))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Shows(page) => match results.shows {
                    None => {
                        results.shows =
                            Some(SearchTypeResults::from_first_page(page.items, page.total))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                SearchResult::Playlists(page) => match results.playlists {
                    None => {
                        results.playlists =
                            Some(SearchTypeResults::from_first_page(page.items, page.total))
                    }
                    Some(ref mut r) => r.append_page(page.items),
                },
                _ => {}
            }
        }
        results
    }
}
