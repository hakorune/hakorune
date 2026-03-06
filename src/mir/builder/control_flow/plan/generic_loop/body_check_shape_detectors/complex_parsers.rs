use crate::ast::ASTNode;
use super::super::facts::stmt_classifier::{
    is_exit_if, is_general_if_full, is_local_decl, is_simple_assignment,
};
use crate::mir::builder::control_flow::plan::canon::generic_loop::matches_loop_increment;

/// Matches the peek parse pattern.
///
/// Complex nested if pattern for parsing.
pub fn matches_peek_parse_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 5 {
        return false;
    }
    if !is_simple_assignment(&body[0], loop_var) {
        return false;
    }
    if !is_exit_if(&body[1]) {
        return false;
    }
    if !is_exit_if(&body[2]) {
        return false;
    }
    if !is_general_if_full(&body[3], loop_var, loop_increment, 0) {
        return false;
    }
    if !matches_loop_increment(&body[4], loop_var, loop_increment) {
        return false;
    }

    let ASTNode::If {
        else_body: Some(outer_else),
        ..
    } = &body[3]
    else {
        return false;
    };
    if outer_else.len() != 1 {
        return false;
    }
    let ASTNode::If {
        then_body: inner_then,
        else_body: Some(inner_else),
        ..
    } = &outer_else[0]
    else {
        return false;
    };
    if inner_then.len() != 1 || !is_simple_assignment(&inner_then[0], loop_var) {
        return false;
    }
    if inner_else.len() != 11 {
        return false;
    }

    let expects = [
        "Local", "Assignment", "Assignment", "If", "Assignment", "Local", "If", "Local", "If",
        "Assignment", "Assignment",
    ];
    for (idx, expect) in expects.iter().enumerate() {
        let stmt = &inner_else[idx];
        let ok = match *expect {
            "Local" => is_local_decl(stmt, loop_var),
            "Assignment" => is_simple_assignment(stmt, loop_var),
            "If" => is_general_if_full(stmt, loop_var, loop_increment, 0),
            _ => false,
        };
        if !ok {
            return false;
        }
    }

    true
}

/// Matches the decode escapes loop shape.
///
/// Shape: 6 statements - two locals, two ifs, non-loop-var assignment, loop increment
pub fn matches_decode_escapes_loop_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 6 {
        return false;
    }
    if !matches!(body[0], ASTNode::Local { .. }) {
        return false;
    }
    if !matches!(body[1], ASTNode::Local { .. }) {
        return false;
    }
    if !matches!(body[2], ASTNode::If { .. }) {
        return false;
    }
    if !matches!(body[3], ASTNode::If { .. }) {
        return false;
    }
    if let ASTNode::Assignment { target, .. } = &body[4] {
        if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
            return false;
        }
    } else {
        return false;
    }
    matches_loop_increment(&body[5], loop_var, loop_increment)
}

/// Matches the parse map pattern.
///
/// Complex nested if pattern for map parsing.
pub fn matches_parse_map_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 7 {
        return false;
    }
    if !is_simple_assignment(&body[0], loop_var) {
        return false;
    }
    if !is_exit_if(&body[1]) {
        return false;
    }
    if !is_exit_if(&body[2]) {
        return false;
    }
    if !is_general_if_full(&body[3], loop_var, loop_increment, 0) {
        return false;
    }
    if !is_simple_assignment(&body[4], loop_var) {
        return false;
    }
    if !is_general_if_full(&body[5], loop_var, loop_increment, 0) {
        return false;
    }
    if !matches_loop_increment(&body[6], loop_var, loop_increment) {
        return false;
    }

    let ASTNode::If {
        then_body,
        else_body: Some(else_body),
        ..
    } = &body[3]
    else {
        return false;
    };
    if then_body.len() != 1 || !is_simple_assignment(&then_body[0], loop_var) {
        return false;
    }
    if else_body.len() != 12 {
        return false;
    }

    let expects = [
        "Local", "Local", "Assignment", "Assignment", "If", "Assignment", "Local", "Assignment",
        "If", "Assignment", "Assignment", "If",
    ];
    for (idx, expect) in expects.iter().enumerate() {
        let stmt = &else_body[idx];
        let ok = match *expect {
            "Local" => is_local_decl(stmt, loop_var),
            "Assignment" => is_simple_assignment(stmt, loop_var),
            "If" => is_general_if_full(stmt, loop_var, loop_increment, 0),
            _ => false,
        };
        if !ok {
            return false;
        }
    }

    true
}

/// Matches the parse block expr pattern.
///
/// Complex nested if pattern for block expression parsing.
pub fn matches_parse_block_expr_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() != 11 {
        return false;
    }
    if !is_simple_assignment(&body[0], loop_var) {
        return false;
    }
    if !is_exit_if(&body[1]) {
        return false;
    }
    if !is_exit_if(&body[2]) {
        return false;
    }
    if !is_local_decl(&body[3], loop_var) {
        return false;
    }
    if !is_local_decl(&body[4], loop_var) {
        return false;
    }
    if !is_simple_assignment(&body[5], loop_var) {
        return false;
    }
    if !is_general_if_full(&body[6], loop_var, loop_increment, 0) {
        return false;
    }
    if !is_simple_assignment(&body[7], loop_var) {
        return false;
    }
    if !is_simple_assignment(&body[8], loop_var) {
        return false;
    }
    if !is_general_if_full(&body[9], loop_var, loop_increment, 0) {
        return false;
    }
    if !matches_loop_increment(&body[10], loop_var, loop_increment) {
        return false;
    }

    let ASTNode::If {
        then_body: outer_then,
        else_body: None,
        ..
    } = &body[6]
    else {
        return false;
    };
    if outer_then.len() != 2 {
        return false;
    }
    if !is_general_if_full(&outer_then[0], loop_var, loop_increment, 0) {
        return false;
    }
    if !is_simple_assignment(&outer_then[1], loop_var) {
        return false;
    }

    let ASTNode::If {
        then_body: progress_then,
        else_body: None,
        ..
    } = &body[9]
    else {
        return false;
    };
    if progress_then.len() != 1 || !is_simple_assignment(&progress_then[0], loop_var) {
        return false;
    }

    true
}
