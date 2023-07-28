use serde::{Deserialize, Serialize};


// TODO: Turn message into ENUM object with variants for different models:
// e.g. OpenAI, HUggingFace, and then have a variant for dynamic dispatching to allow
// for customability..

use crate::commit::NodeID;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// pub trait MsgData: Debug {}

// TODO: What if we want to simultaneously dispatch a Message to two different LLM models,
// such as ChatGPT and HuggingFace? The way messages seems to be stored right now feels kind of application
// specific.

// TODO: Eventually convert to a dynamic setup
// #[derive(Debug)]
// pub struct Msg {
//     pub history: Vec<Rc<Msg>>,
//     // We dynamically dispatch the type because there might be multiple message
//     // types in the same `Messages` table or `CausalChain`
//     pub data: Box<dyn MsgData>,
//     // TOOD: Add creator field, basically a stamp from who created the message,
//     // either a Human, a Program, or an LLM
// }

#[derive(Debug)]
pub struct Msg {
    pub history: Vec<Rc<Msg>>,
    // pub data: T,
    pub role: GptRole,
    pub content: String,
}

impl<T> Msg<T> {
    pub fn new(history: Vec<Rc<Msg<T>>>, msg: String, data: T) -> Self {
        Self { history, data }
    }
}

#[derive(Default)]
pub struct Messages<T>(pub HashMap<NodeID, Rc<Msg<T>>>);

// Deref coercion
impl<T> Deref for Messages<T> {
    type Target = HashMap<NodeID, Rc<Msg<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut trait for your custom type
impl<T> DerefMut for Messages<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
