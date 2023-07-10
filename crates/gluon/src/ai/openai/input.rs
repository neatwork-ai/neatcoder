use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};
use serde_json::json;

pub struct Message {
    pub role: GptRole,
    pub content: String,
}

pub enum GptRole {
    System,
    User,
    Assistant,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let msg = json!({
            "role": self.role.as_str(),
            "content": self.content,
        });

        msg.serialize(serializer)
    }
}

impl GptRole {
    pub fn new(role: &str) -> Result<Self> {
        let role = match role {
            "system" => GptRole::System,
            "user" => GptRole::User,
            "assistant" => GptRole::Assistant,
            _ => return Err(anyhow!(format!("Invalid role {}", role))),
        };

        Ok(role)
    }

    pub fn as_str(&self) -> &str {
        match self {
            GptRole::System => "system",
            GptRole::User => "user",
            GptRole::Assistant => "assistant",
        }
    }
}
