use serde::de::DeserializeOwned;
use serde_json::Value;

use super::AsFormat;
use crate::err::GluonError;

pub fn from_prompt<T: DeserializeOwned>(prompt: &str) -> Result<T, GluonError> {
    let json = prompt.strip_json()?;
    let obj = serde_json::from_value(json)?;

    Ok(obj)
}

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

#[cfg(test)]
mod test {
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    #[derive(Deserialize, Serialize)]
    struct TestStruct {
        field_a: String,
        field_b: u64,
        field_c: Option<String>,
    }

    #[test]
    fn test_parse() -> Result<()> {
        let expected = json!({
            "field_a": String::from("This is a string"),
            "field_b": 10,
            "field_c": None::<String>,
        });

        let json_string = expected.to_string();

        let obj_str = json_string.as_str();

        let prompt = format!(
            "Sure! Here is an example of an instance:\n```json\n{}\n```",
            obj_str
        );

        let actual = prompt.as_str().strip_json()?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
