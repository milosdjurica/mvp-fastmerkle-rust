use std::error::Error;

fn generate_input_set(size: usize) -> Option<Vec<Vec<u8>>> {
    if size < 1 {
        return None;
    }

    let input_set = vec![
        b"Lazar".to_vec(),
        b"Vuksan".to_vec(),
        b"Dusan".to_vec(),
        b"Aleksa".to_vec(),
        b"Yoshiki".to_vec(),
        b"Milos".to_vec(),
        b"Zeljko".to_vec(),
    ];

    let size = size.min(input_set.len());
    Some(input_set[..size].to_vec())
}

fn get_hex_bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hex_bytes = hex::decode(input).unwrap();
    Ok(hex_bytes)
}

fn test_generate_merkle_tree() {}
