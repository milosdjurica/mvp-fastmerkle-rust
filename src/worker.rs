use crate::hasher::{FastHasher, FastHasherPool};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkerPool {
    results_sender: Sender<WorkerResult>,
    results_receiver: Receiver<WorkerResult>,
    hasher_pool: Arc<FastHasherPool>,
}

pub struct WorkerJob {
    pub store_index: usize,
    pub source_data: Vec<Vec<u8>>,
}

pub struct WorkerResult {
    pub store_index: usize,
    pub hash_data: Vec<u8>,
    pub error: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl WorkerPool {
    pub fn new(expected_num_results: usize, hasher_pool: Arc<FastHasherPool>) -> Self {
        let (sender, receiver) = bounded(expected_num_results);
        WorkerPool {
            results_sender: sender,
            results_receiver: receiver,
            hasher_pool,
        }
    }

    pub fn get_result(&self) -> WorkerResult {
        self.results_receiver.recv().unwrap()
    }

    pub fn close(self) {
        drop(self.results_sender);
        drop(self.results_receiver);
    }

    pub fn add_job(&self, job: WorkerJob) {
        let sender = self.results_sender.clone();
        let hasher_pool_clone = Arc::clone(&self.hasher_pool);
        std::thread::spawn(move || {
            let result = Self::run_job(job, hasher_pool_clone);
            sender.send(result).unwrap();
        });
    }

    fn run_job(job: WorkerJob, hasher_pool: Arc<FastHasherPool>) -> WorkerResult {
        let mut hasher = hasher_pool.acquire();

        let mut prepared_array = vec![];
        for data in job.source_data {
            prepared_array.extend(data);
        }

        let error = match hasher.add_to_hash(&prepared_array) {
            Ok(_) => None,
            Err(e) => Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        };

        let hash_data = hasher.get_hash();
        hasher_pool.release(hasher);

        WorkerResult {
            store_index: job.store_index,
            hash_data,
            error,
        }
    }
}
