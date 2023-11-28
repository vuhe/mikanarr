use anitors::Element;
use database::entity::Torrent;

/// 本地名称解析，用于附加 torrent 信息
pub(super) fn name_local_parse(torrent: &mut Torrent) {
    let element = file_name_parse(&torrent.name);

    if torrent.title.is_empty() {
        torrent.title = element.anime_title.unwrap_or_default();
    }
    if torrent.season.is_empty() {
        torrent.season = element.anime_season.unwrap_or_default();
    }
    if torrent.episode.is_empty() {
        torrent.episode = element.episode_number.unwrap_or_default();
    }
}

/// 本地文件名解析，仅做解析返回解析结果
pub(crate) fn file_name_parse(filename: &str) -> Element {
    // FIXME 单独封装一个方法，防止后续出现需要预处理文件名的情况
    Element::parse(filename)
}
