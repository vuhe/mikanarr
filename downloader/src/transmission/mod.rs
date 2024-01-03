use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use anyhow::Result;
use once_cell::sync::Lazy as LazyLock;
use reqwest::Client;

use database::entity::Downloader;

use crate::DownloadItem;

mod handler;
mod receiver;
mod sender;

static CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

struct Session;

impl Session {
    fn inner(&self) -> MutexGuard<'static, HashMap<u32, Arc<str>>> {
        static SESSION: OnceLock<Mutex<HashMap<u32, Arc<str>>>> = OnceLock::new();
        SESSION.get_or_init(Default::default).lock().unwrap()
    }

    fn get(&self, id: u32) -> Option<Arc<str>> {
        self.inner().get(&id).map(|it| it.clone())
    }

    fn set(&self, id: u32, session: &str) {
        self.inner().insert(id, session.into());
    }
}

/// transmission client
/// [技术规范](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md)
pub(crate) struct TR {
    id: u32,
    url: String,
    username: Option<String>,
    password: Option<String>,
    download_dir: String,
}

impl TR {
    pub(crate) fn download_dir(&self) -> &str {
        self.download_dir.as_str()
    }

    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.port_test().await
    }

    pub(crate) async fn download(&self, torrent: &[u8], dir: &str) -> Result<()> {
        self.add_torrent(torrent, dir).await.map(|_| ())
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let list = self.torrent_list().await?;
        let list = list.into_iter().map(|it| it.into());
        Ok(list.collect())
    }

    pub(crate) async fn download_files(&mut self, id: &str) -> Result<Vec<String>> {
        let info = self.torrent_info(id).await?;
        let item: DownloadItem = info.into();
        Ok(item.relative_path)
    }
}

impl From<Downloader> for TR {
    fn from(value: Downloader) -> Self {
        Self {
            id: value.id,
            url: value.url,
            username: value.username,
            password: value.password,
            download_dir: value.download_dir,
        }
    }
}
