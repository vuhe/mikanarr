use poem::web::{Json, Query};
use poem::{delete, get, handler, post, put, Route};
use serde::Deserialize;

use database::entity::{Indexer, IndexerSearch};

use super::ResultResp;

pub(super) fn route() -> Route {
    Route::new()
        .nest("/list", get(list))
        .nest("/add", post(add))
        .nest("/modify", put(modify))
        .nest("/delete", delete(delete_one))
        .nest("/truncate", delete(truncate))
}

#[handler]
async fn list(Query(param): Query<IndexerSearch>) -> Json<ResultResp<Vec<Indexer>>> {
    let list = Indexer::find_by_param(param).await;
    Json(ResultResp::from(list))
}

#[handler]
async fn add(Json(indexer): Json<Indexer>) -> Json<ResultResp<()>> {
    let result = indexer.add().await;
    Json(ResultResp::from(result))
}

#[handler]
async fn modify(Json(indexer): Json<Indexer>) -> Json<ResultResp<()>> {
    let result = indexer.modify().await;
    Json(ResultResp::from(result))
}

#[derive(Deserialize)]
struct DeleteId {
    id: u32,
}

#[handler]
async fn delete_one(Json(param): Json<DeleteId>) -> Json<ResultResp<()>> {
    let result = Indexer::delete_by_id(param.id).await;
    Json(ResultResp::from(result))
}

#[handler]
async fn truncate() -> Json<ResultResp<()>> {
    let result = Indexer::delete_all().await;
    Json(ResultResp::from(result))
}
