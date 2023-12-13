use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use super::util::{attr_value, decode, skip};
use super::xml_ext::{BytesWriter, SerdeXml};

enum TorznabName {
    Size,
    Category,
    Unknown,
}

impl TorznabName {
    fn form_str(s: impl AsRef<str>) -> Self {
        match s.as_ref() {
            "size" => Self::Size,
            "category" => Self::Category,
            _ => Self::Unknown,
        }
    }
}

#[derive(Default)]
pub struct TorznabExt {
    pub size: String,
    pub category: String,
}

impl TorznabExt {
    pub fn append(mut self, reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        let mut name = TorznabName::Unknown;
        let mut value = None;
        for attr in element.attributes().with_checks(false).flatten() {
            match decode(attr.key.as_ref(), reader)?.as_ref() {
                "name" => name = TorznabName::form_str(attr_value(&attr, reader)?),
                "value" => value = Some(attr_value(&attr, reader)?.to_string()),
                _ => {}
            }
        }
        match name {
            TorznabName::Size => self.size = value.unwrap_or_default(),
            TorznabName::Category => self.category = value.unwrap_or_default(),
            TorznabName::Unknown => {}
        }
        skip(element.name(), reader)?;
        Ok(self)
    }
}

impl SerdeXml for TorznabExt {
    fn from_xml(reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        Self::default().append(reader, element)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        fn event<'a>(name: &'a str, value: &'a str) -> Event<'a> {
            let mut element = BytesStart::new("torznab:attr");
            element.push_attribute(("name", name));
            element.push_attribute(("value", value));
            Event::Empty(element)
        }

        writer.write_event(event("size", self.size.as_str()))?;
        writer.write_event(event("category", self.category.as_str()))?;

        Ok(())
    }
}
