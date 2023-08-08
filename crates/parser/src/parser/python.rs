use rustpython_parser::{ast::Suite, parser};

use super::AsFormat;
use crate::err::ParseError;

/// Trait providing methods for working with Python code.
pub trait AsPython: AsFormat {
    /// Converts the object to a Python AST.
    fn as_python(&self) -> Result<Python, ParseError>;

    /// Strips the Python formatting, expecting encapsulation as in OpenAI's format, and returns the Python AST.
    fn strip_python(&self) -> Result<Python, ParseError>;

    /// Strips multiple Python code blocks, assuming the same encapsulation as `strip_python`.
    fn strip_pythons(&self) -> Result<Vec<Python>, ParseError>;
}

impl<'a> AsPython for &'a str {
    /// Implementation for converting a string slice to a Python AST.
    fn as_python(&self) -> Result<Python, ParseError> {
        self.as_format(deserialize_python)
    }

    /// Implementation for stripping Python code from a string slice, assuming encapsulation like OpenAI.
    fn strip_python(&self) -> Result<Python, ParseError> {
        self.strip_format(deserialize_python, "python")
    }

    /// Implementation for stripping multiple Python code blocks from a string slice.
    fn strip_pythons(&self) -> Result<Vec<Python>, ParseError> {
        self.strip_formats(deserialize_python, "python")
    }
}

/// Struct representing a Python code block with both raw text and parsed AST.
#[derive(Debug)]
pub struct Python {
    /// Raw text of the Python code
    pub raw: String,
    /// Abstract syntax tree (AST) representation
    pub ast: Suite,
}

/// Function to deserialize a Python code string into a `Python` struct.
///
/// # Arguments
/// * `python_str` - The Python code string to be deserialized.
///
/// # Returns
/// * A `Result` containing a `Python` struct if successful, or a `ParseError` if an error occurred.
fn deserialize_python(python_str: &str) -> Result<Python, ParseError> {
    // Tokenize the source code
    // let lexer = lexer::make_tokenizer(python_str);

    // Parse the tokens into an AST
    let ast = parser::parse_program(python_str, "<stdin>")?; //.map_error(|e| );

    Ok(Python {
        raw: python_str.to_string(),
        ast,
    })
}
