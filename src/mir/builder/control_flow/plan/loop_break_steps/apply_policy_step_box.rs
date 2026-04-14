//! ApplyPolicyStepBox (Phase 106)
//!
//! Responsibility: apply policy routing for loop-break condition + allow-list.

use crate::ast::ASTNode;

use super::super::loop_break_policy_router::LoopBreakPolicyRouterBox;
use super::super::loop_break_prep_box::LoopBreakPrepInputs;
use super::gather_facts_step_box::LoopBreakPrepFacts;

pub(crate) struct ApplyPolicyStepBox;

impl ApplyPolicyStepBox {
    pub(crate) fn apply(
        condition: &ASTNode,
        body: &[ASTNode],
        facts: LoopBreakPrepFacts,
    ) -> Result<LoopBreakPrepInputs, String> {
        let policy = LoopBreakPolicyRouterBox::route(condition, body)?;

        Ok(LoopBreakPrepInputs {
            loop_var_name: facts.loop_var_name,
            loop_var_id: facts.loop_var_id,
            carrier_info: facts.carrier_info,
            scope: facts.scope,
            captured_env: facts.captured_env,
            join_value_space: facts.join_value_space,
            env: facts.env,
            condition_bindings: facts.condition_bindings,
            body_local_env: facts.body_local_env,
            allowed_body_locals_for_conditions: policy.allowed_body_locals_for_conditions,
            body_local_handling: policy.body_local_handling,
            read_only_body_local_slot: policy.read_only_body_local_slot,
            break_condition_node: policy.break_condition_node,
            is_loop_true_read_digits: policy.is_loop_true_read_digits,
            condition_only_recipe: None,
            body_local_derived_recipe: None,
            body_local_derived_slot_recipe: None,
            balanced_depth_scan_recipe: policy.balanced_depth_scan_recipe,
            carrier_updates_override: policy.carrier_updates_override,
            post_loop_early_return: policy.post_loop_early_return,
            current_static_box_name: None, // Wired by orchestrator (low priority: move here)
        })
    }
}
