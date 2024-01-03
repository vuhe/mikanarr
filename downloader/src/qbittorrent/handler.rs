use anyhow::Result;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{DownloadItem, ItemStatus, DEFAULT_CATEGORY};

use super::{CLIENT, QB};

/// 下载 torrent 参数
#[derive(Serialize)]
struct TorrentAddArgs<'a> {
    torrents: &'a [u8],
    savepath: &'a str,
    category: &'a str,
}

/// torrent 文件信息
#[derive(Deserialize)]
pub(super) struct FileInfo {
    /// 相对路径
    pub(super) name: String,
}

/// torrent 信息
#[derive(Deserialize)]
pub(super) struct TorrentInfo {
    hash: String,
    state: String,
}

impl From<TorrentInfo> for DownloadItem {
    fn from(value: TorrentInfo) -> Self {
        let status = match value.state.as_str() {
            "allocating" | "downloading" | "metaDL" | "pausedDL" | "queuedDL" | "stalledDL"
            | "checkingDL" | "forcedDL" | "checkingResumeData" => ItemStatus::Downloading,
            "uploading" | "pausedUP" | "queuedUP" | "stalledUP" | "checkingUP" | "forcedUP" => {
                ItemStatus::Downloaded
            }
            _ => ItemStatus::Error,
        };
        DownloadItem {
            id: value.hash.clone(),
            info_hash: value.hash,
            status,
            relative_path: Vec::default(),
        }
    }
}

impl QB {
    pub(super) async fn app_version(&self) -> Result<()> {
        let url = format!("{}/api/v2/app/version", self.url);
        let req = CLIENT.get(url);
        self.api_without_resp(req).await
    }

    pub(crate) async fn add_torrent(&self, torrent: &[u8], dir: &str) -> Result<()> {
        let url = format!("{}/api/v2/torrents/add", self.url);
        let req = CLIENT.post(url).form(&TorrentAddArgs {
            torrents: torrent,
            savepath: dir,
            category: DEFAULT_CATEGORY,
        });
        self.api_without_resp(req).await
    }

    pub(super) async fn torrent_info(&self) -> Result<Vec<TorrentInfo>> {
        let url = format!("{}/api/v2/torrents/info", self.url);
        static FIXED_PARAM: [(&str, &str); 1] = [("category", DEFAULT_CATEGORY)];
        let req = CLIENT.get(url).query(&FIXED_PARAM);
        Ok(self.api(req).await?)
    }

    pub(super) async fn torrent_files(&self, id: &str) -> Result<Vec<FileInfo>> {
        let url = format!("{}/api/v2/torrents/files", self.url);
        let req = CLIENT.get(url).query(&[("hash", id)]);
        Ok(self.api(req).await?)
    }

    pub(crate) async fn rename_file(&self, id: &str, old_path: &str, new_path: &str) -> Result<()> {
        let url = format!("{}/api/v2/torrents/renameFile", self.url);
        let param = [("hash", id), ("oldPath", old_path), ("newPath", new_path)];
        let req = CLIENT.post(url).form(&param);
        self.api_without_resp(req).await
    }
}

impl QB {
    /// 尝试登录
    async fn login(&self) -> Result<()> {
        let username = self.username.as_ref().map(String::as_str);
        let password = self.password.as_ref().map(String::as_str);
        let url = format!("{}/api/v2/auth/login", self.url);
        let param = [
            ("username", username.unwrap_or_default()),
            ("password", password.unwrap_or_default()),
        ];

        let req = CLIENT.post(url).form(&param);
        req.send().await?.error_for_status()?;
        Ok(())
    }
}

impl QB {
    async fn send(&self, req: RequestBuilder) -> Result<Response> {
        // 未使用 stream body，clone 不会失败
        let first_req = req.try_clone().unwrap();
        let resp = first_req.send().await?;

        // 如果鉴权失败，那么先尝试进行登录，之后再次进行请求
        match resp.status() {
            StatusCode::FORBIDDEN => {
                self.login().await?;
                Ok(req.send().await?)
            }
            _ => Ok(resp),
        }
    }

    async fn api<R: DeserializeOwned>(&self, req: RequestBuilder) -> Result<R> {
        let resp = self.send(req).await?;
        let resp = resp.error_for_status()?;
        Ok(resp.json().await?)
    }

    async fn api_without_resp(&self, req: RequestBuilder) -> Result<()> {
        let resp = self.send(req).await?;
        resp.error_for_status()?;
        Ok(())
    }
}
