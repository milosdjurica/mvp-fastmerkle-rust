use std::sync::Mutex;
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
    pub fn new(size: usize) -> Self {
        let mut pool = Vec::with_capacity(size);

        for _ in 0..size {
            pool.push(FastHasher::new());
        }
        FastHasherPool {
            pool: Mutex::new(pool),
        }
    }

    pub fn acquire(&self) -> Option<FastHasher> {
        let mut hasher_pool = self.pool.lock().unwrap();
        hasher_pool.pop()
    }

    pub fn release(&self, fh: FastHasher) {
        // println!("INSIDE RELEASE");
        let mut hasher_pool = self.pool.lock().unwrap();
        hasher_pool.push(fh);
    }
}
