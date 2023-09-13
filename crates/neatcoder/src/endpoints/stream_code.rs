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

// TODO
// pub async fn stream_rust(
//     client: &OpenAI,
//     ai_params: &OpenAIParams,
//     prompts: Vec<OpenAIMsg>,
//     callback: Function,
// ) -> Result<String> {
//     log("[INFO] Initiating Stream");

//     let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

//     let chat_stream = client
//         .chat_stream(&callback, ai_params, &prompts, &[], &[])
//         .await?;
//     // Your code here to handle the stream returned from the TypeScript function
//     // ...

//     // We need to build a stream from res_js_value, which is expected to be a stream from nodejs
//     // You would need a mechanism to convert the JsValue stream into a Rust stream
//     // This part may require some research and trial and error to get right.    let mut chat_stream =

// let mut start_delimiter = false;

// while let Some(item) = chat_stream.next().await {
//     match item {
//         Ok(bytes) => {
//             let token = std::str::from_utf8(&bytes)
//                 .expect("Failed to generate utf8 from bytes");
//             if !start_delimiter && ["```rust", "```"].contains(&token) {
//                 start_delimiter = true;
//                 continue;
//             } else if !start_delimiter {
//                 continue;
//             } else {
//                 if token == "```" {
//                     break;
//                 }

//                 // Call the JavaScript callback with the token
//                 let this = JsValue::NULL;
//                 let js_token = JsValue::from_str(&token);
//                 callback.call1(&this, &js_token).unwrap();
//             }
//         }
//         Err(e) => eprintln!("Failed to receive token, with error: {e}"),
//     }
// }
//     Ok(())
// }
