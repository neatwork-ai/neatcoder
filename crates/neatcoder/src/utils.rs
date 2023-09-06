use crate::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use anyhow::{anyhow, Result};
use parser::parser::json::AsJson;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use wasm_bindgen::JsValue;

// Convert a BTreeMap<String, String> to a JsValue
pub fn map_to_jsvalue<K: Serialize, V: Serialize>(
    map: &BTreeMap<K, V>,
) -> JsValue {
    JsValue::from_str(&serde_json::to_string(&map).unwrap())
}

// Convert a JsValue back to a BTreeMap<String, String>
pub fn jsvalue_to_map<
    K: DeserializeOwned + Eq + Hash + Ord,
    T: DeserializeOwned,
>(
    value: JsValue,
) -> BTreeMap<K, T> {
    // if value.is_null() // TODO
    serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsValue::from_str(&e.to_string()))
        .unwrap()
}

pub fn jsvalue_to_hmap<K: DeserializeOwned + Eq + Hash, T: DeserializeOwned>(
    value: JsValue,
) -> HashMap<K, T> {
    // if value.is_null() // TODO
    serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsValue::from_str(&e.to_string()))
        .unwrap()
}

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
