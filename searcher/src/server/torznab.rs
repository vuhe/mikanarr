use poem::http::header::CONTENT_TYPE;
use poem::http::HeaderValue;
use poem::web::Query;
use poem::{handler, IntoResponse, Response, Result as PoemResult};
use serde::Deserialize;

use database::entity::Torrent;
use parser::ParseTorrent;

use crate::rss::new_torznab_rss;

// language=XML
static CAPS_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<caps>
  <server version="1.0" title="mikanarr" source="https://mikanani.me" />
  <limits max="50" default="20" />
  <searching>
    <search available="yes" supportedParams="q" />
    <tv-search available="yes" supportedParams="q,tvdbid,season" />
    <movie-search available="yes" supportedParams="q,imdbid" />
    <audio-search available="no" supportedParams="q" />
    <book-search available="no" supportedParams="q" />
  </searching>
  <categories>
    <category id="1000" name="TV" />
    <category id="2000" name="Movies" />
  </categories>
</caps>"#;

#[handler]
pub(super) async fn torznab(Query(mut param): Query<SearchParam>) -> PoemResult<impl IntoResponse> {
    // cap 请求直接返回
    if matches!(param.function, SearchType::Caps) {
        return Ok(caps_resp());
    }

    // 不支持的 cat 查询，直接返回空信息，仅支持为空或者默认 cat 值 1000, 2000
    if !matches!(param.categories.as_str(), "" | "1000" | "2000") {
        return Ok(empty_resp()?);
    }

    // 存在不匹配的查询类别，直接返回空信息
    if !is_matched_search_type(&mut param) {
        return Ok(empty_resp()?);
    }

    // 查询并返回
    Ok(channel_resp(database_search(&param).await?)?)
}

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
struct SearchParam {
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

/// 类型查询是否匹配
fn is_matched_search_type(param: &mut SearchParam) -> bool {
    let categories = param
        .categories
        .split(",")
        .filter(|it| !it.is_empty())
        .collect::<Vec<_>>();

    let is_matched = match &categories {
        c if c.is_empty() => true,
        c if c.contains(&"1000") && matches!(param.function, SearchType::TVSearch) => true,
        c if c.contains(&"2000") && matches!(param.function, SearchType::MovieSearch) => true,
        _ if matches!(param.function, SearchType::Search) => true,
        _ => false,
    };

    param.is_movie = match categories {
        _ if matches!(param.function, SearchType::TVSearch) => Some(false),
        _ if matches!(param.function, SearchType::MovieSearch) => Some(true),
        c if c.contains(&"1000") => Some(false),
        c if c.contains(&"2000") => Some(true),
        _ => None,
    };

    is_matched
}

/// 数据库搜索
async fn database_search(param: &SearchParam) -> anyhow::Result<Vec<Torrent>> {
    let mut torrent = Torrent::default();
    torrent.name = param.query.clone();
    torrent.imdb_id = param.imdb_id.as_deref().unwrap_or_default().to_owned();
    torrent.tvdb_id = param.tvdb_id.as_deref().and_then(|it| it.parse().ok());
    torrent.season = param.season.as_deref().unwrap_or_default().to_owned();
    torrent.try_parse_detail().await.ok();

    let imdb = match torrent.imdb_id.as_str() {
        "" => None,
        s => Some(s),
    };
    let se = match torrent.season.as_str() {
        "" => None,
        s => Some(s),
    };

    Torrent::find_by_query(imdb, torrent.tvdb_id, se).await
}

/// torznab caps 响应
fn caps_resp() -> Response {
    let mut resp = Response::from(CAPS_XML);
    let xml_type = HeaderValue::from_static("application/xml; charset=utf-8");
    resp.headers_mut().insert(CONTENT_TYPE, xml_type);
    resp
}

/// 空 torznab rss
fn empty_resp() -> anyhow::Result<Response> {
    channel_resp(Vec::default())
}

/// 将 torrent 转换为 torznab rss
fn channel_resp(torrents: Vec<Torrent>) -> anyhow::Result<Response> {
    let mut channel = new_torznab_rss();
    channel.set_torznab_items(torrents);
    let mut response = Response::from(channel.into_bytes()?);
    let rss_type = HeaderValue::from_static("application/rss+xml; charset=utf-8");
    response.headers_mut().insert(CONTENT_TYPE, rss_type);
    Ok(response)
}
