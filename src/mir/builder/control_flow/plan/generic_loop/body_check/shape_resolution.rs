//! Body validation and shape resolution for generic loop analysis
//!
//! This module provides validation and shape detection for generic loop bodies.
//! Handles the resolution of multiple shape matches, including error handling
//! for overlapping shapes.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::GenericLoopV1ShapeId;

use super::super::body_check_extractors::collect_next_step_vars;
use super::super::body_check_shape_detectors::matches_usingcollector_line_scan_shape;
use super::super::facts::stmt_classifier::{
    is_break_else_effect_if, is_conditional_update_if, is_effect_if, is_effect_if_pure,
    is_exit_if, is_general_if_full, is_local_decl, is_simple_assignment,
    unsupported_stmt_detail,
};

// ============================================================================
// Body validation for v0 (more restrictive)
// ============================================================================

pub(in crate::mir::builder) fn body_is_generic_v0(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    check_body_generic_v0(body, loop_var, loop_increment).is_none()
}

/// Checks if body is valid for generic loop v0, returns failure reason if not
pub(in crate::mir::builder) fn check_body_generic_v0(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Option<&'static str> {
    let next_step_vars = collect_next_step_vars(body, loop_var);
    if matches_usingcollector_line_scan_shape(body, loop_var, loop_increment) {
        return None;
    }
    // Non planner_required mode keeps the legacy allowlist for release stability.
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment)
            || super::super::body_check_shape_detectors::matches_loop_var_assign_any(stmt, loop_var, &next_step_vars)
        {
            continue;
        }
        if is_simple_assignment(stmt, loop_var) {
            continue;
        }
        if is_local_decl(stmt, loop_var) {
            continue;
        }
        if is_exit_if(stmt) {
            continue;
        }
        if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if matches!(
            stmt,
            ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::Loop { .. }
        ) {
            continue;
        }
        return Some(unsupported_stmt_detail(stmt, loop_var));
    }
    None
}

// ============================================================================
// Body validation for v1 (more permissive)
// ============================================================================

pub(in crate::mir::builder) fn body_is_generic_v1(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let require_shape =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    matches!(
        check_body_generic_v1(body, loop_var, loop_increment, condition, require_shape),
        Ok(None)
    )
}

/// Checks if body is valid for generic loop v1, returns failure reason if not
pub(in crate::mir::builder) fn check_body_generic_v1(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
    require_shape: bool,
) -> Result<Option<&'static str>, Freeze> {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();

    let shape_id = detect_generic_loop_v1_shape(body, loop_var, loop_increment, condition)?;
    if shape_id.is_some() {
        return Ok(None);
    }
    if require_shape {
        return Ok(Some("generic_loop_v1:shape_required"));
    }

    let next_step_vars = collect_next_step_vars(body, loop_var);
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment)
            || super::super::body_check_shape_detectors::matches_loop_var_assign_any(stmt, loop_var, &next_step_vars)
        {
            continue;
        }
        if planner_required && matches!(stmt, ASTNode::Print { .. }) {
            continue;
        }
        if is_simple_assignment(stmt, loop_var) {
            continue;
        }
        if is_local_decl(stmt, loop_var) {
            continue;
        }
        if is_exit_if(stmt) {
            continue;
        }
        if is_continue_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_effect_if_pure(stmt) {
            continue;
        }
        if is_conditional_update_if(stmt, loop_var) {
            continue;
        }
        if planner_required && is_general_if_full(stmt, loop_var, loop_increment, 0) {
            continue;
        }
        if matches!(
            stmt,
            ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::Loop { .. }
        ) {
            continue;
        }
        return Ok(Some(unsupported_stmt_detail(stmt, loop_var)));
    }
    Ok(None)
}

pub(in crate::mir::builder) fn detect_generic_loop_v1_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> Result<Option<GenericLoopV1ShapeId>, Freeze> {
    let mut matches = Vec::new();
    if super::super::body_check_shape_detectors::matches_parse_block_expr_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::ParseBlockExpr);
    }
    if super::super::body_check_shape_detectors::matches_parse_map_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::ParseMap);
    }
    if super::super::body_check_shape_detectors::matches_peek_parse_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::PeekParse);
    }
    if super::super::body_check_shape_detectors::matches_rewriteknown_itoa_complex_step_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::RewriteKnownItoaComplexStep);
    }
    if super::super::body_check_shape_detectors::matches_rewriteknown_trim_and_methodcall_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::RewriteKnownTrimLoopCondAndMethodCall);
    }
    if super::super::body_check_shape_detectors::matches_while_cap_accum_sum_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::WhileCapAccumSum);
    }
    if super::super::body_check_shape_detectors::matches_decode_escapes_loop_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::DecodeEscapesLoop);
    }
    if super::super::body_check_shape_detectors::matches_scan_all_boxes_next_i_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::ScanAllBoxesNextI);
    }
    if super::super::body_check_shape_detectors::matches_scan_while_predicate_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::ScanWhilePredicate);
    }
    if super::super::body_check_shape_detectors::matches_effect_step_only_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::EffectStepOnly);
    }
    if super::super::body_check_shape_detectors::matches_div_countdown_by10_shape(body, loop_var, loop_increment) {
        matches.push(GenericLoopV1ShapeId::DivCountdownBy10);
    }

    // Parse program2 nested loop patterns
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturn);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturn);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_var_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturnVar);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_local_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturnLocal);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_var_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturnVar);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_local_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturnLocal);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_if_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseIfReturn);
    }

    resolve_v1_shape_matches(&matches)
}

// ============================================================================
// Shape resolution
// ============================================================================

/// Resolves multiple shape matches into a single shape or error.
///
/// # Arguments
/// * `matches` - Slice of detected shape IDs
///
/// # Returns
/// * `Ok(Some(shape))` - Exactly one shape matched
/// * `Ok(None)` - No shapes matched
/// * `Err(Freeze)` - Multiple shapes matched (overlap)
pub fn resolve_v1_shape_matches(
    matches: &[GenericLoopV1ShapeId],
) -> Result<Option<GenericLoopV1ShapeId>, Freeze> {
    use crate::mir::policies::generic_loop_overlap_policy::{
        classify_v1_shape_matches, V1ShapeMatch,
    };

    match classify_v1_shape_matches(matches) {
        V1ShapeMatch::None => Ok(None),
        V1ShapeMatch::Single(shape) => Ok(Some(shape)),
        V1ShapeMatch::Overlap(overlaps) => Err(Freeze::ambiguous(format!(
            "generic_loop_v1: shape overlap: {}",
            overlaps
                .iter()
                .map(|shape| shape.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }
}
