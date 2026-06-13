//! FuncScannerBox.trim/1 target-specific JoinIR lowering facade.
//!
//! Route-local builder code lives in `funcscanner_trim/builder.rs`; MIR-vs-
//! handwritten dispatch lives in `funcscanner_trim/dispatch.rs`.

use crate::mir::join_ir::JoinModule;

pub(super) mod builder;
mod dispatch;

/// Phase 27.9: Toggle dispatcher for trim lowering
/// - Default: handwritten lowering
/// - NYASH_JOINIR_LOWER_FROM_MIR=1: MIR-based lowering
pub fn lower_funcscanner_trim_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    super::common::dispatch_lowering(
        "trim",
        module,
        dispatch::lower_trim_from_mir,
        dispatch::lower_trim_handwritten,
    )
}
