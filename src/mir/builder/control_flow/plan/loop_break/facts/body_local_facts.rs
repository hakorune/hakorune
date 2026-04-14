//! Phase 29ai P12: loop_break body-local promotion facts.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::body_local_facts_helpers::try_extract_loop_break_body_local_facts_inner;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) enum LoopBodyLocalShape {
    TrimSeg { s_var: String, i_var: String },
    DigitPos { digits_var: String, ch_var: String },
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBreakBodyLocalFacts {
    pub loop_var: String,
    pub body_local_var: String,
    pub break_uses_body_local: bool,
    pub shape: LoopBodyLocalShape,
}

pub(in crate::mir::builder) fn try_extract_loop_break_body_local_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakBodyLocalFacts>, Freeze> {
    try_extract_loop_break_body_local_facts_inner(condition, body)
}
