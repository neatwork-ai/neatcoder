use serde_json::Value;

use super::AsFormat;
use crate::err::GluonError;

pub trait AsJson: AsFormat {
    fn as_json(&self) -> Result<Value, GluonError>;
    fn strip_json(&self) -> Result<Value, GluonError>;
    fn strip_jsons(&self) -> Result<Vec<Value>, GluonError>;
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
