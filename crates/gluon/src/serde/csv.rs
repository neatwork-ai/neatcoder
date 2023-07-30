use csv::{Reader, StringRecord};
use std::{
    fmt::Debug,
    io::Cursor,
    ops::{Deref, DerefMut},
};

use super::AsFormat;
use crate::err::GluonError;

#[derive(Debug)]
pub struct CsvTable(Vec<CsvRow>);

impl AsRef<Vec<CsvRow>> for CsvTable {
    fn as_ref(&self) -> &Vec<CsvRow> {
        &self.0
    }
}

impl Deref for CsvTable {
    type Target = Vec<CsvRow>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CsvTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct CsvRow(StringRecord);

impl Deref for CsvRow {
    type Target = StringRecord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CsvRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait AsCsv: AsFormat {
    fn as_csv(&self) -> Result<CsvTable, GluonError>;
    fn strip_csv(&self) -> Result<CsvTable, GluonError>;
    fn strip_csvs(&self) -> Result<Vec<CsvTable>, GluonError>;
}

impl<'a> AsCsv for &'a str {
    fn as_csv(&self) -> Result<CsvTable, GluonError> {
        let deserializer = |s: &str| deserialize_csv(s);

        self.as_format(deserializer)
    }

    // Assumes that the yaml is encapsulated in ```html{actual_html}``` which is how OpenAI does it
    fn strip_csv(&self) -> Result<CsvTable, GluonError> {
        let deserializer = |s: &str| deserialize_csv(s);

        self.strip_format(deserializer, "csv")
    }

    fn strip_csvs(&self) -> Result<Vec<CsvTable>, GluonError> {
        let deserializer = |s: &str| deserialize_csv(s);

        self.strip_formats(deserializer, "csv")
    }
}

fn deserialize_csv(input: &str) -> Result<CsvTable, GluonError> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut records = Vec::new();

    for record in reader.records() {
        records.push(CsvRow(record?));
    }

    Ok(CsvTable(records))
}
