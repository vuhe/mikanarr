use poem::web::Query;
use poem::{handler, IntoResponse, Result};

use crate::torznab::SearchParam;

#[handler]
pub(super) async fn torznab(res: Result<Query<SearchParam>>) -> Result<impl IntoResponse> {
    Ok(res?.0.search().await?)
}
