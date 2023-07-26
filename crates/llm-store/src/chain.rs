use sha2::{Digest, Sha256};
use std::{collections::HashMap, rc::Rc};

use crate::commit::{Commit, HashID, NodeID};
use crate::job::Job;
use crate::msg::Msg;

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

impl Node {
    pub fn new_msg(msg: Rc<Msg>, parent_commit: Option<Commit>) -> Self {
        Self {
            parent_commit,
            inner: NodeType::Msg(msg),
        }
    }

    pub fn new_job(job: Job, parent_commit: Option<Commit>) -> Self {
        Self {
            parent_commit,
            inner: NodeType::Job(job),
        }
    }

    pub fn hash_node(&self) -> NodeID {
        let mut hasher = Sha256::new();

        if let Some(parent) = &self.parent_commit {
            // Using Big Endian given that UTF-8 reads left to right
            hasher.update(parent.to_be_bytes());
        }

        match &self.inner {
            NodeType::Msg(msg) => {
                hasher.update(msg.msg.as_bytes());
            }
            NodeType::Job(_) => {
                panic!("TODO: implement")
            }
        }

        let bytes = hasher.finalize().into();

        HashID(bytes)
    }
}

pub struct CausalChain {
    pub genesis_id: NodeID,
    pub dag: DAG,
    // Mapping between Commits and respective Nodes
    pub commits: HashMap<Commit, Vec<NodeID>>,
}

pub struct DAG {
    pub nodes: HashMap<NodeID, Node>,
    pub edges: HashMap<NodeID, NodeID>,
}

impl CausalChain {
    pub fn genesis(msg: Rc<Msg>) -> Self {
        // Technically the root is the Job that generates the genesis message
        let msg_node = Node::new_msg(msg, None);

        let node_id = msg_node.hash_node();
        let nodes = HashMap::from([(node_id, msg_node)]);
        let genesis_commit = node_id.truncate();

        Self {
            genesis_id: node_id,
            dag: DAG {
                nodes,
                edges: HashMap::new(),
            },
            commits: HashMap::from([(genesis_commit, vec![node_id])]),
        }
    }

    pub fn commit_layer(&mut self, parent_nodes: Vec<NodeID>, mut child_nodes: Vec<Node>) {
        for child in child_nodes.drain(..) {
            let child_id = child.hash_node();

            // For each parent add edges from parent to child
            for parent_id in parent_nodes.iter() {
                self.dag.edges.insert(*parent_id, child_id);
            }

            // Add child Node to the graph
            self.dag.nodes.insert(child_id, child);
        }

        let commit: Commit = HashID::order_invariant_hash_vec(&parent_nodes).truncate();

        self.commits.insert(commit, parent_nodes);
    }
}
