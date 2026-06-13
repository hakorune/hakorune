//! Shared orchestration for target-local generic Case-A probes.
//!
//! This helper does not own route policy. Callers still choose the target,
//! LoopForm options, and route-specific `LoopToJoinLowerer` entrypoint.

use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::query::MirQueryBox;
use crate::mir::{BasicBlockId, MirFunction};
use crate::runtime::get_global_ring0;

use super::cfg_shape::construct_simple_while_loopform;

/// Try the shared generic Case-A path for a target-local lowerer.
///
/// Returns `Some(JoinModule)` only when generic Case-A is enabled and the
/// route-specific lowering succeeds. All fallback behavior remains in the
/// caller so Exec/LowerOnly semantics stay route-owned.
pub fn try_generic_case_a_route<F>(
    tag: &str,
    target_func: &MirFunction,
    entry: BasicBlockId,
    query: &MirQueryBox,
    entry_is_preheader: bool,
    has_break: bool,
    lower_case_a: F,
) -> Option<JoinModule>
where
    F: FnOnce(&LoopToJoinLowerer, &MirFunction, &LoopForm) -> Option<JoinModule>,
{
    if !crate::config::env::joinir_dev::lower_generic_enabled() {
        return None;
    }

    let Some(loop_form) =
        construct_simple_while_loopform(entry, query, entry_is_preheader, has_break)
    else {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{tag}/generic-hook] failed to construct LoopForm from CFG"
            ));
        }
        return None;
    };

    if !super::case_a::is_simple_case_a_loop(&loop_form) {
        return None;
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/{tag}/generic-hook] simple Case A loop detected (LoopToJoinLowerer)"
        ));
    }

    let lowerer = LoopToJoinLowerer::new();
    let out = lower_case_a(&lowerer, target_func, &loop_form);
    if out.is_some() {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{tag}/generic-hook] LoopToJoinLowerer produced JoinIR, returning early"
            ));
        }
    } else if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/{tag}/generic-hook] LoopToJoinLowerer returned None, falling back to handwritten"
        ));
    }
    out
}
