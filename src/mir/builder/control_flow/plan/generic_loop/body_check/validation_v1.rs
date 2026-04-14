use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    is_break_else_if_with_increment, is_continue_if_with_increment, matches_loop_increment,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::super::body_check_extractors::collect_next_step_vars;
use super::super::facts::stmt_classifier::{
    is_break_else_effect_if, is_conditional_update_if, is_effect_if_pure, is_exit_if,
    is_general_if_full, is_local_decl, is_simple_assignment, unsupported_stmt_detail,
};
use super::shape_detection::detect_generic_loop_v1_shape;

pub(in crate::mir::builder) fn body_is_generic_v1(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> bool {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let require_shape = strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    matches!(
        check_body_generic_v1(body, loop_var, loop_increment, condition, require_shape),
        Ok(None)
    )
}

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
            || super::super::body_check_shape_detectors::matches_loop_var_assign_any(
                stmt,
                loop_var,
                &next_step_vars,
            )
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
