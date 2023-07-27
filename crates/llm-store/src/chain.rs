use crate::commit::SmallHash;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::ops::DerefMut;
use std::{collections::HashMap, rc::Rc};

use crate::commit::{Commit, HashID, NodeID};
use crate::job::Job;
use crate::msg::Msg;

// TODO: Consider using a cryptographic accumulator
// which allows to create a large number of unique identifiers that are small in size

// TODO: Consider using petgraph for out-of the box graph support
pub struct Node {
    // Max of 65_535 versions should be more than enough
    pub version: u16,
    // If the version is not `1`, then we add the `original` node's NodeID
    // here in a truncated form
    pub original: Option<SmallHash>,
    pub node: UnversionedNode,
}

pub struct UnversionedNode {
    // Parent Commit is a SetHash over the parents' `NodeID`s
    pub parent_commit: Option<Commit>,
    pub inner: NodeType,
}

impl std::ops::Deref for UnversionedNode {
    // type Target = Rc<T>;
    type Target = NodeType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl UnversionedNode {
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
}

// TODO: Add generic D for data
pub enum NodeType {
    Msg(Rc<Msg>),
    Job(Job),
}

impl Node {
    pub fn init_msg(msg: Rc<Msg>, parent_commit: Option<Commit>) -> Self {
        Self {
            version: 1,
            original: None,
            node: UnversionedNode::new_msg(msg, parent_commit),
        }
    }

    pub fn init_job(job: Job, parent_commit: Option<Commit>) -> Self {
        Self {
            version: 1,
            original: None,
            node: UnversionedNode::new_job(job, parent_commit),
        }
    }

    pub fn new_msg(
        version: u16,
        original_id: Option<SmallHash>,
        parent_commit: Option<Commit>,
        msg: Rc<Msg>,
    ) -> Self {
        if version > 1 && original_id.is_none() {
            panic!("If node is nor original version, then it must point to the original node ID");
        }

        Self {
            version,
            original: original_id,
            node: UnversionedNode::new_msg(msg, parent_commit),
        }
    }

    pub fn new_job(
        version: u16,
        original_id: Option<SmallHash>,
        parent_commit: Option<Commit>,
        job: Job,
    ) -> Self {
        if version > 1 && original_id.is_none() {
            panic!("If node is nor original version, then it must point to the original node ID");
        }

        Self {
            version,
            original: original_id,
            node: UnversionedNode::new_job(job, parent_commit),
        }
    }

    pub fn hash_node(&self) -> NodeID {
        let mut hasher = Sha256::new();

        if let Some(parent) = &self.node.parent_commit {
            // Using Big Endian given that UTF-8 reads left to right
            hasher.update(parent.to_be_bytes());
        }

        match &self.node.inner {
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
    // TODO: Consider if it's possible to have multiple roots
    /// The Root node of the chain
    pub genesis_id: NodeID,
    /// Contains the actual graph
    pub dag: Dag,
    /// Mapping between Commits and respective Nodes. A commit is a
    /// hash that represents a combination of nodes that all feed to a
    /// downstram child node. Given that all nodes store their parent commit
    /// this field allows us to keep track of all the parent nodes of a given node.
    pub commits: HashMap<Commit, Vec<NodeID>>,
}

pub struct Version {
    pub version: u16,
    pub leaf_node: NodeID,
}

#[derive(Default)]
pub struct Dag {
    /// Stores all the nodes in a map queryable by their node ID
    pub nodes: HashMap<NodeID, Rc<Node>>,
    /// Maps parent nodes to child nodes.
    pub edges: HashMap<NodeID, HashSet<NodeID>>,
    // Map of versions and corresponding leaf nodes for that version
    pub versions: HashMap<u16, HashSet<NodeID>>,
}

impl Dag {
    pub fn merge(&mut self, other: Dag) {
        let Dag {
            nodes,
            edges,
            versions,
        } = other;

        self.nodes.extend(nodes);
        self.edges.extend(edges);
        // Overwrites ought to happens as sub-dags extend the main dag
        self.versions.extend(versions);
    }
}

impl CausalChain {
    // === Write Methods ===

    pub fn genesis(msg: Rc<Msg>) -> Self {
        // Technically the root is the Job that generates the genesis message
        let msg_node = Node::init_msg(msg, None);

        let node_id = msg_node.hash_node();
        let nodes = HashMap::from([(node_id, Rc::new(msg_node))]);
        let genesis_commit = node_id.truncate();

        let leaf_nodes = HashSet::from([node_id]);

        Self {
            genesis_id: node_id,
            dag: Dag {
                nodes,
                edges: HashMap::new(),
                versions: HashMap::from([(1, leaf_nodes)]),
            },
            commits: HashMap::from([(genesis_commit, vec![node_id])]),
        }
    }

    pub fn add_child_nodes(&mut self, parent_nodes: Vec<NodeID>, mut child_nodes: Vec<Node>) {
        for child in child_nodes.drain(..) {
            let child_id = child.hash_node();

            // For each parent add edges from parent to child
            for parent_id in parent_nodes.iter() {
                self.add_edge(*parent_id, child_id);
            }

            // Add child Node to the graph
            self.dag.nodes.insert(child_id, Rc::new(child));
        }

        let commit: Commit = HashID::order_invariant_hash_vec(&parent_nodes).truncate();

        self.commits.insert(commit, parent_nodes);
    }

    pub fn add_new_node_version(&mut self, new_node: UnversionedNode) {}

    // === Read Methods ===

    pub fn fetch_node(&self, node_id: NodeID) -> &Node {
        self.dag
            .nodes
            .get(&node_id)
            .expect(&format!("Could not fing NodeID: {:?}", node_id))
    }

    pub fn get_parent_ids(&self, parent_commit: &Commit) -> &Vec<HashID> {
        self.commits
            .get(parent_commit)
            .expect(&format!("Could not fing Commit: {:?}", parent_commit))
    }

    pub fn get_parent_nodes(&self, parent_commit: &Commit) -> Vec<(NodeID, Rc<Node>)> {
        let parent_ids = self
            .commits
            .get(parent_commit)
            .expect(&format!("Could not fing Commit: {:?}", parent_commit));

        // Ideally something more efficient like the experimental
        // `get_many_mut` but without the `mut`
        parent_ids
            .iter()
            .map(|parent_id| {
                (
                    *parent_id,
                    self.dag
                        .nodes
                        .get(&parent_id)
                        .expect(&format!("Could not fing NodeID: {:?}", parent_id))
                        .clone(), // Rc cloning,
                )
            })
            .collect()
    }

    // === Sub-Dag Read Methods ===

    pub fn walk_up(&self, start_node_id: NodeID) -> Dag {
        if start_node_id == self.genesis_id {
            // TODO: What happens if there are multiple genesis nodes?
            // Is that even possible?
            panic!("Already in the genesis node");
        }

        self.walk_up_(start_node_id)
    }

    pub fn get_dag_version(&self, version: u16) -> Option<Dag> {
        let leaf_nodes = self.dag.versions.get(&version);

        if let Some(leaf_nodes) = leaf_nodes {
            let mut version_dag = Dag::default();

            for leaf_node in leaf_nodes.iter() {
                let sub_dag = self.walk_up(*leaf_node);
                version_dag.merge(sub_dag);
            }

            return Some(version_dag);
        }

        None
    }

    // === Private Methods ===

    fn add_edge(&mut self, parent_node: NodeID, child_node: NodeID) {
        if let Some(children) = self.dag.edges.get_mut(&parent_node) {
            children.insert(child_node);
        } else {
            let children = HashSet::from([child_node]);
            self.dag.edges.insert(parent_node, children);
        }
    }

    fn walk_up_(&self, start_node_id: NodeID) -> Dag {
        let start_node = self.fetch_node(start_node_id);

        let mut sub_dag = Dag::default();

        // If there are any parents, then apply function recursively
        if let Some(parent_commit) = start_node.parent_commit {
            let parent_ids = self.get_parent_ids(&parent_commit);

            // Apply recursion
            for node_id in parent_ids {
                let parent_dag = self.walk_up_(*node_id);

                // TODO: Check what is faster, merge DAGs iteratively accross the recursion
                // or collect all nodes and edges and submit them only in the end to the main dag?
                sub_dag.merge(parent_dag);
            }
        }

        sub_dag
    }
}
