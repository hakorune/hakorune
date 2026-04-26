use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;
use crate::mir::builder::control_flow::plan::loop_break_prep_box::{
    BodyLocalHandlingPolicy, LoopBreakPrepInputs,
};
use crate::mir::builder::MirBuilder;

use super::super::super::body_local_policy::{
    classify_loop_break_body_local_route, BodyLocalRoute,
};

pub(super) struct PromotePreparation {
    pub has_body_locals_in_conditions: bool,
    pub promoted_pairs: Vec<(String, String)>,
}

pub(super) fn prepare_promoted_inputs(
    builder: &MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    inputs: &mut LoopBreakPrepInputs,
) -> Result<PromotePreparation, String> {
    use crate::mir::join_ir::lowering::digitpos_condition_normalizer::DigitPosConditionNormalizer;
    use crate::mir::loop_route_detection::support::condition_scope::LoopConditionScopeBox;

    let cond_scope = LoopConditionScopeBox::analyze(
        &inputs.loop_var_name,
        &vec![condition, &inputs.break_condition_node],
        Some(&inputs.scope),
    );

    let cond_body_local_vars: Vec<String> = cond_scope
        .vars
        .iter()
        .filter(|v| {
            matches!(
                v.scope,
                crate::mir::loop_route_detection::support::condition_scope::CondVarScope::LoopBodyLocal
            )
        })
        .map(|v| v.name.clone())
        .collect();

    let has_body_locals_in_conditions = cond_scope.has_loop_body_local();
    let mut promoted_pairs = Vec::new();

    if has_body_locals_in_conditions {
        if matches!(
            inputs.body_local_handling,
            BodyLocalHandlingPolicy::SkipPromotion
        ) {
            // no-op: lowerers will populate LoopBodyLocalEnv via init/derived emission.
        } else if !inputs.is_loop_true_read_digits {
            match classify_loop_break_body_local_route(
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

                    inputs.carrier_info.merge_from(&promoted_carrier);
                    inputs
                        .carrier_info
                        .promoted_body_locals
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
                    return Err(format!(
                        "[loop_break/api/promote] LoopBreak 未対応エラー（LoopBodyLocal {:?}）: {}",
                        cond_body_local_vars, reason
                    ));
                }
                PolicyDecision::None => {}
            }
        }
    }

    Ok(PromotePreparation {
        has_body_locals_in_conditions,
        promoted_pairs,
    })
}
