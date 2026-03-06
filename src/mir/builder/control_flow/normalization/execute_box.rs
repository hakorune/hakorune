//! NormalizationExecuteBox: Execute normalization plan (Phase 134 P0)
//!
//! ## Responsibility
//!
//! - Execute a NormalizationPlan from PlanBox
//! - Build StepTree, lower to JoinIR, merge into MIR
//! - SSOT for "how to execute" normalization
//!
//! ## Contract
//!
//! - Modifies builder state (adds blocks, instructions, updates variable_map)
//! - Uses DirectValue mode (no PHI generation)
//! - Returns Ok(()) on success, Err(_) on failure

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use super::plan::{NormalizationPlan, PlanKind};
use std::collections::BTreeMap;

/// Box-First: Execute normalization plan
pub struct NormalizationExecuteBox;

impl NormalizationExecuteBox {
    /// Execute a normalization plan
    ///
    /// ## Phase 141 P1.5: Added prefix_variables parameter
    ///
    /// Returns:
    /// - Ok(value_id): Successfully executed, returns result value
    /// - Err(_): Lowering or merge failed
    pub fn execute(
        builder: &mut MirBuilder,
        plan: &NormalizationPlan,
        remaining: &[ASTNode],
        func_name: &str,
        debug: bool,
        prefix_variables: Option<&BTreeMap<String, ValueId>>,
    ) -> Result<ValueId, String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        if debug {
            trace.routing(
                "normalization/execute",
                func_name,
                &format!(
                    "Executing plan: kind={:?}, consumed={}",
                    plan.kind, plan.consumed
                ),
            );
        }

        // Validate consumed vs remaining
        if plan.consumed > remaining.len() {
            return Err(format!(
                "[normalization/execute] Plan wants to consume {} statements but only {} available",
                plan.consumed,
                remaining.len()
            ));
        }

        match &plan.kind {
            PlanKind::LoopOnly => {
                Self::execute_loop_only(builder, remaining, func_name, debug, prefix_variables)
            }
        }
    }

    /// Execute Phase 131: Loop-only shape
    ///
    /// ## Phase 141 P1.5: Added prefix_variables parameter
    fn execute_loop_only(
        builder: &mut MirBuilder,
        remaining: &[ASTNode],
        func_name: &str,
        debug: bool,
        prefix_variables: Option<&BTreeMap<String, ValueId>>,
    ) -> Result<ValueId, String> {
        use crate::ast::Span;
        use crate::mir::control_tree::normalized_shadow::env_layout::EnvLayout;
        use crate::mir::control_tree::normalized_shadow::available_inputs_collector::AvailableInputsCollectorBox;
        use crate::mir::control_tree::normalized_shadow::StepTreeNormalizedShadowLowererBox;
        use crate::mir::control_tree::StepTreeBuilderBox;

        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Build StepTree from loop AST
        let loop_ast = if let ASTNode::Loop { condition, body, .. } = &remaining[0] {
            ASTNode::Loop {
                condition: condition.clone(),
                body: body.clone(),
                span: Span::unknown(),
            }
        } else {
            return Err("[normalization/execute] First statement is not a loop".to_string());
        };

        let tree = StepTreeBuilderBox::build_from_ast(&loop_ast);

        // Collect available inputs (Phase 141 P1.5: with prefix variables)
        let available_inputs = AvailableInputsCollectorBox::collect(builder, None, prefix_variables);
        let env_layout = EnvLayout::from_contract(&tree.contract, &available_inputs);
        let env_fields = env_layout.env_fields();

        if debug {
            trace.routing(
                "normalization/execute/loop_only",
                func_name,
                &format!("Available inputs: {}", available_inputs.len()),
            );
        }

        // Try Normalized lowering
        let (join_module, join_meta) =
            match StepTreeNormalizedShadowLowererBox::try_lower_if_only(&tree, &available_inputs) {
                Ok(Some(result)) => result,
                Ok(None) => {
                    return Err(
                        "[normalization/execute] StepTree lowering returned None (out of scope)"
                            .to_string(),
                    );
                }
                Err(e) => {
                    if crate::config::env::joinir_dev::strict_enabled() {
                        use crate::mir::join_ir::lowering::error_tags;
                        return Err(error_tags::freeze_with_hint(
                            "phase134/normalization/loop_only",
                            &e,
                            "Loop should be supported by Normalized but lowering failed",
                        ));
                    }
                    return Err(format!("[normalization/execute] Lowering failed: {}", e));
                }
            };

        // Merge JoinIR into MIR (wire env inputs explicitly)
        Self::merge_normalized_joinir(
            builder,
            join_module,
            join_meta,
            &available_inputs,
            &env_fields,
            func_name,
            debug,
        )?;

        // Return void constant (loop doesn't produce a value)
        use crate::mir::{ConstValue, MirInstruction};
        let void_id = builder.next_value_id();
        builder.emit_instruction(MirInstruction::Const {
            dst: void_id,
            value: ConstValue::Void,
        })?;

        Ok(void_id)
    }
    /// Merge Normalized JoinModule into MIR builder
    ///
    /// Extracted from routing.rs and suffix_router_box.rs
    fn merge_normalized_joinir(
        builder: &mut MirBuilder,
        join_module: crate::mir::join_ir::JoinModule,
        join_meta: crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta,
        available_inputs: &BTreeMap<String, ValueId>,
        env_fields: &[String],
        func_name: &str,
        debug: bool,
    ) -> Result<(), String> {
        use crate::mir::builder::control_flow::joinir::merge;
        use crate::mir::join_ir::frontend::JoinFuncMetaMap;
        use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, ExitReconnectMode};
        use crate::mir::join_ir::lowering::inline_boundary::{JoinInlineBoundary, LoopExitBinding};
        use crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir_with_meta;
        use std::collections::BTreeMap;

        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Build exit_bindings from meta
        let exit_bindings: Vec<LoopExitBinding> = join_meta
            .exit_meta
            .exit_values
            .iter()
            .map(|(carrier_name, join_exit_value)| {
                // Get host_slot from variable_map
                let host_slot = builder
                    .variable_ctx
                    .variable_map
                    .get(carrier_name)
                    .copied()
                    .unwrap_or_else(|| {
                        panic!(
                            "[Phase 134 P0] Carrier '{}' not in variable_map (available: {:?})",
                            carrier_name,
                            builder.variable_ctx.variable_map.keys().collect::<Vec<_>>()
                        )
                    });

                LoopExitBinding {
                    carrier_name: carrier_name.clone(),
                    join_exit_value: *join_exit_value,
                    host_slot,
                    role: CarrierRole::LoopState,
                }
            })
            .collect();

        // Create boundary with DirectValue mode
        //
        // Phase 143: Normalized shadow loops can reference prefix variables in conditions.
        // The merger must seed JoinIR "env params" from host values explicitly.
        //
        // Contract:
        // - `env_fields` is the SSOT order (writes + inputs)
        // - `available_inputs` provides the host ValueId for each env field
        // - `join_inputs` are the JoinIR entry params in the same order as env_fields
        let entry_id = join_module
            .entry
            .ok_or_else(|| "[normalization/execute] JoinModule missing entry".to_string())?;
        let entry_func = join_module
            .functions
            .get(&entry_id)
            .ok_or_else(|| "[normalization/execute] JoinModule entry function missing".to_string())?;
        if entry_func.params.len() != env_fields.len() {
            return Err(format!(
                "[normalization/execute] env arity mismatch: entry params={} env_fields={}",
                entry_func.params.len(),
                env_fields.len()
            ));
        }
        let join_inputs = entry_func.params.clone();
        let mut host_inputs: Vec<ValueId> = Vec::with_capacity(env_fields.len());
        for name in env_fields {
            let host_vid = available_inputs.get(name).copied().ok_or_else(|| {
                format!(
                    "[normalization/execute] missing host input for env field '{name}' (available_inputs keys={:?})",
                    available_inputs.keys().collect::<Vec<_>>()
                )
            })?;
            host_inputs.push(host_vid);
        }

        let mut boundary = JoinInlineBoundary::new_with_exit_bindings(
            join_inputs,
            host_inputs,
            exit_bindings,
        );
        boundary.exit_reconnect_mode = ExitReconnectMode::DirectValue; // No PHI
        boundary.continuation_func_ids = join_meta.continuation_funcs.clone();

        if debug {
            trace.routing(
                "normalization/execute/merge",
                func_name,
                &format!(
                    "Merging JoinModule: {} functions, {} exit bindings",
                    join_module.functions.len(),
                    boundary.exit_bindings.len()
                ),
            );
        }

        // Bridge JoinIR to MIR
        //
        // Note: Normalized shadow emitters often mark modules as `JoinIrPhase::Normalized`
        // for dev/strict structural verification.
        //
        // The JoinIR→MIR bridge entry used here expects a Structured JoinModule.
        // For shadow modules, the instruction vocabulary is still compatible with the
        // Structured converter, so we bridge using a "Structured snapshot" (phase-only)
        // to keep the verifier contract intact while unblocking MIR conversion.
        let mut bridge_module = join_module.clone();
        if bridge_module.is_normalized() {
            bridge_module.phase = crate::mir::join_ir::JoinIrPhase::Structured;
        }
        let empty_meta: JoinFuncMetaMap = BTreeMap::new();
        // Phase 256 P1.5: Pass boundary to bridge for ValueId remapping
        let mir_module = bridge_joinir_to_mir_with_meta(&bridge_module, &empty_meta, Some(&boundary))
            .map_err(|e| format!("[normalization/execute] MIR conversion failed: {:?}", e))?;

        // Merge with boundary
        let _exit_phi_result = merge::merge_joinir_mir_blocks(
            builder,
            &mir_module,
            Some(&boundary),
            debug,
        )?;

        if debug {
            trace.routing(
                "normalization/execute/merge",
                func_name,
                "Merge + reconnection completed",
            );
        }

        Ok(())
    }
}
