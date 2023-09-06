use crate::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use anyhow::{anyhow, Result};
use js_sys::Reflect;
use parser::parser::json::AsJson;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;

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

pub fn from_extern<ExternType: JsCast, V: DeserializeOwned>(
    extern_schemas: ExternType,
) -> Result<BTreeMap<String, V>, JsValue> {
    let type_name = std::any::type_name::<ExternType>();

    let js_value = extern_schemas.dyn_into::<JsValue>().map_err(|_| {
        JsValue::from_str(&format!(
            "Failed to convert {} to JsValue",
            type_name
        ))
    })?;

    serde_wasm_bindgen::from_value(js_value).map_err(|e| {
        let error_msg = format!(
            "Failed to convert {} JsValue to BTreeMap<String, {}>: {:?}",
            type_name,
            std::any::type_name::<V>(),
            e,
        );
        console::error_1(&JsValue::from_str(&error_msg));
        JsValue::from_str(&error_msg)
    })
}

pub fn to_extern<ExternType: JsCast>(
    map: BTreeMap<String, String>,
) -> Result<ExternType, JsValue> {
    // Create a new JavaScript object
    let js_object = js_sys::Object::new();

    // Set properties on the JavaScript object from the BTreeMap
    for (key, value) in map.iter() {
        Reflect::set(&js_object, &key.into(), &value.into()).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to set property on JsValue: {:?}",
                e
            ))
        })?;
    }

    // Try to cast the JsValue to ExternType
    // let ischemas: ExternType = js_object.dyn_into().map_err(|e| {
    //     JsValue::from_str(&format!(
    //         "Failed to cast Object `{:?}` to ExternType. Error: {:?}",
    //         js_object, e
    //     ))
    // })?;

    // Unchecked here can potentially lead to problems down the line.
    // However somehow `js_object.dyn_into()` messes up the casting..
    let extern_type = js_object.unchecked_into::<ExternType>();

    Ok(extern_type)
}
