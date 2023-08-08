use scraper::Html;

use super::AsFormat;
use crate::err::ParseError;

/// A trait to provide methods to parse HTML from a string.
pub trait AsHtml: AsFormat {
    /// Parse the complete string as HTML.
    ///
    /// # Returns
    /// - `Result<Dom, ParseError>`: The parsed DOM or a parse error.
    fn as_html(&self) -> Result<Dom, ParseError>;

    /// Extract and parse an HTML segment encapsulated by a specific format (e.g. "```html") within the string.
    ///
    /// # Returns
    /// - `Result<Dom, ParseError>`: The parsed DOM or a parse error.
    fn strip_html(&self) -> Result<Dom, ParseError>;

    /// Extract and parse multiple HTML segments encapsulated by a specific format within the string.
    ///
    /// # Returns
    /// - `Result<Vec<Dom>, ParseError>`: A vector of parsed DOMs or a parse error.
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

/// Represents a parsed HTML document.
#[derive(Debug)]
pub struct Dom {
    /// The raw HTML as a string.
    pub raw: String,
    /// The parsed HTML document.
    pub html: Html,
}

/// Deserialize an HTML string into a `Dom` object.
///
/// # Arguments
/// - `html_str`: A reference to the HTML string.
///
/// # Returns
/// - `Result<Dom, ParseError>`: The parsed DOM or a parse error.
fn deserialize_html(html_str: &str) -> Result<Dom, ParseError> {
    Ok(Dom {
        raw: html_str.to_string(),
        html: Html::parse_document(html_str),
    })
}
