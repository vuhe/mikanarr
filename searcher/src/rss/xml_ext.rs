use anyhow::Result;
use bytes::BytesMut;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};

pub(super) type BytesWriter = Writer<bytes::buf::Writer<BytesMut>>;

pub(super) trait SerdeXml: Sized {
    fn from_xml(reader: &mut Reader<&[u8]>, element: BytesStart) -> Result<Self>;
    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()>;
}

impl<'a, T: SerdeXml> SerdeXml for &'a T {
    fn from_xml(_reader: &mut Reader<&[u8]>, _element: BytesStart) -> Result<Self> {
        unreachable!()
    }

    fn to_xml(&self, writer: &mut BytesWriter) -> Result<()> {
        (*self).to_xml(writer)
    }
}

pub(super) trait WriterExt {
    fn write_text_element(&mut self, name: impl AsRef<str>, text: impl AsRef<str>) -> Result<()>;

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<()>
    where
        N: AsRef<str>,
        T: AsRef<str>,
        I: IntoIterator<Item = T>;

    fn write_cdata_element(&mut self, name: impl AsRef<str>, text: impl AsRef<str>) -> Result<()>;

    fn write_object<T: SerdeXml>(&mut self, object: T) -> Result<()>;

    fn write_objects<T, I>(&mut self, objects: I) -> Result<()>
    where
        T: SerdeXml,
        I: IntoIterator<Item = T>;
}

impl WriterExt for BytesWriter {
    fn write_text_element(&mut self, name: impl AsRef<str>, text: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::new(name)))?;
        self.write_event(Event::Text(BytesText::new(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<()>
    where
        N: AsRef<str>,
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        for value in values {
            self.write_text_element(&name, value)?;
        }

        Ok(())
    }

    fn write_cdata_element(&mut self, name: impl AsRef<str>, text: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::new(name)))?;
        self.write_event(Event::CData(BytesCData::new(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    #[inline]
    fn write_object<T: SerdeXml>(&mut self, object: T) -> Result<()> {
        object.to_xml(self)
    }

    fn write_objects<T, I>(&mut self, objects: I) -> Result<()>
    where
        T: SerdeXml,
        I: IntoIterator<Item = T>,
    {
        for object in objects {
            object.to_xml(self)?;
        }

        Ok(())
    }
}
