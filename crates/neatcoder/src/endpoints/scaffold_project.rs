use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    models::state::AppState,
    openai::{
        client::OpenAI,
        msg::{GptRole, OpenAIMsg},
        params::OpenAIParams,
    },
    utils::write_json,
};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScaffoldParams {
    pub(crate) specs: String,
}

#[wasm_bindgen]
impl ScaffoldParams {
    #[wasm_bindgen(constructor)]
    pub fn new(specs: String) -> ScaffoldParams {
        ScaffoldParams { specs }
    }

    #[wasm_bindgen(getter)]
    pub fn specs(&self) -> JsString {
        self.specs.clone().into()
    }
}

pub async fn scaffold_project(
    client: &OpenAI,
    ai_params: &OpenAIParams,
    client_params: &ScaffoldParams,
    app_state: &AppState,
) -> Result<Value> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    if app_state.scaffold.is_some() {
        return Err(anyhow!("Scaffold already exists. Skipping..."));
    }

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust based on the following project description:\n{}\n
The API should retrieve the relevant data from a MySQL database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json", client_params.specs);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (_, scaffold_json) = write_json(client, &ai_params, &prompts).await?;

    Ok(scaffold_json)
}
