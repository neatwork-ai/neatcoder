use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

use crate::err::GluonError;

pub trait AsFormat {
    fn as_format<F, T, E>(msg: &str, deserializer: F) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug,
    {
        deserializer(msg).map_err(|e| e.into())
    }

    fn strip_format<F, T, E>(msg: &str, deserializer: F, format: &str) -> Result<T, GluonError>
    where
        F: Fn(&str) -> Result<T, E> + Copy,
        E: Into<GluonError>,
        T: DeserializeOwned + Debug,
    {
        // // TODO: Generalise whenever needed
        let start_delimiter_string = format!("```{}", format);
        let start_delimiter = start_delimiter_string.as_str();
        let end_delimiter = "```";

        let start_loc = msg.find(start_delimiter).expect(&format!(
            "Unable to convert LLM output to {fmt}. The {fmt} object seems to be missing in: \n{}",
            msg,
            fmt = format,
        ));

        let start_index = start_loc + start_delimiter.len();

        let end_loc = msg[start_index..].find(end_delimiter).expect(&format!(
            "Unable to convert LLM output to {fmt}. Could not find ending backticks '```': \n{}",
            msg,
            fmt = format,
        ));

        let end_index = start_index + end_loc;
        let format_str = &msg[start_index..end_index];

        Self::as_format(format_str, deserializer)
    }

    fn strip_formats<F, T, E>(
        msg: &String,
        deserializer: F,
        format: &str,
    ) -> Result<Vec<T>, GluonError>
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

        let mut msg_ = msg.as_str();

        while let Some(start_loc) = msg_.find(start_delimiter) {
            let start_index = start_loc + start_delimiter.len();

            let format = if let Some(end_loc) = msg_[start_index..].find(end_delimiter) {
                let end_index = start_index + end_loc;

                let format_str = msg_[start_index..end_index + end_delimiter.len()].to_string();

                msg_ = &msg_[end_index + end_delimiter.len()..];

                format_str
            } else {
                // TODO: This can be handled gracefully insted
                panic!(
                    "Unable to parse last {fmt} {}",
                    &msg_[start_index..],
                    fmt = format
                );
            };

            formats.push(Self::as_format(&format, deserializer)?);
        }

        Ok(formats)
    }
}

pub trait AsJson: AsFormat {
    fn as_json(msg: &str) -> Result<Value, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        Self::as_format(msg, deserializer)
    }

    // Assumes that the json is encapsulated in ```json{actual_json}``` which is how OpenAI does it
    fn strip_json(msg: &str) -> Result<Value, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        Self::strip_format(msg, deserializer, "json")
    }

    fn strip_jsons(msg: &String) -> Result<Vec<Value>, GluonError> {
        // The function `serde_json::from_str` has a signature of
        // `fn(&'a str) -> Result<T, serde_json::Error>`. In this case, 'a
        // is tied to the specific input str's lifetime, it is not for any
        // possible lifetime 'a, hence it can't satisfy the for<'a> in
        // the higher-rank trait bound.
        //
        // To solve this problem, we wrap `serde_json::from_str` in a
        // closure that has a HRTB
        let deserializer = |s: &str| serde_json::from_str(s);

        Self::strip_formats(msg, deserializer, "json")
    }
}
// pub trait AsJson {
//     fn as_json(msg: &str) -> Value {
//         serde_json::from_str(msg).expect(&format!("Unable to convert LLM output to json: {}", msg))
//     }

//     // Assumes that the json is encapsulated in ```json{actual_json}``` which is how OpenAI does it
//     fn strip_json(msg: &str) -> Value {
//         // TODO: Generalise whenever needed
//         let start_delimiter = "```json";
//         let end_delimiter = "```";

//         let start_loc = msg.find(start_delimiter).expect(&format!(
//             "Unable to convert LLM output to json. The json object seems to be missing in: \n{}",
//             msg
//         ));

//         let start_index = start_loc + start_delimiter.len();

//         let end_loc = msg[start_index..].find(end_delimiter).expect(&format!(
//             "Unable to convert LLM output to json. Could not find ending backticks '```': \n{}",
//             msg
//         ));

//         let end_index = start_index + end_loc;
//         let json_str = &msg[start_index..end_index];

//         Self::as_json(json_str)
//     }

//     fn strip_jsons(msg: &String) -> Vec<Value> {
//         // TODO: Generalise whenever needed
//         let start_delimiter = "```json";
//         let end_delimiter = "```";

//         let mut jsons = Vec::new();

//         let mut msg_ = msg.as_str();

//         while let Some(start_loc) = msg_.find(start_delimiter) {
//             let start_index = start_loc + start_delimiter.len();

//             let json = if let Some(end_loc) = msg_[start_index..].find(end_delimiter) {
//                 let end_index = start_index + end_loc;

//                 let json_str = msg_[start_index..end_index + end_delimiter.len()].to_string();

//                 msg_ = &msg_[end_index + end_delimiter.len()..];

//                 json_str
//             } else {
//                 // TODO: This can be handled gracefully insted
//                 panic!("Unable to parse last json {}", &msg_[start_index..]);
//             };

//             jsons.push(Self::as_json(&json));
//         }

//         jsons
//     }
// }

pub trait AsYaml {
    fn as_yaml(msg: &str) -> Value {
        serde_yaml::from_str(msg).expect(&format!("Unable to convert LLM output to yaml: {}", msg))
    }

    // Assumes that the yaml is encapsulated in ```yaml{actual_yaml}``` which is how OpenAI does it
    fn strip_yaml(msg: &str) -> Value {
        // // TODO: Generalise whenever needed
        let start_delimiter = "```yaml";
        let end_delimiter = "```";

        let start_loc = msg.find(start_delimiter).expect(&format!(
            "Unable to convert LLM output to yaml. The yaml object seems to be missing in: \n{}",
            msg
        ));

        let start_index = start_loc + start_delimiter.len();

        let end_loc = msg[start_index..].find(end_delimiter).expect(&format!(
            "Unable to convert LLM output to yaml. Could not find ending backticks '```': \n{}",
            msg
        ));

        let end_index = start_index + end_loc;
        let yaml_str = &msg[start_index..end_index];

        Self::as_yaml(yaml_str)
    }

    fn strip_yamls(msg: &String) -> Vec<Value> {
        // TODO: Generalise whenever needed
        let start_delimiter = "```yaml";
        let end_delimiter = "```";

        let mut yamls = Vec::new();

        let mut msg_ = msg.as_str();

        while let Some(start_loc) = msg_.find(start_delimiter) {
            let start_index = start_loc + start_delimiter.len();

            let yaml = if let Some(end_loc) = msg_[start_index..].find(end_delimiter) {
                let end_index = start_index + end_loc;

                let yaml_str = msg_[start_index..end_index + end_delimiter.len()].to_string();

                msg_ = &msg_[end_index + end_delimiter.len()..];

                yaml_str
            } else {
                // TODO: This can be handled gracefully insted
                panic!("Unable to parse last yaml {}", &msg_[start_index..]);
            };

            yamls.push(Self::as_yaml(&yaml));
        }

        yamls
    }
}
