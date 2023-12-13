use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset, Local};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;

use database::entity::Torrent;

use super::enclosure::Enclosure;
use super::guid::Guid;
use super::torrent::TorrentExt;
use super::torznab::TorznabExt;
use super::util::{decode, element_date, element_text, skip};
use super::xml_ext::{BytesWriter, SerdeXml, WriterExt};

static ITEM_EOF_ERR: &str =
    "The end of the input was reached without finding a complete item element.";

#[derive(Default)]
pub struct Item {
    /// The title of the item.
    pub title: Option<String>,
    /// The URL of the item.
    pub link: Option<String>,
    /// The item synopsis.
    pub description: Option<String>,
    /// The email address of author of the item.
    pub author: Option<String>,
    /// The URL for the comments page of the item.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item.
    pub enclosure: Option<Enclosure>,
    /// A unique identifier for the item.
    pub guid: Option<Guid>,
    /// The date the item was published as an RFC 2822 timestamp.
    pub pub_date: Option<DateTime<FixedOffset>>,
    /// The HTML contents of the item.
    pub content: Option<String>,
    /// The torrent extension for the item.
    pub torrent_ext: Option<TorrentExt>,
    /// The torznab extension for the item.
    pub torznab_ext: Option<TorznabExt>,
}

impl Item {
    pub(super) fn new_torznab(torrent: Torrent) -> Self {
        // todo size
        let mut torznab_ext = TorznabExt::default();
        torznab_ext.category = (if torrent.is_movie { "2000" } else { "1000" }).into();

        // todo title, link, description
        let mut item = Self::default();
        item.guid = Some(Guid::new(torrent.id));
        item.torznab_ext = Some(torznab_ext);

        item
    }

    pub(super) fn into_torrent(self) -> Torrent {
        let mut torrent = Torrent::default();
        torrent.name = self.title.unwrap_or_default();
        torrent.download_url = self.enclosure.map(|it| it.url).unwrap_or_default();

        let mut date = self.pub_date;
        if let Some(torrent_ext) = self.torrent_ext {
            date = date.or(torrent_ext.pub_date);
            torrent.id = torrent_ext.info_hash.unwrap_or_default();
        }

        torrent.pub_date = date.unwrap_or_else(|| Local::now().into());
        torrent
    }
}

impl SerdeXml for Item {
    fn from_xml(reader: &mut Reader<&[u8]>, _element: BytesStart) -> Result<Self> {
        let mut item = Self::default();

        loop {
            match reader.read_event()? {
                Event::Start(e) => match decode(e.name().as_ref(), reader)?.as_ref() {
                    "guid" => item.guid = Some(Guid::from_xml(reader, e)?),
                    "enclosure" => item.enclosure = Some(Enclosure::from_xml(reader, e)?),
                    "title" => item.title = element_text(reader)?,
                    "link" => item.link = element_text(reader)?,
                    "description" => item.description = element_text(reader)?,
                    "author" => item.author = element_text(reader)?,
                    "comments" => item.comments = element_text(reader)?,
                    "pubDate" => item.pub_date = element_date(reader)?,
                    "content:encoded" => item.content = element_text(reader)?,
                    "torznab:attr" => {
                        item.torznab_ext = match item.torznab_ext {
                            None => Some(TorznabExt::from_xml(reader, e)?),
                            Some(it) => Some(it.append(reader, e)?),
                        }
                    }
                    n if n.starts_with("torrent") => {
                        item.torrent_ext = match item.torrent_ext {
                            None => Some(TorrentExt::from_xml(reader, e)?),
                            Some(it) => Some(it.append(reader, e)?),
                        }
                    }
                    _ => skip(e.name(), reader)?,
                },
                Event::End(_) => break,
                Event::Eof => bail!(ITEM_EOF_ERR),
                _ => {}
            }
        }

        Ok(item)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        let name = "item";

        writer.write_event(Event::Start(BytesStart::new(name)))?;

        if let Some(title) = self.title.as_ref() {
            writer.write_text_element("title", title)?;
        }

        if let Some(link) = self.link.as_ref() {
            writer.write_text_element("link", link)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_cdata_element("description", description)?;
        }

        if let Some(author) = self.author.as_ref() {
            writer.write_text_element("author", author)?;
        }

        if let Some(comments) = self.comments.as_ref() {
            writer.write_text_element("comments", comments)?;
        }

        if let Some(enclosure) = self.enclosure.as_ref() {
            writer.write_object(enclosure)?;
        }

        if let Some(guid) = self.guid.as_ref() {
            writer.write_object(guid)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element("pubDate", pub_date.to_rfc2822())?;
        }

        if let Some(content) = self.content.as_ref() {
            writer.write_cdata_element("content:encoded", content)?;
        }

        if let Some(torrent_ext) = self.torrent_ext.as_ref() {
            writer.write_object(torrent_ext)?;
        }

        if let Some(torznab_ext) = self.torznab_ext.as_ref() {
            writer.write_object(torznab_ext)?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}
