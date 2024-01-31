use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Role::System => serializer.serialize_str("system"),
            Role::User => serializer.serialize_str("user"),
            Role::Assistant => serializer.serialize_str("assistant"),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            _ => panic!("Invalid variant `{:?}`", s.as_str()),
        }
    }
}

impl Role {
    pub fn new(role: &str) -> Result<Self> {
        let role = match role {
            "system" => Role::System,
            "user" => Role::User,
            "assistant" => Role::Assistant,
            _ => return Err(anyhow!(format!("Invalid role {}", role))),
        };

        Ok(role)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}
