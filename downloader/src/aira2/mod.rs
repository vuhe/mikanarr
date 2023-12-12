use std::sync::OnceLock;

use anyhow::Result;
use reqwest::Client;

use database::entity::Downloader;

use crate::DownloadItem;

mod handler;
mod receiver;

fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::default)
}

/// aira2 client
/// [技术规范](https://aria2.github.io/manual/en/html/aria2c.html#methods)
pub(crate) struct AR {
    _id: u32,
    url: String,
    secret: Option<String>,
    download_dir: String,
}

impl AR {
    pub(crate) fn download_dir(&self) -> &str {
        self.download_dir.as_str()
    }

    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.get_version().await
    }

    pub(crate) async fn download(&self, torrent: &[u8], dir: &str) -> Result<()> {
        self.add_torrent(torrent, dir).await.map(|_| ())
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let vec = [
            self.tell_waiting().await?,
            self.tell_active().await?,
            self.tell_stopped().await?,
        ];
        let list = vec.into_iter();
        let list = list.flatten();
        let list = list.map(|it| it.into());
        Ok(list.collect())
    }

    pub(crate) async fn download_files(&mut self, id: &str) -> Result<Vec<String>> {
        let status = self.tell_status(id).await?;
        let item = DownloadItem::from(status);
        Ok(item.relative_path)
    }
}

impl From<Downloader> for AR {
    fn from(value: Downloader) -> Self {
        Self {
            _id: value.id,
            url: value.url,
            secret: value.password,
            download_dir: value.download_dir,
        }
    }
}
