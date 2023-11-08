use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Client};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::json;
use std::{collections::HashMap, fmt};

use crate::{consts::BASE_BETA_URL, openai::params::OpenAIModels};

#[derive(Serialize, Debug)]
pub struct AssistantRequest {
    pub name: String,
    pub instructions: String,
    pub tools: Vec<String>, // TODO: "tools": [{"type": "code_interpreter"}]
    pub model: OpenAIModels,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    id: String,
    object: String,
    created_at: u32, // TODO: Should be a timestamp
    name: String,
    description: Option<String>,
    model: OpenAIModels,
    instructions: Option<String>,
    tools: Vec<Tool>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub enum Tool {
    CodeInterpreter,
    Retrieval,
    FunctionCall,
}

impl Tool {
    pub fn new(tool: String) -> Self {
        let tool = match tool.as_str() {
            "code_interpreter" => Tool::CodeInterpreter,
            "retrieval" => Tool::Retrieval,
            "function" => Tool::FunctionCall,
            _ => panic!("Invalid tool {}", tool),
        };

        tool
    }

    pub fn as_string(&self) -> String {
        match self {
            Tool::CodeInterpreter => String::from("code_interpreter"),
            Tool::Retrieval => String::from("retrieval"),
            Tool::FunctionCall => String::from("function"),
        }
    }
}

impl<'de> Deserialize<'de> for Tool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToolVisitor;

        impl<'de> Visitor<'de> for ToolVisitor {
            type Value = Tool;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an OpenAI model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Tool, E>
            where
                E: de::Error,
            {
                match value {
                    "code_interpreter" => Ok(Tool::CodeInterpreter),
                    "retrieval" => Ok(Tool::Retrieval),
                    "function" => Ok(Tool::FunctionCall),
                    _ => Err(E::custom(format!(
                        "unexpected OpenAI tool: {}",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_str(ToolVisitor)
    }
}

impl AssistantRequest {
    pub async fn create_assistant(
        self,
        client: &Client,
        headers: &HeaderMap,
    ) -> Result<Assistant> {
        let response = client
            .post(&format!("{}/assistants", BASE_BETA_URL))
            .headers(headers.clone())
            .json(&json!({
                "name": self.name, // "Math Tutor",
                "instructions": self.instructions, // "You are a personal math tutor. Write and run code to answer math questions.",
                "tools": self.tools, // [{"type": "code_interpreter"}],
                "model": self.model, // "gpt-4-1106-preview"
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let assistant = response.json::<Assistant>().await?;
            println!("Create Assistant response: {:?}", assistant);
            Ok(assistant)
        } else {
            // If not successful, perhaps you want to parse it differently or handle the error
            Err(anyhow!(response.status()))
        }
    }
}
