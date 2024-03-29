///< Contains utility functions and helpers.
use crate::openai::request::chat_raw;
use crate::openai::{msg::OpenAIMsg, params::OpenAIParams};
use crate::JsError;
use anyhow::{anyhow, Result};
use js_sys::Function;
use parser::parser::json::AsJson;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::hash::Hash;
use wasm_bindgen::JsValue;
use web_sys::console;

pub async fn write_json(
    ai_params: &OpenAIParams,
    prompts: &Vec<&OpenAIMsg>,
    request_callback: &Function,
) -> Result<(String, Value)> {
    let mut retries = 3;

    loop {
        log("[INFO] Prompting the LLM..."); // TODO: remove this log in the next release

        let chat =
            chat_raw(request_callback, ai_params, prompts, &[], &[]).await?;

        let answer = chat
            .choices
            .first()
            .ok_or_else(|| anyhow!("LLM Respose seems to be empty :("))?
            .message
            .content
            .clone();

        match answer.as_str().strip_json() {
            Ok(result) => {
                log("[INFO] Received LLM answer...");
                break Ok((answer, result));
            }
            Err(e) => {
                log(&format!("Failed to parse json: \n{}", e));
                retries -= 1;

                if retries <= 0 {
                    return Err(anyhow!("Failed to parse json."));
                }

                log("Retrying...");
            }
        }
    }
}

// TODO: This function is on life support and it will be removed in the next serde generalisatoin cycles.
pub fn jsvalue_to_hmap<K: DeserializeOwned + Eq + Hash, T: DeserializeOwned>(
    value: JsValue,
) -> Result<HashMap<K, T>, JsError> {
    serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsError::from_str(&e.to_string()))
}

pub fn log(msg: &str) {
    console::log_1(&JsValue::from_str(msg));
}

pub fn log_err(msg: &str) {
    console::error_1(&JsValue::from_str(&msg));
}
