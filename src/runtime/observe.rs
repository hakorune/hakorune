//! Lightweight execution observability flags used by runner policies
//! (e.g., Gate‑C(Core) OOB strict fail‑fast).

use std::sync::atomic::{AtomicBool, Ordering};

static OOB_SEEN: AtomicBool = AtomicBool::new(false);

/// Reset all transient observation flags before a run.
pub fn reset() {
    OOB_SEEN.store(false, Ordering::Relaxed);
}

/// Mark that an out‑of‑bounds access was observed in the runtime.
pub fn mark_oob() {
    OOB_SEEN.store(true, Ordering::Relaxed);
}

/// Returns true if an out‑of‑bounds access was observed during the run.
pub fn oob_seen() -> bool {
    OOB_SEEN.load(Ordering::Relaxed)
}
