//! hako.worker substrate exports for allocator-internal worker identity.
//!
//! This is a single-worker proof seam. It does not expose source-level worker
//! identity semantics or a native thread pool.

#[no_mangle]
pub extern "C" fn hako_worker_current_id_i64(_lane: i64) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_worker_id_is_single_worker_zero() {
        assert_eq!(hako_worker_current_id_i64(0), 0);
        assert_eq!(hako_worker_current_id_i64(99), 0);
    }
}
