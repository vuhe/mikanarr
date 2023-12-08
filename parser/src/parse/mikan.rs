use std::sync::OnceLock;

use anyhow::{ensure, Context, Result};
use lazy_regex::regex_captures;
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

use database::entity::{MikanTmdb, Torrent};

fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| Client::builder().redirect(Policy::none()).build().unwrap())
}

pub(super) async fn mikan_direct_mapping(torrent: &mut Torrent) {
    // hash 值为空时无法映射，不处理
    if torrent.id.is_empty() {
        return;
    }

    let bangumi_id = match parse_mikan_bangumi_id(&torrent.id).await {
        Ok(it) => it,
        Err(e) => {
            log::debug!("{e:#?}");
            log::warn!("{e}");
            return;
        }
    };

    let Some(tmdb) = MikanTmdb::find_by_bangumi_id(&bangumi_id).await else {
        log::info!("mikan bangumi({bangumi_id}) no matching data, use other parser.");
        return;
    };

    torrent.tmdb_id = Some(tmdb.tmdb_id);
    torrent.is_movie = tmdb.is_movie;
    torrent.season = tmdb.season;
}

async fn parse_mikan_bangumi_id(hash: &str) -> Result<String> {
    let url = format!("https://mikanani.me/Home/Episode/{}", hash.to_lowercase());
    let resp = client().get(url).send().await?;
    let resp = resp.error_for_status()?;
    ensure!(
        resp.status() != StatusCode::MOVED_PERMANENTLY && resp.status() != StatusCode::FOUND,
        "mikan episode not found"
    );
    let text = resp.text().await?;

    static SELECTOR: OnceLock<Selector> = OnceLock::new();
    // language=JQuery-CSS
    let selector =
        SELECTOR.get_or_init(|| Selector::parse(r#"a[href^="/Home/Bangumi/"]"#).unwrap());

    let html = Html::parse_document(&text);
    let element = html.select(selector).next();
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
