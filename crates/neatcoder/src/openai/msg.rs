use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIMsg {
    pub role: GptRole,
    pub(crate) content: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum GptRole {
    System,
    User,
    Assistant,
}

impl OpenAIMsg {
    pub fn user(content: &str) -> Self {
        Self {
            role: GptRole::User,
            content: String::from(content),
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            role: GptRole::System,
            content: String::from(content),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: GptRole::Assistant,
            content: String::from(content),
        }
    }
}

impl Serialize for GptRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GptRole::System => serializer.serialize_str("system"),
            GptRole::User => serializer.serialize_str("user"),
            GptRole::Assistant => serializer.serialize_str("assistant"),
        }
    }
}

impl<'de> Deserialize<'de> for GptRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "system" => Ok(GptRole::System),
            "user" => Ok(GptRole::User),
            "assistant" => Ok(GptRole::Assistant),
            _ => panic!("Invalid variant `{:?}`", s.as_str()),
        }
    }
}

impl GptRole {
    pub fn new(role: &str) -> Result<Self> {
        let role = match role {
            "system" => GptRole::System,
            "user" => GptRole::User,
            "assistant" => GptRole::Assistant,
            _ => return Err(anyhow!(format!("Invalid role {}", role))),
        };

        Ok(role)
    }

    pub fn as_str(&self) -> &str {
        match self {
            GptRole::System => "system",
            GptRole::User => "user",
            GptRole::Assistant => "assistant",
        }
    }
}
