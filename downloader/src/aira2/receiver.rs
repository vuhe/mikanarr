use serde::Deserialize;

use crate::{DownloadItem, ItemStatus};

#[derive(Debug, Deserialize)]
pub(super) struct RespError {
    pub(super) message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response<T> {
    Ok { result: T },
    Error { error: RespError },
}

#[derive(Debug, Deserialize)]
struct DownloadFileInfo {
    path: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct DownloadStatus {
    gid: String,
    #[serde(rename = "infoHash")]
    info_hash: String,
    #[serde(rename = "totalLength")]
    total: u64,
    #[serde(rename = "completedLength")]
    completed: u64,
    status: String,
    #[serde(default)]
    files: Vec<DownloadFileInfo>,
}

impl From<DownloadStatus> for DownloadItem {
    fn from(value: DownloadStatus) -> Self {
        let status = match value.status.as_str() {
            _ if value.total == 0 => ItemStatus::Error,
            "active" | "paused" if value.total == value.completed => ItemStatus::Downloaded,
            "active" | "waiting" | "paused" => ItemStatus::Downloading,
            "complete" => ItemStatus::Complete,
            _ => ItemStatus::Error,
        };
        let paths = value.files.into_iter().map(|it| it.path);
        DownloadItem {
            id: value.gid,
            info_hash: value.info_hash,
            status,
            relative_path: paths.collect(),
        }
    }
}
