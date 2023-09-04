use anyhow::{anyhow, Result};
use futures::StreamExt;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    models::{interfaces::AsContext, state::AppState},
    openai::{
        client::OpenAI,
        msg::{GptRole, OpenAIMsg},
        params::OpenAIParams,
    },
};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CodeGenParams {
    pub(crate) filename: String,
}

#[wasm_bindgen]
impl CodeGenParams {
    #[wasm_bindgen(constructor)]
    pub fn new(filename: String) -> CodeGenParams {
        CodeGenParams { filename }
    }

    #[wasm_bindgen(getter, js_name = getFilename)]
    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }
}

pub async fn stream_code(
    app_state: &AppState,
    client: &OpenAI,
    ai_params: &OpenAIParams,
    task_params: CodeGenParams,
    codebase: HashMap<String, String>,
    callback: Function,
) -> Result<()> {
    let mut prompts = Vec::new();

    let CodeGenParams { filename } = task_params;

    println!("[INFO] Running `CodeGen` Job: {}", filename);

    if app_state.scaffold.is_none() {
        return Err(anyhow!("No project scaffold config available.."));
    }

    let project_scaffold = app_state.scaffold.as_ref().unwrap();
    let project_description = app_state.specs.as_ref().unwrap();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in Rust.",
        ),
    });

    for (_, interface) in app_state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(project_description),
    });

    for file in codebase.keys() {
        let code = codebase.get(file).unwrap();

        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: code.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        // Needs to be optimized
        content: project_scaffold.to_string(),
    });

    let main_prompt = format!(
        "
        You are a Rust engineer tasked with creating an API in Rust.
        You are assigned to build the API based on the project folder structure
        Your current task is to write the module `{}.rs
        ",
        filename
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    stream_rust(client, ai_params, prompts, callback).await?;

    Ok(())
}

pub async fn stream_rust(
    client: &OpenAI,
    ai_params: &OpenAIParams,
    prompts: Vec<OpenAIMsg>,
    callback: Function,
) -> Result<()> {
    println!("[INFO] Initiating Stream");

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let mut chat_stream =
        client.chat_stream(ai_params, &prompts, &[], &[]).await?;

    let mut start_delimiter = false;
    while let Some(item) = chat_stream.next().await {
        match item {
            Ok(bytes) => {
                let token = std::str::from_utf8(&bytes)
                    .expect("Failed to generate utf8 from bytes");
                if !start_delimiter && ["```rust", "```"].contains(&token) {
                    start_delimiter = true;
                    continue;
                } else if !start_delimiter {
                    continue;
                } else {
                    if token == "```" {
                        break;
                    }

                    // Call the JavaScript callback with the token
                    let this = JsValue::NULL;
                    let js_token = JsValue::from_str(&token);
                    callback.call1(&this, &js_token).unwrap();
                }
            }
            Err(e) => eprintln!("Failed to receive token, with error: {e}"),
        }
    }
    Ok(())
}
