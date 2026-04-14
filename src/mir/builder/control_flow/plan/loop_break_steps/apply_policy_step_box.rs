//! ApplyPolicyStepBox (Phase 106)
//!
//! Responsibility: apply policy routing for loop-break condition + allow-list.

use crate::ast::ASTNode;

use super::super::loop_break_policy_router::LoopBreakPolicyRouterBox;
use super::super::loop_break_prep_box::LoopBreakPrepInputs;
use super::apply_policy_inputs::build_loop_break_prep_inputs;
use super::gather_facts_step_box::LoopBreakPrepFacts;

pub(crate) struct ApplyPolicyStepBox;

impl ApplyPolicyStepBox {
    pub(crate) fn apply(
        condition: &ASTNode,
        body: &[ASTNode],
        facts: LoopBreakPrepFacts,
    ) -> Result<LoopBreakPrepInputs, String> {
        let policy = LoopBreakPolicyRouterBox::route(condition, body)?;
        Ok(build_loop_break_prep_inputs(facts, policy))
    }
}
