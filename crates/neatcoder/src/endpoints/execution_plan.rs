use anyhow::{anyhow, Result};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{
    consts::{CONFIG_EXTENSIONS, CONFIG_FILES},
    models::{interfaces::AsContext, language::Language, state::AppState},
    openai::{
        client::OpenAI,
        msg::{GptRole, OpenAIMsg},
        params::OpenAIParams,
    },
    utils::{log, write_json},
};

pub async fn build_execution_plan(
    language: &Language,
    client: &OpenAI,
    params: &OpenAIParams,
    app_state: &AppState,
    request_callback: &Function,
) -> Result<Value> {
    let mut prompts = Vec::new();

    if app_state.interfaces.is_empty() {
        log("[INFO] No Interfaces detected. Proceeding...");
    }

    let api_description = &app_state.specs.as_ref().ok_or_else(|| {
        anyhow!("It seems that the the field `specs` is missing..")
    })?;

    let project_scaffold = app_state
        .scaffold
        .as_ref()
        .ok_or_else(|| anyhow!("No folder scaffold config available.."))?;

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: format!(
            "You are a software engineer who is specialised in building software in {}.", language.name()
        ),
    });

    for (_, interface) in app_state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: api_description.to_string(),
    });

    // TODO: Consider adding the specs here...
    let main_prompt = format!("
You are a software engineer tasked with creating a project in {}.
You are assigned to build the API based on the project folder structure. Your current task is to order the files in accordance to the order of work that best fits the file dependencies.
The project scaffold is the following:\n{}\n

Answer in JSON format. Define the order by adding the file names to an ordered list (START WITH THE DELIMITER '```json').
Use the following schema:

```json
{{'order': [...]}}
```
", language.name(), project_scaffold);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (_, tasks) =
        write_json(client, params, &prompts, request_callback).await?;

    Ok(tasks)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files(pub VecDeque<String>);

impl AsRef<VecDeque<String>> for Files {
    fn as_ref(&self) -> &VecDeque<String> {
        &self.0
    }
}

impl Deref for Files {
    type Target = VecDeque<String>;

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
    pub fn new(files: VecDeque<String>) -> Files {
        Files(files)
    }

    pub fn from_schedule(
        job_schedule: &Value,
        language: &Language,
    ) -> Result<Self> {
        let mut files: Files =
            match from_value::<Files>(job_schedule["order"].clone()) {
                Ok(files) => files,
                Err(e) => {
                    // Handle the error
                    return Err(anyhow!(
                    "Error converting dependecy graph to `Files` struct: {e}"
                ));
                }
            };

        // Remove 'src/' prefix if it exists
        for file in &mut files.0 {
            if file.starts_with('/') {
                *file = file.trim_start_matches('/').to_string();
            }

            if file.starts_with("src/") {
                *file = file.trim_start_matches("src/").to_string();
            }
        }

        // Filter out files that are configuration files, both by extension name
        // or by filename if no extension exists
        files.retain(|file| {
            let path = Path::new(file);
            if file.ends_with('/') {
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
                for file in &mut files.0 {
                    let path = Path::new(file);
                    if path.extension().is_none() {
                        file.push_str(default_ext);
                    }
                }
            }
        }

        Ok(files)
    }
}
