use anyhow::{bail, Result};
use bytes::{BufMut, Bytes, BytesMut};
use chrono::{DateTime, FixedOffset};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};

use database::entity::Torrent;

use super::item::Item;
use super::util::{decode, element_date, element_text, skip};
use super::xml_ext::{BytesWriter, SerdeXml, WriterExt};
use super::Category;

static CHANNEL_EOF_ERR: &str =
    "The end of the input was reached without finding a complete channel element.";

#[derive(Default)]
pub(crate) struct Channel {
    /// The name of the channel.
    pub title: String,
    /// The URL for the website corresponding to the channel.
    pub link: String,
    /// A description of the channel.
    pub description: String,
    /// The language of the channel.
    pub language: Option<String>,
    /// The publication date for the content of the channel as an RFC822 timestamp.
    pub pub_date: Option<DateTime<FixedOffset>>,
    /// The items in the channel.
    pub items: Vec<Item>,
    /// The rss type, use for namespace
    pub rss_type: Category,
}

impl Channel {
    /// 读取 rss 信息，仅支持 torznab 和 torrent 格式
    pub(super) fn read_from(bytes: &[u8], rss_type: Category) -> Result<Self> {
        let mut reader = Reader::from_reader(bytes);
        reader.trim_text(true).expand_empty_elements(true);

        let mut channel: Option<Self> = None;

        // for parsing RSS 0.9, 1.0 feeds
        let mut items: Vec<Item> = Vec::new();

        // find opening element
        loop {
            match reader.read_event()? {
                Event::Start(element) => match decode(element.name().as_ref(), &reader)?.as_ref() {
                    "rss" | "rdf:RDF" => break,
                    _ => bail!("The input didn't begin with an opening `<rss>` tag."),
                },
                Event::Eof => bail!(CHANNEL_EOF_ERR),
                _ => continue,
            }
        }

        loop {
            match reader.read_event()? {
                Event::Start(e) => match decode(e.name().as_ref(), &reader)?.as_ref() {
                    "channel" => channel = Some(Channel::from_xml(&mut reader, e)?),
                    "item" => items.push(Item::from_xml(&mut reader, e)?),
                    _ => skip(e.name(), &mut reader)?,
                },
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
        }

        match channel {
            Some(mut channel) => {
                channel.items.append(&mut items);
                channel.rss_type = rss_type;
                Ok(channel)
            }
            None => bail!(CHANNEL_EOF_ERR),
        }
    }

    /// 设置 torrent 信息
    pub(crate) fn set_torznab_items(&mut self, torrents: Vec<Torrent>) {
        self.items = torrents.into_iter().map(Item::new_torznab).collect();
    }

    /// 转换为 torrent 信息
    pub(crate) fn into_torrents(self) -> Vec<Torrent> {
        self.items.into_iter().map(Item::into_torrent).collect()
    }

    // noinspection HttpUrlsUsage
    /// 将 rss 信息转换为 bytes
    pub(crate) fn into_bytes(self) -> Result<Bytes> {
        let mut writer = Writer::new(BytesMut::new().writer());
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;

        let name = "rss";
        let mut element = BytesStart::new(name);
        element.push_attribute(("version", "2.0"));

        let namespace = match self.rss_type {
            Category::Torrent => ("xmlns:torrent", "https://github.com/vuhe/mikanarr"),
            Category::Torznab => ("xmlns:torznab", "http://torznab.com/schemas/2015/feed"),
        };
        element.push_attribute(namespace);

        writer.write_event(Event::Start(element))?;

        self.to_xml(&mut writer)?;

        writer.write_event(Event::End(BytesEnd::new(name)))?;

        Ok(writer.into_inner().into_inner().freeze())
    }
}

impl SerdeXml for Channel {
    fn from_xml(reader: &mut Reader<&[u8]>, _element: BytesStart) -> Result<Self> {
        let mut channel = Self::default();

        loop {
            match reader.read_event()? {
                Event::Start(e) => match decode(e.name().as_ref(), reader)?.as_ref() {
                    "item" => channel.items.push(Item::from_xml(reader, e)?),
                    "title" => channel.title = element_text(reader)?.unwrap_or_default(),
                    "link" => channel.link = element_text(reader)?.unwrap_or_default(),
                    "description" => {
                        channel.description = element_text(reader)?.unwrap_or_default()
                    }
                    "language" => channel.language = element_text(reader)?,
                    "pubDate" => channel.pub_date = element_date(reader)?,
                    _ => skip(e.name(), reader)?,
                },
                Event::End(_) => break,
                Event::Eof => bail!(CHANNEL_EOF_ERR),
                _ => {}
            }
        }

        Ok(channel)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        let name = "channel";

        writer.write_event(Event::Start(BytesStart::new(name)))?;

        writer.write_text_element("title", &self.title)?;
        writer.write_text_element("link", &self.link)?;
        writer.write_text_element("description", &self.description)?;

        if let Some(language) = self.language.as_ref() {
            writer.write_text_element("language", language)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element("pubDate", pub_date.to_rfc2822())?;
        }

        writer.write_objects(&self.items)?;

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}
