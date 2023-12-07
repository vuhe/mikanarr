use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{bail, ensure, Context, Error, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use sha1::{Digest, Sha1};

/// 解析下载链接，获取 torrent hash 值
pub(super) async fn parse_url_hash(url: &str) -> Result<String> {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    let url = Url::parse(url)?;
    let resp = match url.scheme() {
        "magnet" => return parse_magnet_hash(url),
        "http" | "https" => CLIENT.get_or_init(Client::default).get(url).send().await?,
        _ => bail!("Invalid URI scheme: {}", url.scheme()),
    };
    let bytes = resp.bytes().await?;
    parse_torrent_hash(&bytes)
}

/// 解析 magnet hash 值
fn parse_magnet_hash(magnet: Url) -> Result<String> {
    let mut info_hash: Option<InfoHash> = None;

    fn hybrid_hash(hash1: Option<InfoHash>, hash2: &str) -> Result<InfoHash> {
        match hash1 {
            None => Ok(hash2.parse()?),
            Some(hash) => hash.hybrid(hash2.parse()?),
        }
    }

    for (key, val) in magnet.query_pairs() {
        match key.as_ref() {
            "xt" if val.starts_with("urn:btih:") => {
                info_hash = Some(hybrid_hash(
                    info_hash,
                    val.strip_prefix("urn:btih:").unwrap(),
                )?);
            }
            "xt" if val.starts_with("urn:btmh:1220") => {
                info_hash = Some(hybrid_hash(
                    info_hash,
                    val.strip_prefix("urn:btmh:1220").unwrap(),
                )?);
            }
            _ => {}
        }
    }

    let info_hash = info_hash.context("No hash found (only btih/btmh hashes are supported)")?;
    ensure!(info_hash.support_v1(), "unsupported pure v2 magnet");
    Ok(info_hash.into_hash())
}

/// 解析 torrent hash 值
fn parse_torrent_hash(torrent: &[u8]) -> Result<String> {
    let torrent: Torrent = bt_bencode::from_slice(torrent)?;
    let info: Info = bt_bencode::from_slice(torrent.info)?;
    let info_hash = match info.version {
        None | Some(1) => InfoHash::from_v1_bytes(torrent.info),
        Some(2) if info.file_tree.is_some() => {
            let mut hash = InfoHash::from_v2_bytes(torrent.info);
            if info.length.is_some() || info.files.is_some() {
                hash = hash.hybrid(InfoHash::from_v1_bytes(torrent.info))?;
            }
            hash
        }
        Some(2) => bail!("torrent v2 without 'file_tree' field"),
        Some(v) => bail!("Wrong torrent version: {v}, only v1 and v2 are supported"),
    };
    Ok(info_hash.into_hash())
}

#[derive(Deserialize)]
struct Torrent<'a> {
    info: &'a [u8],
}

#[derive(Deserialize)]
struct Info<'a> {
    #[serde(rename = "meta version")]
    version: Option<u64>,
    #[serde(rename = "name")]
    _name: String,
    /// Torrent v1/hybrid (only for single-file torrents)
    length: Option<u64>,
    /// Torrent v1 (only for multi-files torrents)
    #[serde(borrow)]
    files: Option<&'a [u8]>,
    /// Torrent v2 (for both single and multi-files torrents)
    #[serde(borrow, rename = "file tree")]
    file_tree: Option<&'a [u8]>,
}

#[derive(Clone, Debug, PartialEq)]
enum InfoHash {
    V1(String),
    V2(String),
    Hybrid((String, String)),
}

impl InfoHash {
    fn from_v1_bytes(bytes: &[u8]) -> Self {
        let hash = hex::encode(Sha1::digest(bytes));
        debug_assert!(hash.len() == 40);
        Self::V1(hash)
    }

    fn from_v2_bytes(bytes: &[u8]) -> Self {
        let hash = sha256::digest(bytes);
        debug_assert!(hash.len() == 64);
        Self::V2(hash)
    }

    fn support_v1(&self) -> bool {
        matches!(self, Self::V1(_) | Self::Hybrid(_))
    }

    fn hybrid(self, with: InfoHash) -> Result<Self> {
        match (self, with) {
            (Self::V1(hash1), Self::V2(hash2)) => Ok(Self::Hybrid((hash1, hash2))),
            (Self::V2(hash2), Self::V1(hash1)) => Ok(Self::Hybrid((hash1, hash2))),
            (Self::V1(_), Self::V1(_)) => bail!("can't make hybrid out of two V1 hashes"),
            (Self::V2(_), Self::V2(_)) => bail!("can't make hybrid out of two V2 hashes"),
            _ => bail!("can't make a hybrid out of an already-hybrid info hash"),
        }
    }

    /// 转换为 hash 值，
    fn into_hash(self) -> String {
        match self {
            InfoHash::V1(v1) => v1,
            InfoHash::V2(v2) => v2,
            // 为保证兼容性，混合 hash 使用 v1
            InfoHash::Hybrid((v1, _)) => v1,
        }
    }
}

impl FromStr for InfoHash {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        ensure!(
            s.as_bytes().iter().all(u8::is_ascii_hexdigit),
            "hash contains non-hex characters: {s}"
        );
        match s.len() {
            40 => Ok(Self::V1(s.to_string())),
            64 => Ok(Self::V2(s.to_string())),
            len => bail!("hash has invalid length {len} (expected 40 or 64): {s}"),
        }
    }
}
