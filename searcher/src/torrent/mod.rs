use anyhow::Result;
use chrono::{DateTime, Local};
use rss::Channel;

use database::entity::Torrent;
use helper::{append_date_time, parse_rss_torrent_ext};

mod helper;

pub(crate) trait TorrentExt: Sized {
    fn parse_torrent_rss(bytes: &[u8]) -> Result<Self>;
    fn into_torrents(self) -> Vec<Torrent>;
}

impl TorrentExt for Channel {
    fn parse_torrent_rss(bytes: &[u8]) -> Result<Self> {
        let mut channel = Channel::read_from(bytes)?;
        let ext = parse_rss_torrent_ext(bytes);

        for item in channel.items_mut() {
            let link = item.link().unwrap_or_default();
            append_date_time(item, ext.get(link));
        }
        Ok(channel)
    }

    fn into_torrents(self) -> Vec<Torrent> {
        self.items
            .into_iter()
            .map(|it| {
                let mut torrent = Torrent::default();
                torrent.name = it.title.unwrap_or_default();
                let enclosure = it.enclosure;
                torrent.download_url = enclosure.map(|it| it.url).unwrap_or_default();
                torrent.pub_date = DateTime::parse_from_rfc2822(&it.pub_date.unwrap_or_default())
                    .unwrap_or_else(|_| Local::now().into());
                torrent
            })
            .collect()
    }
}
