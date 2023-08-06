use rustpython_parser::{ast::Suite, parser};

use super::AsFormat;
use crate::err::ParseError;

pub trait AsPython: AsFormat {
    fn as_python(&self) -> Result<Python, ParseError>;
    fn strip_python(&self) -> Result<Python, ParseError>;
    fn strip_pythons(&self) -> Result<Vec<Python>, ParseError>;
}

impl<'a> AsPython for &'a str {
    fn as_python(&self) -> Result<Python, ParseError> {
        self.as_format(deserialize_python)
    }

    fn strip_python(&self) -> Result<Python, ParseError> {
        self.strip_format(deserialize_python, "python")
    }

    fn strip_pythons(&self) -> Result<Vec<Python>, ParseError> {
        self.strip_formats(deserialize_python, "python")
    }
}

#[derive(Debug)]
pub struct Python {
    pub raw: String,
    pub ast: Suite,
}

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
