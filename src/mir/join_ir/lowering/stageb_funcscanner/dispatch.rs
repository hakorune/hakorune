//! Route-local dispatch for `StageBFuncScannerBox.scan_all_boxes/1`.

use crate::mir::join_ir::lowering::common::{
    ensure_entry_has_succs, log_fallback, try_generic_case_a_route,
};
use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQueryBox;
use crate::runtime::get_global_ring0;

use super::builder::build_stageb_funcscanner_joinir;

/// MIR ベースの軽量パターンチェック（最低限）
pub(super) fn lower_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stageb_funcscanner/mir] Starting MIR-based lowering");
    }

    let target_func = module
        .functions
        .get("StageBFuncScannerBox.scan_all_boxes/1")?;

    let query = MirQueryBox::new(target_func);
    let entry = target_func.entry_block;
    if !ensure_entry_has_succs(&query, entry) {
        log_fallback("stageb_funcscanner", "entry has no successors");
        return lower_handwritten(module);
    }

    // stageb_funcscanner: entry_is_preheader=true, has_break=true
    if let Some(jm) = try_generic_case_a_route(
        "stageb_funcscanner",
        target_func,
        entry,
        &query,
        true,
        true,
        |lowerer, func, loop_form| lowerer.lower_case_a_for_stageb_funcscanner(func, loop_form),
    ) {
        return Some(jm);
    }

    build_stageb_funcscanner_joinir(module)
}

/// 手書き版（MIR 形状に依存しない）
pub(super) fn lower_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stageb_funcscanner/hand] Using handwritten lowering");
    }
    build_stageb_funcscanner_joinir(module)
}
