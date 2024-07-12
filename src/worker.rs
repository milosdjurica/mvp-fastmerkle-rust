use crossbeam_channel::{Receiver, Sender};

pub struct WorkerPool {
    results_sender: Sender<WorkerResult>,
    results_receiver: Receiver<WorkerResult>,
}

pub struct WorkerJob {
    store_index: usize,
    source_data: Vec<Vec<u8>>,
}

pub struct WorkerResult {
    store_index: usize,
    hash_data: Vec<u8>,
    error: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl WorkerPool {}
