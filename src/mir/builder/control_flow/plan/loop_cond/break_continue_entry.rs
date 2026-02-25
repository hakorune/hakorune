//! Public entry points for loop_cond_break_continue facts extraction.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::loop_cond_shared::planner_required_for_loop_cond;
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::break_continue_helpers::matches_parse_string2_shape;
use super::break_continue_types::{LoopCondBreakContinueFacts, MAX_NESTED_LOOPS};
use super::break_continue_facts::try_extract_loop_cond_break_continue_facts_inner;

/// Extract loop_cond_break_continue facts from a loop(condition) { body }.
///
/// This is the main entry point for the facts extractor.
pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    let strict_or_dev =
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = planner_required_for_loop_cond();
    let allow_extended = planner_required || (strict_or_dev && matches_parse_string2_shape(body));
    let allow_nested = planner_required;
    let debug = crate::config::env::is_joinir_debug();
    try_extract_loop_cond_break_continue_facts_inner(
        condition,
        body,
        allow_nested,
        allow_extended,
        debug,
        MAX_NESTED_LOOPS,
        None,
    )
}

/// Extract loop_cond_break_continue facts with custom limits.
///
/// Used for testing and specialized extraction scenarios.
pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts_with_limit(
    condition: &ASTNode,
    body: &[ASTNode],
    max_nested_loops: usize,
    require_nested_loops: Option<usize>,
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    let strict_or_dev =
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = planner_required_for_loop_cond();
    let allow_extended = planner_required || (strict_or_dev && matches_parse_string2_shape(body));
    let allow_nested = planner_required;
    let debug = crate::config::env::is_joinir_debug();
    try_extract_loop_cond_break_continue_facts_inner(
        condition,
        body,
        allow_nested,
        allow_extended,
        debug,
        max_nested_loops,
        require_nested_loops,
    )
}

/// Extract loop_cond_break_continue facts for nested loop contexts.
///
/// This variant disables further nested loop detection to prevent infinite recursion.
pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts_for_nested(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    if !planner_required_for_loop_cond() {
        return Ok(None);
    }
    let debug = crate::config::env::is_joinir_debug();
    try_extract_loop_cond_break_continue_facts_inner(
        condition,
        body,
        false,
        true,
        debug,
        MAX_NESTED_LOOPS,
        None,
    )
}
