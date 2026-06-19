#[hako_identity]
struct Counter {
    value: i64,
}

impl Counter {
    fn get(&self) -> i64 {
        return self.value;
    }
}

fn make_counter(value: i64) -> i64 {
    let next: i64 = value + 1;
    return next;
}
