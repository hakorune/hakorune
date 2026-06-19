struct Id(i64);

impl Id {
    const ZERO: Id = Id(0);
}

fn zero_id() -> Id {
    Id::ZERO
}
