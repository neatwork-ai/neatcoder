use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::commit::NodeID;

pub trait MsgData: Debug {}

pub struct Msg {
    pub history: Vec<Rc<Msg>>,
    pub msg: String,
    pub data: Box<dyn MsgData>,
    // TOOD: Add creator field, basically a stamp from who created the message,
    // either a Human, a Program, or an LLM
}

impl Msg {
    pub fn new(history: Vec<Rc<Msg>>, msg: String, data: Box<dyn MsgData>) -> Self {
        Self { history, msg, data }
    }
}

#[derive(Default)]
pub struct Messages(pub HashMap<NodeID, Rc<Msg>>);

// Deref coercion
impl Deref for Messages {
    type Target = HashMap<NodeID, Rc<Msg>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut trait for your custom type
impl DerefMut for Messages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
