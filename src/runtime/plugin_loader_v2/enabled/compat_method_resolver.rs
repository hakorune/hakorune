use crate::bid::{BidError, BidResult};

pub(super) fn resolve_method_id_from_file(box_type: &str, method_name: &str) -> BidResult<u32> {
    match (box_type, method_name) {
        ("StringBox", "concat") => Ok(102),
        ("StringBox", "upper") => Ok(103),
        ("CounterBox", "inc") => Ok(102),
        ("CounterBox", "get") => Ok(103),
        _ => Err(BidError::InvalidMethod),
    }
}
