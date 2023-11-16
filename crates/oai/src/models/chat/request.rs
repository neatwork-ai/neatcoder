use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::ops::Deref;

use crate::models::{
    chat::{params::ChatParams, response::ChatResponse},
    message::GptMessage,
};

#[cfg(feature = "default")]
pub mod native {
    use super::{super::Chat, *};
    use bytes::Bytes;
    use futures::Stream;
    use reqwest::Client;
    use std::{ops::Deref, time::Duration};

    impl Chat {
        pub async fn chat(
            &self,
            job: impl Deref<Target = ChatParams>,
            msgs: &[&GptMessage],
            funcs: &[&String],
            stop_seq: &[String],
        ) -> Result<ChatResponse> {
            let client = Client::new();

            // fill in your own data as needed
            let req_body =
                super::request_body(job, msgs, funcs, stop_seq, false)?;
            println!("[DEBUG] Sending reqeust to OpenAI...");

            let res = client
                .post("https://api.openai.com/v1/chat/completions")
                .headers(self.headers.clone())
                .json(&req_body)
                .send()
                .await?;

            println!("[DEBUG] Got response.....");
            let status = res.status();

            if !status.is_success() {
                return Err(anyhow!("Failed with status: {}", status));
            }

            // let body = res.text().await?;
            let body = res.json::<Value>().await?;
            let api_response = serde_json::from_value(body)?;

            Ok(api_response)
        }

        pub async fn chat_stream(
            &self,
            job: &ChatParams,
            msgs: &[&GptMessage],
            funcs: &[&String],
            stop_seq: &[String],
        ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>> {
            let client = Client::new();

            // fill in your own data as needed
            let req_body = request_body(job, msgs, funcs, stop_seq, true)?;

            let mut retries = 3; // Number of retries
            loop {
                println!("[DEBUG] Sending request to OpenAI...");

                let res = tokio::time::timeout(
                    Duration::from_secs(5),
                    client
                        .post("https://api.openai.com/v1/chat/completions")
                        .headers(self.headers.clone())
                        .json(&req_body)
                        .send(),
                )
                .await?;

                match res {
                    Ok(response) => {
                        println!("[DEBUG] Got response.....");
                        let stream = response.bytes_stream();

                        return Ok(stream);
                    }
                    Err(e) => {
                        retries -= 1;
                        if retries == 0 {
                            return Err(anyhow!(
                                "Failed after maximum retries: {:?}",
                                e
                            ));
                        }

                        println!("[DEBUG] Request failed, retrying...");
                    }
                }
            }
        }
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use crate::{
        foreign::IGptMessage,
        models::{
            chat::params::wasm::ChatParamsWasm, message::wasm::GptMessageWasm,
        },
    };

    use super::*;
    use js_sys::{Function, Promise};
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use wasmer::{log, JsError, WasmType};

    pub async fn chat(
        request_callback: &Function,
        ai_params: &ChatParamsWasm,
        msgs: &[&GptMessageWasm],
        funcs: &[&String],
        stop_seq: &[String],
    ) -> Result<String> {
        log("[DEBUG] Getting Chat Raw...");
        let chat = chat_raw(request_callback, ai_params, msgs, funcs, stop_seq)
            .await?;

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
        ai_params: &ChatParamsWasm,
        msgs: &[&GptMessageWasm],
        funcs: &[&String],
        stop_seq: &[String],
    ) -> Result<ChatResponse> {
        let msgs: Vec<&GptMessage> =
            msgs.iter().map(|&m_wasm| m_wasm.deref()).collect();

        let msg_slice: &[&GptMessage] = &msgs;

        let req_body =
            request_body(ai_params.deref(), msg_slice, funcs, stop_seq, false)?;

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

        let body: Result<ChatResponse, _> = from_value(res_js_value);

        match body {
            Ok(body) => Ok(body),
            Err(e) => {
                Err(anyhow!("Could not convert JsValue to Response: {:?}", e))
            }
        }
    }

    #[wasm_bindgen(js_name = requestBody)]
    pub fn request_body_wasm(
        msgs: IGptMessage,
        job: ChatParamsWasm,
        stream: bool,
    ) -> Result<JsValue, JsError> {
        let msgs: Vec<GptMessageWasm> =
            Vec::from_extern(msgs).map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to convert msgs to native Wasm type: {:?}",
                    e
                ))
            })?;

        let mut data = json!({
            "model": job.model.as_str(),
            "messages": msgs,
        });

        if stream {
            data["stream"] = serde_json::Value::Bool(true);
        }

        let serialized_str = serde_json::to_string(&data).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to serialize to string: {:?}",
                e
            ))
        })?;

        Ok(JsValue::from_str(&serialized_str))
    }
}

pub fn request_stream(
    ai_params: impl Deref<Target = ChatParams>,
    msgs: &[&GptMessage],
    funcs: &[&String],
    stop_seq: &[String],
) -> Result<String> {
    let req_body = request_body(ai_params, msgs, funcs, stop_seq, true)?;

    let body_json = serde_json::to_string(&req_body)?;

    Ok(body_json)
}

pub fn request_body(
    job: impl Deref<Target = ChatParams>,
    msgs: &[&GptMessage],
    funcs: &[&String],   // TODO: Add to ChatParams
    stop_seq: &[String], // TODO: Add to ChatParams
    stream: bool,
) -> Result<Value> {
    let mut data = json!({
        "model": job.model.as_str(),
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
    use anyhow::Result;

    #[cfg(feature = "wasm")]
    use serde_wasm_bindgen::{from_value, to_value};
    #[cfg(feature = "wasm")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg(feature = "wasm")]
    #[wasm_bindgen_test]
    fn parse_js_callback_response() -> Result<()> {
        use crate::models::chat::response::ChatResponse;

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

        let json_value: ChatResponse = serde_json::from_str(raw_response)?;
        let js_value = to_value(&json_value).unwrap();

        let body: Result<ChatResponse, _> = from_value(js_value);

        if let Err(e) = body {
            panic!("Upsie: {:?}", e);
        }

        Ok(())
    }

    #[cfg(feature = "default")]
    #[test]
    fn parse_response() -> Result<()> {
        use crate::models::chat::response::ChatResponse;

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

        let body: ChatResponse = serde_json::from_str(raw_response).unwrap();

        Ok(())
    }
}
