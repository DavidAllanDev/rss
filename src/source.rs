// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use error::Error;
use fromxml::FromXml;
use toxml::ToXml;
use util::element_text;

/// Represents the source of an RSS item.
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Source {
    /// The URL of the source.
    url: String,
    /// The title of the source.
    title: Option<String>,
}

impl Source {
    /// Return the URL of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_url("http://example.com");
    /// assert_eq!(source.url(), "http://example.com");
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Set the URL of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_url("http://example.com");
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<String>,
    {
        self.url = url.into();
    }

    /// Return the title of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Source Title".to_string());
    /// assert_eq!(source.title(), Some("Source Title"));
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }

    /// Set the title of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Source Title".to_string());
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Option<String>>,
    {
        self.title = title.into();
    }
}

impl FromXml for Source {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, mut atts: Attributes) -> Result<Self, Error> {
        let mut source = Source::default();

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"url" {
                    source.url = attr.unescape_and_decode_value(reader)?;
                    break;
                }
            }
        }

        source.title = element_text(reader)?;
        Ok(source)
    }
}

impl ToXml for Source {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"source";
        let mut element = BytesStart::borrowed(name, name.len());
        element.push_attribute(("url", &*self.url));

        writer.write_event(Event::Start(element))?;

        if let Some(text) = self.title.as_ref().map(|s| s.as_bytes()) {
            writer.write_event(Event::Text(BytesText::borrowed(text)))?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}
