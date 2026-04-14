//! Local variable extraction helpers for loop_break facts.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_substring_char(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(String, String, ASTNode)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let ch_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable {
        name: haystack_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    // end = i + 1
    match &arguments[1] {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
                return None;
            }
            if !matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    ..
                }
            ) {
                return None;
            }
        }
        _ => return None,
    }

    Some((ch_var, haystack_var.clone(), expr.as_ref().clone()))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_local_this_index_of(
    stmt: &ASTNode,
    ch_var: &str,
) -> Option<(String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let d_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "index_of" || arguments.len() != 2 {
        return None;
    }
    if !matches!(object.as_ref(), ASTNode::This { .. } | ASTNode::Me { .. }) {
        return None;
    }
    let ASTNode::Variable {
        name: digits_var, ..
    } = &arguments[0]
    else {
        return None;
    };
    if !matches!(&arguments[1], ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }

    Some((digits_var.clone(), d_var))
}

pub(in crate::mir::builder::control_flow::plan) fn match_indexof_local(
    stmt: &ASTNode,
) -> Option<(String, String, String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let j_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "indexOf" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable {
        name: haystack_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(sep_lit),
        ..
    } = &arguments[0]
    else {
        return None;
    };
    let ASTNode::Variable { name: loop_var, .. } = &arguments[1] else {
        return None;
    };

    Some((
        j_var,
        haystack_var.clone(),
        sep_lit.clone(),
        loop_var.clone(),
    ))
}

pub(in crate::mir::builder::control_flow::plan) fn match_local_empty_string(
    stmt: &ASTNode,
) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let seg_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(value),
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if value != "" {
        return None;
    }
    Some(seg_var)
}

pub(in crate::mir::builder::control_flow::plan::facts) fn find_local_init_expr(
    body: &[ASTNode],
    name: &str,
) -> Option<ASTNode> {
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
        if variables[0] != name {
            continue;
        }
        let Some(expr) = initial_values[0].as_ref() else {
            return None;
        };
        return Some((*expr.clone()).clone());
    }
    None
}
