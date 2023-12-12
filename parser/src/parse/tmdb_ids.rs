use std::sync::OnceLock;

use anyhow::Result;

use database::entity::Torrent;
use tmdb::{Language, Tmdb};

fn tmdb() -> &'static Tmdb {
    static TMDB: OnceLock<Tmdb> = OnceLock::new();
    TMDB.get_or_init(Tmdb::default)
}

pub(super) async fn append_extra_ids(torrent: &mut Torrent) -> Result<()> {
    // tmdb id 为空，先解析 tmdb id
    if torrent.tmdb_id.is_none() {
        search_by_title(torrent).await?;
    }

    // 如果 tmdb id 存在，解析其他 id
    if let Some(tmdb_id) = torrent.tmdb_id {
        search_extra_ids(tmdb_id, torrent).await?;
    }

    Ok(())
}

async fn search_by_title(torrent: &mut Torrent) -> Result<()> {
    // 之前标题解析成功时查询
    if !torrent.title.is_empty() {
        let req = tmdb().search_tv(&torrent.title).execute().await?;

        match req.results.into_iter().next() {
            None => log::info!(
                "tmdb did not find any results related to `{}`",
                torrent.title
            ),
            Some(it) => torrent.tmdb_id = Some(it.id),
        }
    }

    Ok(())
}

async fn search_extra_ids(tmdb_id: i64, torrent: &mut Torrent) -> Result<()> {
    let req = tmdb()
        .tv_series_detail(tmdb_id)
        .append_to_response("external_ids")
        .language(Language::ZhCn)
        .execute()
        .await?;

    torrent.title = req.name;
    torrent.tvdb_id = req.external_ids.tvdb_id;
    torrent.imdb_id = req.external_ids.imdb_id.unwrap_or_default();

    Ok(())
}
