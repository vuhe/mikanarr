use poem::{get, Route};

pub(crate) use fetch::fetch_and_save_torrents;

mod fetch;
mod torznab;

/// search api
pub fn search() -> Route {
    Route::new().nest("/torznab", get(torznab::torznab))
}
