//! Phase 29ai P11: Pattern2BreakFacts type definitions

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern2BreakFacts {
    pub loop_var: String,
    pub carrier_var: String,
    pub loop_condition: ASTNode,
    pub break_condition: ASTNode,
    pub carrier_update_in_break: Option<ASTNode>,
    pub carrier_update_in_body: ASTNode,
    pub loop_increment: ASTNode,
    pub step_placement: LoopBreakStepPlacement,
}
