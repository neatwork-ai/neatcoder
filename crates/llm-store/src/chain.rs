// use sha2::{Digest, Sha256};
// use std::collections::HashSet;
// use std::{collections::HashMap, rc::Rc};

// use crate::commit::{Commit, HashID, NodeID};
// use crate::job::Job;
// use crate::msg::Msg;

// // TODO: Consider using a cryptographic accumulator
// // which allows to create a large number of unique identifiers that are small in size

// // TODO: Consider using petgraph for out-of the box graph support
// pub struct Node {
//     /// Max of 65_535 versions should be more than enough
//     pub version: u16,
//     /// If the version is not `1`, then we add the `original` node's NodeID
//     /// here in a truncated form
//     // TODO: Make this Small Hash
//     pub original: Option<NodeID>,
//     pub node: UnversionedNode,
// }

// pub struct UnversionedNode {
//     // Parent Commit is a SetHash over the parents' `NodeID`s
//     pub parent_commit: Option<Commit>,
//     pub inner: NodeType,
// }

// impl std::ops::Deref for UnversionedNode {
//     // type Target = Rc<T>;
//     type Target = NodeType;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl UnversionedNode {
//     pub fn new_msg(msg: Rc<Msg>, parent_commit: Option<Commit>) -> Self {
//         Self {
//             parent_commit,
//             inner: NodeType::Msg(msg),
//         }
//     }

//     pub fn new_job(job: Job, parent_commit: Option<Commit>) -> Self {
//         Self {
//             parent_commit,
//             inner: NodeType::Job(job),
//         }
//     }
// }

// // TODO: Add generic D for data
// pub enum NodeType {
//     Msg(Rc<Msg>),
//     Job(Job),
// }

// impl Node {
//     pub fn init_msg(msg: Rc<Msg>, parent_commit: Option<Commit>) -> Self {
//         Self {
//             version: 1,
//             original: None,
//             node: UnversionedNode::new_msg(msg, parent_commit),
//         }
//     }

//     pub fn init_job(job: Job, parent_commit: Option<Commit>) -> Self {
//         Self {
//             version: 1,
//             original: None,
//             node: UnversionedNode::new_job(job, parent_commit),
//         }
//     }

//     pub fn new_msg(
//         version: u16,
//         original_id: Option<NodeID>,
//         parent_commit: Option<Commit>,
//         msg: Rc<Msg>,
//     ) -> Self {
//         if version > 1 && original_id.is_none() {
//             panic!("If node is nor original version, then it must point to the original node ID");
//         }

//         Self {
//             version,
//             original: original_id,
//             node: UnversionedNode::new_msg(msg, parent_commit),
//         }
//     }

//     pub fn new_job(
//         version: u16,
//         original_id: Option<NodeID>,
//         parent_commit: Option<Commit>,
//         job: Job,
//     ) -> Self {
//         if version > 1 && original_id.is_none() {
//             panic!("If node is nor original version, then it must point to the original node ID");
//         }

//         Self {
//             version,
//             original: original_id,
//             node: UnversionedNode::new_job(job, parent_commit),
//         }
//     }

//     pub fn hash_node(&self) -> NodeID {
//         let mut hasher = Sha256::new();

//         if let Some(parent) = &self.node.parent_commit {
//             // Using Big Endian given that UTF-8 reads left to right
//             hasher.update(parent.to_be_bytes());
//         }

//         match &self.node.inner {
//             NodeType::Msg(msg) => {
//                 hasher.update(msg.msg.as_bytes());
//             }
//             NodeType::Job(_) => {
//                 panic!("TODO: implement")
//             }
//         }

//         let bytes = hasher.finalize().into();

//         HashID(bytes)
//     }
// }

// // TODO:
// // - Move all NodeIDs to SmallIDs except in the source of truth such as `nodes` and `edges`
// // - Only merge the DAG diffs to avoid having collision false positives
// // - With this last point implemented, we can then apply fall-back logic to compare the full hash
// // in case a real collision occurs. This will allow us to shorten the SmallID even more from 64 to 32 bits
// // giving a probability of collision of 1 in `77,163`.
// // - A node is only made immutable once committed, so we can implement a 4-bit flag
// // to control for collisions. When a collision is detected we bump one bit in the flag
// // and proceed to rehash the node. A 4-bit flag gives us `4!` possible combinations which
// // raises the probability of a asymptotic collission to `77,162^24` which is equal to
// // `2.65*10^68` which is close to SHA-256 resistance.
// pub struct CausalChain {
//     // TODO: Consider if it's possible to have multiple roots
//     /// The Root node of the chain
//     pub genesis_id: NodeID,
//     /// Contains the actual graph
//     pub dag: Dag,
//     /// Mapping between Commits and respective Nodes. A commit is a
//     /// hash that represents a combination of nodes that all feed to a
//     /// downstram child node. Given that all nodes store their parent commit
//     /// this field allows us to keep track of all the parent nodes of a given node.
//     pub commits: HashMap<Commit, Vec<NodeID>>,
// }

// pub struct Version {
//     pub version: u16,
//     pub leaf_node: NodeID,
// }

// #[derive(Default)]
// pub struct Dag {
//     /// Stores all the nodes in a map queryable by their node ID
//     pub nodes: HashMap<NodeID, Rc<Node>>,
//     /// Maps parent nodes to child nodes.
//     pub edges: HashMap<NodeID, HashSet<NodeID>>,
//     // Map of versions and corresponding leaf nodes for that version
//     pub versions: HashMap<u16, HashSet<NodeID>>,
//     // Maps the versions of each original node in the graph, if the version is > 1
//     // This is because, not storing it if the version is 1 it will save space and it
//     // still allows the user to infer the version to be 1.
//     pub node_versions: HashMap<NodeID, u16>,
// }

// impl Dag {
//     pub fn merge(&mut self, other: Dag) {
//         let Dag {
//             nodes,
//             edges,
//             versions,
//             node_versions,
//         } = other;

//         self.nodes.extend(nodes);
//         self.edges.extend(edges);
//         // Overwrites ought to happens as sub-dags extend the main dag
//         self.versions.extend(versions);
//         self.node_versions.extend(node_versions);
//     }
// }

// impl CausalChain {
//     // === Write Methods ===

//     pub fn genesis(msg: Rc<Msg>) -> Self {
//         // Technically the root is the Job that generates the genesis message
//         let msg_node = Node::init_msg(msg, None);

//         let node_id = msg_node.hash_node();
//         let nodes = HashMap::from([(node_id, Rc::new(msg_node))]);
//         let genesis_commit = node_id.truncate();

//         let leaf_nodes = HashSet::from([node_id]);

//         Self {
//             genesis_id: node_id,
//             dag: Dag {
//                 nodes,
//                 edges: HashMap::new(),
//                 versions: HashMap::from([(1, leaf_nodes)]),
//                 node_versions: HashMap::new(),
//             },
//             commits: HashMap::from([(genesis_commit, vec![node_id])]),
//         }
//     }

//     pub fn add_child_nodes(&mut self, parent_nodes: Vec<NodeID>, mut child_nodes: Vec<Node>) {
//         for child in child_nodes.drain(..) {
//             let child_id = child.hash_node();

//             // For each parent add edges from parent to child
//             for parent_id in parent_nodes.iter() {
//                 self.add_edge(*parent_id, child_id);
//             }

//             // Add child Node to the graph
//             self.dag.nodes.insert(child_id, Rc::new(child));
//         }

//         let commit: Commit = HashID::order_invariant_hash_vec(&parent_nodes).truncate();

//         self.commits.insert(commit, parent_nodes);
//     }

//     // An assumption that this function is making is that new versions of the original node
//     // have the same parents as the original node. We need to reevaluate this assumption in case
//     // new versions need to rely on extra input from the application --> This needs to be researched further
//     pub fn add_new_node_version(&mut self, new_node: UnversionedNode, original_node_id: &NodeID) {
//         let original_node = self.get_node(original_node_id);

//         // Check that original_node is in fact original
//         if original_node.original.is_some() {
//             panic!(
//                 "The parameter given `original_node_id` does not correspond to an original node"
//             );
//         }

//         // Get latest version of the node
//         let bumped_version = if let Some(latest) = self.dag.node_versions.get_mut(&original_node_id)
//         {
//             // Bumps the latest version
//             *latest = *latest + 1;
//             // Returns bumped version
//             *latest
//         } else {
//             // This block runs when the version is the initial version 1
//             // We therefore bump the version and add it to the the hashmap
//             let latest = 2;

//             // Adding version
//             self.dag.node_versions.insert(*original_node_id, latest);

//             // Returns bumped version
//             latest
//         };

//         // Add version to the new node
//         let new_node = Node {
//             version: bumped_version,
//             original: Some(*original_node_id),
//             node: new_node,
//         };

//         // TODO: Here
//     }

//     // === Read Methods ===

//     pub fn get_node(&self, node_id: &NodeID) -> &Node {
//         self.dag
//             .nodes
//             .get(&node_id)
//             .expect(&format!("Could not fing NodeID: {:?}", node_id))
//     }

//     pub fn get_parent_ids(&self, parent_commit: &Commit) -> &Vec<HashID> {
//         self.commits
//             .get(parent_commit)
//             .expect(&format!("Could not fing Commit: {:?}", parent_commit))
//     }

//     pub fn get_parent_nodes(&self, parent_commit: &Commit) -> Vec<(NodeID, Rc<Node>)> {
//         let parent_ids = self
//             .commits
//             .get(parent_commit)
//             .expect(&format!("Could not fing Commit: {:?}", parent_commit));

//         // Ideally something more efficient like the experimental
//         // `get_many_mut` but without the `mut`
//         parent_ids
//             .iter()
//             .map(|parent_id| {
//                 (
//                     *parent_id,
//                     self.dag
//                         .nodes
//                         .get(&parent_id)
//                         .expect(&format!("Could not fing NodeID: {:?}", parent_id))
//                         .clone(), // Rc cloning,
//                 )
//             })
//             .collect()
//     }

//     // === Sub-Dag Read Methods ===

//     pub fn walk_up(&self, start_node_id: NodeID) -> Dag {
//         if start_node_id == self.genesis_id {
//             // TODO: What happens if there are multiple genesis nodes?
//             // Is that even possible?
//             panic!("Already in the genesis node");
//         }

//         self.walk_up_(start_node_id)
//     }

//     pub fn get_dag_version(&self, version: u16) -> Option<Dag> {
//         let leaf_nodes = self.dag.versions.get(&version);

//         if let Some(leaf_nodes) = leaf_nodes {
//             let mut version_dag = Dag::default();

//             for leaf_node in leaf_nodes.iter() {
//                 let sub_dag = self.walk_up(*leaf_node);
//                 version_dag.merge(sub_dag);
//             }

//             return Some(version_dag);
//         }

//         None
//     }

//     // === Private Methods ===

//     fn add_edge(&mut self, parent_node: NodeID, child_node: NodeID) {
//         if let Some(children) = self.dag.edges.get_mut(&parent_node) {
//             children.insert(child_node);
//         } else {
//             let children = HashSet::from([child_node]);
//             self.dag.edges.insert(parent_node, children);
//         }
//     }

//     fn walk_up_(&self, start_node_id: NodeID) -> Dag {
//         let start_node = self.get_node(&start_node_id);

//         let mut sub_dag = Dag::default();

//         // If there are any parents, then apply function recursively
//         if let Some(parent_commit) = start_node.node.parent_commit {
//             let parent_ids = self.get_parent_ids(&parent_commit);

//             // Apply recursion
//             for node_id in parent_ids {
//                 let parent_dag = self.walk_up_(*node_id);

//                 // TODO: Check what is faster, merge DAGs iteratively accross the recursion
//                 // or collect all nodes and edges and submit them only in the end to the main dag?
//                 sub_dag.merge(parent_dag);
//             }
//         }

//         sub_dag
//     }
// }
