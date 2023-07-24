use anyhow::Result;
use gluon::ai::openai::input::GptRole;
use gluon::ai::openai::input::Message;
use sha2::{Digest, Sha256};
use std::ops::{Deref, DerefMut};
use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Id([u8; 32]);

type MsgId = Id;

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Default)]
pub struct Messages(pub HashMap<MsgId, Rc<Msg>>);

// Deref coercion
impl Deref for Messages {
    type Target = HashMap<MsgId, Rc<Msg>>;

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

// TODO: Consider using petgraph for out-of the box graph support
pub struct Node<T> {
    pub hash: MsgId,
    pub inner: Rc<T>,
}

impl<T> std::ops::Deref for Node<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Msg {
    // TODO: `role` should not be in the same struct
    // The database needs to have some STAR structure in which model specific info
    // gets stored in a separate table.
    pub role: GptRole,
    pub history: Vec<Rc<Msg>>,
    pub msg: String,
}

// Eventually only one generic type will be needed
impl Into<Msg> for Message {
    fn into(self) -> Msg {
        Msg {
            role: self.role,
            history: vec![],
            msg: self.content,
        }
    }
}

pub struct Edge {
    pub from: MsgId,
    pub to: MsgId,
}

pub struct CausalChain {
    pub genesis_id: MsgId,
    pub nodes: HashMap<MsgId, Rc<Msg>>,
    pub edges: Vec<Edge>,
}

impl Msg {
    pub fn new(role: GptRole, history: Vec<Rc<Msg>>, msg: String) -> Self {
        Self { role, history, msg }
    }
}

impl Node<Msg> {
    pub fn new(msg: Rc<Msg>, parent: Option<MsgId>) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(msg.msg.as_bytes());

        if let Some(parent) = &parent {
            hasher.update(parent);
        }

        let bytes = hasher.finalize().into();
        let hash: MsgId = Id(bytes);

        Self { hash, inner: msg }
    }
}

impl CausalChain {
    pub fn genesis(msg: Rc<Msg>) -> Self {
        let Node {
            hash,
            inner: genesis_msg,
        } = Node::new(msg, None);

        let msgs = HashMap::from([(hash, genesis_msg)]);

        Self {
            genesis_id: hash,
            nodes: msgs,
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, msg: Rc<Msg>, parent: Option<MsgId>) -> Result<MsgId> {
        let Node { hash, inner: msg } = Node::new(msg, None);

        if let Some(parent) = parent {
            let edge = Edge {
                from: hash,
                to: parent,
            };

            self.edges.push(edge);
        }

        self.nodes.insert(hash, msg);

        Ok(hash)
    }
}
