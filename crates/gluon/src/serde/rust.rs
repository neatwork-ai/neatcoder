use std::ops::{Deref, DerefMut};
use syn::File;

use super::AsFormat;
use crate::err::GluonError;

pub trait AsRust: AsFormat {
    fn as_rust(&self) -> Result<Rust, GluonError>;
    fn strip_rust(&self) -> Result<Rust, GluonError>;
    fn strip_rusts(&self) -> Result<Vec<Rust>, GluonError>;
}

impl<'a> AsRust for &'a str {
    fn as_rust(&self) -> Result<Rust, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.as_format(deserializer)
    }

    fn strip_rust(&self) -> Result<Rust, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.strip_format(deserializer, "rust")
    }

    fn strip_rusts(&self) -> Result<Vec<Rust>, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.strip_formats(deserializer, "rust")
    }
}

#[derive(Debug)]
pub struct Rust(File);

impl AsRef<File> for Rust {
    fn as_ref(&self) -> &File {
        &self.0
    }
}

impl Deref for Rust {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rust {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deserialize_rust(rust_str: &str) -> Result<Rust, GluonError> {
    let syntax_tree = syn::parse_file(rust_str)?;

    Ok(Rust(syntax_tree))
}
