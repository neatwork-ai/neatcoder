use anyhow::Result;
use std::collections::HashMap;

use gluon::ai::openai::input::GptRole;
use sha2::{Digest, Sha256};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct NodeId([u8; 32]);

impl AsRef<[u8]> for NodeId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

// TODO: Consider using petgraph for out-of the box graph support
pub struct Node<T> {
    pub hash: NodeId,
    pub inner: T,
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Msg {
    // TODO: this should not be in the same struct
    // The database needs to have some STAR structure in which model specific info
    // gets stored in a separate table.
    pub role: GptRole,
    pub history: Vec<NodeId>,
    pub msg: String,
}

pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
}

pub struct CausalChain {
    pub genesis_id: NodeId,
    pub nodes: HashMap<NodeId, Msg>,
    pub edges: Vec<Edge>,
}

impl Msg {
    pub fn new(role: GptRole, history: Vec<NodeId>, msg: String) -> Self {
        Self { role, history, msg }
    }
}

impl Node<Msg> {
    pub fn new(role: GptRole, history: Vec<NodeId>, parent: Option<NodeId>, msg: String) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(msg.as_bytes());

        if let Some(parent) = &parent {
            hasher.update(parent);
        }

        let bytes = hasher.finalize().into();
        let hash = NodeId(bytes);

        Self {
            hash,
            inner: Msg::new(role, history, msg),
        }
    }
}

impl CausalChain {
    pub fn genesis(
        role: GptRole,
        // The reason why genesis messages can still have history is because
        // history can be mocked
        history: Vec<NodeId>,
        msg: String,
    ) -> Self {
        let Node {
            hash,
            inner: genesis_msg,
        } = Node::new(role, history, None, msg);

        let msgs = HashMap::from([(hash, genesis_msg)]);

        Self {
            genesis_id: hash,
            nodes: msgs,
            edges: vec![],
        }
    }

    pub fn add_node(
        &mut self,
        role: GptRole,
        history: Vec<NodeId>,
        parent: Option<NodeId>,
        msg: String,
    ) -> Result<NodeId> {
        let Node { hash, inner: msg } = Node::new(role, history, None, msg);

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
