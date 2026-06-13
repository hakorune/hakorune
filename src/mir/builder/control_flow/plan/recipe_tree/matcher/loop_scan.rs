use super::utils::*;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;

/// Recipe-first verification for loop_scan_phi_vars_v0.
pub(super) fn verify_loop_scan_phi_vars_v0_recipe(
    scan_phi_vars: &crate::mir::builder::control_flow::facts::loop_scan_phi_vars_v0::LoopScanPhiVarsV0Facts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::LoopScanPhiSegment;

    for (idx, segment) in scan_phi_vars.segments.iter().enumerate() {
        match segment {
            LoopScanPhiSegment::Linear(recipe) => {
                let ctx = format!("loop_scan_phi_vars_v0_linear_{idx}");
                verify_no_exit_block_recipe(recipe, &ctx)?;
            }
            LoopScanPhiSegment::NestedLoop(nested) => {
                let ctx = format!("loop_scan_phi_vars_v0_nested_{idx}");
                verify_nested_loop_stmt_only_if_available(nested, &ctx)?;
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:scan_phi_vars] verified OK"));
    }
    Ok(())
}
