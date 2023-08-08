use csv::{Reader, StringRecord};
use std::{
    fmt::Debug,
    io::Cursor,
    ops::{Deref, DerefMut},
};

use crate::err::ParseError;

use super::AsFormat;

/// A structure representing a CSV table, essentially a transparent
/// newtype wrapper around a Vector of `CsvRow`.
#[derive(Debug)]
pub struct CsvTable(Vec<CsvRow>);

impl AsRef<Vec<CsvRow>> for CsvTable {
    /// Provides a reference to the underlying Vector of `CsvRow`.
    fn as_ref(&self) -> &Vec<CsvRow> {
        &self.0
    }
}

impl Deref for CsvTable {
    type Target = Vec<CsvRow>;

    /// Dereferences to the underlying Vector of `CsvRow`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CsvTable {
    /// Mutably dereferences to the underlying Vector of `CsvRow`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A structure representing a CSV row, essentially a transparent
/// newtype wrapper around a `StringRecord`.
#[derive(Debug)]
pub struct CsvRow(StringRecord);

impl AsRef<StringRecord> for CsvRow {
    /// Provides a reference to the underlying `StringRecord`.
    fn as_ref(&self) -> &StringRecord {
        &self.0
    }
}

impl Deref for CsvRow {
    type Target = StringRecord;

    /// Dereferences to the underlying `StringRecord`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CsvRow {
    /// Mutably dereferences to the underlying `StringRecord`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A trait for types that can be converted to CSV, with methods for extracting
/// CSV tables directly or by stripping specific CSV delimiters.
pub trait AsCsv: AsFormat {
    /// Converts the object to a CSV table.
    fn as_csv(&self) -> Result<CsvTable, ParseError>;

    /// Strips the CSV format and converts the object to a CSV table.
    fn strip_csv(&self) -> Result<CsvTable, ParseError>;

    /// Strips multiple CSV formats and converts the object to a vector of CSV tables.
    fn strip_csvs(&self) -> Result<Vec<CsvTable>, ParseError>;
}

impl<'a> AsCsv for &'a str {
    /// Converts string slices to a CSV table.
    /// Assumes that the whole string is the CSV object
    fn as_csv(&self) -> Result<CsvTable, ParseError> {
        self.as_format(deserialize_csv)
    }

    /// Stripts the CSV object from a string slice and converts it into
    /// a native CsvTable object.
    /// Assumes there is only one object in the string.
    fn strip_csv(&self) -> Result<CsvTable, ParseError> {
        self.strip_format(deserialize_csv, "csv")
    }

    /// Iteratively stripts CSV objects from a string slice and converts them
    /// into a vector of native CsvTable object.
    /// Assumes that there may bemore than one object in the string.
    fn strip_csvs(&self) -> Result<Vec<CsvTable>, ParseError> {
        self.strip_formats(deserialize_csv, "csv")
    }
}

/// Deserializes a CSV string into a `CsvTable`.
///
/// # Parameters
/// * `input`: The input string containing CSV data.
///
/// # Returns
/// A result containing a `CsvTable` if the deserialization is successful, or a `ParseError` otherwise.
fn deserialize_csv(input: &str) -> Result<CsvTable, ParseError> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut records = Vec::new();

    for record in reader.records() {
        records.push(CsvRow(record?));
    }

    Ok(CsvTable(records))
}
