use anyhow::Result;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;

use super::util::{decode, element_text};
use super::xml_ext::{BytesWriter, SerdeXml};

pub struct Guid {
    /// The value of the GUID.
    pub value: String,
    /// Indicates if the GUID is a permalink.
    pub permalink: bool,
}

impl Guid {
    pub(super) fn new(value: impl Into<String>) -> Self {
        let mut guid = Self::default();
        guid.value = value.into();
        guid
    }
}

impl SerdeXml for Guid {
    fn from_xml(reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self> {
        let mut guid = Guid::default();

        for attr in element.attributes().with_checks(false).flatten() {
            if decode(attr.key.as_ref(), reader)?.as_ref() == "isPermaLink" {
                guid.permalink = &*attr.value != b"false";
                break;
            }
        }

        guid.value = element_text(reader)?.unwrap_or_default();
        Ok(guid)
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        let name = "guid";
        let mut element = BytesStart::new(name);
        if !self.permalink {
            element.push_attribute(("isPermaLink", "false"));
        }
        writer.write_event(Event::Start(element))?;
        writer.write_event(Event::Text(BytesText::new(&self.value)))?;
        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

impl Default for Guid {
    #[inline]
    fn default() -> Self {
        Self {
            value: Default::default(),
            permalink: true,
        }
    }
}
