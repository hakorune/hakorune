//! Loop-break policy router (Phase 108)
//!
//! Responsibility (SSOT):
//! - Decide which loop-break policy route applies (balanced depth-scan / loop(true) read-digits / default).
//! - Normalize the outputs into a single routing struct consumed by ApplyPolicyStepBox.
//!
//! NOTE: This box does not emit JoinIR. It only provides "what to do" facts.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::common::body_local_slot::ReadOnlyBodyLocalSlotBox;

use super::loop_break_condition_policy_router::LoopBreakConditionPolicyRouterBox;
use super::loop_break_prep_box::BodyLocalHandlingPolicy;
use crate::mir::builder::control_flow::cleanup::policies::balanced_depth_scan_policy_box::BalancedDepthScanPolicyBox;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;

#[derive(Debug)]
pub(crate) struct LoopBreakPolicyRouting {
    pub allowed_body_locals_for_conditions: Vec<String>,
    pub body_local_handling: BodyLocalHandlingPolicy,
    pub read_only_body_local_slot:
        Option<crate::mir::join_ir::lowering::common::body_local_slot::ReadOnlyBodyLocalSlot>,
    pub break_condition_node: ASTNode,
    pub is_loop_true_read_digits: bool,
    pub balanced_depth_scan_recipe: Option<
        crate::mir::join_ir::lowering::common::balanced_depth_scan_emitter::BalancedDepthScanRecipe,
    >,
    pub carrier_updates_override: Option<
        std::collections::BTreeMap<
            String,
            crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr,
        >,
    >,
    pub post_loop_early_return:
        Option<crate::mir::policies::post_loop_early_return_plan::PostLoopEarlyReturnPlan>,
}

pub(crate) struct LoopBreakPolicyRouterBox;

impl LoopBreakPolicyRouterBox {
    pub(crate) fn route(
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> Result<LoopBreakPolicyRouting, String> {
        // Route 1 (Phase 107): balanced depth-scan (return-in-loop normalization).
        match BalancedDepthScanPolicyBox::decide(condition, body) {
            PolicyDecision::Use(result) => {
                return Ok(LoopBreakPolicyRouting {
                    allowed_body_locals_for_conditions: result.allowed_body_locals_for_conditions,
                    body_local_handling: BodyLocalHandlingPolicy::SkipPromotion,
                    read_only_body_local_slot: None,
                    break_condition_node: result.break_condition_node,
                    is_loop_true_read_digits: false,
                    balanced_depth_scan_recipe: Some(result.derived_recipe),
                    carrier_updates_override: Some(result.carrier_updates_override),
                    post_loop_early_return: Some(result.post_loop_early_return),
                });
            }
            PolicyDecision::Reject(reason) => return Err(reason),
            PolicyDecision::None => {}
        }

        // Route 2 (Phase 105): loop(true) read-digits family + default break-cond SSOT.
        let break_routing = LoopBreakConditionPolicyRouterBox::route(condition, body)?;

        let read_only_body_local_slot =
            if break_routing.allowed_body_locals_for_conditions.is_empty() {
                None
            } else {
                Some(ReadOnlyBodyLocalSlotBox::extract_single(
                    &break_routing.allowed_body_locals_for_conditions,
                    body,
                )?)
            };

        Ok(LoopBreakPolicyRouting {
            allowed_body_locals_for_conditions: break_routing.allowed_body_locals_for_conditions,
            body_local_handling: BodyLocalHandlingPolicy::DefaultPromotion,
            read_only_body_local_slot,
            break_condition_node: break_routing.break_condition_node,
            is_loop_true_read_digits: break_routing.is_loop_true_read_digits,
            balanced_depth_scan_recipe: None,
            carrier_updates_override: None,
            post_loop_early_return: None,
        })
    }
}
