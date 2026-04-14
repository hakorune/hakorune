use crate::ast::{ASTNode, BinaryOperator};

pub(super) fn release_enabled() -> bool {
    true
}

fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

pub(super) fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_name(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}

pub(super) fn declares_single_local(stmt: &ASTNode) -> Option<String> {
    let ASTNode::Local { variables, .. } = stmt else {
        return None;
    };
    if variables.len() != 1 {
        return None;
    }
    Some(variables[0].clone())
}

pub(super) fn extract_step_var_from_tail(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    Some(as_var_name(value.as_ref())?.to_string())
}
