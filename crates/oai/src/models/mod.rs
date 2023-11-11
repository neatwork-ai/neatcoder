pub mod chat;
pub mod message;
pub mod role;

use ::anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Models {
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
}

impl Models {
    pub fn from_str(file_purpose: &str) -> Result<Self> {
        serde_json::from_str(file_purpose).map_err(Into::into)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Models::Gpt432k => "gpt-4-32k",
            Models::Gpt4 => "gpt-4",
            Models::Gpt35Turbo => "gpt-3.5-turbo",
            Models::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
            Models::Gpt35Turbo1106 => "gpt-3.5-turbo-1106",
            Models::Gpt41106Preview => "gpt-4-1106-preview",
        }
    }
}
