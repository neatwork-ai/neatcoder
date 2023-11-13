use anyhow::Result;
use chrono::{DateTime, Utc};
use js_sys::{Date as IDate, Function, JsString};
use oai::{
    foreign::{IMessages, IModels},
    models::{message::wasm::GptMessageWasm as GptMessage, Models as AiModels},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::endpoints::get_chat_title::get_chat_title;

use wasmer::{JsError, WasmType};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub(crate) session_id: String,
    pub(crate) title: String,
    pub(crate) models: HashMap<String, ModelData>,
    pub(crate) messages: Vec<MessageData>,
}

#[wasm_bindgen]
impl Chat {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: String, title: String) -> Chat {
        Self {
            session_id,
            title,
            models: HashMap::new(),
            messages: Vec::new(),
        }
    }

    #[wasm_bindgen(getter, js_name = sessionId)]
    pub fn session_id(&self) -> JsString {
        self.session_id.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsString {
        self.title.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn models(&self) -> Result<IModels, JsError> {
        HashMap::to_extern(self.models.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn messages(&self) -> Result<IMessages, JsError> {
        Vec::to_extern(self.messages.clone())
    }

    #[wasm_bindgen(js_name = setTitle)]
    pub async fn set_title(
        &mut self,
        request_callback: &Function,
    ) -> Result<(), JsError> {
        if !self.messages.is_empty() {
            // Get the first element using indexing (index 0)
            let first_msg = self.messages[0].clone();
            let title =
                get_chat_title(&first_msg.payload.content, request_callback)
                    .await
                    .map_err(|e| JsError::from_str(&e.to_string()))?;

            self.title = title;

            Ok(())
        } else {
            Err(JsError::from(JsValue::from_str(
                "Unable to create title. No messages in the Chat.",
            )))
        }
    }

    #[wasm_bindgen(js_name = addMessage)]
    pub fn add_message(&mut self, message: MessageData) {
        self.messages.push(message);
    }

    #[wasm_bindgen(js_name = setMessages)]
    pub fn set_messages(&mut self, messages: IMessages) -> Result<(), JsError> {
        let messages = Vec::from_extern(messages)?;

        self.messages = messages;

        Ok(())
    }

    #[wasm_bindgen(js_name = addModel)]
    pub fn add_model(&mut self, model: AiModels) -> Result<(), JsError> {
        let model_id = model.as_str();

        self.models
            .insert(model_id.to_string(), ModelData::new(model_id.to_string()));

        Ok(())
    }

    #[wasm_bindgen(js_name = castFromString)]
    pub fn cast_from_string(json: String) -> Result<Chat, JsError> {
        let chat = serde_json::from_str(&json)
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        Ok(chat)
    }

    #[wasm_bindgen(js_name = castToString)]
    pub fn cast_to_string(&self) -> Result<JsString, JsError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        Ok(json.into())
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", rename = "Model")]
pub struct ModelData {
    pub(crate) id: String,
    pub(crate) uri: String,
    pub(crate) interface: String,
}

#[wasm_bindgen]
impl ModelData {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> ModelData {
        Self {
            id,
            uri: String::from("https://api.openai.com/v1/chat/completions"),
            interface: String::from("OpenAI"),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsString {
        self.id.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> JsString {
        self.uri.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn interface(&self) -> JsString {
        self.interface.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", rename = "Message")]
pub struct MessageData {
    pub(crate) user: String,
    pub(crate) ts: DateTime<Utc>,
    pub(crate) payload: GptMessage,
}

#[wasm_bindgen]
impl MessageData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        user: String,
        ts: IDate,
        payload: GptMessage,
    ) -> Result<MessageData, JsValue> {
        let datetime = DateTime::from_extern(ts.into())?;
        Ok(Self {
            user,
            ts: datetime,
            payload,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn user(&self) -> JsString {
        self.user.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn ts(&self) -> Result<IDate, JsError> {
        DateTime::to_extern(self.ts.clone().into())
    }

    #[wasm_bindgen(getter)]
    pub fn payload(&self) -> GptMessage {
        self.payload.clone()
    }
}
