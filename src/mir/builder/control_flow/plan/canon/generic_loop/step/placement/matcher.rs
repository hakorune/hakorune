use crate::ast::ASTNode;

pub(super) fn collect_direct_step_indices(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Vec<usize> {
    let mut indices = Vec::new();
    let next_vars = collect_next_step_vars(body, loop_var, loop_increment);
    for (idx, stmt) in body.iter().enumerate() {
        if matches_loop_increment(stmt, loop_var, loop_increment)
            || matches_next_var_step(stmt, loop_var, &next_vars)
        {
            indices.push(idx);
        }
    }
    indices
}

pub(super) fn collect_conditional_step_indices(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> (Vec<usize>, Vec<usize>) {
    let mut continue_indices = Vec::new();
    let mut break_else_indices = Vec::new();
    for (idx, stmt) in body.iter().enumerate() {
        if is_continue_if_with_increment(stmt, loop_var, loop_increment) {
            continue_indices.push(idx);
        }
        if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
            break_else_indices.push(idx);
        }
    }
    (continue_indices, break_else_indices)
}

fn collect_next_step_vars(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Vec<String> {
    let mut vars = Vec::new();
    for stmt in body {
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        else {
            continue;
        };
        if variables.len() != 1 || initial_values.len() != 1 {
            continue;
        }
        let name = &variables[0];
        if name == loop_var {
            continue;
        }
        let Some(init) = &initial_values[0] else {
            continue;
        };
        if init.as_ref() == loop_increment {
            vars.push(name.clone());
        }
    }
    vars
}

fn matches_next_var_step(stmt: &ASTNode, loop_var: &str, next_vars: &[String]) -> bool {
    if next_vars.is_empty() {
        return false;
    }
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    let ASTNode::Variable {
        name: value_name, ..
    } = value.as_ref()
    else {
        return false;
    };
    next_vars.iter().any(|var| var == value_name)
}

pub(crate) fn matches_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    value.as_ref() == loop_increment
}

pub(crate) fn is_continue_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
        return false;
    }
    let mut saw_increment = false;
    for inner in then_body {
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if saw_increment {
                return false;
            }
            saw_increment = true;
        }
    }
    saw_increment
}

pub(crate) fn is_break_else_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(else_body) = else_body else {
        return false;
    };
    if else_body.len() != 1 || !matches!(else_body[0], ASTNode::Break { .. }) {
        return false;
    }
    if then_body.is_empty() {
        return false;
    }
    let mut saw_increment = false;
    for inner in then_body {
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if saw_increment {
                return false;
            }
            saw_increment = true;
            continue;
        }
        return false;
    }
    saw_increment
}
