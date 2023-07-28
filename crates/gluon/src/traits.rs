use csv::{Reader, StringRecord};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::io::Cursor;
use std::{fmt::Debug, ops::DerefMut};

use crate::{
    err::GluonError,
    trait_interface::{AsCsv, AsFormat, AsJson, AsYaml, CsvTable},
};

impl<'a> AsFormat for &'a str {
    fn as_format<F, T, E>(&self, deserializer: F) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug,
    {
        deserializer(self).map_err(|e| e.into())
    }

    fn strip_format<F, T, E>(&self, deserializer: F, format: &str) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug,
    {
        // // TODO: Generalise whenever needed
        let start_delimiter_string = format!("```{}", format);
        let start_delimiter = start_delimiter_string.as_str();
        let end_delimiter = "```";

        let start_loc = self.find(start_delimiter).expect(&format!(
            "Unable to convert LLM output to {fmt}. The {fmt} object seems to be missing in: \n{}",
            self,
            fmt = format,
        ));

        let start_index = start_loc + start_delimiter.len();

        let end_loc = self[start_index..].find(end_delimiter).expect(&format!(
            "Unable to convert LLM output to {fmt}. Could not find ending backticks '```': \n{}",
            self,
            fmt = format,
        ));

        let end_index = start_index + end_loc;
        let format_str = &self[start_index..end_index];

        format_str.as_format(deserializer)
    }

    fn strip_formats<F, T, E>(&self, deserializer: F, format: &str) -> Result<Vec<T>, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug,
    {
        // TODO: Generalise whenever needed
        let start_delimiter_string = format!("```{}", format);
        let start_delimiter = start_delimiter_string.as_str();
        let end_delimiter = "```";

        let mut formats = Vec::new();

        let mut msg_ = *self;

        while let Some(start_loc) = msg_.find(start_delimiter) {
            let start_index = start_loc + start_delimiter.len();

            let format = if let Some(end_loc) = msg_[start_index..].find(end_delimiter) {
                let end_index = start_index + end_loc;

                // Improve this code block, ideally no need to create extra string?
                let format_string = msg_[start_index..end_index + end_delimiter.len()].to_string();
                msg_ = &msg_[end_index + end_delimiter.len()..];
                format_string
            } else {
                // TODO: This can be handled gracefully instead
                panic!(
                    "Unable to parse last {fmt} {}",
                    &msg_[start_index..],
                    fmt = format
                );
            };

            formats.push(format.as_str().as_format(deserializer)?);
        }

        Ok(formats)
    }
}

impl<'a> AsJson for &'a str {
    fn as_json(&self) -> Result<Value, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        self.as_format(deserializer)
    }

    // Assumes that the json is encapsulated in ```json{actual_json}``` which is how OpenAI does it
    fn strip_json(&self) -> Result<Value, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        self.strip_format(deserializer, "json")
    }

    fn strip_jsons(&self) -> Result<Vec<Value>, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        self.strip_formats(deserializer, "json")
    }
}

impl<'a> AsYaml for &'a str {
    fn as_yaml(&self) -> Result<Value, GluonError> {
        // The function `serde_yaml::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_yaml::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_yaml::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_yaml::from_str(s);

        self.as_format(deserializer)
    }

    // Assumes that the yaml is encapsulated in ```yaml{actual_yaml}``` which is how OpenAI does it
    fn strip_yaml(&self) -> Result<Value, GluonError> {
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

    fn strip_yamls(&self) -> Result<Vec<Value>, GluonError> {
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

impl<'a> AsCsv for &'a str {
    fn as_csv(&self) -> Result<Vec<StringRecord>, GluonError> {
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
    fn strip_csv(&self) -> Result<Vec<StringRecord>, GluonError> {
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

    fn strip_csvs(&self) -> Result<Vec<Vec<StringRecord>>, GluonError> {
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

// TODO: Implement this as a DeserializeOwned Trait

fn deserialize_csv(input: &str) -> Result<Vec<StringRecord>, GluonError> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut records = Vec::new();

    for record in reader.records() {
        records.push(record?);
    }

    // Ok(CsvTable::new(records))
    Ok(records)
}
