use crate::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use anyhow::{anyhow, Result};
use parser::parser::{
    json::AsJson,
    rust::{AsRust, Rust},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use wasm_bindgen::JsValue;

// Convert a HashMap<String, String> to a JsValue
pub fn map_to_jsvalue<T: Serialize>(map: &HashMap<String, T>) -> JsValue {
    JsValue::from_str(&serde_json::to_string(&map).unwrap())
}

// Convert a JsValue back to a HashMap<String, String>
pub fn jsvalue_to_map<T: DeserializeOwned>(
    value: &JsValue,
) -> HashMap<String, T> {
    serde_json::from_str::<HashMap<String, T>>(&value.as_string().unwrap())
        .unwrap()
}

pub async fn write_rust(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Rust)> {
    let mut retries = 3;

    loop {
        let answer = client.chat(params.clone(), prompts, &[], &[]).await?;

        match answer.as_str().strip_rust() {
            Ok(result) => {
                break Ok((answer, result));
            }
            Err(e) => {
                println!("Error while parsing rust code: \n{}", e);
                retries -= 1;

                if retries <= 0 {
                    return Err(anyhow!("Failed to parse rust code."));
                }

                println!("Retrying...");
            }
        }
    }
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
