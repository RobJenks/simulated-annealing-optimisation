use crate::worker::Worker;

pub struct Solver {
    workers: Vec<Worker>,
    pool_target: usize
}

impl Solver {
    pub fn new() -> Self {
        Self { workers: vec![], pool_target: 0 }
    }

    pub fn set_pool_size(&mut self, size: usize) {
        self.pool_target = size;
    }
}