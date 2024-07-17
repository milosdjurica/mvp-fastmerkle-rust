use crate::{
    hasher::FastHasherPool,
    tree::{MerkleTree, Node},
    worker::{WorkerJob, WorkerPool},
};
use std::{error::Error, sync::Arc};

pub fn generate_merkle_tree(
    input_data: Vec<Vec<u8>>,
    hasher_pool: Arc<FastHasherPool>,
) -> Result<MerkleTree, Box<dyn Error + Send + Sync>> {
    if input_data.is_empty() {
        return Err("empty data set provided".into());
    }

    // Create the worker pool and put them on standby
    let worker_pool = WorkerPool::new(input_data.len() + 1, hasher_pool);
    // Generate the leaves of the Merkle tree
    let mut nodes = generate_leaves(input_data, worker_pool.clone())?;

    while nodes.len() > 1 {
        nodes = adjust_level_size(nodes);

        for i in (0..nodes.len()).step_by(2) {
            worker_pool.add_job(WorkerJob {
                store_index: i,
                source_data: vec![nodes[i].hash.clone(), nodes[i + 1].hash.clone()],
            });
        }

        for _ in 0..nodes.len() / 2 {
            let result = worker_pool.get_result();
            if let Some(error) = result.error {
                return Err(error);
            }

            let parent = Node {
                hash: result.hash_data,
                left: Some(Box::new(nodes[result.store_index].clone())),
                right: Some(Box::new(nodes[result.store_index + 1].clone())),
                parent: None,
            };

            if let Some(left) = &mut nodes[result.store_index].parent {
                *left = &parent as *const Node as *mut Node;
            }

            if let Some(right) = &mut nodes[result.store_index + 1].parent {
                *right = &parent as *const Node as *mut Node;
            }

            nodes[result.store_index] = parent;
        }

        nodes = pack_level_results(nodes);
    }

    Ok(MerkleTree {
        root: Some(Box::new(nodes[0].clone())),
    })
}

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
