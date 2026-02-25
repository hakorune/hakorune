//! Nested loop carrier propagation helpers.

use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

/// Apply nested loop's final_values to variable_map and current_bindings.
pub(super) fn apply_loop_final_values_to_bindings(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plan: &LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return;
    };
    for (name, value_id) in &loop_plan.final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
        if current_bindings.contains_key(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}

/// Extend nested loop with outer carrier PHIs and final_values.
pub(super) fn extend_nested_loop_carriers(
    builder: &mut MirBuilder,
    outer_carriers: &[String],
    pre_loop_map: &BTreeMap<String, crate::mir::ValueId>,
    post_loop_map: &BTreeMap<String, crate::mir::ValueId>,
    plan: &mut LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return; // no-op for non-Loop plans
    };
    // Collect existing final_values names to avoid duplicates
    let existing_names: std::collections::BTreeSet<String> = loop_plan
        .final_values
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[joinir/extend_carriers] existing_final_values={:?}",
            existing_names
        ));
    }

    for var in outer_carriers {
        if existing_names.contains(var) {
            continue; // already processed
        }
        let Some(&init_val) = pre_loop_map.get(var) else {
            continue;
        };
        let Some(&next_val) = post_loop_map.get(var) else {
            continue;
        };
        if init_val == next_val {
            continue; // value unchanged
        }
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let phi_dst = builder.alloc_typed(ty);
        if crate::config::env::is_joinir_debug() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[joinir/extend_carriers] adding PHI for var={} init={:?} next={:?} phi_dst={:?}",
                var, init_val, next_val, phi_dst
            ));
        }
        loop_plan.phis.push(loop_carriers::build_loop_phi_info(
            loop_plan.header_bb,
            loop_plan.preheader_bb,
            loop_plan.step_bb,
            phi_dst,
            init_val,
            next_val,
            format!("nested_outer_carrier_{}", var),
        ));
        loop_plan.final_values.push((var.clone(), phi_dst));
    }
}
