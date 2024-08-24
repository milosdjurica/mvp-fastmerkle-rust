use sha3::Digest;
use sha3::Keccak256;
use std::sync::Mutex;

pub struct FastHasher {
    hash_engine: Keccak256,
}

impl FastHasher {
    pub fn new() -> Self {
        FastHasher {
            hash_engine: Keccak256::new(),
        }
    }

    pub fn add_to_hash(&mut self, input_data: &[u8]) -> Result<(), std::io::Error> {
        self.hash_engine.update(input_data);
        Ok(())
    }

    pub fn get_hash(&mut self) -> Vec<u8> {
        self.hash_engine.finalize_reset().to_vec()
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

    pub fn acquire(&self) -> FastHasher {
        let mut hasher_pool = self.pool.lock().unwrap();
        hasher_pool.pop().unwrap()
    }

    pub fn release(&self, fh: FastHasher) {
        // println!("INSIDE RELEASE");
        let mut hasher_pool = self.pool.lock().unwrap();
        hasher_pool.push(fh);
    }
}
