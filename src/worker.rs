use crossbeam_channel::{bounded, Receiver, Sender};

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

impl WorkerPool {
    pub fn new(expected_num_results: usize) -> Self {
        let (sender, receiver) = bounded(expected_num_results);
        WorkerPool {
            results_sender: sender,
            results_receiver: receiver,
        }
    }

    pub fn get_result(&self) -> WorkerResult {
        self.results_receiver.recv().unwrap()
    }

    pub fn close(&self) {
        drop(&self.results_sender);
        drop(&self.results_receiver);
    }
}
