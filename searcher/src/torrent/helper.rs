use std::collections::HashMap;

use anyhow::Result;
use chrono::{format::ParseErrorKind::TooShort, DateTime, FixedOffset, Local};
use quick_xml::events::Event;
use quick_xml::Reader;
use rss::Item;

/// 修正 mikan rss 的 pubDate
pub(super) fn append_date_time(item: &mut Item, datetime: Option<&DateTime<FixedOffset>>) {
    if item.pub_date.is_none() {
        let datetime = datetime
            .map(|it| it.to_rfc2822())
            .unwrap_or_else(|| Local::now().to_rfc2822());
        item.set_pub_date(datetime);
    }
}

pub(super) fn parse_rss_torrent_ext(bytes: &[u8]) -> HashMap<String, DateTime<FixedOffset>> {
    let mut map = HashMap::new();
    let mut reader = Reader::from_reader(bytes);
    reader.trim_text(true).expand_empty_elements(true);

    loop {
        let event = match reader.read_event() {
            Ok(it) => it,
            Err(_) => break,
        };
        match event {
            Event::Start(e) => {
                if e.local_name().as_ref() == b"torrent" {
                    match parse_torrent_tag(&mut reader) {
                        Ok((link, time)) => {
                            map.insert(link, time);
                        }
                        Err(_) => continue,
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    map
}

fn parse_torrent_tag(reader: &mut Reader<&[u8]>) -> Result<(String, DateTime<FixedOffset>)> {
    let mut link = String::default();
    let mut pub_date = String::default();

    loop {
        match reader.read_event()? {
            Event::Start(e) => match e.local_name().as_ref() {
                b"link" => link = element_text(reader)?,
                b"pubDate" => pub_date = element_text(reader)?,
                _ => {}
            },
            Event::End(e) if e.local_name().as_ref() == b"torrent" => break,
            Event::Eof => break,
            _ => {}
        }
    }

    Ok((link, parse_rfc3339_date(pub_date)?))
}

fn parse_rfc3339_date(mut date: String) -> Result<DateTime<FixedOffset>> {
    match DateTime::parse_from_rfc3339(&date) {
        // 当出现错误时，添加蜜柑默认时区(+8)重试一次
        Err(e) if e.kind() == TooShort => {
            date.push_str("+08:00");
            Ok(DateTime::parse_from_rfc3339(&date)?)
        }
        it => Ok(it?),
    }
}

fn element_text(reader: &mut Reader<&[u8]>) -> Result<String> {
    let mut content = String::new();

    loop {
        match reader.read_event()? {
            Event::Start(element) => {
                reader.read_to_end(element.name())?;
            }
            Event::Text(element) => {
                let decoded = element.unescape()?;
                content.push_str(decoded.as_ref());
            }
            Event::CData(element) => {
                content.push_str(&reader.decoder().decode(&element)?);
            }
            Event::End(_) | Event::Eof => break,
            _ => {}
        }
    }

    Ok(content)
}
