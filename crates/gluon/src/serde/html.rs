use scraper::Html;

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
pub struct Dom {
    pub raw: String,
    pub html: Html,
}

fn deserialize_html(html_str: &str) -> Result<Dom, GluonError> {
    Ok(Dom {
        raw: html_str.to_string(),
        html: Html::parse_document(html_str),
    })
}
