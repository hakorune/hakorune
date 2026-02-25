use crate::mir::join_ir::lowering::stage1_using_resolver::lower_stage1_usingresolver_to_joinir;
use crate::mir::join_ir::lowering::stageb_body::lower_stageb_body_to_joinir;
use crate::mir::join_ir::lowering::stageb_funcscanner::lower_stageb_funcscanner_to_joinir;
use crate::mir::MirModule;
use crate::runtime::get_global_ring0;

/// Stage1UsingResolverBox.resolve_for_source/5 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stage1_usingresolver(module: &MirModule, quiet_pipe: bool) -> bool {
    let should_log = !quiet_pipe
        && (crate::config::env::joinir_vm_bridge_debug()
            || crate::config::env::cli_verbose_enabled());
    if should_log {
        get_global_ring0().log.info(
            "[joinir/vm_bridge] Attempting JoinIR path for Stage1UsingResolverBox.resolve_for_source",
        );
    }

    match lower_stage1_usingresolver_to_joinir(module) {
        Some(join_module) => {
            if should_log {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ Stage-1 JoinIR module generated ({} functions)",
                    join_module.functions.len()
                ));
                get_global_ring0().log.info(
                    "[joinir/vm_bridge] Note: ArrayBox/MapBox args not yet supported in JoinValue",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path for actual execution");
            }
            false // 実行はまだサポートしていない
        }
        None => {
            if should_log {
                get_global_ring0().log.warn(
                    "[joinir/vm_bridge] lower_stage1_usingresolver_to_joinir returned None",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
            false
        }
    }
}

/// StageBBodyExtractorBox.build_body_src/2 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stageb_body(module: &MirModule, quiet_pipe: bool) -> bool {
    let should_log = !quiet_pipe
        && (crate::config::env::joinir_vm_bridge_debug()
            || crate::config::env::cli_verbose_enabled());
    if should_log {
        get_global_ring0().log.info(
            "[joinir/vm_bridge] Attempting JoinIR path for StageBBodyExtractorBox.build_body_src",
        );
    }

    match lower_stageb_body_to_joinir(module) {
        Some(join_module) => {
            if should_log {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ Stage-B Body JoinIR module generated ({} functions)",
                    join_module.functions.len()
                ));
                get_global_ring0().log.info(
                    "[joinir/vm_bridge] Note: ArrayBox/MapBox args not yet supported in JoinValue",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path for actual execution");
            }
            false // 実行はまだサポートしていない
        }
        None => {
            if should_log {
                get_global_ring0()
                    .log
                    .warn("[joinir/vm_bridge] lower_stageb_body_to_joinir returned None");
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
            false
        }
    }
}

/// StageBFuncScannerBox.scan_all_boxes/1 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stageb_funcscanner(module: &MirModule, quiet_pipe: bool) -> bool {
    let should_log = !quiet_pipe
        && (crate::config::env::joinir_vm_bridge_debug()
            || crate::config::env::cli_verbose_enabled());
    if should_log {
        get_global_ring0().log.info(
            "[joinir/vm_bridge] Attempting JoinIR path for StageBFuncScannerBox.scan_all_boxes",
        );
    }

    match lower_stageb_funcscanner_to_joinir(module) {
        Some(join_module) => {
            if should_log {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ Stage-B FuncScanner JoinIR module generated ({} functions)",
                    join_module.functions.len()
                ));
                get_global_ring0().log.info(
                    "[joinir/vm_bridge] Note: ArrayBox/MapBox args not yet supported in JoinValue",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path for actual execution");
            }
            false // 実行はまだサポートしていない
        }
        None => {
            if should_log {
                get_global_ring0().log.warn(
                    "[joinir/vm_bridge] lower_stageb_funcscanner_to_joinir returned None",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
            false
        }
    }
}
