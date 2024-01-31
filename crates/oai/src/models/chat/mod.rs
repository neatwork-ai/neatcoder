pub mod params;
pub mod request;
pub mod response;

#[cfg(not(feature = "wasm"))]
use reqwest::header::HeaderMap;

#[cfg(feature = "wasm")]
use crate::foreign::{IMessagesData, IModelsData};
#[cfg(feature = "wasm")]
use js_sys::{Date as IDate, JsString};
#[cfg(feature = "wasm")]
use wasmer::{JsError, WasmType};

use super::{message::GptMessage, model::ModelData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chat {
    #[serde(skip)]
    #[cfg(not(feature = "wasm"))]
    pub(crate) headers: HeaderMap,
    pub session_id: String,
    pub title: String,
    pub models: HashMap<String, ModelData>,
    pub messages: Vec<MessageData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageData {
    pub user: String,
    pub ts: DateTime<Utc>,
    pub payload: GptMessage,
}

impl Chat {
    pub fn new(
        #[cfg(not(feature = "wasm"))] headers: HeaderMap,
        session_id: String,
        title: String,
    ) -> Chat {
        Self {
            #[cfg(not(feature = "wasm"))]
            headers,
            session_id,
            title,
            models: HashMap::new(),
            messages: Vec::new(),
        }
    }
}

/// === WASM ===

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use crate::models::{
        message::wasm::GptMessageWasm,
        model::{wasm::ModelDataWasm, GptModel},
    };
    use anyhow::Result;
    use derive_more::{AsRef, Deref, DerefMut};
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen]
    #[derive(Debug, Deserialize, Serialize, Clone, AsRef, Deref, DerefMut)]
    #[serde(rename_all = "camelCase", rename = "Chat")]
    pub struct ChatWasm(Chat);

    #[wasm_bindgen]
    #[derive(Debug, Deserialize, Serialize, Clone, AsRef, Deref, DerefMut)]
    #[serde(rename_all = "camelCase", rename = "Message")]
    pub struct MessageDataWasm(pub(crate) MessageData);

    #[wasm_bindgen]
    impl ChatWasm {
        #[wasm_bindgen(constructor)]
        pub fn new(session_id: String, title: String) -> ChatWasm {
            // The headers are not used in WASM so it's dummy
            Self(Chat::new(
                #[cfg(not(feature = "wasm"))]
                HeaderMap::new(),
                session_id,
                title,
            ))
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
        pub fn models(&self) -> Result<IModelsData, JsError> {
            let map = self
                .models
                .iter()
                .map(|(k, model)| (k.clone(), ModelDataWasm(model.clone()))) // automatically dereferences
                .collect();

            HashMap::to_extern(map)
        }

        #[wasm_bindgen(getter)]
        pub fn messages(&self) -> Result<IMessagesData, JsError> {
            let msgs = self
                .messages
                .iter()
                .map(|msg| MessageDataWasm(msg.clone()))
                .collect();

            Vec::to_extern(msgs)
        }

        #[wasm_bindgen(js_name = addMessage)]
        pub fn add_message(&mut self, message: MessageDataWasm) {
            let MessageDataWasm(msg_data) = message;
            self.messages.push(msg_data);
        }

        #[wasm_bindgen(js_name = setMessages)]
        pub fn set_messages(
            &mut self,
            messages: IMessagesData,
        ) -> Result<(), JsError> {
            let mut messages: Vec<MessageDataWasm> =
                Vec::from_extern(messages)?;

            let messages: Vec<MessageData> = messages
                .drain(..)
                .map(|msg| {
                    let MessageDataWasm(msg_data) = msg;
                    msg_data
                })
                .collect();

            self.messages = messages;

            Ok(())
        }

        #[wasm_bindgen(js_name = addModel)]
        pub fn add_model(&mut self, model: GptModel) -> Result<(), JsError> {
            let model_id = model.as_str();

            self.models.insert(
                model_id.to_string(),
                ModelData::new(model_id.to_string()),
            );

            Ok(())
        }

        #[wasm_bindgen(js_name = castFromString)]
        pub fn cast_from_string(json: String) -> Result<ChatWasm, JsError> {
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
    impl MessageDataWasm {
        #[wasm_bindgen(constructor)]
        pub fn new(
            user: String,
            ts: IDate,
            payload: GptMessageWasm,
        ) -> Result<MessageDataWasm, JsValue> {
            let datetime = DateTime::from_extern(ts.into())?;

            let GptMessageWasm(payload) = payload;

            Ok(MessageDataWasm(MessageData {
                user,
                ts: datetime,
                payload,
            }))
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
        pub fn payload(&self) -> GptMessageWasm {
            GptMessageWasm(self.payload.clone())
        }
    }
}
