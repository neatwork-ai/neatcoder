use ::anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GptModel {
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

impl GptModel {
    pub fn from_str(file_purpose: &str) -> Result<Self> {
        serde_json::from_str(file_purpose).map_err(Into::into)
    }

    pub fn as_str(&self) -> &str {
        match self {
            GptModel::Gpt432k => "gpt-4-32k",
            GptModel::Gpt4 => "gpt-4",
            GptModel::Gpt35Turbo => "gpt-3.5-turbo",
            GptModel::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
            GptModel::Gpt35Turbo1106 => "gpt-3.5-turbo-1106",
            GptModel::Gpt41106Preview => "gpt-4-1106-preview",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", rename = "Model")]
pub struct ModelData {
    pub id: String,
    pub uri: String,
    pub interface: String,
}

impl ModelData {
    pub fn new(id: String) -> ModelData {
        Self {
            id,
            // TODO: Should not be hardcoded
            uri: String::from("https://api.openai.com/v1/chat/completions"),
            interface: String::from("OpenAI"),
        }
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use derive_more::{AsRef, Deref, DerefMut};
    use js_sys::JsString;

    #[wasm_bindgen]
    #[derive(Debug, Deserialize, Serialize, Clone, AsRef, Deref, DerefMut)]
    #[serde(rename_all = "camelCase", rename = "Model")]
    pub struct ModelDataWasm(pub(crate) ModelData);

    #[wasm_bindgen]
    impl ModelDataWasm {
        #[wasm_bindgen(constructor)]
        pub fn new(id: String) -> ModelDataWasm {
            ModelDataWasm(ModelData::new(id))
        }

        #[wasm_bindgen(getter)]
        pub fn id(&self) -> JsString {
            self.id.clone().into()
        }

        #[wasm_bindgen(getter)]
        pub fn uri(&self) -> JsString {
            self.uri.clone().into()
        }

        #[wasm_bindgen(getter)]
        pub fn interface(&self) -> JsString {
            self.interface.clone().into()
        }
    }
}
