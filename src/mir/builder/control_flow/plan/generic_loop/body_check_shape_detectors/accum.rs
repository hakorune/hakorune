use super::super::body_check_extractors::extract_local_init_var;
use super::utils::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::canon::generic_loop::matches_loop_increment;
use crate::mir::builder::control_flow::plan::coreloop_body_contract::is_effect_only_stmt;

/// Matches the while cap accum sum pattern.
///
/// Shape: 2-4 statements - accum add, optional effect statements, loop increment
pub fn matches_while_cap_accum_sum_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if body.len() == 2 {
        if matches_accum_add_loop_var(&body[0], loop_var).is_none() {
            return false;
        }
        return matches_loop_increment(&body[1], loop_var, loop_increment);
    }
    if body.len() == 3 {
        if let Some(local_var) = extract_local_init_var(&body[0], loop_var) {
            if matches_accum_add_var(&body[1], &local_var).is_none() {
                return false;
            }
            return matches_loop_increment(&body[2], loop_var, loop_increment);
        }
        if matches_accum_add_loop_var(&body[0], loop_var).is_none() {
            return false;
        }
        if !is_effect_only_stmt(&body[1]) {
            return false;
        }
        return matches_loop_increment(&body[2], loop_var, loop_increment);
    }
    if body.len() == 4 {
        let Some(local_var) = extract_local_init_var(&body[0], loop_var) else {
            return false;
        };
        if matches_accum_add_var(&body[1], &local_var).is_none() {
            return false;
        }
        if !is_effect_only_stmt(&body[2]) {
            return false;
        }
        return matches_loop_increment(&body[3], loop_var, loop_increment);
    }
    false
}

/// Matches accum add loop var pattern.
pub fn matches_accum_add_loop_var(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    matches_accum_add_var(stmt, loop_var)
}

/// Matches accum add var pattern.
///
/// Shape: `target = target + var` or `target = var + target`
pub fn matches_accum_add_var(stmt: &ASTNode, rhs_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable {
        name: target_name, ..
    } = target.as_ref()
    else {
        return None;
    };
    if target_name == rhs_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    let is_accum =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == target_name);
    let is_rhs = |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == rhs_var);

    if (is_accum(left.as_ref()) && is_rhs(right.as_ref()))
        || (is_rhs(left.as_ref()) && is_accum(right.as_ref()))
    {
        return Some(target_name.clone());
    }

    None
}

/// Matches the int_to_str loop shape.
///
/// Shape: 4 statements - modulo by 10, effect local, effect assignment, division by 10
pub fn matches_div_countdown_by10_shape(
    body: &[ASTNode],
    loop_var: &str,
    _loop_increment: &ASTNode,
) -> bool {
    if body.len() != 4 {
        return false;
    }

    // Check body[0]: local d = v % 10 (10 fixed)
    if !matches_modulo_by_10_local(&body[0], loop_var) {
        return false;
    }

    // Check body[3]: v = v / 10 (10 fixed)
    if !matches_div_by_10_assignment(&body[3], loop_var) {
        return false;
    }

    // body[1], body[2] are effect statements (local, assign)
    // Keep checks minimal for this shape
    true
}

/// Matches `local <name> = <loop_var> % 10` pattern.
pub fn matches_modulo_by_10_local(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Local { initial_values, .. } = stmt else {
        return false;
    };
    if initial_values.len() != 1 {
        return false;
    }
    let Some(init) = &initial_values[0] else {
        return false;
    };
    matches_mod_ten_of_loop_var(init.as_ref(), loop_var)
}

/// Matches `<loop_var> = <loop_var> / 10` pattern.
pub fn matches_div_by_10_assignment(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };

    // target = loop_var
    let ASTNode::Variable {
        name: target_name, ..
    } = target.as_ref()
    else {
        return false;
    };
    if target_name != loop_var {
        return false;
    }

    // value = loop_var / 10
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return false;
    };
    if *operator != BinaryOperator::Divide {
        return false;
    }

    // left = loop_var
    let ASTNode::Variable {
        name: left_name, ..
    } = left.as_ref()
    else {
        return false;
    };
    if left_name != loop_var {
        return false;
    }

    // right = 10
    matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(10),
            ..
        }
    )
}
