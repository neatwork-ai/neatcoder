use anyhow::{anyhow, Result};
use js_sys::{Function, JsString};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    models::language::Language,
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
    language: &Language,
    client: &OpenAI,
    ai_params: &OpenAIParams,
    client_params: &ScaffoldParams,
    request_callback: &Function,
) -> Result<Value> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: format!(
            "You are a software engineer who is specialised in building software in {}.", language.name()
        ),
    });

    // TODO: We should add the Database and API interfaces in previous messages, and add the name of the files here in order to index them
    let main_prompt = format!("
You are a software engineer tasked with creating project in {} based on the following project description:\n{}\n
The project should retrieve the relevant data from the database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json", language.name(), client_params.specs);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (_, mut scaffold_json) =
        write_json(client, &ai_params, &prompts, request_callback).await?;

    process_response(&mut scaffold_json)?;

    Ok(scaffold_json)
}

fn process_response(llm_response: &mut Value) -> Result<()> {
    let obj = llm_response
        .as_object_mut()
        // This should be typed...
        .ok_or_else(|| anyhow!("LLM Respose seems to corrupted :("))?;

    // Create an src object if it doesn't exist
    if !obj.contains_key("src") {
        obj.insert("src".to_string(), Value::Object(Map::new()));
    }

    // Move other keys into the src object
    let keys_to_move: Vec<String> = obj.keys().cloned().collect();

    for key in keys_to_move {
        if key != "src" {
            if let Some(value) = obj.remove(&key) {
                let src_obj =
                    obj.get_mut("src").unwrap().as_object_mut().unwrap(); // Safe to unwrap
                src_obj.insert(key, value);
            }
        }
    }

    Ok(())
}
