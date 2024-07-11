use std::sync::{Mutex, MutexGuard};
use tiny_keccak::{Hasher, Keccak};

pub struct FastHasher {
    hash_engine: Keccak,
}

impl FastHasher {
    pub fn new() -> Self {
        FastHasher {
            hash_engine: Keccak::v256(),
        }
    }

    pub fn add_to_hash(&mut self, input_data: &[u8]) -> Result<(), std::io::Error> {
        self.hash_engine.update(input_data);
        Ok(())
    }

    pub fn get_hash(&mut self) -> Vec<u8> {
        let mut result = [0u8; 32];
        self.hash_engine.clone().finalize(&mut result);
        result.to_vec()
    }
}

pub struct FastHasherPool {
    pool: Mutex<Vec<FastHasher>>,
}

impl FastHasherPool {
    pub fn new() -> Self {
        FastHasherPool {
            pool: Mutex::new(vec![]),
        }
    }

    pub fn acquire(&self) -> MutexGuard<Vec<FastHasher>> {
        self.pool.lock().unwrap()
    }

    pub fn release(&self, fh: FastHasher) {
        self.pool.lock().unwrap().push(fh);
    }
}
