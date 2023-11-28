use poem::{get, Route};

pub(crate) use fetch::fetch_and_save_torrents;
use route::torznab;

mod fetch;
mod route;

/// search api
pub fn search() -> Route {
    Route::new().nest("/torznab", get(torznab))
}
