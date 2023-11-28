pub use server::search;

mod server;
mod torrent;
mod torznab;

/// 加载后台服务
pub fn load() {
    let _ = tokio::spawn(server::fetch_and_save_torrents());
}
