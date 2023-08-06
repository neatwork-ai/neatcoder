use std::fmt::Debug;

use crate::err::ParseError;

pub mod csv;
pub mod html;
pub mod json;
pub mod python;
pub mod rust;
pub mod sql;
pub mod yaml;

pub trait AsFormat {
    fn as_format<F, T, E>(&self, deserializer: F) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug;

    fn strip_format<F, T, E>(&self, deserializer: F, format: &str) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug;

    fn strip_formats<F, T, E>(&self, deserializer: F, format: &str) -> Result<Vec<T>, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug;
}

impl<'a> AsFormat for &'a str {
    fn as_format<F, T, E>(&self, deserializer: F) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug,
    {
        deserializer(self).map_err(|e| e.into())
    }

    fn strip_format<F, T, E>(&self, deserializer: F, format: &str) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug,
    {
        // // TODO: Generalise whenever needed
        let start_delimiter_string = format!("```{}", format);
        let start_delimiter = start_delimiter_string.as_str();
        let default_delimiter = "```";
        let mut default = false;

        let start_loc = self.find(start_delimiter).unwrap_or_else(|| {
            println!(
                "Unable to find the primary delimiter. Attempting to use the fallback delimiter in: \n{}",
                self
            );

            default = true;
        
            // Fallback logic: try finding the fallback delimiter
            self.find("```").expect(&format!(
                "Unable to convert LLM output to {fmt}. Delimiters seem to be missing in: \n{input}",
                input = self,
                fmt = format
            ))
        });

        let start_index = start_loc + if default != true {start_delimiter.len()} else {default_delimiter.len()};

        let end_loc = self[start_index..].find(default_delimiter).expect(&format!(
            "Unable to convert LLM output to {fmt}. Could not find ending backticks '```': \n{}",
            self,
            fmt = format,
        ));

        let end_index = start_index + end_loc;
        let format_str = &self[start_index..end_index];

        format_str.as_format(deserializer)
    }

    fn strip_formats<F, T, E>(&self, deserializer: F, format: &str) -> Result<Vec<T>, ParseError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<ParseError>,
        T: Debug,
    {
        // TODO: Generalise whenever needed
        // TODO: Need to implement fallback logig just like in the function `strip_format`
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
