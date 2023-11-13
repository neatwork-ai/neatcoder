use anyhow::{anyhow, Result};
use js_sys::JsString;
use oai::models::{
    chat::{
        params::wasm::ChatParamsWasm as ChatParams, request::request_stream,
    },
    message::{
        wasm::GptMessageWasm as GptMessage, GptMessage as GptMessageInner,
    },
    role::Role as GptRole,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};
use wasm_bindgen::prelude::wasm_bindgen;
use wasmer::log;

use crate::models::app_data::{interfaces::AsContext, AppData};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenParams {
    pub(crate) filename: String,
    pub(crate) description: String,
}

#[wasm_bindgen]
impl CodeGenParams {
    #[wasm_bindgen(constructor)]
    pub fn new(filename: String, description: String) -> CodeGenParams {
        CodeGenParams {
            filename,
            description,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn filename(&self) -> JsString {
        self.filename.clone().into()
    }
}

pub fn stream_code(
    app_state: &AppData,
    ai_params: &ChatParams,
    task_params: &CodeGenParams,
    codebase: BTreeMap<String, String>,
) -> Result<String> {
    if app_state.language.is_none() {
        return Err(anyhow!("No programming lancuage specified"));
    }

    let language = app_state.language.clone().unwrap();

    let mut prompts = Vec::new();

    let CodeGenParams {
        filename,
        description,
    } = task_params;

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

    prompts.push(GptMessage::new(
        GptRole::System,
        format!(
            "You are a software engineer who is specialised in {}.",
            language.name()
        ),
    ));

    prompts.push(GptMessage::new(
        GptRole::User,
        String::from(project_description),
    ));

    for file in codebase.keys() {
        let code = codebase
            .get(file)
            .ok_or_else(|| anyhow!("Unable to find fild {:?}", file))?;

        prompts.push(GptMessage::new(GptRole::User, code.clone()));
    }

    prompts.push(GptMessage::new(GptRole::User, project_scaffold.to_string()));

    let mut main_prompt = format!(
        "
        You are a {} engineer and you're assigned to build the project
        defined in the previous prompts.

        Your current task is to write the module `{}.rs`.
        Consider the description of the module: {}\n
        ",
        language.name(),
        filename,
        description
    );

    if !app_state.interfaces.is_empty() {
        main_prompt.push_str(
            "Consider the following interfaces relevant to this project:\n",
        )
    }

    for (_, interface) in app_state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;

        main_prompt.push_str(&format!(
            "- Name {}; Type {}",
            interface.name(),
            interface.itype()
        ));
    }

    prompts.push(GptMessage::new(GptRole::User, main_prompt));

    // Assuming prompts is a Vec<&GptMessageWasm>
    let msgs: Vec<&GptMessageInner> =
        prompts.iter().map(|m_wasm| (*m_wasm).deref()).collect();

    let prompts_slice: &[&GptMessageInner] = &msgs;

    let request_body =
        request_stream(ai_params.deref(), &prompts_slice, &[], &[])?;

    Ok(request_body)
}
