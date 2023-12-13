use std::borrow::Cow;

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

pub(super) fn decode<'s>(bytes: &'s [u8], reader: &Reader<&[u8]>) -> Result<Cow<'s, str>> {
    Ok(reader.decoder().decode(bytes)?)
}

pub(super) fn skip(end: QName, reader: &mut Reader<&[u8]>) -> Result<()> {
    reader.read_to_end(end)?;
    Ok(())
}

pub(super) fn attr_value<'s>(attr: &Attribute<'s>, reader: &Reader<&[u8]>) -> Result<Cow<'s, str>> {
    Ok(attr.decode_and_unescape_value(reader)?)
}

pub(super) fn element_text(reader: &mut Reader<&[u8]>) -> Result<Option<String>> {
    let mut content = String::new();

    loop {
        match reader.read_event()? {
            Event::Start(element) => {
                skip(element.name(), reader)?;
            }
            Event::Text(element) => {
                let decoded = element.unescape()?;
                content.push_str(decoded.as_ref());
            }
            Event::CData(element) => {
                content.push_str(decode(&element, reader)?.as_ref());
            }
            Event::End(_) | Event::Eof => break,
            _ => {}
        }
    }

    Ok(Some(content).filter(|c| !c.is_empty()))
}

pub(super) fn element_date(reader: &mut Reader<&[u8]>) -> Result<Option<DateTime<FixedOffset>>> {
    let mut date_string = match element_text(reader) {
        Ok(Some(it)) => it,
        Ok(None) => return Ok(None),
        Err(e) => return Err(e),
    };

    // 优先尝试 rss 标准 rfc2822
    if let Ok(date) = DateTime::parse_from_rfc2822(&date_string) {
        return Ok(Some(date));
    }

    // 其次尝试 rss 标准 rfc2822
    if let Ok(date) = DateTime::parse_from_rfc3339(&date_string) {
        return Ok(Some(date));
    }

    // 添加时区做最终尝试
    date_string.push_str("+08:00");
    return match DateTime::parse_from_rfc3339(&date_string) {
        Ok(it) => Ok(Some(it)),
        Err(e) => {
            log::debug!("{e:?}");
            log::warn!("rss parse date `{date_string}` fail: {e}");
            Ok(None)
        }
    };
}
