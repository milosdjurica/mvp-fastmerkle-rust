use hex;
use std::error::Error;

fn benchmark_generate_merkle_tree_5() {}
fn benchmark_generate_merkle_tree_50() {}
fn benchmark_generate_merkle_tree_500() {}
fn benchmark_generate_merkle_tree_1_000() {}
fn benchmark_generate_merkle_tree_10_000() {}
fn benchmark_generate_merkle_tree_1_000_000() {}

fn generate_random_data(size: usize) -> Vec<Vec<u8>> {}

fn generate_input_set(size: usize) -> Vec<Vec<u8>> {
    let input_set = vec![
        b"Lazar".to_vec(),
        b"Vuksan".to_vec(),
        b"Dusan".to_vec(),
        b"Aleksa".to_vec(),
        b"Yoshiki".to_vec(),
        b"Milos".to_vec(),
        b"Zeljko".to_vec(),
    ];

    if size > input_set.len() {
        input_set.clone()
    } else {
        input_set[..size].to_vec()
    }
}

fn get_hex_bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hex_bytes = hex::decode(input).unwrap();
    Ok(hex_bytes)
}

fn test_generate_merkle_tree() {}
