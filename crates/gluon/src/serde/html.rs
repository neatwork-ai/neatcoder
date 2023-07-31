use scraper::Html;
use std::ops::{Deref, DerefMut};

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

impl AsRef<Html> for Dom {
    fn as_ref(&self) -> &Html {
        &self.0
    }
}

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

fn deserialize_html(html_str: &str) -> Result<Dom, GluonError> {
    Ok(Dom(Html::parse_document(html_str)))
}
