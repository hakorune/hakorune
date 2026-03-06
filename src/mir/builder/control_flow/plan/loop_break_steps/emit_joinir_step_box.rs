//! EmitJoinIRStepBox (Phase 106)
//!
//! Responsibility: call loop_break route JoinIR lowerer and build inline boundary.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::loop_break_prep_box::{LoopBreakDebugLog, LoopBreakPrepInputs};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

use std::collections::BTreeMap;

pub(crate) struct EmitJoinIRStepOutput {
    pub join_module: crate::mir::join_ir::JoinModule,
    pub boundary: JoinInlineBoundary,
}

pub(crate) struct EmitJoinIRStepBox;

impl EmitJoinIRStepBox {
    pub(crate) fn emit(
        builder: &mut MirBuilder,
        condition: &ASTNode,
        body_ast: &[ASTNode],
        effective_break_condition: &ASTNode,
        carrier_updates: &BTreeMap<String, crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr>,
        inputs: &mut LoopBreakPrepInputs,
        debug: bool,
        verbose: bool,
        skeleton: Option<&crate::mir::loop_canonicalizer::LoopSkeleton>,
    ) -> Result<EmitJoinIRStepOutput, String> {
        use crate::mir::join_ir::lowering::loop_with_break_minimal::lower_loop_with_break_minimal;

        let log = LoopBreakDebugLog::new(verbose);

        let lowering_inputs = crate::mir::join_ir::lowering::loop_with_break_minimal::LoopWithBreakLoweringInputs {
            scope: inputs.scope.clone(),
            condition,
            break_condition: effective_break_condition,
            env: &inputs.env,
            carrier_info: &inputs.carrier_info,
            carrier_updates,
            body_ast,
            body_local_env: Some(&mut inputs.body_local_env),
            allowed_body_locals_for_conditions: if inputs.allowed_body_locals_for_conditions.is_empty() {
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
            current_static_box_name: inputs.current_static_box_name.clone(), // Phase 252
        };

        let (join_module, fragment_meta) = match lower_loop_with_break_minimal(lowering_inputs) {
            Ok((module, meta)) => (module, meta),
            Err(e) => {
                crate::mir::builder::control_flow::joinir::trace::trace()
                    .debug("loop_break", &format!("LoopBreak lowerer failed: {}", e));
                return Err(format!("[cf_loop/loop_break] Lowering failed: {}", e));
            }
        };

        let exit_meta = &fragment_meta.exit_meta;
        use crate::mir::builder::control_flow::joinir::merge::exit_line::ExitMetaCollector;
        let mut exit_bindings =
            ExitMetaCollector::collect(builder, exit_meta, Some(&inputs.carrier_info), debug);
        // Phase 29af P0: Exit reconnection targets LoopState only.
        exit_bindings.retain(|binding| {
            binding.role == crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState
        });
        // Phase 29af P0: Reject duplicate carrier_name in exit_bindings.
        #[cfg(debug_assertions)]
        {
            use std::collections::HashSet;
            let mut seen = HashSet::new();
            for binding in &exit_bindings {
                debug_assert!(
                    seen.insert(&binding.carrier_name),
                    "Phase 29af Fail-Fast: duplicate exit_binding carrier '{}'",
                    binding.carrier_name
                );
            }
        }

        // Phase 256.8.5: Use JoinModule.entry.params as SSOT (no hardcoded ValueIds)
        use crate::mir::builder::control_flow::plan::common::get_entry_function;
        let main_func = get_entry_function(&join_module, "emit_joinir")?;

        // SSOT: Use actual params allocated by JoinIR lowerer
        let join_input_slots = main_func.params.clone();

        // Build host_input_values in same order (loop_var + carriers)
        let mut host_input_values = vec![inputs.loop_var_id];
        for carrier in inputs.carrier_info.carriers.iter() {
            use crate::mir::builder::control_flow::plan::common::{
                decide_carrier_binding_policy, CarrierBindingPolicy,
            };
            match decide_carrier_binding_policy(carrier) {
                CarrierBindingPolicy::BindFromHost => {
                    if carrier.host_id == crate::mir::ValueId(0) {
                        return Err(format!(
                            "[emit_joinir] Phase 29af Fail-Fast: FromHost carrier '{}' has host_id=0",
                            carrier.name
                        ));
                    }
                    host_input_values.push(carrier.host_id);
                }
                CarrierBindingPolicy::SkipBinding => {
                    // Placeholder: SkipBinding does not require a host slot.
                    host_input_values.push(crate::mir::ValueId(0));
                }
            }
        }

        // Verify count consistency (fail-fast)
        if join_input_slots.len() != host_input_values.len() {
            return Err(format!(
                "[emit_joinir] Params count mismatch: join_inputs={}, host_inputs={}",
                join_input_slots.len(), host_input_values.len()
            ));
        }

        use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(join_input_slots, host_input_values)
            .with_condition_bindings(std::mem::take(&mut inputs.condition_bindings))
            .with_exit_bindings(exit_bindings.clone())
            .with_expr_result(fragment_meta.expr_result)
            .with_loop_var_name(Some(inputs.loop_var_name.clone()))
            .with_carrier_info(inputs.carrier_info.clone())
            .build();

        log.log("emit", "JoinIR module + boundary constructed");

        Ok(EmitJoinIRStepOutput { join_module, boundary })
    }
}
