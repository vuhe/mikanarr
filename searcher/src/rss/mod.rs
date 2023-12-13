use anyhow::Result;

use channel::Channel;

mod channel;
mod enclosure;
mod guid;
mod item;
mod torrent;
mod torznab;
mod util;
mod xml_ext;

pub(crate) enum Category {
    Torrent,
    Torznab,
}

impl Default for Category {
    fn default() -> Self {
        Self::Torznab
    }
}

pub(crate) fn parse_torrent_rss(bytes: &[u8]) -> Result<Channel> {
    Channel::read_from(bytes, Category::Torrent)
}

pub(crate) fn parse_torznab_rss(bytes: &[u8]) -> Result<Channel> {
    Channel::read_from(bytes, Category::Torznab)
}

pub(crate) fn new_torznab_rss() -> Channel {
    Channel {
        title: "mikanarr search".into(),
        link: "https://github.com/vuhe/mikanarr".into(),
        description: "This search result is generated based on local database info.".into(),
        rss_type: Category::Torznab,
        ..Default::default()
    }
}
