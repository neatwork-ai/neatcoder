use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::json;
use std::collections::HashMap;

use super::OpenAIModels;
use crate::print_;
use crate::utils::post_api;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    pub id: String,
    pub object: String,
    pub created_at: u32, // TODO: Should be a timestamp
    pub name: String,
    pub description: Option<String>,
    pub model: OpenAIModels,
    pub instructions: Option<String>,
    pub tools: Vec<Tool>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Tool {
    CodeInterpreter,
    Retrieval,
    FunctionCall,
}

impl Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let tool_str = self.as_string();
        let mut map_ser = serializer.serialize_map(Some(1))?;
        map_ser.serialize_entry("type", &tool_str)?;
        map_ser.end()
    }
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"a map with a key "type" representing an OpenAI tool"#)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Tool, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut tool_type = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "type" {
                        tool_type = Some(map.next_value::<String>()?);
                        break;
                    }
                }
                match tool_type {
                    Some(t) => match t.as_ref() {
                        "code_interpreter" => Ok(Tool::CodeInterpreter),
                        "retrieval" => Ok(Tool::Retrieval),
                        "function_call" => Ok(Tool::FunctionCall),
                        _ => Err(de::Error::unknown_field(&t, FIELDS)),
                    },
                    None => Err(de::Error::missing_field("type")),
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["type"];
        deserializer.deserialize_map(ToolVisitor)
    }
}

impl Assistant {
    pub async fn create_assistant(
        client: &Client,
        headers: &HeaderMap,
        name: String,
        instructions: String,
        tools: Vec<Tool>,
        model: OpenAIModels,
    ) -> Result<Assistant> {
        let payload = json!({
            "name": name, // "Math Tutor",
            "instructions": instructions, // "You are a personal math tutor. Write and run code to answer math questions.",
            "tools": tools, // [{"type": "code_interpreter"}],
            "model": model.as_string(), // "gpt-4-1106-preview"
        });

        let response_body = post_api(client, headers, "assistants", &payload).await?;
        let assistant: Assistant = serde_json::from_value(response_body)?;

        print_!("Assistant: {:?}", assistant);

        Ok(assistant)
    }

    pub async fn add_file() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() -> Result<()> {
        let json_data = json!({
            "created_at": 1699447210,
            "description": null,
            "file_ids": [],
            "id": "asst_I1qwkQwPuWRQYlQtXQrny8il",
            "instructions": "You are a personal math tutor. Write and run code to answer math questions.",
            "metadata": {},
            "model": "gpt-4-1106-preview",
            "name": "Math Tutor",
            "object": "assistant",
            "tools": [{"type": "code_interpreter"}]
        }).to_string();

        let assistant: Assistant = serde_json::from_str(&json_data)?;

        assert_eq!(assistant.id, "asst_I1qwkQwPuWRQYlQtXQrny8il");
        assert_eq!(assistant.name, "Math Tutor");
        assert!(matches!(assistant.model, OpenAIModels::Gpt41106Preview));
        assert!(!assistant.file_ids.iter().any(|id| id == "unexpected_id"));

        assert!(matches!(
            assistant.tools.as_slice(),
            [Tool::CodeInterpreter]
        ));

        Ok(())
    }
}
