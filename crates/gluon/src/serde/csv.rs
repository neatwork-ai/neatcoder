use csv::{Reader, StringRecord};
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{
    fmt::{self, Debug},
    io::Cursor,
    ops::{Deref, DerefMut},
};

use super::AsFormat;
use crate::err::GluonError;

// TODO: Implement AsRef
#[derive(Debug)]
pub struct CsvTable(Vec<CsvRow>);

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

struct RowVisitor;

impl<'de> Visitor<'de> for RowVisitor {
    type Value = CsvRow;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<CsvRow, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut string_record = StringRecord::new();
        while let Some(value) = seq.next_element::<String>()? {
            string_record.push_field(&value);
        }
        Ok(CsvRow(string_record))
    }
}

impl<'de> Deserialize<'de> for CsvRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RowVisitor)
    }
}

struct TableVisitor;

impl<'de> Visitor<'de> for TableVisitor {
    type Value = CsvTable;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<CsvTable, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut string_record = Vec::new();
        while let Some(value) = seq.next_element::<CsvRow>()? {
            string_record.push(value);
        }
        Ok(CsvTable(string_record))
    }
}

impl<'de> Deserialize<'de> for CsvTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TableVisitor)
    }
}

pub trait AsCsv: AsFormat {
    fn as_csv(&self) -> Result<CsvTable, GluonError>;
    fn strip_csv(&self) -> Result<CsvTable, GluonError>;
    fn strip_csvs(&self) -> Result<Vec<CsvTable>, GluonError>;
}

impl<'a> AsCsv for &'a str {
    fn as_csv(&self) -> Result<CsvTable, GluonError> {
        // The function `serde_yaml::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_yaml::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_yaml::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| deserialize_csv(s);

        self.as_format(deserializer)
    }

    // Assumes that the yaml is encapsulated in ```yaml{actual_yaml}``` which is how OpenAI does it
    fn strip_csv(&self) -> Result<CsvTable, GluonError> {
        // The function `serde_yaml::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_yaml::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_yaml::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_yaml::from_str(s);

        self.strip_format(deserializer, "yaml")
    }

    fn strip_csvs(&self) -> Result<Vec<CsvTable>, GluonError> {
        // The function `serde_yaml::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_yaml::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_yaml::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_yaml::from_str(s);

        self.strip_formats(deserializer, "yaml")
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
