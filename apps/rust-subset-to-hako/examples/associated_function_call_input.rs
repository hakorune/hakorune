struct Id(i64);

impl Id {
    fn new(value: i64) -> Self {
        Id(value)
    }
}

fn make_id(value: i64) -> Id {
    Id::new(value)
}
