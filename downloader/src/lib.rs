use anyhow::{bail, Result};

use database::entity::{Downloader, DownloaderType};

mod aira2;
mod qbittorrent;
mod transmission;

/// 下载类别，用于区分获取下载种子列表
static DEFAULT_CATEGORY: &str = "mikanarr";

/// 下载状态
pub enum ItemStatus {
    /// 下载中
    Downloading,
    /// 下载完成，做种中
    Downloaded,
    /// 下载完成，做种完成
    Complete,
    /// 下载错误
    Error,
}

/// 下载项信息
pub struct DownloadItem {
    /// 下载项 id
    pub id: String,
    /// 下载项 hash(v1)
    pub info_hash: String,
    /// 下载状态
    pub status: ItemStatus,
    /// 文件相对路径
    pub relative_path: Vec<String>,
}

/// 下载客户端
pub struct DownloadClient(DownloaderInner);

enum DownloaderInner {
    Aira2(aira2::AR),
    Qbittorrent(qbittorrent::QB),
    Transmission(transmission::TR),
}

impl DownloadClient {
    /// 下载文件夹基础路径
    pub fn download_dir(&self) -> &str {
        match &self.0 {
            DownloaderInner::Aira2(it) => it.download_dir(),
            DownloaderInner::Qbittorrent(it) => it.download_dir(),
            DownloaderInner::Transmission(it) => it.download_dir(),
        }
    }

    /// 是否支持文件改名操作
    pub fn supported_file_rename(&self) -> bool {
        matches!(&self.0, DownloaderInner::Qbittorrent(_))
    }

    /// 下载器连接测试
    pub async fn connect_test(&mut self) -> Result<()> {
        match &mut self.0 {
            DownloaderInner::Aira2(it) => it.connect_test().await,
            DownloaderInner::Qbittorrent(it) => it.connect_test().await,
            DownloaderInner::Transmission(it) => it.connect_test().await,
        }
    }

    /// 下载器添加 torrent
    pub async fn download(&mut self, torrent: &[u8], dir: &str) -> Result<()> {
        match &mut self.0 {
            DownloaderInner::Aira2(it) => it.download(torrent, dir).await,
            DownloaderInner::Qbittorrent(it) => it.add_torrent(torrent, dir).await,
            DownloaderInner::Transmission(it) => it.download(torrent, dir).await,
        }
    }

    /// 获取下载列表
    pub async fn download_list(&mut self) -> Result<Vec<DownloadItem>> {
        match &mut self.0 {
            DownloaderInner::Aira2(it) => it.download_list().await,
            DownloaderInner::Qbittorrent(it) => it.download_list().await,
            DownloaderInner::Transmission(it) => it.download_list().await,
        }
    }

    /// 获取单个下载项的文件路径（相对路径）
    pub async fn download_files(&mut self, id: &str) -> Result<Vec<String>> {
        match &mut self.0 {
            DownloaderInner::Aira2(it) => it.download_files(id).await,
            DownloaderInner::Qbittorrent(it) => it.download_files(id).await,
            DownloaderInner::Transmission(it) => it.download_files(id).await,
        }
    }

    /// 重命名下载文件
    pub async fn rename_file(&mut self, id: &str, old_path: &str, new_path: &str) -> Result<()> {
        match &mut self.0 {
            DownloaderInner::Aira2(_) => bail!("aira2 unsupported rename file"),
            DownloaderInner::Qbittorrent(it) => it.rename_file(id, old_path, new_path).await,
            DownloaderInner::Transmission(_) => bail!("transmission unsupported rename file"),
        }
    }
}

impl From<Downloader> for DownloadClient {
    fn from(value: Downloader) -> Self {
        match value.cat {
            DownloaderType::Aira2 => Self(DownloaderInner::Aira2(value.into())),
            DownloaderType::Qbittorrent => Self(DownloaderInner::Qbittorrent(value.into())),
            DownloaderType::Transmission => Self(DownloaderInner::Transmission(value.into())),
        }
    }
}
