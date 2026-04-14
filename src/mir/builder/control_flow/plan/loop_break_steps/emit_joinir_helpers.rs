use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::common::{
    decide_carrier_binding_policy, get_entry_function, CarrierBindingPolicy,
};
use crate::mir::builder::control_flow::plan::loop_break_prep_box::LoopBreakPrepInputs;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, JoinFragmentMeta};
use crate::mir::join_ir::lowering::inline_boundary::{JoinInlineBoundary, LoopExitBinding};
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_canonicalizer::LoopSkeleton;
use crate::mir::ValueId;

pub(super) fn lower_loop_break_joinir_fragment(
    condition: &ASTNode,
    body_ast: &[ASTNode],
    effective_break_condition: &ASTNode,
    carrier_updates: &BTreeMap<String, UpdateExpr>,
    inputs: &mut LoopBreakPrepInputs,
    skeleton: Option<&LoopSkeleton>,
) -> Result<(JoinModule, JoinFragmentMeta), String> {
    use crate::mir::join_ir::lowering::loop_with_break_minimal::lower_loop_with_break_minimal;

    let lowering_inputs =
        crate::mir::join_ir::lowering::loop_with_break_minimal::LoopWithBreakLoweringInputs {
            scope: inputs.scope.clone(),
            condition,
            break_condition: effective_break_condition,
            env: &inputs.env,
            carrier_info: &inputs.carrier_info,
            carrier_updates,
            body_ast,
            body_local_env: Some(&mut inputs.body_local_env),
            allowed_body_locals_for_conditions: if inputs
                .allowed_body_locals_for_conditions
                .is_empty()
            {
                None
            } else {
                Some(inputs.allowed_body_locals_for_conditions.as_slice())
            },
            join_value_space: &mut inputs.join_value_space,
            skeleton,
            condition_only_recipe: inputs.condition_only_recipe.as_ref(),
            body_local_derived_recipe: inputs.body_local_derived_recipe.as_ref(),
            body_local_derived_slot_recipe: inputs.body_local_derived_slot_recipe.as_ref(),
            balanced_depth_scan_recipe: inputs.balanced_depth_scan_recipe.as_ref(),
            current_static_box_name: inputs.current_static_box_name.clone(),
        };

    lower_loop_with_break_minimal(lowering_inputs)
}

pub(super) fn build_loop_break_inline_boundary(
    builder: &mut MirBuilder,
    join_module: &JoinModule,
    fragment_meta: &JoinFragmentMeta,
    inputs: &mut LoopBreakPrepInputs,
    debug: bool,
) -> Result<JoinInlineBoundary, String> {
    let exit_bindings = collect_loop_break_exit_bindings(
        builder,
        &fragment_meta.exit_meta,
        &inputs.carrier_info,
        debug,
    );
    let join_input_slots = get_entry_function(join_module, "emit_joinir")?
        .params
        .clone();
    let host_input_values = collect_loop_break_host_inputs(inputs)?;

    if join_input_slots.len() != host_input_values.len() {
        return Err(format!(
            "[emit_joinir] Params count mismatch: join_inputs={}, host_inputs={}",
            join_input_slots.len(),
            host_input_values.len()
        ));
    }

    use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;
    Ok(JoinInlineBoundaryBuilder::new()
        .with_inputs(join_input_slots, host_input_values)
        .with_condition_bindings(std::mem::take(&mut inputs.condition_bindings))
        .with_exit_bindings(exit_bindings)
        .with_expr_result(fragment_meta.expr_result)
        .with_loop_var_name(Some(inputs.loop_var_name.clone()))
        .with_carrier_info(inputs.carrier_info.clone())
        .build())
}

fn collect_loop_break_exit_bindings(
    builder: &mut MirBuilder,
    exit_meta: &crate::mir::join_ir::lowering::carrier_info::ExitMeta,
    carrier_info: &crate::mir::join_ir::lowering::carrier_info::CarrierInfo,
    debug: bool,
) -> Vec<LoopExitBinding> {
    use crate::mir::builder::control_flow::joinir::merge::exit_line::ExitMetaCollector;

    let mut exit_bindings =
        ExitMetaCollector::collect(builder, exit_meta, Some(carrier_info), debug);
    exit_bindings.retain(|binding| binding.role == CarrierRole::LoopState);

    #[cfg(debug_assertions)]
    {
        let mut seen = BTreeSet::new();
        for binding in &exit_bindings {
            debug_assert!(
                seen.insert(binding.carrier_name.as_str()),
                "Phase 29af Fail-Fast: duplicate exit_binding carrier '{}'",
                binding.carrier_name
            );
        }
    }

    exit_bindings
}

fn collect_loop_break_host_inputs(inputs: &LoopBreakPrepInputs) -> Result<Vec<ValueId>, String> {
    let mut host_input_values = vec![inputs.loop_var_id];

    for carrier in &inputs.carrier_info.carriers {
        match decide_carrier_binding_policy(carrier) {
            CarrierBindingPolicy::BindFromHost => {
                if carrier.host_id == ValueId(0) {
                    return Err(format!(
                        "[emit_joinir] Phase 29af Fail-Fast: FromHost carrier '{}' has host_id=0",
                        carrier.name
                    ));
                }
                host_input_values.push(carrier.host_id);
            }
            CarrierBindingPolicy::SkipBinding => host_input_values.push(ValueId(0)),
        }
    }

    Ok(host_input_values)
}
