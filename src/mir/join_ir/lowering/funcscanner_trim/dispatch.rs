use super::build_funcscanner_trim_joinir;
use crate::mir::join_ir::JoinModule;
use crate::runtime::get_global_ring0;

/// Phase 27.11: Handwritten lowering wrapper for FuncScannerBox.trim/1
///
/// This is a thin wrapper that calls the shared build_funcscanner_trim_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
pub(super) fn lower_trim_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/trim/handwritten] Using handwritten lowering path");
    }
    build_funcscanner_trim_joinir(module)
}

/// Phase 27.9: MIR-based lowering for FuncScannerBox.trim/1
/// - Lightweight CFG sanity checks
/// - Fallback to handwritten if MIR structure is unexpected
pub(super) fn lower_trim_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use super::super::common::{
        ensure_entry_has_succs, has_binop, has_const_string, has_string_method, log_fallback,
    };
    use crate::mir::query::MirQueryBox;
    use crate::mir::BinaryOp;

    let target_func = module.functions.get("FuncScannerBox.trim/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/trim/mir] Found FuncScannerBox.trim/1 (MIR-based lowering)");
        ring0.log.debug(&format!(
            "[joinir/trim/mir] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    let query = MirQueryBox::new(target_func);
    let entry_id = target_func.entry_block;

    if !ensure_entry_has_succs(&query, entry_id) {
        log_fallback("trim", "entry has no successors");
        return lower_trim_handwritten(module);
    }

    if !has_const_string(&query, entry_id, "")
        || !has_string_method(&query, entry_id, "length")
        || !has_binop(&query, entry_id, BinaryOp::Add)
    {
        log_fallback(
            "trim",
            "entry block missing expected patterns (Const(\"\"), String.length, or BinOp(Add))",
        );
        return lower_trim_handwritten(module);
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/trim/mir] CFG sanity checks passed ✅");
    }

    if crate::config::env::joinir_dev::lower_generic_enabled() {
        use crate::mir::join_ir::lowering::common::construct_simple_while_loopform;
        use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;

        let Some(loop_form) = construct_simple_while_loopform(entry_id, &query, true, true) else {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0()
                    .log
                    .debug("[joinir/trim/generic-hook] failed to construct LoopForm from CFG");
            }
            return build_funcscanner_trim_joinir(module);
        };
        if crate::mir::join_ir::lowering::common::case_a::is_simple_case_a_loop(&loop_form) {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/trim/generic-hook] simple Case A loop detected (LoopToJoinLowerer)",
                );
            }
            let lowerer = LoopToJoinLowerer::new();
            if let Some(jm) = lowerer.lower_case_a_for_trim(target_func, &loop_form) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(
                        "[joinir/trim/generic-hook] LoopToJoinLowerer produced JoinIR, returning early",
                    );
                }
                return Some(jm);
            }
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/trim/generic-hook] LoopToJoinLowerer returned None, falling back to handwritten",
                );
            }
        }
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[joinir/trim/mir] Calling build_funcscanner_trim_joinir() after CFG validation",
        );
    }
    build_funcscanner_trim_joinir(module)
}
