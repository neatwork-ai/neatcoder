use super::{msg::OpenAIMsg, output::Body, params::OpenAIParams};
use crate::utils::log;
use anyhow::{anyhow, Result};
use js_sys::{Function, Promise};
use serde_json::{json, Value};
use std::{fmt, ops::Deref};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
pub struct OpenAI {
    api_key: Option<String>,
}

#[wasm_bindgen]
impl OpenAI {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
        }
    }

    pub fn empty() -> Self {
        Self { api_key: None }
    }
}

impl OpenAI {
    // === Setter methods with chaining ===

    pub fn api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    // === OpenAI IO ===

    pub async fn chat(
        &self,
        request_callback: &Function,
        ai_params: impl Deref<Target = OpenAIParams>,
        msgs: &[&OpenAIMsg],
        funcs: &[&String],
        stop_seq: &[String],
    ) -> Result<String> {
        log("[DEBUG] Getting Chat Raw...");
        let chat = self
            .chat_raw(request_callback, ai_params, msgs, funcs, stop_seq)
            .await?;
        log("[DEBUG] Got answer.");
        let answer = chat.choices.first().unwrap().message.content.as_str();

        Ok(String::from(answer))
    }

    pub async fn chat_raw(
        &self,
        request_callback: &Function,
        ai_params: impl Deref<Target = OpenAIParams>,
        msgs: &[&OpenAIMsg],
        funcs: &[&String],
        stop_seq: &[String],
    ) -> Result<Body> {
        let req_body =
            self.request_body(ai_params, msgs, funcs, stop_seq, false)?;

        let body_json = serde_json::to_string(&req_body)?;

        log("[DEBUG] Getting promise...");
        let js_promise: Promise = request_callback
            .call1(&JsValue::NULL, &JsValue::from_str(&body_json))
            .map_err(|e| anyhow!("Error performing request callback: {:?}", e))?
            .dyn_into()
            .map_err(|e| {
                anyhow!("Error processing request callback promise: {:?}", e)
            })?;
        log("[DEBUG] Resolving promise...");
        let res_js_value: JsValue =
            JsFuture::from(js_promise).await.map_err(|e| {
                anyhow!("Error processing request callback result: {:?}", e)
            })?;

        log("[DEBUG] Promise resolved...");
        log(&format!("Correct Request body: {:?}", res_js_value));

        if let Ok(res) = res_js_value.dyn_into::<Response>() {
            // Now `res` is a `web_sys::Response` instance and you can work with it using web-sys APIs.
            // For example, to get the response as text:
            let text_js_promise = res.text().map_err(|e| {
                anyhow!("Error fetching request body promise: {:?}", e)
            })?;
            let body_js_value: JsValue = JsFuture::from(text_js_promise)
                .await
                .map_err(|e| anyhow!("Error fetching request body: {:?}", e))?;
            let body: String = body_js_value.as_string().unwrap();

            let api_response = serde_json::from_str(body.as_str())?;
            return Ok(api_response);
        } else {
            // Handle the case where the JsValue could not be cast to a Response
            return Err(anyhow!("Could not convert JsValue to Response",));
        }
    }

    pub async fn chat_stream(
        &self,
        _job: &OpenAIParams,
        _msgs: &[&OpenAIMsg],
        _funcs: &[&String],
        _stop_seq: &[String],
    ) -> Result<()> {
        // ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>> {
        // let client = Client::new();

        // let req_body = self.request_body(job, msgs, funcs, stop_seq, true)?;

        // let response = client
        //     .post("https://api.openai.com/v1/chat/completions")
        //     .header(
        //         "Authorization",
        //         format!(
        //             "Bearer {}",
        //             self.api_key.as_ref().expect("No API Keys provided")
        //         ),
        //     )
        //     .header("Content-Type", "application/json")
        //     .json(&req_body)
        //     .send()
        //     .await?;

        // let stream = response.bytes_stream();

        todo!();

        // return Ok(stream);
    }

    pub fn request_body(
        &self,
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
            data["frequency_penalty"] =
                serde_json::to_value(frequency_penalty)?;
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
}

/// We manually implement `Debug` to intentionally maskl the API Key with
/// the value `Some` or `None`, for security reasons.
impl fmt::Debug for OpenAI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = if self.api_key.is_some() {
            "Some"
        } else {
            "None"
        };

        // TODO: Consider indicating at least if API key is Some or None
        f.debug_struct("OpenAI")
            .field("api_key", &value) // We hide the API Key and only indicate if it is Some or None
            .finish()
    }
}
