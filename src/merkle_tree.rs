use crate::{hasher::FastHasherPool, tree::Node, worker::WorkerPool};
use std::sync::Arc;

pub fn generate_merkle_tree(input_data: Vec<Vec<u8>>, hasher_pool: Arc<FastHasherPool>) {}

pub fn pack_level_results(nodes: Vec<Node>) -> Vec<Node> {
    let mut new_nodes = Vec::with_capacity(nodes.len() / 2);
    for i in (0..nodes.len()).step_by(2) {
        new_nodes.push(nodes[i].clone());
    }
    new_nodes
}

pub fn generate_leaves(input_data: Vec<Vec<u8>>, worker_pool: WorkerPool) {}

pub fn adjust_level_size(mut nodes: Vec<Node>) -> Vec<Node> {
    if nodes.len() % 2 != 0 {
        nodes.push(nodes.last().unwrap().duplicate());
    }
    nodes
}
