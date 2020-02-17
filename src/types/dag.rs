/*
** src/types/dag.rs
*/

use std::collections::HashMap;
use std::hash::Hash;

struct DAGNode<T> {
    item: T,
    succ: Option<usize>,
}

impl<T> DAGNode<T> {
    fn new(item: T, succ: Option<usize>) -> Self {
        Self { item, succ }
    }

    fn root(item: T) -> Self {
        Self::new(item, None)
    }
}

pub struct DAG<T>
where T: Clone + Hash + Eq
{
    nodes: Vec<DAGNode<T>>,
    // map an item to the index of its node for constant-time lookups
    node_idxs: HashMap<T, usize>,
}

impl<T> DAG<T>
where T: Clone + Hash + Eq
{
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            node_idxs: HashMap::new(),
        }
    }

    pub fn insert_root(&mut self, item: T) {
        // ensure the nodes are cleared out
        self.nodes.clear();
        self.nodes.push(DAGNode::root(item));
        self.node_idxs.insert(self.nodes[0].item.clone(), 0);
    }

    pub fn insert(&mut self, item: T, succ: T) {
        // get index of the successor
        // fall through without inserting otherwise
        if let Some(succ_idx) = self.node_idxs.get(&succ) {
            self.nodes.push(DAGNode::new(item, Some(*succ_idx)));
            let idx = self.nodes.len() - 1;
            self.node_idxs.insert(self.nodes[idx].item.clone(), idx);
        }
    }

    // NOTE: does not include item in the path
    pub fn path_to_root<'a>(&'a self, item: T) -> Vec<&'a T> {
        let mut path = vec![];

        // get the item node
        if let Some(node_idx) = self.node_idxs.get(&item) {
            // traverse until the root node is reached
            let mut succ = self.nodes[*node_idx].succ;
            while let Some(succ_idx) = succ {
                let succ_node = &self.nodes[succ_idx];
                path.push(&succ_node.item);
                succ = succ_node.succ;
            }
        }
        // add the root
        path.push(&self.nodes[0].item);

        path
    }
}
