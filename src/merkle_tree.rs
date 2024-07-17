use crate::{
    hasher::FastHasherPool,
    tree::Node,
    worker::{WorkerJob, WorkerPool},
};
use std::{error::Error, sync::Arc};

pub fn generate_merkle_tree(input_data: Vec<Vec<u8>>, hasher_pool: Arc<FastHasherPool>) {}

pub fn pack_level_results(nodes: Vec<Node>) -> Vec<Node> {
    let mut new_nodes = Vec::with_capacity(nodes.len() / 2);
    for i in (0..nodes.len()).step_by(2) {
        new_nodes.push(nodes[i].clone());
    }
    new_nodes
}

pub fn generate_leaves(
    input_data: Vec<Vec<u8>>,
    worker_pool: WorkerPool,
) -> Result<Vec<Node>, Box<dyn Error + Send + Sync>> {
    let mut leaves: Vec<Node> = Vec::with_capacity(input_data.len());

    for (i, input) in input_data.into_iter().enumerate() {
        worker_pool.add_job(WorkerJob {
            store_index: i,
            source_data: vec![input],
        })
    }

    for _ in 0..leaves.capacity() {
        let result = worker_pool.get_result();
        if let Some(error) = result.error {
            return Err(error);
        }

        leaves.push(Node {
            hash: result.hash_data,
            left: None,
            right: None,
            parent: None,
        });
    }
    Ok(adjust_level_size(leaves))
}

pub fn adjust_level_size(mut nodes: Vec<Node>) -> Vec<Node> {
    if nodes.len() % 2 != 0 {
        nodes.push(nodes.last().unwrap().duplicate());
    }
    nodes
}
