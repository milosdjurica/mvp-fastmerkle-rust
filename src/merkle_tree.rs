use crate::{hasher::FastHasherPool, tree::Node, worker::WorkerPool};
use std::sync::Arc;

pub fn generate_merkle_tree(input_data: Vec<Vec<u8>>, hasher_pool: Arc<FastHasherPool>) {}

pub fn pack_level_results(nodes: Vec<Node>) {}

pub fn generate_leaves(input_data: Vec<Vec<u8>>, worker_pool: WorkerPool) {}

pub fn adjust_level_size(nodes: Vec<Node>) {}
