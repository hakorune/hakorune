use std::sync::{Mutex, OnceLock};

pub(super) fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub(super) fn ensure_ring0_initialized() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
}
