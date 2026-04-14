//! loop_break facts namespace entry.
//!
//! This is a structural entry-point only: the concrete extractors still live under
//! `plan/facts/` until the larger loop_break facts migration is scheduled.
//!
//! Current source modules:
//! - `loop_break/facts/types.rs`
//! - `loop_break/facts/helpers_common.rs`
//! - `loop_break/facts/helpers_break_if.rs`
//! - `loop_break/facts/helpers_condition.rs`
//! - `loop_break/facts/helpers_loop.rs`
//! - `loop_break/facts/helpers_local.rs`
//! - `loop_break/facts/body_local_subset.rs`
//! - `loop_break/facts/parse_integer.rs`
//! - `loop_break/facts/read_digits.rs`
//! - `loop_break/facts/realworld.rs`
//! - `loop_break/facts/step_before_break.rs`
//! - `loop_break/facts/trim_whitespace*.rs`
//! - `plan/facts/loop_break_core.rs`
//! - `plan/facts/loop_break_body_local_facts.rs`
//! - `plan/facts/loop_break_helpers*.rs`

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

mod types;
pub(in crate::mir::builder::control_flow::plan) mod body_local_subset;
pub(in crate::mir::builder::control_flow::plan) mod helpers_break_if;
pub(in crate::mir::builder::control_flow::plan) mod helpers_condition;
pub(in crate::mir::builder::control_flow::plan) mod helpers_common;
pub(in crate::mir::builder::control_flow::plan) mod helpers_local;
pub(in crate::mir::builder::control_flow::plan) mod helpers_loop;
pub(in crate::mir::builder::control_flow::plan) mod parse_integer;
pub(in crate::mir::builder::control_flow::plan) mod read_digits;
pub(in crate::mir::builder::control_flow::plan) mod realworld;
pub(in crate::mir::builder::control_flow::plan) mod step_before_break;
pub(in crate::mir::builder::control_flow::plan) mod trim_whitespace;
pub(in crate::mir::builder::control_flow::plan) mod trim_whitespace_helpers;

pub(in crate::mir::builder) use types::LoopBreakFacts;
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
