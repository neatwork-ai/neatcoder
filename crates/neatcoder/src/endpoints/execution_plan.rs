use anyhow::{anyhow, Result};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use crate::{
    models::{interfaces::AsContext, state::AppState},
    openai::{
        client::OpenAI,
        msg::{GptRole, OpenAIMsg},
        params::OpenAIParams,
    },
    utils::{log, write_json},
};

pub async fn build_execution_plan(
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
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
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

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust.
You are assigned to build the API based on the project folder structure. Your current task is to order the files in accordance to the order of work that best fits the file dependencies.
The project scaffold is the following:\n{}\n

Answer in JSON format. Define the order by adding the file names to an ordered list (START WITH THE DELIMITER '```json').
Use the following schema:

```json
{{'order': [...]}}
```
", project_scaffold);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (answer, tasks) =
        write_json(client, params, &prompts, request_callback).await?;

    log(&format!("[DEBUG] LLM: {}", answer));

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
    pub fn from_schedule(job_schedule: &Value) -> Result<Self> {
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

        // Filter out files that are not rust files
        files.retain(|file| {
            if file.ends_with(".rs") {
                true
            } else {
                log(&format!("[WARN] Filtered out: {}", file));
                false
            }
        });

        Ok(files)
    }
}
