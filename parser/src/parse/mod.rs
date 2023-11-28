use anyhow::Result;
use async_trait::async_trait;

use database::entity::Torrent;

mod info_hash;
mod mikan;
mod name_info;
mod tmdb_ids;

#[async_trait]
pub trait ParseTorrent {
    /// 尝试解析 hash 值
    async fn try_parse_hash(&mut self) -> Result<()>;
    /// 尝试解析详细信息
    async fn try_parse_detail(&mut self) -> Result<()>;
}

#[async_trait]
impl ParseTorrent for Torrent {
    async fn try_parse_hash(&mut self) -> Result<()> {
        self.id = info_hash::parse_url_hash(&self.download_url).await?;
        Ok(())
    }

    async fn try_parse_detail(&mut self) -> Result<()> {
        // 优先使用 mikan 直接映射
        mikan::mikan_direct_mapping(self).await;
        // 本地文件名解析处理
        name_info::name_local_parse(self);
        // 附加 tmdb 的准确信息
        tmdb_ids::append_extra_ids(self).await?;
        Ok(())
    }
}
