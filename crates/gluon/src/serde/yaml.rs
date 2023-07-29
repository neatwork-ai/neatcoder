use serde_yaml::Value;

use super::AsFormat;
use crate::err::GluonError;

pub trait AsYaml {
    fn as_yaml(&self) -> Result<Value, GluonError>;
    fn strip_yaml(&self) -> Result<Value, GluonError>;
    fn strip_yamls(&self) -> Result<Vec<Value>, GluonError>;
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
