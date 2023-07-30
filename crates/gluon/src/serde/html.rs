use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use scraper::Html;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use super::AsFormat;
use crate::err::GluonError;

pub trait AsHtml: AsFormat {
    fn as_html(&self) -> Result<Dom, GluonError>;
    fn strip_html(&self) -> Result<Dom, GluonError>;
    fn strip_htmls(&self) -> Result<Vec<Dom>, GluonError>;
}

impl<'a> AsHtml for &'a str {
    fn as_html(&self) -> Result<Dom, GluonError> {
        let deserializer = |s: &str| deserialize_html(s);

        self.as_format(deserializer)
    }

    fn strip_html(&self) -> Result<Dom, GluonError> {
        let deserializer = |s: &str| deserialize_html(s);

        self.strip_format(deserializer, "html")
    }

    fn strip_htmls(&self) -> Result<Vec<Dom>, GluonError> {
        let deserializer = |s: &str| deserialize_html(s);

        self.strip_formats(deserializer, "html")
    }
}

#[derive(Debug)]
pub struct Dom(Html);

// TODO: impl AsRef
impl Deref for Dom {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct DomVisitor;

impl<'de> Visitor<'de> for DomVisitor {
    type Value = Dom;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Dom(Html::parse_document(s)))
    }
}

impl<'de> Deserialize<'de> for Dom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DomVisitor)
    }
}

fn deserialize_html(html_str: &str) -> Result<Dom, GluonError> {
    Ok(Dom(Html::parse_document(html_str)))
}
