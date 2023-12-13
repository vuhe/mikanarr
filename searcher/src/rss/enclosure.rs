use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use super::util::{attr_value, decode, skip};
use super::xml_ext::{BytesWriter, SerdeXml};

#[derive(Default)]
pub struct Enclosure {
    /// The URL of the enclosure.
    pub url: String,
    /// The length of the enclosure in bytes.
    pub length: String,
    /// The MIME type of the enclosure.
    pub mime_type: String,
}

impl SerdeXml for Enclosure {
    fn from_xml(reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        let mut enclosure = Self::default();
        for attr in element.attributes().with_checks(false).flatten() {
            match decode(attr.key.as_ref(), reader)?.as_ref() {
                "url" => enclosure.url = attr_value(&attr, reader)?.to_string(),
                "length" => enclosure.length = attr_value(&attr, reader)?.to_string(),
                "type" => enclosure.mime_type = attr_value(&attr, reader)?.to_string(),
                _ => {}
            }
        }
        skip(element.name(), reader)?;
        Ok(enclosure)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        let name = "enclosure";

        let mut element = BytesStart::new(name);

        element.push_attribute(("url", self.url.as_str()));
        element.push_attribute(("length", self.length.as_str()));
        element.push_attribute(("type", self.mime_type.as_str()));

        writer.write_event(Event::Empty(element))?;
        Ok(())
    }
}
