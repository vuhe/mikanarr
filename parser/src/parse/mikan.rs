use anyhow::{ensure, Context, Result};
use lazy_regex::regex_captures;
use once_cell::sync::Lazy;
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

use database::entity::{MikanTmdb, Torrent};

static CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().redirect(Policy::none()).build().unwrap());

pub(super) async fn mikan_direct_mapping(torrent: &mut Torrent) {
    // hash 值为空时无法映射，不处理
    if torrent.id.is_empty() {
        return;
    }

    let bangumi_id = match parse_mikan_bangumi_id(&torrent.id).await {
        Ok(it) => it,
        Err(e) => {
            tracing::debug!("{e:#?}");
            tracing::warn!("{e}");
            return;
        }
    };

    let Some(tmdb) = MikanTmdb::find_by_bangumi_id(&bangumi_id).await else {
        tracing::info!("mikan bangumi({bangumi_id}) no matching data, use other parser.");
        return;
    };

    torrent.tmdb_id = Some(tmdb.tmdb_id);
    torrent.is_movie = tmdb.is_movie;
    torrent.season = tmdb.season;
}

async fn parse_mikan_bangumi_id(hash: &str) -> Result<String> {
    let url = format!("https://mikanani.me/Home/Episode/{}", hash.to_lowercase());
    let resp = CLIENT.get(url).send().await?;
    let resp = resp.error_for_status()?;
    ensure!(
        resp.status() != StatusCode::MOVED_PERMANENTLY && resp.status() != StatusCode::FOUND,
        "mikan episode not found"
    );
    let text = resp.text().await?;

    let html = Html::parse_document(&text);
    // language=JQuery-CSS
    static SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse(r#"a[href^="/Home/Bangumi/"]"#).unwrap());
    let element = html.select(&SELECTOR).next();
    let element = element.context("can't find mikan bangumi id")?;
    let href = element.value().attr("href").unwrap_or_default();
    let (_, bangumi, _group) = regex_captures!(r"^/Home/Bangumi/(\d+)(?:#(\d+))?$", href)
        .context("can't parse mikan bangumi id")?;
    Ok(bangumi.to_owned())
}

#[cfg(test)]
mod test {
    use tokio_test::block_on;

    use super::*;

    #[test]
    fn test_parse_bangumi_id() {
        let req = parse_mikan_bangumi_id("c3d6743d88645c8d85fd4be637b44644ae5ad5a5");
        let id = block_on(req).unwrap();
        let id = id.as_ref().map(|it| it.as_str());
        assert!(matches!(id, Ok("3172")));
    }
}
