//! loop_break facts namespace entry.
//!
//! This is a structural entry-point only: the concrete extractors still live under
//! `plan/facts/` until the larger loop_break facts migration is scheduled.
//!
//! Current source modules:
//! - `plan/facts/loop_break_core.rs`
//! - `plan/facts/loop_break_body_local_facts.rs`
//! - `plan/facts/loop_break_{parse_integer,read_digits,realworld,step_before_break}.rs`
//! - `plan/facts/loop_break_trim_whitespace*.rs`
//! - `plan/facts/loop_break_helpers*.rs`

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(in crate::mir::builder) type LoopBreakFacts =
    crate::mir::builder::control_flow::plan::facts::LoopBreakFacts;
pub(in crate::mir::builder) type LoopBodyLocalShape =
    crate::mir::builder::control_flow::plan::facts::loop_break_body_local_facts::LoopBodyLocalShape;
pub(in crate::mir::builder) type LoopBreakBodyLocalFacts =
    crate::mir::builder::control_flow::plan::facts::loop_break_body_local_facts::LoopBreakBodyLocalFacts;

pub(in crate::mir::builder) fn try_extract_loop_break_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakFacts>, Freeze> {
    crate::mir::builder::control_flow::plan::facts::loop_break_core::try_extract_loop_break_facts(
        condition,
        body,
    )
}

pub(in crate::mir::builder) fn try_extract_loop_break_body_local_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakBodyLocalFacts>, Freeze> {
    crate::mir::builder::control_flow::plan::facts::loop_break_body_local_facts::try_extract_loop_break_body_local_facts(
        condition,
        body,
    )
}
