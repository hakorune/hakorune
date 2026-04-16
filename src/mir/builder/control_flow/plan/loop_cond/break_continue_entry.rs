//! Public entry points for loop_cond_break_continue facts extraction.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::ast_feature_extractor::{
    detect_break_in_body, detect_continue_in_body,
};
use crate::mir::builder::control_flow::plan::loop_cond::planner_gate::planner_required_for_loop_cond;
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::break_continue_facts::try_extract_loop_cond_break_continue_facts_inner;
use super::break_continue_helpers::matches_parse_string2_shape;
use super::break_continue_types::{LoopCondBreakContinueFacts, MAX_NESTED_LOOPS};

fn allow_extended_for_loop_cond_break_continue(
    body: &[ASTNode],
    strict_or_dev: bool,
    planner_required: bool,
) -> bool {
    let has_return_stmt = body.iter().any(ASTNode::contains_return_stmt);
    planner_required || (strict_or_dev && matches_parse_string2_shape(body)) || has_return_stmt
}

fn allow_nested_for_loop_cond_break_continue(body: &[ASTNode], planner_required: bool) -> bool {
    if planner_required {
        return true;
    }
    // Release route: enable nested-loop observation only for explicit
    // break+continue loops (exit-driven shapes).
    detect_break_in_body(body) && detect_continue_in_body(body)
}

/// Extract loop_cond_break_continue facts from a loop(condition) { body }.
///
/// This is the main entry point for the facts extractor.
pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required = planner_required_for_loop_cond();
    let allow_extended =
        allow_extended_for_loop_cond_break_continue(body, strict_or_dev, planner_required);
    let allow_nested = allow_nested_for_loop_cond_break_continue(body, planner_required);
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
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required = planner_required_for_loop_cond();
    let allow_extended =
        allow_extended_for_loop_cond_break_continue(body, strict_or_dev, planner_required);
    let allow_nested = allow_nested_for_loop_cond_break_continue(body, planner_required);
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
