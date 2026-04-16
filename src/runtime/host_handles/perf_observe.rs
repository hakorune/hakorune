#[derive(Copy, Clone)]
pub enum ObjectWithHandleCaller {
    Generic,
    ArrayStoreStrSource,
    SubstringPlan,
    DecodeArrayFast,
    DecodeAnyArg,
    DecodeAnyIndex,
}

#[cfg(feature = "perf-observe")]
mod imp {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::ObjectWithHandleCaller;

    static LATEST_FRESH_HANDLE: AtomicU64 = AtomicU64::new(0);
    static OBJECT_GET_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_ARRAY_STORE_STR_SOURCE_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_SUBSTRING_PLAN_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_DECODE_ARRAY_FAST_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_DECODE_ANY_ARG_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_WITH_HANDLE_DECODE_ANY_INDEX_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_PAIR_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static OBJECT_TRIPLE_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static TEXT_READ_HANDLE_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static TEXT_READ_PAIR_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);
    static TEXT_READ_TRIPLE_LATEST_FRESH: AtomicU64 = AtomicU64::new(0);

    #[inline(always)]
    fn is_latest_fresh_handle(handle: u64) -> bool {
        handle > 0 && LATEST_FRESH_HANDLE.load(Ordering::Relaxed) == handle
    }

    #[inline(always)]
    pub(super) fn mark_latest_fresh_handle(handle: u64) {
        LATEST_FRESH_HANDLE.store(handle, Ordering::Relaxed);
    }

    #[inline(always)]
    pub(super) fn object_get(handle: u64) {
        if is_latest_fresh_handle(handle) {
            OBJECT_GET_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(super) fn object_with_handle(handle: u64, caller: ObjectWithHandleCaller) {
        if is_latest_fresh_handle(handle) {
            OBJECT_WITH_HANDLE_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
            match caller {
                ObjectWithHandleCaller::Generic => {}
                ObjectWithHandleCaller::ArrayStoreStrSource => {
                    OBJECT_WITH_HANDLE_ARRAY_STORE_STR_SOURCE_LATEST_FRESH
                        .fetch_add(1, Ordering::Relaxed);
                }
                ObjectWithHandleCaller::SubstringPlan => {
                    OBJECT_WITH_HANDLE_SUBSTRING_PLAN_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
                }
                ObjectWithHandleCaller::DecodeArrayFast => {
                    OBJECT_WITH_HANDLE_DECODE_ARRAY_FAST_LATEST_FRESH
                        .fetch_add(1, Ordering::Relaxed);
                }
                ObjectWithHandleCaller::DecodeAnyArg => {
                    OBJECT_WITH_HANDLE_DECODE_ANY_ARG_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
                }
                ObjectWithHandleCaller::DecodeAnyIndex => {
                    OBJECT_WITH_HANDLE_DECODE_ANY_INDEX_LATEST_FRESH
                        .fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    #[inline(always)]
    pub(super) fn object_pair(a: u64, b: u64) {
        if is_latest_fresh_handle(a) || is_latest_fresh_handle(b) {
            OBJECT_PAIR_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(super) fn object_triple(a: u64, b: u64, c: u64) {
        if is_latest_fresh_handle(a) || is_latest_fresh_handle(b) || is_latest_fresh_handle(c) {
            OBJECT_TRIPLE_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(super) fn text_read_handle(handle: u64) {
        if is_latest_fresh_handle(handle) {
            TEXT_READ_HANDLE_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(super) fn text_read_pair(a: u64, b: u64) {
        if is_latest_fresh_handle(a) || is_latest_fresh_handle(b) {
            TEXT_READ_PAIR_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(super) fn text_read_triple(a: u64, b: u64, c: u64) {
        if is_latest_fresh_handle(a) || is_latest_fresh_handle(b) || is_latest_fresh_handle(c) {
            TEXT_READ_TRIPLE_LATEST_FRESH.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(super) fn snapshot() -> [u64; 12] {
        [
            OBJECT_GET_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_PAIR_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_TRIPLE_LATEST_FRESH.load(Ordering::Relaxed),
            TEXT_READ_HANDLE_LATEST_FRESH.load(Ordering::Relaxed),
            TEXT_READ_PAIR_LATEST_FRESH.load(Ordering::Relaxed),
            TEXT_READ_TRIPLE_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_ARRAY_STORE_STR_SOURCE_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_SUBSTRING_PLAN_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_DECODE_ARRAY_FAST_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_DECODE_ANY_ARG_LATEST_FRESH.load(Ordering::Relaxed),
            OBJECT_WITH_HANDLE_DECODE_ANY_INDEX_LATEST_FRESH.load(Ordering::Relaxed),
        ]
    }
}

#[cfg(not(feature = "perf-observe"))]
mod imp {
    use super::ObjectWithHandleCaller;

    #[inline(always)]
    pub(super) fn mark_latest_fresh_handle(_handle: u64) {}

    #[inline(always)]
    pub(super) fn object_get(_handle: u64) {}

    #[inline(always)]
    pub(super) fn object_with_handle(_handle: u64, _caller: ObjectWithHandleCaller) {}

    #[inline(always)]
    pub(super) fn object_pair(_a: u64, _b: u64) {}

    #[inline(always)]
    pub(super) fn object_triple(_a: u64, _b: u64, _c: u64) {}

    #[inline(always)]
    pub(super) fn text_read_handle(_handle: u64) {}

    #[inline(always)]
    pub(super) fn text_read_pair(_a: u64, _b: u64) {}

    #[inline(always)]
    pub(super) fn text_read_triple(_a: u64, _b: u64, _c: u64) {}

    pub(super) fn snapshot() -> [u64; 12] {
        [0; 12]
    }
}

#[inline(always)]
pub(super) fn mark_latest_fresh_handle(handle: u64) {
    imp::mark_latest_fresh_handle(handle);
}

#[inline(always)]
pub(super) fn object_get(handle: u64) {
    imp::object_get(handle);
}

#[inline(always)]
pub(super) fn object_with_handle(handle: u64, caller: ObjectWithHandleCaller) {
    imp::object_with_handle(handle, caller);
}

#[inline(always)]
pub(super) fn object_pair(a: u64, b: u64) {
    imp::object_pair(a, b);
}

#[inline(always)]
pub(super) fn object_triple(a: u64, b: u64, c: u64) {
    imp::object_triple(a, b, c);
}

#[inline(always)]
pub(super) fn text_read_handle(handle: u64) {
    imp::text_read_handle(handle);
}

#[inline(always)]
pub(super) fn text_read_pair(a: u64, b: u64) {
    imp::text_read_pair(a, b);
}

#[inline(always)]
pub(super) fn text_read_triple(a: u64, b: u64, c: u64) {
    imp::text_read_triple(a, b, c);
}

pub(super) fn snapshot() -> [u64; 12] {
    imp::snapshot()
}
