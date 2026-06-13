//! Dispatch policy for target-specific MIR-based vs handwritten lowerers.

use crate::runtime::get_global_ring0;

/// Log fallback to handwritten lowering with reason.
pub fn log_fallback(tag: &str, reason: &str) {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/{}/mir] unexpected MIR shape: {}, falling back to handwritten",
            tag, reason
        ));
    }
}

/// Dispatch between MIR-based and handwritten lowering based on environment variable.
///
/// Checks `NYASH_JOINIR_LOWER_FROM_MIR` and dispatches to the appropriate
/// lowering function. This consolidates the toggle pattern used across
/// target-specific JoinIR lowerers.
pub fn dispatch_lowering<F1, F2>(
    tag: &str,
    module: &crate::mir::MirModule,
    mir_based: F1,
    handwritten: F2,
) -> Option<crate::mir::join_ir::JoinModule>
where
    F1: FnOnce(&crate::mir::MirModule) -> Option<crate::mir::join_ir::JoinModule>,
    F2: FnOnce(&crate::mir::MirModule) -> Option<crate::mir::join_ir::JoinModule>,
{
    if crate::config::env::joinir_dev::lower_from_mir_enabled() {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{}] Using MIR-based lowering (NYASH_JOINIR_LOWER_FROM_MIR=1)",
                tag
            ));
        }
        mir_based(module)
    } else {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{}] Using handwritten lowering (default)",
                tag
            ));
        }
        handwritten(module)
    }
}
