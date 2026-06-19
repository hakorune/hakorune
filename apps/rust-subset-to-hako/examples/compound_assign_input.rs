pub struct Counter {
    next_id: u32,
}

impl Counter {
    pub fn next(&mut self) {
        self.next_id += 1;
    }
}
