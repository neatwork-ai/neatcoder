use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut},
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chats(BTreeMap<String, Chat>);

#[wasm_bindgen]
impl Chats {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Chats, JsValue> {
        Ok(Self(BTreeMap::new()))
    }
}

impl AsRef<BTreeMap<String, Chat>> for Chats {
    fn as_ref(&self) -> &BTreeMap<String, Chat> {
        &self.0
    }
}

impl Deref for Chats {
    type Target = BTreeMap<String, Chat>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Chats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
    pub fn new() -> Result<Chat, JsValue> {
        Ok(Self {
            session_id: String::from("TODO"),
            title: String::from("TODO"),
            models: HashMap::new(),
            messages: Vec::new(),
        })
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub(crate) id: String,
    pub(crate) model: String,
    pub(crate) uri: String,
    pub(crate) interface: String,
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Model, JsValue> {
        Ok(Self {
            id: String::from("TODO"),
            model: String::from("TODO"),
            uri: String::from("TODO"),
            interface: String::from("TODO"),
        })
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub(crate) user: String,
    pub(crate) ts: String,
    pub(crate) text: String,
}

#[wasm_bindgen]
impl Message {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Message, JsValue> {
        Ok(Self {
            user: String::from("TODO"),
            ts: String::from("TODO"),
            text: String::from("TODO"),
        })
    }
}
