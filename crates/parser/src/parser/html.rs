use scraper::Html;

use super::AsFormat;
use crate::err::ParseError;

pub trait AsHtml: AsFormat {
    fn as_html(&self) -> Result<Dom, ParseError>;
    fn strip_html(&self) -> Result<Dom, ParseError>;
    fn strip_htmls(&self) -> Result<Vec<Dom>, ParseError>;
}

impl<'a> AsHtml for &'a str {
    fn as_html(&self) -> Result<Dom, ParseError> {
        self.as_format(deserialize_html)
    }

    fn strip_html(&self) -> Result<Dom, ParseError> {
        self.strip_format(deserialize_html, "html")
    }

    fn strip_htmls(&self) -> Result<Vec<Dom>, ParseError> {
        self.strip_formats(deserialize_html, "html")
    }
}

#[derive(Debug)]
pub struct Dom {
    pub raw: String,
    pub html: Html,
}

fn deserialize_html(html_str: &str) -> Result<Dom, ParseError> {
    Ok(Dom {
        raw: html_str.to_string(),
        html: Html::parse_document(html_str),
    })
}
