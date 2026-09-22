use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::generic_loop::carrier_representation::PreparedGenericLoopCarrierRepresentationV1;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::parts::var_map_scope::publish_emission_cache;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePhiInfo};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::helpers::collect_loop_carrier_targets;

pub(in crate::mir::builder) struct GenericLoopV1CarrierState<K = String> {
    pub phi_bindings: BTreeMap<K, crate::mir::ValueId>,
    pub carrier_step_phis: BTreeMap<K, crate::mir::ValueId>,
    pub loop_var_step_phi: crate::mir::ValueId,
    pub carrier_infos: Vec<GenericLoopExtraCarrierV1<K>>,
}

pub(in crate::mir::builder) struct GenericLoopExtraCarrierV1<K> {
    pub key: K,
    pub label: String,
    pub init: crate::mir::ValueId,
    pub header: crate::mir::ValueId,
    pub step: crate::mir::ValueId,
}

/// Already-resolved physical input; this row is not semantic authority.
pub(in crate::mir::builder) struct GenericLoopCarrierEntryV1<K> {
    pub key: K,
    pub label: String,
    pub init: crate::mir::ValueId,
    pub ty: MirType,
}

pub(in crate::mir::builder) fn prepare_generic_loop_v1_carriers(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    loop_var_current: crate::mir::ValueId,
    carrier_representation: &PreparedGenericLoopCarrierRepresentationV1,
) -> GenericLoopV1CarrierState {
    let carrier_targets = collect_loop_carrier_targets(&facts.body.body);
    prepare_generic_loop_v1_carriers_from_targets(
        builder,
        &facts.loop_var,
        &carrier_targets,
        loop_var_current,
        carrier_representation,
    )
}

pub(in crate::mir::builder) fn prepare_generic_loop_v1_carriers_from_targets(
    builder: &mut MirBuilder,
    loop_var: &str,
    carrier_targets: &[String],
    loop_var_current: crate::mir::ValueId,
    carrier_representation: &PreparedGenericLoopCarrierRepresentationV1,
) -> GenericLoopV1CarrierState {
    let pre_loop_map = builder.function_state.variable_ctx.variable_map.clone();
    let carrier_vars = carrier_targets
        .iter()
        .filter(|name| name.as_str() != loop_var && pre_loop_map.contains_key(name.as_str()));
    let entries = carrier_vars
        .filter_map(|var| {
            let init = pre_loop_map.get(var.as_str()).copied()?;
            let ty = builder
                .function_state
                .type_ctx
                .get_type(init)
                .cloned()
                .unwrap_or(MirType::Unknown);
            Some(GenericLoopCarrierEntryV1 {
                key: var.clone(),
                label: var.clone(),
                init,
                ty,
            })
        })
        .collect::<Vec<_>>();
    allocate_generic_loop_v1_carriers(
        builder,
        loop_var.to_owned(),
        loop_var_current,
        carrier_representation,
        entries,
    )
}

/// Sole carrier allocation core for raw names and co-sealed source keys.
/// Callers validate source membership/entry coverage before entering this owner.
pub(in crate::mir::builder) fn allocate_generic_loop_v1_carriers<K: Ord + Clone>(
    builder: &mut MirBuilder,
    induction: K,
    loop_var_current: crate::mir::ValueId,
    carrier_representation: &PreparedGenericLoopCarrierRepresentationV1,
    entries: Vec<GenericLoopCarrierEntryV1<K>>,
) -> GenericLoopV1CarrierState<K> {
    let mut phi_bindings = BTreeMap::from([(induction.clone(), loop_var_current)]);
    let loop_var_step_phi = builder.alloc_typed(carrier_representation.exact_type().clone());
    let mut carrier_step_phis = BTreeMap::from([(induction, loop_var_step_phi)]);
    let mut carrier_infos = Vec::new();
    for entry in entries {
        let phi_dst = builder.alloc_typed(entry.ty.clone());
        let step_phi_dst = builder.alloc_typed(entry.ty);
        phi_bindings.insert(entry.key.clone(), phi_dst);
        carrier_step_phis.insert(entry.key.clone(), step_phi_dst);
        carrier_infos.push(GenericLoopExtraCarrierV1 {
            key: entry.key,
            label: entry.label,
            init: entry.init,
            header: phi_dst,
            step: step_phi_dst,
        });
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
    loop_var_init: crate::mir::ValueId,
    loop_var_current: crate::mir::ValueId,
    post_body_map: &BTreeMap<String, crate::mir::ValueId>,
    body_has_continue_edge: bool,
) -> Result<(), String> {
    let mut final_values = loop_plan.final_values.raw_rows()?.to_vec();
    // Preserve the legacy fallback only at the raw facade. The shared closure
    // requires an exact incoming for every extra carrier when a backedge exists.
    let incoming = carrier_state
        .carrier_infos
        .iter()
        .map(|row| {
            (
                row.key.clone(),
                post_body_map.get(&row.key).copied().unwrap_or(row.header),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let phis = close_generic_loop_v1_carrier_phis(
        loop_plan,
        &carrier_state,
        loop_var,
        loop_var_init,
        loop_var_current,
        body_has_continue_edge.then_some(&incoming),
    )?;
    for row in carrier_state.carrier_infos {
        final_values.push((row.key.clone(), row.header));
        publish_emission_cache(builder, row.key, row.header);
    }
    loop_plan.phis = phis;
    loop_plan.final_values = final_values.into();
    publish_emission_cache(builder, loop_var.to_string(), loop_var_current);
    Ok(())
}

/// Existing PHI closure with keys kept separate from diagnostic labels.
/// None denotes the existing no-backedge path; it does not synthesize input.
pub(in crate::mir::builder) fn close_generic_loop_v1_carrier_phis<K: Ord>(
    loop_plan: &CoreLoopPlan,
    carrier_state: &GenericLoopV1CarrierState<K>,
    loop_label: &str,
    loop_var_init: crate::mir::ValueId,
    loop_var_current: crate::mir::ValueId,
    backedge_inputs: Option<&BTreeMap<K, crate::mir::ValueId>>,
) -> Result<Vec<CorePhiInfo>, String> {
    if let Some(inputs) = backedge_inputs {
        for row in &carrier_state.carrier_infos {
            if !inputs.contains_key(&row.key) {
                return Err("[freeze:contract][generic-loop/carrier-backedge-missing]".to_owned());
            }
        }
    }
    let mut phis = loop_plan.phis.clone();
    if backedge_inputs.is_some() {
        phis.push(CorePhiInfo {
            block: loop_plan.step_bb,
            dst: carrier_state.loop_var_step_phi,
            inputs: Vec::new(),
            tag: format!("loop_step_in_{}", loop_label),
        });
        phis.push(loop_carriers::build_loop_phi_info(
            loop_plan.header_bb,
            loop_plan.preheader_bb,
            loop_plan.step_bb,
            loop_var_current,
            loop_var_init,
            carrier_state.loop_var_step_phi,
            format!("loop_var_{}", loop_label),
        ));
    } else {
        phis.push(loop_carriers::build_preheader_only_phi_info(
            loop_plan.header_bb,
            loop_plan.preheader_bb,
            loop_var_current,
            loop_var_init,
            format!("loop_var_{}", loop_label),
        ));
    }
    for row in &carrier_state.carrier_infos {
        if let Some(inputs) = backedge_inputs {
            let incoming = *inputs.get(&row.key).ok_or_else(|| {
                "[freeze:contract][generic-loop/carrier-backedge-missing]".to_owned()
            })?;
            phis.push(CorePhiInfo {
                block: loop_plan.step_bb,
                dst: row.step,
                inputs: vec![(loop_plan.body_bb, incoming)],
                tag: format!("loop_step_in_{}", row.label),
            });
            phis.push(loop_carriers::build_loop_phi_info(
                loop_plan.header_bb,
                loop_plan.preheader_bb,
                loop_plan.step_bb,
                row.header,
                row.init,
                row.step,
                format!("loop_carrier_{}", row.label),
            ));
        } else {
            phis.push(loop_carriers::build_preheader_only_phi_info(
                loop_plan.header_bb,
                loop_plan.preheader_bb,
                row.header,
                row.init,
                format!("loop_carrier_{}", row.label),
            ));
        }
    }
    Ok(phis)
}
