use anyhow::Result;
use poem::Response;

use database::entity::Torrent;
use parser::ParseTorrent;

use super::response::{caps_resp, channel_resp, empty_resp};
use super::{SearchParam, SearchType};

impl SearchParam {
    /// cap 搜索响应
    pub(super) fn caps(&self) -> Option<Response> {
        match self.function {
            SearchType::Caps => Some(caps_resp()),
            _ => None,
        }
    }

    /// 不支持的 cat 查询，仅支持为空或者默认 cat 值 1000, 2000
    pub(super) fn unsupported_cat(&self) -> Result<Option<Response>> {
        match self.categories.as_str() {
            "" | "1000" | "2000" => Ok(None),
            _ => Ok(Some(empty_resp()?)),
        }
    }

    /// 不匹配的类型查询
    pub(super) fn unmatched_search_type(&mut self) -> Result<Option<Response>> {
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
    pub(super) async fn search_result(&self) -> Result<Response> {
        channel_resp(self.database_search().await?)
    }

    /// 数据库搜索
    async fn database_search(&self) -> Result<Vec<Torrent>> {
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
