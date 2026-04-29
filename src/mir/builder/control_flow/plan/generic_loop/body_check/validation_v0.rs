use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::generic_loop::{
    is_break_else_if_with_increment, matches_loop_increment,
};

use super::super::body_check_extractors::collect_next_step_vars;
use super::super::body_check_shape_detectors::matches_usingcollector_line_scan_shape;
use super::super::facts::stmt_classifier::{
    is_break_else_effect_if, is_effect_if, is_exit_if, is_local_decl, is_simple_assignment,
    unsupported_stmt_detail,
};

pub(in crate::mir::builder) fn check_body_generic_v0(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Option<&'static str> {
    let next_step_vars = collect_next_step_vars(body, loop_var);
    if matches_usingcollector_line_scan_shape(body, loop_var, loop_increment) {
        return None;
    }

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
