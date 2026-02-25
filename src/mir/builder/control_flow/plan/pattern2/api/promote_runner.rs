//! Phase 263 P0.2: Pattern2 promotion runner (SSOT entry point)
//!
//! Single entry point for all Pattern2 promotion logic.
//! All callers should use `try_promote()` instead of accessing internals directly.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::super::body_local_policy::{classify_for_pattern2, BodyLocalRoute};
use super::super::super::pattern2_inputs_facts_box::Pattern2Inputs;
use crate::mir::builder::control_flow::plan::policies::PolicyDecision;

use super::promote_decision::{PromoteDecision, PromoteStepResult};

/// Phase 263 P0.2: Try to promote LoopBodyLocal variables for Pattern2
///
/// This is the single entry point for Pattern2 promotion logic.
/// Returns PromoteDecision to indicate success, applicability, or freeze.
pub(in crate::mir::builder) fn try_promote(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    inputs: Pattern2Inputs,
    debug: bool,
    _verbose: bool,
) -> Result<PromoteDecision, String> {
    let mut inputs = inputs;
    use crate::mir::join_ir::lowering::digitpos_condition_normalizer::DigitPosConditionNormalizer;
    use crate::mir::loop_pattern_detection::loop_condition_scope::LoopConditionScopeBox;

    let cond_scope = LoopConditionScopeBox::analyze(
        &inputs.loop_var_name,
        &vec![condition, &inputs.break_condition_node],
        Some(&inputs.scope),
    );

    let mut promoted_pairs: Vec<(String, String)> = Vec::new();
    let cond_body_local_vars: Vec<String> = cond_scope
        .vars
        .iter()
        .filter(|v| matches!(v.scope, crate::mir::loop_pattern_detection::loop_condition_scope::CondVarScope::LoopBodyLocal))
        .map(|v| v.name.clone())
        .collect();

    let has_body_locals_in_conditions = cond_scope.has_loop_body_local();

    if has_body_locals_in_conditions {
        // Policy-controlled: some families must not run promotion/slot heuristics here.
        // Example: balanced depth-scan uses derived vars and doesn't have a break-guard node.
        if matches!(
            inputs.body_local_handling,
            crate::mir::builder::control_flow::plan::pattern2_inputs_facts_box::BodyLocalHandlingPolicy::SkipPromotion
        ) {
            // no-op: lowerers will populate LoopBodyLocalEnv via init/derived emission.
        } else if !inputs.is_loop_true_read_digits {
            match classify_for_pattern2(
                builder,
                &inputs.loop_var_name,
                &inputs.scope,
                &inputs.break_condition_node,
                &cond_scope,
                body,
            ) {
                PolicyDecision::Use(BodyLocalRoute::Promotion {
                    promoted_carrier,
                    promoted_var,
                    carrier_name,
                }) => {
                    let is_trim_promotion = promoted_carrier.trim_helper().is_some();
                    if !is_trim_promotion {
                        promoted_pairs.push((promoted_var.clone(), carrier_name.clone()));
                    }

                    #[cfg(feature = "normalized_dev")]
                    {
                        use crate::mir::join_ir::lowering::carrier_binding_assigner::CarrierBindingAssigner;
                        let mut promoted_carrier = promoted_carrier;
                        CarrierBindingAssigner::assign_promoted_binding(
                            builder,
                            &mut promoted_carrier,
                            &promoted_var,
                            &carrier_name,
                        )
                        .map_err(|e| format!("[phase78/binding_assign] {:?}", e))?;
                        inputs.carrier_info.merge_from(&promoted_carrier);
                    }
                    #[cfg(not(feature = "normalized_dev"))]
                    {
                        inputs.carrier_info.merge_from(&promoted_carrier);
                    }

                    inputs
                        .carrier_info
                        .promoted_loopbodylocals
                        .push(promoted_var.clone());

                    if !is_trim_promotion {
                        inputs.break_condition_node = DigitPosConditionNormalizer::normalize(
                            &inputs.break_condition_node,
                            &promoted_var,
                            &carrier_name,
                        );
                    }
                }
                PolicyDecision::Use(BodyLocalRoute::ReadOnlySlot(slot)) => {
                    inputs.allowed_body_locals_for_conditions = vec![slot.name.clone()];
                    inputs.read_only_body_local_slot = Some(slot);
                }
                PolicyDecision::Use(BodyLocalRoute::DerivedSlot(recipe)) => {
                    inputs.allowed_body_locals_for_conditions = vec![recipe.name.clone()];
                    inputs.body_local_derived_slot_recipe = Some(recipe);
                }
                PolicyDecision::Reject(reason) => {
                    // Phase 263 P0.1: Reject を PromoteDecision で二分化（型安全）
                    // 対象だが未対応（freeze級）: 実装バグ or 将来実装予定 → Freeze で Fail-Fast
                    return Ok(PromoteDecision::Freeze(format!(
                        "[pattern2/api/promote] Pattern2 未対応エラー（LoopBodyLocal {:?}）: {}",
                        cond_body_local_vars, reason
                    )));
                }
                PolicyDecision::None => {}
            }
        }
    }

    // Allocate join_ids for carriers and register bindings.
    for carrier in &mut inputs.carrier_info.carriers {
        let carrier_join_id = inputs.join_value_space.alloc_param();
        carrier.join_id = Some(carrier_join_id);
        #[cfg(feature = "normalized_dev")]
        if let Some(binding_id) = carrier.binding_id {
            use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
            match carrier.role {
                CarrierRole::ConditionOnly => inputs.env.register_condition_binding(binding_id, carrier_join_id),
                CarrierRole::LoopState => inputs.env.register_carrier_binding(binding_id, carrier_join_id),
            }
        }
    }

    for (promoted_var, promoted_carrier_name) in promoted_pairs {
        let join_id = inputs
            .carrier_info
            .find_carrier(&promoted_carrier_name)
            .and_then(|c| c.join_id)
            .ok_or_else(|| format!("[phase229] promoted carrier '{}' has no join_id", promoted_carrier_name))?;
        inputs.env.insert(promoted_var, join_id);
    }

    // ExprLowerer validation (best-effort; unchanged behavior)
    {
        use crate::mir::join_ir::lowering::expr_lowerer::{ExprContext, ExprLowerer, ExprLoweringError};
        use crate::mir::join_ir::lowering::scope_manager::Pattern2ScopeManager;

        let scope_manager = Pattern2ScopeManager {
            condition_env: &inputs.env,
            loop_body_local_env: Some(&inputs.body_local_env),
            captured_env: Some(&inputs.captured_env),
            carrier_info: &inputs.carrier_info,
        };

        match ExprLowerer::new(&scope_manager, ExprContext::Condition, builder)
            .with_debug(debug)
            .lower(&inputs.break_condition_node)
        {
            Ok(_) => {}
            Err(ExprLoweringError::UnsupportedNode(_)) => {}
            Err(_) => {}
        }
    }

    if has_body_locals_in_conditions {
        Ok(PromoteDecision::Promoted(PromoteStepResult { inputs }))
    } else {
        Ok(PromoteDecision::NotApplicable(PromoteStepResult { inputs }))
    }
}
