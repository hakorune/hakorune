use crate::mir::join_ir::lowering::stage1_using_resolver::lower_stage1_usingresolver_to_joinir;
use crate::mir::join_ir::lowering::stageb_body::lower_stageb_body_to_joinir;
use crate::mir::join_ir::lowering::stageb_funcscanner::lower_stageb_funcscanner_to_joinir;
use crate::mir::MirModule;
use crate::runtime::get_global_ring0;

fn observe_lower_only_route<F>(
    quiet_pipe: bool,
    attempt_label: &str,
    success_label: &str,
    failure_label: &str,
    lower: F,
) -> bool
where
    F: FnOnce() -> Option<crate::mir::join_ir::JoinModule>,
{
    let should_log = !quiet_pipe
        && (crate::config::env::joinir_vm_bridge_debug()
            || crate::config::env::cli_verbose_enabled());
    if should_log {
        get_global_ring0().log.info(&format!(
            "[joinir/vm_bridge] Attempting JoinIR path for {}",
            attempt_label
        ));
    }

    match lower() {
        Some(join_module) => {
            if should_log {
                get_global_ring0().log.info(&format!(
                    "[joinir/vm_bridge] ✅ {} JoinIR module generated ({} functions)",
                    success_label,
                    join_module.functions.len()
                ));
                get_global_ring0().log.info(
                    "[joinir/vm_bridge] Note: ArrayBox/MapBox args not yet supported in JoinValue",
                );
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path for actual execution");
            }
        }
        None => {
            if should_log {
                get_global_ring0().log.warn(&format!(
                    "[joinir/vm_bridge] {} returned None",
                    failure_label
                ));
                get_global_ring0()
                    .log
                    .info("[joinir/vm_bridge] Falling back to normal VM path");
            }
        }
    }

    false
}

/// Stage1UsingResolverBox.resolve_for_source/5 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stage1_usingresolver(module: &MirModule, quiet_pipe: bool) -> bool {
    observe_lower_only_route(
        quiet_pipe,
        "Stage1UsingResolverBox.resolve_for_source",
        "Stage-1",
        "lower_stage1_usingresolver_to_joinir",
        || lower_stage1_usingresolver_to_joinir(module),
    )
}

/// StageBBodyExtractorBox.build_body_src/2 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stageb_body(module: &MirModule, quiet_pipe: bool) -> bool {
    observe_lower_only_route(
        quiet_pipe,
        "StageBBodyExtractorBox.build_body_src",
        "Stage-B Body",
        "lower_stageb_body_to_joinir",
        || lower_stageb_body_to_joinir(module),
    )
}

/// StageBFuncScannerBox.scan_all_boxes/1 用 JoinIR ブリッジ（LowerOnly: 構造検証専用）
///
/// ArrayBox/MapBox 引数がまだ JoinValue でサポートされていないため、
/// JoinIR lowering / Bridge 構造検証のみ行い、実行は VM Route A にフォールバック。
pub(crate) fn try_run_stageb_funcscanner(module: &MirModule, quiet_pipe: bool) -> bool {
    observe_lower_only_route(
        quiet_pipe,
        "StageBFuncScannerBox.scan_all_boxes",
        "Stage-B FuncScanner",
        "lower_stageb_funcscanner_to_joinir",
        || lower_stageb_funcscanner_to_joinir(module),
    )
}
