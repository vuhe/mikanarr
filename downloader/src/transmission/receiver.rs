use serde::Deserialize;

use crate::{DownloadItem, ItemStatus, DEFAULT_CATEGORY};

#[derive(Debug, Deserialize)]
pub(super) struct Response<T> {
    pub(super) result: String,
    pub(super) arguments: T,
}

#[derive(Debug, Deserialize)]
pub(super) struct PortTestResp {
    #[serde(rename = "port-is-open")]
    pub(super) port_is_open: bool,
}

#[derive(Debug, Deserialize)]
pub(super) struct AddedTorrent {
    #[serde(rename = "id")]
    _id: u16,
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "hashString")]
    hash_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum AddTorrentResp {
    TorrentAdded(AddedTorrent),
    TorrentDuplicate(AddedTorrent),
}

impl AddTorrentResp {
    pub(super) fn into_hash_string(self) -> String {
        match self {
            AddTorrentResp::TorrentAdded(it) => it.hash_string,
            AddTorrentResp::TorrentDuplicate(it) => it.hash_string,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct TorrentList {
    pub(super) torrents: Vec<TorrentInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TorrentInfo {
    id: u16,
    hash_string: String,
    percent_done: f64,
    is_finished: bool,
    status: u8,
    labels: Vec<String>,
    #[serde(default)]
    files: Vec<FileInfo>,
}

impl TorrentInfo {
    pub(super) fn match_category(&self) -> bool {
        self.labels
            .iter()
            .find(|it| it.as_str() == DEFAULT_CATEGORY)
            .is_some()
    }
}

/// torrent 文件信息
#[derive(Debug, Deserialize)]
struct FileInfo {
    /// 相对路径
    name: String,
}

impl From<TorrentInfo> for DownloadItem {
    fn from(value: TorrentInfo) -> Self {
        let downloaded = value.percent_done >= 1.0;
        let status = match value.status {
            // Torrent is stopped, but downloaded
            0 if downloaded => ItemStatus::Downloaded,
            // Torrent is stopped
            0 => ItemStatus::Downloading,
            // Torrent is verifying or downloading
            1 | 2 | 3 | 4 => ItemStatus::Downloading,
            // Torrent is seeding
            5 | 6 => ItemStatus::Downloaded,
            // Torrent is finished and downloaded
            _ if value.is_finished && downloaded => ItemStatus::Complete,
            _ => ItemStatus::Error,
        };
        let paths = value.files.into_iter().map(|it| it.name);
        DownloadItem {
            id: value.id.to_string(),
            info_hash: value.hash_string,
            status,
            relative_path: paths.collect(),
        }
    }
}
