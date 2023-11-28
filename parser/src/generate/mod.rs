use anyhow::{ensure, Result};

use database::entity::Torrent;

pub trait GenerateTorrentInfo {
    /// 根据基础路径，生成下载路径（用于设置下载路径）
    fn generate_download_path(&self, base_path: &str) -> Result<String>;
    /// 根据现有文件路径，生成格式化后的简单文件路径（用于下载器改名）
    fn generate_simple_file_path(&self, file_path: &str) -> Result<String>;
    /// 生成格式化后的完整标题（用于 sonarr 或 radarr 识别）
    fn generate_full_title(&self) -> Result<String>;
}

impl GenerateTorrentInfo for Torrent {
    fn generate_download_path(&self, _base_path: &str) -> Result<String> {
        ensure!(self.tmdb_id.is_some(), "torrent tmdb info empty!");

        // if self.is_movie {
        //     Ok(format!("{path}/{} ({})", self.title, self.year))
        // } else {
        //     let regex = lazy_regex!("^S(\\d+)$");
        //     ensure!(regex.is_match(&self.season), "only support single season.");
        //     let season = regex.replace(&self.season, "Season $1");
        //     Ok(format!("{path}/{}/{season}", self.title))
        // }

        todo!()
    }

    fn generate_simple_file_path(&self, _file_path: &str) -> Result<String> {
        ensure!(self.tmdb_id.is_some(), "torrent tmdb info empty!");

        // if self.is_movie {
        //     Ok(format!("{} ({})", self.title, self.year))
        // } else {
        //     let episode = file_name_parse(filename).episode_number;
        //     let episode = episode.with_context(|| format!("file({filename}) ep is empty."))?;
        //     Ok(format!("{} - {}{}", self.title, self.season, episode))
        // }

        todo!()
    }

    fn generate_full_title(&self) -> Result<String> {
        ensure!(self.tmdb_id.is_some(), "torrent tmdb info empty!");

        todo!()
    }
}
