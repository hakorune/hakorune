//! Type trace helpers (dev-only; default OFF)
//!
//! Enable with `NYASH_MIR_TYPE_TRACE=1` to dump type/origin events during MIR build.
//! 既存の value_types / value_origin_newbox に触れる箇所へ薄く差し込むだけで、
//! TypeRegistry の移行なしに観測ラインを確保する小粒ガードだよ。

use crate::mir::{MirType, ValueId};
use crate::runtime::get_global_ring0;
use std::sync::OnceLock;

fn enabled() -> bool {
    static FLAG: OnceLock<bool> = OnceLock::new();
    *FLAG.get_or_init(|| crate::config::env::builder_mir_type_trace())
}

/// Trace when a newbox/class origin is registered.
pub fn origin(event: &str, vid: ValueId, class: &str) {
    if enabled() {
        get_global_ring0().log.debug(&format!(
            "[type-trace] origin:{} %{} ← {}",
            event, vid.0, class
        ));
    }
}

/// Trace when a concrete MirType is recorded.
pub fn ty(event: &str, vid: ValueId, ty: &MirType) {
    if enabled() {
        get_global_ring0().log.debug(&format!(
            "[type-trace] type:{} %{} ← {:?}",
            event, vid.0, ty
        ));
    }
}

/// Trace propagation between ValueIds.
pub fn propagate(event: &str, src: ValueId, dst: ValueId) {
    if enabled() {
        get_global_ring0().log.debug(&format!(
            "[type-trace] propagate:{} %{} → %{}",
            event, src.0, dst.0
        ));
    }
}
