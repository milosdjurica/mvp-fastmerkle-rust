mod hasher;
mod merkle_test;
mod merkle_tree;
mod tree;
mod worker;

use hasher::{FastHasher, FastHasherPool};
use merkle_tree::generate_merkle_tree;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");

    let mut hasher = FastHasher::new();

    let data = b"Hello, world!";

    hasher
        .add_to_hash(data)
        .expect("Failed to add data to hash");

    let hash_result = hasher.get_hash();
    println!("Hash result {:#?}", hash_result);

    let hasher_pool = Arc::new(FastHasherPool::new(4));
    let data = vec![
        b"data1".to_vec(),
        b"data2".to_vec(),
        b"data3".to_vec(),
        b"data4".to_vec(),
    ];

    match generate_merkle_tree(data, Arc::clone(&hasher_pool)) {
        Ok(tree) => {
            if let Some(root_hash) = tree.root_hash() {
                println!("Merkle Tree Root Hash: {:?}", root_hash);
            } else {
                println!("Merkle Tree is empty.");
            }
        }
        Err(e) => eprintln!("Error generating Merkle Tree: {}", e),
    }
}
