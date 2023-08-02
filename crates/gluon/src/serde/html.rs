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
        self.as_format(deserialize_html)
    }

    fn strip_html(&self) -> Result<Dom, GluonError> {
        self.strip_format(deserialize_html, "html")
    }

    fn strip_htmls(&self) -> Result<Vec<Dom>, GluonError> {
        self.strip_formats(deserialize_html, "html")
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
