use rustpython_parser::{ast::Suite, parser};
use std::ops::{Deref, DerefMut};

use super::AsFormat;
use crate::err::GluonError;

pub trait AsPython: AsFormat {
    fn as_python(&self) -> Result<Python, GluonError>;
    fn strip_python(&self) -> Result<Python, GluonError>;
    fn strip_pythons(&self) -> Result<Vec<Python>, GluonError>;
}

impl<'a> AsPython for &'a str {
    fn as_python(&self) -> Result<Python, GluonError> {
        let deserializer = |s: &str| deserialize_python(s);

        self.as_format(deserializer)
    }

    fn strip_python(&self) -> Result<Python, GluonError> {
        let deserializer = |s: &str| deserialize_python(s);

        self.strip_format(deserializer, "python")
    }

    fn strip_pythons(&self) -> Result<Vec<Python>, GluonError> {
        let deserializer = |s: &str| deserialize_python(s);

        self.strip_formats(deserializer, "python")
    }
}

#[derive(Debug)]
pub struct Python(Suite);

impl AsRef<Suite> for Python {
    fn as_ref(&self) -> &Suite {
        &self.0
    }
}

impl Deref for Python {
    type Target = Suite;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Python {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deserialize_python(python_str: &str) -> Result<Python, GluonError> {
    // Tokenize the source code
    // let lexer = lexer::make_tokenizer(python_str);

    // Parse the tokens into an AST
    let ast = parser::parse_program(python_str, "<stdin>")?; //.map_error(|e| );

    Ok(Python(ast))
}
