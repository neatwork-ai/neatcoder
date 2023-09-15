use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    models::{interfaces::AsContext, state::AppState},
    openai::{
        client::OpenAI,
        msg::{GptRole, OpenAIMsg},
        params::OpenAIParams,
    },
    utils::log,
};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenParams {
    pub(crate) filename: String,
}

#[wasm_bindgen]
impl CodeGenParams {
    #[wasm_bindgen(constructor)]
    pub fn new(filename: String) -> CodeGenParams {
        CodeGenParams { filename }
    }

    #[wasm_bindgen(getter)]
    pub fn filename(&self) -> JsString {
        self.filename.clone().into()
    }
}

pub fn stream_code(
    app_state: &AppState,
    client: &OpenAI,
    ai_params: &OpenAIParams,
    task_params: &CodeGenParams,
    codebase: BTreeMap<String, String>,
) -> Result<String> {
    if app_state.language.is_none() {
        return Err(anyhow!("No programming lancuage specified"));
    }

    let language = app_state.language.clone().unwrap();

    let mut prompts = Vec::new();

    let CodeGenParams { filename } = task_params;

    log(&format!("[INFO] Running `CodeGen` Job: {}", filename));

    if app_state.scaffold.is_none() {
        return Err(anyhow!("No project scaffold config available.."));
    }

    let project_scaffold = app_state
        .scaffold
        .as_ref()
        .ok_or_else(|| anyhow!("No folder scaffold config available.."))?;

    let project_description = app_state.specs.as_ref().ok_or_else(|| {
        anyhow!("It seems that the the field `specs` is missing..")
    })?;

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: format!(
            "You are a software engineer who is specialised in {}.",
            language.name()
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
        let code = codebase
            .get(file)
            .ok_or_else(|| anyhow!("Unable to find fild {:?}", file))?;

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
        You are an engineer tasked with creating a in {}.
        You are assigned to build the API based on the project folder structure
        Your current task is to write the module `{}.rs`
        ",
        language.name(),
        filename
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let request_body = client.request_stream(ai_params, &prompts, &[], &[])?;

    Ok(request_body)
}
