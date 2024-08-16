mod hasher;
mod merkle_test;
mod merkle_tree;
mod tree;
mod worker;
use hasher::FastHasher;

fn main() {
    println!("Hello, world!");

    let mut hasher = FastHasher::new();

    let data = b"Hello, world!";

    hasher
        .add_to_hash(data)
        .expect("Failed to add data to hash");

    let hash_result = hasher.get_hash();
    println!("Hash result {:#?}", hash_result);
}
