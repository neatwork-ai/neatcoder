use anyhow::Result;
use chrono::{DateTime, Utc};
use js_sys::{Date as IDate, Function, JsString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::{
    endpoints::get_chat_title::get_chat_title,
    openai::{msg::OpenAIMsg, params::OpenAIModels},
    typescript::{IMessages, IModels},
    JsError, WasmType,
};

// TODO: Do we need to store all chates in a BTreeMap or just a
// reference to all chats? We could lazily read the chats as they're opened
// in the webview as opposed to having all the chats in the BTreeMap on start

// #[wasm_bindgen]
// #[derive(Debug, Deserialize, Serialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Chats(BTreeMap<String, Chat>);

// #[wasm_bindgen]
// impl Chats {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Result<Chats, JsValue> {
//         Ok(Self(BTreeMap::new()))
//     }

//     #[wasm_bindgen(js_name = insertChat)]
//     pub fn insert_chat(&mut self, chat: Chat) {
//         self.insert(chat.session_id.clone(), chat);
//     }

//     #[wasm_bindgen(js_name = removeChat)]
//     pub fn remove_chat(&mut self, chat_id: String) {
//         self.remove(&chat_id);
//     }
// }

// impl AsRef<BTreeMap<String, Chat>> for Chats {
//     fn as_ref(&self) -> &BTreeMap<String, Chat> {
//         &self.0
//     }
// }

// impl Deref for Chats {
//     type Target = BTreeMap<String, Chat>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Chats {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub(crate) session_id: String,
    pub(crate) title: String,
    pub(crate) models: HashMap<String, Model>,
    pub(crate) messages: Vec<Message>,
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
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    #[wasm_bindgen(js_name = setMessages)]
    pub fn set_messages(&mut self, messages: IMessages) -> Result<(), JsError> {
        let messages = Vec::from_extern(messages)?;

        self.messages = messages;

        Ok(())
    }

    #[wasm_bindgen(js_name = addModel)]
    pub fn add_model(&mut self, model: OpenAIModels) {
        let model_id = model.as_string();
        self.models.insert(model_id.clone(), Model::new(model_id));
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
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub(crate) id: String,
    pub(crate) uri: String,
    pub(crate) interface: String,
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Model {
        Self {
            id,
            uri: String::from(
                "https://openai-proxy-66mt7edr2a-ew.a.run.app/chat",
            ),
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
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub(crate) user: String,
    pub(crate) ts: DateTime<Utc>,
    pub(crate) payload: OpenAIMsg,
}

#[wasm_bindgen]
impl Message {
    #[wasm_bindgen(constructor)]
    pub fn new(
        user: String,
        ts: IDate,
        payload: OpenAIMsg,
    ) -> Result<Message, JsValue> {
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
    pub fn payload(&self) -> OpenAIMsg {
        self.payload.clone()
    }
}
