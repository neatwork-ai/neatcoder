use super::{msg::OpenAIMsg, params::OpenAIParams, response::ResponseBody};
use crate::{utils::log, JsError, WasmType};
use anyhow::{anyhow, Result};
use js_sys::{Function, Promise};
use serde_json::{json, Value};
use serde_wasm_bindgen::from_value;
use std::ops::Deref;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<OpenAIMsg>")]
    pub type IOpenAIMsg;
}

#[wasm_bindgen(js_name = requestBody)]
pub fn request_body_(
    msgs: IOpenAIMsg,
    stream: bool,
) -> Result<JsValue, JsError> {
    let msgs: Vec<OpenAIMsg> = Vec::from_extern(msgs).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to convert msgs to native Wasm type: {:?}",
            e
        ))
    })?;

    log(&format!("Converted messages are: {:?}", msgs));

    let job = OpenAIParams::default();

    let mut data = json!({
        "model": job.model.as_string(),
        "messages": msgs,
        // "stop": self.stop,
    });

    if stream {
        data["stream"] = serde_json::Value::Bool(true);
    }

    log(&format!("Payload is: {:?}", data));

    // let data = serde_wasm_bindgen::to_value(&data).map_err(|e| {
    //     JsValue::from_str(&format!(
    //         "Failed to convert payload Value to JsValue: {:?}",
    //         e
    //     ))
    // })?;

    // Ok(data)

    let serialized_str = serde_json::to_string(&data).map_err(|e| {
        JsValue::from_str(&format!("Failed to serialize to string: {:?}", e))
    })?;

    Ok(JsValue::from_str(&serialized_str))
}

pub async fn chat(
    request_callback: &Function,
    ai_params: impl Deref<Target = OpenAIParams>,
    msgs: &[&OpenAIMsg],
    funcs: &[&String],
    stop_seq: &[String],
) -> Result<String> {
    log("[DEBUG] Getting Chat Raw...");
    let chat =
        chat_raw(request_callback, ai_params, msgs, funcs, stop_seq).await?;

    log("[DEBUG] Got answer.");

    let answer = chat
        .choices
        .first()
        .ok_or_else(|| anyhow!("LLM Respose seems to be empty :("))?
        .message
        .content
        .as_str();

    Ok(String::from(answer))
}

pub async fn chat_raw(
    request_callback: &Function,
    ai_params: impl Deref<Target = OpenAIParams>,
    msgs: &[&OpenAIMsg],
    funcs: &[&String],
    stop_seq: &[String],
) -> Result<ResponseBody> {
    let req_body = request_body(ai_params, msgs, funcs, stop_seq, false)?;

    let body_json = serde_json::to_string(&req_body)?;

    log("[DEBUG] Getting promise...");
    let js_promise: Promise = request_callback
        .call1(&JsValue::NULL, &JsValue::from_str(&body_json))
        .map_err(|e| anyhow!("Error performing request callback: {:?}", e))?
        .dyn_into()
        .map_err(|e| {
            anyhow!("Error processing request callback promise: {:?}", e)
        })?;
    log("[INFO] Prompting the LLM...");
    let res_js_value: JsValue =
        JsFuture::from(js_promise).await.map_err(|e| {
            anyhow!("Error processing request callback result: {:?}", e)
        })?;

    log("[INFO] Receive response from LLM..");
    log(&format!("LLM response body: {:?}", res_js_value));

    let body: Result<ResponseBody, _> = from_value(res_js_value);

    match body {
        Ok(body) => Ok(body),
        Err(e) => {
            Err(anyhow!("Could not convert JsValue to Response: {:?}", e))
        }
    }
}

pub fn request_stream(
    ai_params: impl Deref<Target = OpenAIParams>,
    msgs: &[&OpenAIMsg],
    funcs: &[&String],
    stop_seq: &[String],
) -> Result<String> {
    let req_body = request_body(ai_params, msgs, funcs, stop_seq, true)?;

    let body_json = serde_json::to_string(&req_body)?;

    Ok(body_json)
}

pub fn request_body(
    job: impl Deref<Target = OpenAIParams>,
    msgs: &[&OpenAIMsg],
    // TODO: Add to OpenAIParams
    funcs: &[&String],
    // TODO: Add to OpenAIParams
    stop_seq: &[String],
    stream: bool,
) -> Result<Value> {
    let mut data = json!({
        "model": job.model.as_string(),
        "messages": msgs,
        // "stop": self.stop,
    });

    if let Some(temperature) = job.temperature {
        data["temperature"] = serde_json::to_value(temperature)?;
    }

    if let Some(top_p) = job.top_p {
        data["top_p"] = serde_json::to_value(top_p)?;
    }

    if !funcs.is_empty() {
        data["functions"] = serde_json::to_value(funcs)?;
    }

    if !stop_seq.is_empty() {
        if stop_seq.len() > 4 {
            return Err(anyhow!(
                "Maximum limit of stop sequences reached. {} out of 4",
                stop_seq.len()
            ));
        };

        data["stop"] = serde_json::to_value(stop_seq)?;
    }

    if let Some(user) = &job.user {
        data["user"] = serde_json::to_value(user)?;
    }

    if !job.logit_bias.is_empty() {
        data["logit_bias"] = serde_json::to_value(&job.logit_bias)?;
    }

    if let Some(frequency_penalty) = &job.frequency_penalty {
        data["frequency_penalty"] = serde_json::to_value(frequency_penalty)?;
    }

    if let Some(presence_penalty) = &job.presence_penalty {
        data["presence_penalty"] = serde_json::to_value(presence_penalty)?;
    }

    if let Some(max_tokens) = &job.max_tokens {
        data["max_tokens"] = serde_json::to_value(max_tokens)?;
    }

    if let Some(n) = &job.n {
        data["n"] = serde_json::to_value(n)?;
    }

    if stream {
        data["stream"] = serde_json::Value::Bool(true);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::openai::response::ResponseBody;
    use anyhow::Result;
    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn parse_js_callback_response() -> Result<()> {
        let raw_response = r#"
                {
                    "id":"chatcmpl-7wRVa4TZBNplfgGR6vfm6mT3j7uFT",
                    "object":"chat.completion",
                    "created":1694163122,
                    "model":"gpt-3.5-turbo-16k-0613",
                    "choices":[
                        {
                            "index":0,
                            "message":{
                                "role":"assistant",
                                "content":"```json\n{\n  \"src\": {\n    \"main.rs\": \"The main entry point of the Rust application\",\n    \"config.rs\": \"A module for loading and managing configuration\",\n    \"database.rs\": \"A module for handling database connection and queries\",\n    \"models.rs\": \"A module defining the data models used in the application\",\n    \"handlers\": {\n      \"mod.rs\": \"A module for defining request handlers\",\n      \"product_handler.rs\": \"A module for handling product-related requests\",\n      \"order_handler.rs\": \"A module for handling order-related requests\",\n      \"cart_handler.rs\": \"A module for handling shopping cart-related requests\"\n    },\n    \"middlewares\": {\n      \"mod.rs\": \"A module for defining middlewares\",\n      \"authentication.rs\": \"A middleware for handling authentication\",\n      \"authorization.rs\": \"A middleware for handling authorization\"\n    },\n    \"routes\": {\n      \"mod.rs\": \"A module for defining API routes\",\n      \"product_routes.rs\": \"A module for defining product-related routes\",\n      \"order_routes.rs\": \"A module for defining order-related routes\",\n      \"cart_routes.rs\": \"A module for defining shopping cart-related routes\"\n    },\n    \"errors.rs\": \"A module defining custom error types and error handling\",\n    \"util.rs\": \"A module containing utility functions used throughout the application\"\n  }\n}\n```"
                            },
                            "finish_reason":"stop"
                        }
                    ],
                    "usage":{
                        "prompt_tokens":112,
                        "completion_tokens":282,
                        "total_tokens":394
                    }
                }"#;

        let json_value: ResponseBody = serde_json::from_str(raw_response)?;
        let js_value = to_value(&json_value).unwrap();

        let body: Result<ResponseBody, _> = from_value(js_value);

        if let Err(e) = body {
            panic!("Upsie: {:?}", e);
        }

        Ok(())
    }
}
