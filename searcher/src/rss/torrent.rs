use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::rss::util::{decode, element_date, element_text, skip};

use super::xml_ext::{BytesWriter, SerdeXml, WriterExt};

static TORRENT_EOF_ERR: &str =
    "The end of the input was reached without finding a complete torrent element.";

#[derive(Default)]
pub struct TorrentExt {
    pub info_hash: Option<String>,
    pub link: Option<String>,
    pub pub_date: Option<DateTime<FixedOffset>>,
    pub content_length: Option<String>,
}

impl TorrentExt {
    pub fn append(self, reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        match decode(element.local_name().as_ref(), reader)?.as_ref() {
            "torrent" => self.parse_obj(reader),
            _ => self.parse_attr(reader, element),
        }
    }

    fn parse_obj(mut self, reader: &mut Reader<&[u8]>) -> Result<Self> {
        loop {
            match reader.read_event()? {
                Event::Start(e) => match decode(e.name().as_ref(), reader)?.as_ref() {
                    "infoHash" => self.info_hash = element_text(reader)?,
                    "link" => self.link = element_text(reader)?,
                    "pubDate" => self.pub_date = element_date(reader)?,
                    "contentLength" => self.content_length = element_text(reader)?,
                    _ => skip(e.name(), reader)?,
                },
                Event::End(_) => break,
                Event::Eof => bail!(TORRENT_EOF_ERR),
                _ => {}
            }
        }
        Ok(self)
    }

    fn parse_attr(mut self, reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        match decode(element.local_name().as_ref(), reader)?.as_ref() {
            "infoHash" => self.info_hash = element_text(reader)?,
            "link" => self.link = element_text(reader)?,
            "pubDate" => self.pub_date = element_date(reader)?,
            "contentLength" => self.content_length = element_text(reader)?,
            _ => {}
        }
        skip(element.name(), reader)?;
        Ok(self)
    }
}

impl SerdeXml for TorrentExt {
    fn from_xml(reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        Self::default().append(reader, element)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        if let Some(info_hash) = self.info_hash.as_ref() {
            writer.write_text_element("torrent:infoHash", info_hash)?;
        }

        if let Some(link) = self.link.as_ref() {
            writer.write_text_element("torrent:link", link)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element("torrent:pubDate", pub_date.to_rfc2822())?;
        }

        if let Some(content_length) = self.content_length.as_ref() {
            writer.write_text_element("torrent:contentLength", content_length)?;
        }

        Ok(())
    }
}
