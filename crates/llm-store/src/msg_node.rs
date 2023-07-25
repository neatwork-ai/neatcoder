use anyhow::Result;
// use sha3::{Sha3_256, Digest};
use sha2::{Digest, Sha256};
use std::ops::{Deref, DerefMut};
use std::{collections::HashMap, rc::Rc};

use crate::commit::{IdHash, SmallHash};
use crate::job::{Job, LLMJob, ProgramJob};
use crate::msg::Msg;
use std::collections::HashSet;

pub type NodeID = IdHash;
pub type Commit = SmallHash;

// TODO: Consider using a cryptographic accumulator
// which allows to create a large number of unique identifiers that are small in size

// TODO: Consider using petgraph for out-of the box graph support
pub struct Node {
    // Parent Commit is a SetHash over the parents' `NodeID`s
    pub parent_commit: Option<Commit>,
    pub inner: NodeType,
}

// TODO: Add generic D for data
pub enum NodeType {
    Msg(Rc<Msg>),
    Job(Job),
}

impl std::ops::Deref for Node {
    // type Target = Rc<T>;
    type Target = NodeType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct CausalChain {
    pub genesis_id: NodeID,
    pub nodes: HashMap<Node, Node>,
    pub edges: HashMap<NodeID, NodeID>,
    // Mapping between Commits and respective Nodes
    pub commits: HashMap<Commit, Vec<NodeID>>,
}

impl Node {
    pub fn new_msg(msg: Rc<Msg>, parent_commit: Option<Commit>) -> Self {
        let mut hasher = Sha256::new();

        if let Some(parent) = &parent_commit {
            hasher.update(parent);
        }

        hasher.update(msg.msg.as_bytes());

        let bytes = hasher.finalize().into();
        let hash: MsgId = Id(bytes);

        Self {
            hash,
            inner: NodeType::Msg(msg),
        }
    }

    // TODO
    // pub fn new_generator(parent: Option<MsgId>) -> Self {
    //     let mut hasher = Sha256::new();

    //     hasher.update(msg.msg.as_bytes());

    //     if let Some(parent) = &parent {
    //         hasher.update(parent);
    //     }

    //     let bytes = hasher.finalize().into();
    //     let hash: MsgId = Id(bytes);

    //     Self {
    //         hash,
    //         inner: NodeType::Generator,
    //     }
    // }
}

impl CausalChain {
    pub fn genesis(msg: Rc<Msg>) -> Self {
        let Node {
            hash,
            inner: genesis_msg,
        } = Node::new_msg(msg, None);

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
