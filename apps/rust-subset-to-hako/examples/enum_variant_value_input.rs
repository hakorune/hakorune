pub enum Status {
    Ready,
    Waiting,
}

pub fn current_status() -> Status {
    Status::Ready
}

pub fn is_ready(status: Status) -> bool {
    status == Status::Ready
}
