use anyhow::Result;
use poem::endpoint::StaticFilesEndpoint;
use poem::{get, post, EndpointExt, Route};
use serde::Serialize;

mod auth;
mod indexer;
mod setting;

pub(crate) fn route() -> Route {
    Route::new()
        .nest("/api", api_route().around(auth::auth))
        .nest("/", static_files())
}

fn api_route() -> Route {
    Route::new()
        .nest("/login", post(auth::login))
        .nest("/username", get(auth::username))
        .nest("/search", searcher::search())
        .nest("/indexer", indexer::route())
        .nest("/setting", setting::route())
}

fn static_files() -> StaticFilesEndpoint {
    let res_data_path = "./resources/dist";
    log::info!("webui dir path: `{}`", res_data_path);
    StaticFilesEndpoint::new(res_data_path)
        .show_files_listing()
        .index_file("index.html")
        .fallback_to_index()
}

#[derive(Serialize)]
struct ResultResp<T: Serialize> {
    code: u16,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> From<u16> for ResultResp<T> {
    fn from(value: u16) -> Self {
        ResultResp {
            code: value,
            message: None,
            data: None,
        }
    }
}

impl<T: Serialize> From<Result<T>> for ResultResp<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(data) => ResultResp {
                code: 200,
                message: None,
                data: Some(data),
            },
            Err(e) => ResultResp {
                code: 500,
                message: Some(e.to_string()),
                data: None,
            },
        }
    }
}
