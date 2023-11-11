///< Contains utility functions and helpers.
use anyhow::{anyhow, Result};
use js_sys::Function;
use oai::models::chat::{
    params::wasm::ChatParamsWasm as ChatParams, request::wasm::chat_raw,
};
use oai::models::message::wasm::MessageWasm as GptMessage;
use parser::parser::json::AsJson;
use serde_json::Value;
use wasmer::log;

pub async fn write_json(
    ai_params: &ChatParams,
    prompts: &Vec<&GptMessage>,
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
