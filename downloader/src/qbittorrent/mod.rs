use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Client;

use database::entity::Downloader;

use crate::DownloadItem;

mod handler;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::builder().cookie_store(true).build().unwrap());

/// qbittorrent client
/// [技术规范](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
pub(crate) struct QB {
    _id: u32,
    url: String,
    username: Option<String>,
    password: Option<String>,
    download_dir: String,
}

impl QB {
    pub(crate) fn download_dir(&self) -> &str {
        self.download_dir.as_str()
    }

    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.app_version().await
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let list = self.torrent_info().await?;
        let list = list.into_iter().map(|it| it.into());
        Ok(list.collect())
    }

    pub(crate) async fn download_files(&mut self, id: &str) -> Result<Vec<String>> {
        let list = self.torrent_files(id).await?;
        let list = list.into_iter().map(|it| it.name);
        Ok(list.collect())
    }
}

impl From<Downloader> for QB {
    fn from(value: Downloader) -> Self {
        Self {
            _id: value.id,
            url: value.url,
            username: value.username,
            password: value.password,
            download_dir: value.download_dir,
        }
    }
}
