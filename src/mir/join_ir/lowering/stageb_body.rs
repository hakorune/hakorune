//! StageBBodyExtractorBox.build_body_src/2 target-specific JoinIR lowering facade.
//!
//! Route-local builder code lives in `stageb_body/builder.rs`; MIR-vs-
//! handwritten dispatch lives in `stageb_body/dispatch.rs`.

use crate::mir::join_ir::JoinModule;

mod builder;
mod dispatch;

/// Public dispatcher (MIR-based vs handwritten)
pub fn lower_stageb_body_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    super::common::dispatch_lowering(
        "stageb_body",
        module,
        dispatch::lower_from_mir,
        dispatch::lower_handwritten,
    )
}
