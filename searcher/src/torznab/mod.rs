use anyhow::Result;
use poem::Response;
use serde::Deserialize;

mod handler;
mod response;

#[derive(Deserialize)]
enum SearchType {
    #[serde(rename = "caps")]
    Caps,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "tvsearch")]
    TVSearch,
    #[serde(rename = "movie")]
    MovieSearch,
}

#[derive(Deserialize)]
pub(crate) struct SearchParam {
    #[serde(rename = "t")]
    function: SearchType,
    #[serde(rename = "q", default)]
    query: String,
    #[serde(rename = "cat", default)]
    categories: String,
    #[serde(rename = "imdbid")]
    imdb_id: Option<String>,
    #[serde(rename = "tvdbid")]
    tvdb_id: Option<String>,
    season: Option<String>,
    /// 是否为电影，此参数作为处理其他参数时辅助参数
    #[serde(skip)]
    is_movie: Option<bool>,
}

impl SearchParam {
    /// 根据参数执行搜索
    pub(crate) async fn search(mut self) -> Result<Response> {
        // cap 请求直接返回
        if let Some(resp) = self.caps() {
            return Ok(resp);
        }
        // 不支持的 cat，直接返回空信息
        if let Some(resp) = self.unsupported_cat()? {
            return Ok(resp);
        }
        // 存在不匹配的查询类别，直接返回空信息
        if let Some(resp) = self.unmatched_search_type()? {
            return Ok(resp);
        }

        // 查询并返回
        self.search_result().await
    }
}
