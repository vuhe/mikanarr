use std::sync::OnceLock;
use std::time::Duration;

use anyhow::Result;
use bytes::Bytes;
use reqwest::Client;
use rss::Channel;
use tokio::time::sleep;

use database::entity::{Indexer, IndexerCategory, Torrent};
use parser::ParseTorrent;

use crate::torrent::TorrentExt;

/// 循环拉取并保存 torrent
pub(crate) async fn fetch_and_save_torrents() {
    loop {
        let indexers = match Indexer::find_all_enable().await {
            Ok(it) => it,
            Err(e) => {
                log::debug!("{e:#?}");
                log::warn!("get Indexer from database error, try again later: {e}");
                Vec::default()
            }
        };

        for indexer in indexers {
            let torrents = match indexer.category {
                IndexerCategory::Rss => fetch_torrent_rss(&indexer.url).await,
                IndexerCategory::Torznab => fetch_torznab(&indexer.url).await,
            };
            let torrents = match torrents {
                Ok(it) => it,
                Err(e) => {
                    log::debug!("{e:#?}");
                    log::warn!("fetch `{}` torrent error, skip it: {e}", indexer.name);
                    continue;
                }
            };
            for torrent in torrents {
                let torrent_name = torrent.name.clone();
                if let Err(e) = parse_info_and_save(torrent).await {
                    log::debug!("{e:#?}");
                    log::warn!("parse `{torrent_name}` torrent error, skip it: {e}");
                }
            }
        }

        sleep(Duration::from_secs(10 * 60)).await;
    }
}

/// 解析 torrent 信息并存入数据库
async fn parse_info_and_save(mut torrent: Torrent) -> Result<()> {
    torrent.try_parse_hash().await?;
    // 重复 torrent 不再进行解析
    if !torrent.exist().await {
        torrent.try_parse_detail().await?;
        torrent.insert().await?;
    }
    Ok(())
}

/// 拉取 rss torrent 信息
async fn fetch_torrent_rss(rss_url: &str) -> Result<Vec<Torrent>> {
    let bytes = request_xml(rss_url).await?;
    Ok(Channel::parse_torrent_rss(&bytes)?.into_torrents())
}

/// 拉取 torznab 信息
async fn fetch_torznab(torznab_url: &str) -> Result<Vec<Torrent>> {
    let bytes = request_xml(torznab_url).await?;
    Ok(Channel::read_from(&*bytes)?.into_torrents())
}

async fn request_xml(url: &str) -> Result<Bytes> {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    let resp = CLIENT.get_or_init(Client::default).get(url).send().await?;
    let resp = resp.error_for_status()?;
    Ok(resp.bytes().await?)
}
