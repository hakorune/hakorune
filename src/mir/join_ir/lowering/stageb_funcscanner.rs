//! StageBFuncScannerBox.scan_all_boxes/1 target-specific JoinIR lowering facade.
//!
//! Route-local builder code lives in `stageb_funcscanner/builder.rs`; MIR-vs-
//! handwritten dispatch lives in `stageb_funcscanner/dispatch.rs`.

use crate::mir::join_ir::JoinModule;

mod builder;
mod dispatch;

/// Public dispatcher (MIR-based vs handwritten)
pub fn lower_stageb_funcscanner_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    super::common::dispatch_lowering(
        "stageb_funcscanner",
        module,
        dispatch::lower_from_mir,
        dispatch::lower_handwritten,
    )
}
