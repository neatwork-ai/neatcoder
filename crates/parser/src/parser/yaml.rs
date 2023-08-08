use serde::de::DeserializeOwned;
use serde_yaml::Value;

use super::AsFormat;
use crate::err::ParseError;

/// Function to create a type `T` by deserializing it from a YAML string contained within a prompt.
///
/// # Arguments
/// * `prompt` - A string that contains YAML to be deserialized.
///
/// # Returns
/// * A `Result` containing the deserialized object if successful, or a `ParseError` if an error occurred.
pub fn from_prompt<T: DeserializeOwned>(prompt: &str) -> Result<T, ParseError> {
    let yaml = prompt.strip_yaml()?;
    let obj = serde_yaml::from_value(yaml)?;

    Ok(obj)
}

/// Trait providing methods for working with YAML code.
pub trait AsYaml {
    /// Converts the object to a YAML value.
    fn as_yaml(&self) -> Result<Value, ParseError>;

    /// Strips the YAML formatting and returns the YAML value.
    fn strip_yaml(&self) -> Result<Value, ParseError>;

    /// Strips multiple YAML code blocks and returns them as a vector of `Value` objects.
    fn strip_yamls(&self) -> Result<Vec<Value>, ParseError>;
}

impl<'a> AsYaml for &'a str {
    /// Implementation of converting a string slice to a YAML value.
    fn as_yaml(&self) -> Result<Value, ParseError> {
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

    /// Implementation of stripping YAML code from a string slice.
    /// Assumes that the YAML is encapsulated in ```yaml{actual_yaml}```.
    fn strip_yaml(&self) -> Result<Value, ParseError> {
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

    /// Implementation of stripping multiple YAML code blocks from a string slice.
    fn strip_yamls(&self) -> Result<Vec<Value>, ParseError> {
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
