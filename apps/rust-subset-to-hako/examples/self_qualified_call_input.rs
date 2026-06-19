pub struct Counter {
    next_id: u32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { next_id: 0 }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
