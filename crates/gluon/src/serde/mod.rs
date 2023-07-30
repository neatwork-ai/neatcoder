use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::err::GluonError;

pub mod csv;
pub mod json;
pub mod yaml;
pub mod html;

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
