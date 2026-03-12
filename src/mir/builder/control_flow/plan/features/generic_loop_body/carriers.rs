use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePhiInfo};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::helpers::collect_loop_carriers;

pub(in crate::mir::builder) struct GenericLoopV1CarrierState {
    pub phi_bindings: BTreeMap<String, crate::mir::ValueId>,
    pub carrier_step_phis: BTreeMap<String, crate::mir::ValueId>,
    pub loop_var_step_phi: crate::mir::ValueId,
    pub carrier_infos: Vec<(
        String,
        crate::mir::ValueId,
        crate::mir::ValueId,
        crate::mir::ValueId,
    )>,
}

pub(in crate::mir::builder) fn prepare_generic_loop_v1_carriers(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    loop_var_current: crate::mir::ValueId,
) -> GenericLoopV1CarrierState {
    let pre_loop_map = builder.variable_ctx.variable_map.clone();
    let carrier_vars = collect_loop_carriers(&facts.body.body, &pre_loop_map, &facts.loop_var);
    let mut phi_bindings =
        loop_carriers::build_loop_bindings(&[(&facts.loop_var, loop_var_current)]);
    let loop_var_step_phi = builder.alloc_typed(
        builder
            .type_ctx
            .get_type(loop_var_current)
            .cloned()
            .unwrap_or(MirType::Unknown),
    );
    let mut carrier_step_phis = BTreeMap::new();
    carrier_step_phis.insert(facts.loop_var.clone(), loop_var_step_phi);
    let mut carrier_infos = Vec::new();
    for var in carrier_vars {
        let Some(init_val) = pre_loop_map.get(&var) else {
            continue;
        };
        let ty = builder
            .type_ctx
            .get_type(*init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let phi_dst = builder.alloc_typed(ty.clone());
        let step_phi_dst = builder.alloc_typed(ty);
        phi_bindings.insert(var.clone(), phi_dst);
        carrier_step_phis.insert(var.clone(), step_phi_dst);
        carrier_infos.push((var, *init_val, phi_dst, step_phi_dst));
    }
    GenericLoopV1CarrierState {
        phi_bindings,
        carrier_step_phis,
        loop_var_step_phi,
        carrier_infos,
    }
}

pub(in crate::mir::builder) fn finalize_generic_loop_v1_carriers(
    builder: &mut MirBuilder,
    loop_plan: &mut CoreLoopPlan,
    carrier_state: GenericLoopV1CarrierState,
    loop_var: &str,
    loop_var_current: crate::mir::ValueId,
    post_body_map: &BTreeMap<String, crate::mir::ValueId>,
) {
    let mut phis = loop_plan.phis.clone();
    let mut final_values = loop_plan.final_values.clone();
    let loop_var_step_input = post_body_map
        .get(loop_var)
        .copied()
        .unwrap_or(loop_var_current);
    phis.push(CorePhiInfo {
        block: loop_plan.step_bb,
        dst: carrier_state.loop_var_step_phi,
        inputs: vec![(loop_plan.body_bb, loop_var_step_input)],
        tag: format!("loop_step_in_{}", loop_var),
    });
    for (var, init_val, phi_dst, step_phi_dst) in carrier_state.carrier_infos {
        let step_input = post_body_map.get(&var).copied().unwrap_or(phi_dst);
        phis.push(CorePhiInfo {
            block: loop_plan.step_bb,
            dst: step_phi_dst,
            inputs: vec![(loop_plan.body_bb, step_input)],
            tag: format!("loop_step_in_{}", var),
        });
        phis.push(loop_carriers::build_loop_phi_info(
            loop_plan.header_bb,
            loop_plan.preheader_bb,
            loop_plan.step_bb,
            phi_dst,
            init_val,
            step_phi_dst,
            format!("loop_carrier_{}", var),
        ));
        final_values.push((var.clone(), phi_dst));
        builder.variable_ctx.variable_map.insert(var, phi_dst);
    }
    loop_plan.phis = phis;
    loop_plan.final_values = final_values;
    builder
        .variable_ctx
        .variable_map
        .insert(loop_var.to_string(), loop_var_current);
}
