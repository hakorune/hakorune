use super::utils::*;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_v0::LoopScanSegment;

/// Recipe-first verification for loop_scan_methods_v0.
pub fn verify_loop_scan_methods_v0_recipe(
    scan_methods: &crate::mir::builder::control_flow::facts::loop_scan_methods_v0::LoopScanMethodsV0Facts,
) -> Result<(), Freeze> {

    for (idx, segment) in scan_methods.recipe.segments.iter().enumerate() {
        match segment {
            LoopScanSegment::Linear(recipe) => {
                let ctx = format!("loop_scan_methods_v0_linear_{idx}");
                verify_no_exit_block_recipe(recipe, &ctx)?;
            }
            LoopScanSegment::NestedLoop(nested) => {
                let ctx = format!("loop_scan_methods_v0_nested_{idx}");
                verify_nested_loop_stmt_only_if_available(nested, &ctx)?;
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:scan_methods] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_scan_methods_block_v0.
pub fn verify_loop_scan_methods_block_v0_recipe(
    scan_methods_block: &crate::mir::builder::control_flow::facts::loop_scan_methods_block_v0::LoopScanMethodsBlockV0Facts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::{
        LinearBlockRecipe, ScanSegment,
    };

    for (idx, segment) in scan_methods_block.recipe.segments.iter().enumerate() {
        match segment {
            ScanSegment::Linear(recipe) => match recipe {
                LinearBlockRecipe::NoExit(block) => {
                    let ctx = format!("loop_scan_methods_block_v0_linear_no_exit_{idx}");
                    verify_no_exit_block_recipe(&block, &ctx)?;
                }
                LinearBlockRecipe::ExitAllowed(block) => {
                    let ctx = format!("loop_scan_methods_block_v0_linear_exit_allowed_{idx}");
                    verify_exit_allowed_block_recipe(&block, &ctx)?;
                }
            },
            ScanSegment::NestedLoop(nested) => {
                let ctx = format!("loop_scan_methods_block_v0_nested_{idx}");
                verify_nested_loop_stmt_only_if_available(&nested, &ctx)?;
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:scan_methods_block] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_scan_phi_vars_v0.
pub fn verify_loop_scan_phi_vars_v0_recipe(
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

/// Recipe-first verification for loop_scan_v0.
pub fn verify_loop_scan_v0_recipe(
    scan_v0: &crate::mir::builder::control_flow::plan::loop_scan_v0::LoopScanV0Facts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_scan_v0::recipe::LoopScanSegment;

    for (idx, segment) in scan_v0.segments.iter().enumerate() {
        match segment {
            LoopScanSegment::Linear(recipe) => {
                let ctx = format!("loop_scan_v0_linear_{idx}");
                verify_exit_allowed_block_recipe(recipe, &ctx)?;
            }
            LoopScanSegment::NestedLoop(nested) => {
                let ctx = format!("loop_scan_v0_nested_{idx}");
                verify_nested_loop_stmt_only_if_available(nested, &ctx)?;
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[recipe:scan_v0] verified OK"));
    }
    Ok(())
}
