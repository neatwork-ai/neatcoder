use anyhow::{anyhow, Result};
use js_sys::{Function, JsString};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    path::Path,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    consts::{CONFIG_EXTENSIONS, CONFIG_FILES},
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
) -> Result<(Value, Files)> {
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

    let mut files = Files::empty();

    let src_json = scaffold_json.get("src").ok_or_else(|| {
        anyhow!("Unable to find `src` folder in scaffold response")
    })?;

    if src_json.is_object() {
        for (key, value) in src_json.as_object().unwrap() {
            files.add_files(value, key, None);
        }
    } else {
        return Err(anyhow!("Unable to parse scaffold json."));
    }

    Ok((scaffold_json, files))
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Files(pub VecDeque<File>);

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub description: String,
    pub parent: Option<String>,
}

impl AsRef<VecDeque<File>> for Files {
    fn as_ref(&self) -> &VecDeque<File> {
        &self.0
    }
}

impl Deref for Files {
    type Target = VecDeque<File>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Files {
    pub fn empty() -> Files {
        Files(VecDeque::new())
    }

    pub fn new(files: VecDeque<File>) -> Files {
        Files(files)
    }

    pub fn add_files(
        &mut self,
        json: &Value,
        current_key: &str,
        parent_key: Option<&str>,
    ) {
        match json {
            Value::Object(map) => {
                for (key, value) in map.iter() {
                    self.add_files(value, key, Some(current_key));
                }
            }
            // For other types of Value (Number, String, Bool, Null), consider them as leaf values.
            _ => {
                self.push_back(File {
                    name: current_key.to_string(),
                    description: json.to_string(),
                    parent: parent_key.map(|s| s.to_string()),
                });
            }
        }
    }

    pub fn cleanup(&mut self, language: &Language) -> Result<()> {
        // Remove 'src/' prefix if it exists
        for file in &mut self.0 {
            if file.name.starts_with('/') {
                file.name = file.name.trim_start_matches('/').to_string();
            }

            if file.name.starts_with("src/") {
                file.name = file.name.trim_start_matches("src/").to_string();
            }
        }

        // Filter out files that are configuration files, both by extension name
        // or by filename if no extension exists
        self.retain(|file| {
            let path = Path::new(&file.name);
            // This indicates that its not a file but a folder
            if file.name.ends_with('/') {
                return false;
            }

            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if CONFIG_FILES.contains(&file_name) {
                    return false;
                }
            }
            if let Some(extension) =
                path.extension().and_then(|ext| ext.to_str())
            {
                if CONFIG_EXTENSIONS.contains(&extension) {
                    return false;
                }
            }
            true
        });

        if !language.is_custom() {
            let default_extension = language.language.default_extension();

            // If GPT forgot to add the file extensions we add them here..
            if let Some(default_ext) = default_extension {
                for file in &mut self.0 {
                    let path = Path::new(&file.name);
                    if path.extension().is_none() {
                        file.name.push_str(default_ext);
                    }
                }
            }
        }

        Ok(())
    }
}
