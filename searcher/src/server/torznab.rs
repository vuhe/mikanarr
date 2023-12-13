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
    if let Some(resp) = param.caps() {
        return Ok(resp);
    }
    // 不支持的 cat，直接返回空信息
    if let Some(resp) = param.unsupported_cat()? {
        return Ok(resp);
    }
    // 存在不匹配的查询类别，直接返回空信息
    if let Some(resp) = param.unmatched_search_type()? {
        return Ok(resp);
    }

    // 查询并返回
    Ok(param.search_result().await?)
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

impl SearchParam {
    /// cap 搜索响应
    fn caps(&self) -> Option<Response> {
        match self.function {
            SearchType::Caps => Some(caps_resp()),
            _ => None,
        }
    }

    /// 不支持的 cat 查询，仅支持为空或者默认 cat 值 1000, 2000
    fn unsupported_cat(&self) -> anyhow::Result<Option<Response>> {
        match self.categories.as_str() {
            "" | "1000" | "2000" => Ok(None),
            _ => Ok(Some(empty_resp()?)),
        }
    }

    /// 不匹配的类型查询
    fn unmatched_search_type(&mut self) -> anyhow::Result<Option<Response>> {
        let categories = self
            .categories
            .split(",")
            .filter(|it| !it.is_empty())
            .collect::<Vec<_>>();

        let is_matched = match &categories {
            c if c.is_empty() => true,
            c if c.contains(&"1000") && matches!(self.function, SearchType::TVSearch) => true,
            c if c.contains(&"2000") && matches!(self.function, SearchType::MovieSearch) => true,
            _ if matches!(self.function, SearchType::Search) => true,
            _ => false,
        };

        self.is_movie = match categories {
            _ if matches!(self.function, SearchType::TVSearch) => Some(false),
            _ if matches!(self.function, SearchType::MovieSearch) => Some(true),
            c if c.contains(&"1000") => Some(false),
            c if c.contains(&"2000") => Some(true),
            _ => None,
        };

        match is_matched {
            true => Ok(None),
            false => Ok(Some(empty_resp()?)),
        }
    }

    /// 执行搜索，返回结果
    async fn search_result(&self) -> anyhow::Result<Response> {
        channel_resp(self.database_search().await?)
    }

    /// 数据库搜索
    async fn database_search(&self) -> anyhow::Result<Vec<Torrent>> {
        let mut torrent = Torrent::default();
        torrent.name = self.query.clone();
        torrent.imdb_id = self.imdb_id.as_deref().unwrap_or_default().to_owned();
        torrent.tvdb_id = self.tvdb_id.as_deref().and_then(|it| it.parse().ok());
        torrent.season = self.season.as_deref().unwrap_or_default().to_owned();
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
