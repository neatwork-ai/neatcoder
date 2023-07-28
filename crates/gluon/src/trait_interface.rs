use csv::StringRecord;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::err::GluonError;

pub trait AsFormat {
    fn as_format<F, T, E>(&self, deserializer: F) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug;

    fn strip_format<F, T, E>(&self, deserializer: F, format: &str) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug;

    fn strip_formats<F, T, E>(&self, deserializer: F, format: &str) -> Result<Vec<T>, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug;
}

pub trait AsJson: AsFormat {
    fn as_json(&self) -> Result<Value, GluonError>;
    fn strip_json(&self) -> Result<Value, GluonError>;
    fn strip_jsons(&self) -> Result<Vec<Value>, GluonError>;
}

pub trait AsYaml {
    fn as_yaml(&self) -> Result<Value, GluonError>;
    fn strip_yaml(&self) -> Result<Value, GluonError>;
    fn strip_yamls(&self) -> Result<Vec<Value>, GluonError>;
}

pub trait AsCsv: AsFormat {
    fn as_csv(&self) -> Result<Vec<StringRecord>, GluonError>;
    fn strip_csv(&self) -> Result<Vec<StringRecord>, GluonError>;
    fn strip_csvs(&self) -> Result<Vec<Vec<StringRecord>, GluonError>;
}

#[derive(Debug)]
pub struct CsvTable(Vec<StringRecord>);

impl Deref for CsvTable {
    type Target = Vec<StringRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CsvTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CsvTable {
    pub fn new(table: Vec<StringRecord>) -> Self {
        Self(table)
    }
}
