use crate::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use anyhow::{anyhow, Result};
use parser::parser::json::AsJson;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::hash::Hash;
use wasm_bindgen::JsValue;

pub async fn write_json(
    client: &OpenAI,
    ai_params: &OpenAIParams,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Value)> {
    let mut retries = 3;

    loop {
        println!("[INFO] Prompting the LLM...");
        let answer = client.chat(ai_params, prompts, &[], &[]).await?;

        match answer.as_str().strip_json() {
            Ok(result) => {
                println!("[INFO] Received LLM answer...");
                break Ok((answer, result));
            }
            Err(e) => {
                println!("Failed to parse json: \n{}", e);
                retries -= 1;

                if retries <= 0 {
                    return Err(anyhow!("Failed to parse json."));
                }

                println!("Retrying...");
            }
        }
    }
}

// TODO: This function is on life support and it will be removed in the next serde generalisatoin cycles.
pub fn jsvalue_to_hmap<K: DeserializeOwned + Eq + Hash, T: DeserializeOwned>(
    value: JsValue,
) -> HashMap<K, T> {
    serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsError::from_str(&e.to_string()))
        .unwrap()
}
