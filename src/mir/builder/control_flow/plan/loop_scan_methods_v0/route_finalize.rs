use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::{CorePhiInfo, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

const LOOP_SCAN_METHODS_ERR: &str = "[normalizer] loop_scan_methods_v0";

pub(in crate::mir::builder) struct LoopScanMethodsRouteFinalize {
    pub phis: Vec<CorePhiInfo>,
    pub final_values: Vec<(String, ValueId)>,
}

pub(in crate::mir::builder) fn finalize_loop_scan_methods_route(
    builder: &mut MirBuilder,
    body_plans: &mut Vec<LoweredRecipe>,
    carrier_inits: &BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    preheader_bb: BasicBlockId,
    header_bb: BasicBlockId,
    step_bb: BasicBlockId,
    after_bb: BasicBlockId,
) -> Result<LoopScanMethodsRouteFinalize, String> {
    body_plans.push(crate::mir::builder::control_flow::plan::CorePlan::Exit(
        crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args(
            builder,
            carrier_step_phis,
            current_bindings,
            LOOP_SCAN_METHODS_ERR,
        )?,
    ));

    let mut phis = Vec::new();
    let mut final_values = Vec::new();
    for (var, header_phi_dst) in carrier_phis {
        let init_val = *carrier_inits.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_methods_v0] missing init for {var}")
        })?;
        let step_phi_dst = *carrier_step_phis.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_methods_v0] missing step phi for {var}")
        })?;
        let after_phi_dst = *break_phi_dsts.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_methods_v0] missing after phi for {var}")
        })?;

        phis.push(loop_carriers::build_step_join_phi_info(
            step_bb,
            step_phi_dst,
            format!("loop_scan_methods_v0_step_join_{}", var),
        ));
        phis.push(loop_carriers::build_loop_phi_info(
            header_bb,
            preheader_bb,
            step_bb,
            *header_phi_dst,
            init_val,
            step_phi_dst,
            format!("loop_scan_methods_v0_carrier_{}", var),
        ));
        phis.push(loop_carriers::build_after_merge_phi_info(
            after_bb,
            after_phi_dst,
            [header_bb],
            *header_phi_dst,
            format!("loop_scan_methods_v0_after_{}", var),
        ));
        final_values.push((var.clone(), after_phi_dst));
    }

    Ok(LoopScanMethodsRouteFinalize { phis, final_values })
}
