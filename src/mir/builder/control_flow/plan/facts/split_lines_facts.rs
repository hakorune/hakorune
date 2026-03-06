//! Phase 29bg P1: SplitLinesFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    has_break_statement, has_continue_statement, has_return_statement,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SplitLinesFacts {
    pub loop_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub haystack_var: String,
    pub result_var: String,
    pub start_var: String,
    pub delimiter_lit: String,
}

pub(in crate::mir::builder) fn try_extract_split_lines_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<SplitLinesFacts>, Freeze> {
    let Some(loop_var) = match_loop_condition(condition) else {
        return Ok(None);
    };

    if has_break_statement(body) || has_continue_statement(body) || has_return_statement(body) {
        return Ok(None);
    }

    let flat_body = flatten_scope_boxes(body);

    let mut ch_info: Option<(String, String)> = None;
    let mut if_stmt: Option<&ASTNode> = None;
    let mut loop_increment: Option<ASTNode> = None;

    for stmt in &flat_body {
        if ch_info.is_none() {
            if let Some(info) = match_local_substring(stmt, &loop_var) {
                ch_info = Some(info);
            }
        }
        if if_stmt.is_none() {
            if matches!(stmt, ASTNode::If { .. }) {
                if_stmt = Some(*stmt);
            }
        }
        if loop_increment.is_none() {
            if let Some(stmt) = match_loop_increment_stmt(stmt, &loop_var) {
                loop_increment = Some(stmt);
            }
        }
    }

    let Some(if_stmt) = if_stmt else {
        return Ok(None);
    };
    let ch_var = ch_info.as_ref().map(|(ch_var, _)| ch_var.as_str());
    let Some((delimiter_lit, result_var, start_var, haystack_var)) =
        match_delim_then(if_stmt, &loop_var, ch_var)
    else {
        return Ok(None);
    };

    if let Some((_, ch_haystack)) = ch_info {
        if ch_haystack != haystack_var {
            return Ok(None);
        }
    }

    let Some(loop_increment) = loop_increment else {
        return Ok(None);
    };

    Ok(Some(SplitLinesFacts {
        loop_var,
        loop_condition: condition.clone(),
        loop_increment,
        haystack_var,
        result_var,
        start_var,
        delimiter_lit,
    }))
}

fn flatten_scope_boxes<'a>(body: &'a [ASTNode]) -> Vec<&'a ASTNode> {
    let mut out = Vec::new();
    fn push_node<'a>(node: &'a ASTNode, out: &mut Vec<&'a ASTNode>) {
        match node {
            ASTNode::ScopeBox { body, .. } => {
                for inner in body {
                    push_node(inner, out);
                }
            }
            _ => out.push(node),
        }
    }
    for stmt in body {
        push_node(stmt, &mut out);
    }
    out
}

fn match_loop_condition(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(right.as_ref(), ASTNode::Variable { .. }) {
        return None;
    }

    Some(loop_var.clone())
}

fn match_local_substring(stmt: &ASTNode, loop_var: &str) -> Option<(String, String)> {
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
    let Some(init) = &initial_values[0] else {
        return None;
    };

    let (object, method, arguments) = match_method_call(init.as_ref())?;
    if method != "substring" || arguments.len() != 2 {
        return None;
    }

    let haystack_var = match_substring_call(object, arguments, loop_var)?;

    Some((ch_var, haystack_var))
}

fn match_delim_then(
    stmt: &ASTNode,
    loop_var: &str,
    ch_var: Option<&str>,
) -> Option<(String, String, String, String)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }
    let flat_then = flatten_scope_boxes(then_body);
    if flat_then.len() != 2 {
        return None;
    }

    let (delimiter_lit, cond_haystack) = match_delim_condition(condition, ch_var, loop_var)?;

    let mut result_var: Option<String> = None;
    let mut start_var: Option<String> = None;
    let mut push_haystack: Option<String> = None;

    for then_stmt in &flat_then {
        if result_var.is_none() {
            if let Some((result, start, haystack)) =
                match_push_stmt(then_stmt, loop_var)
            {
                result_var = Some(result);
                start_var = Some(start);
                push_haystack = Some(haystack);
                continue;
            }
        }
        if start_var.is_none() {
            if let Some(start) = match_start_update_stmt(then_stmt, loop_var) {
                start_var = Some(start);
                continue;
            }
        }
    }

    let result_var = result_var?;
    let start_var = start_var?;
    let push_haystack = push_haystack?;

    if !flat_then
        .iter()
        .any(|stmt| match_start_update_stmt(stmt, loop_var).is_some())
    {
        return None;
    }

    if !flat_then.iter().any(|stmt| {
        match_push_stmt(stmt, loop_var)
            .map(|(_, start, _)| start == start_var)
            .unwrap_or(false)
    }) {
        return None;
    }

    if let Some(cond_haystack) = cond_haystack.as_ref() {
        if cond_haystack != &push_haystack {
            return None;
        }
    }

    Some((delimiter_lit, result_var, start_var, push_haystack))
}

fn match_delim_condition(
    condition: &ASTNode,
    ch_var: Option<&str>,
    loop_var: &str,
) -> Option<(String, Option<String>)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    if let Some(ch_var) = ch_var {
        if let Some(lit) = match_var_literal(left.as_ref(), right.as_ref(), ch_var) {
            return Some((lit, None));
        }
        if let Some(lit) = match_var_literal(right.as_ref(), left.as_ref(), ch_var) {
            return Some((lit, None));
        }
    }

    if let Some((lit, haystack)) = match_substring_literal(left.as_ref(), right.as_ref(), loop_var)
    {
        return Some((lit, Some(haystack)));
    }
    if let Some((lit, haystack)) = match_substring_literal(right.as_ref(), left.as_ref(), loop_var)
    {
        return Some((lit, Some(haystack)));
    }

    None
}

fn match_push_stmt(stmt: &ASTNode, loop_var: &str) -> Option<(String, String, String)> {
    let (object, method, arguments) = match_method_call(stmt)?;
    if method != "push" || arguments.len() != 1 {
        return None;
    }
    let ASTNode::Variable { name: result_var, .. } = object else {
        return None;
    };

    let arg = &arguments[0];
    let (substr_object, substr_method, substr_args) = match_method_call(arg)?;
    if substr_method != "substring" || substr_args.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: haystack, .. } = substr_object else {
        return None;
    };
    let ASTNode::Variable { name: start_var, .. } = &substr_args[0] else {
        return None;
    };
    if !matches!(&substr_args[1], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }

    Some((result_var.clone(), start_var.clone(), haystack.clone()))
}

fn match_start_update_stmt(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name: start_var, .. } = target.as_ref() else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
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

    Some(start_var.clone())
}

fn match_loop_increment_stmt(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
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

    Some(value.as_ref().clone())
}

fn match_method_call<'a>(
    node: &'a ASTNode,
) -> Option<(&'a ASTNode, &'a str, &'a [ASTNode])> {
    match node {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => Some((object.as_ref(), method.as_str(), arguments.as_slice())),
        ASTNode::Call { callee, arguments, .. } => match callee.as_ref() {
            ASTNode::FieldAccess { object, field, .. } => {
                Some((object.as_ref(), field.as_str(), arguments.as_slice()))
            }
            _ => None,
        },
        _ => None,
    }
}

fn match_substring_call(object: &ASTNode, arguments: &[ASTNode], loop_var: &str) -> Option<String> {
    let ASTNode::Variable { name: haystack_var, .. } = object else {
        return None;
    };
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !match_add_var_lit(&arguments[1], loop_var, 1) {
        return None;
    }
    Some(haystack_var.clone())
}

fn match_add_var_lit(expr: &ASTNode, var: &str, lit: i64) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let (var_side, lit_side) = match (left.as_ref(), right.as_ref()) {
        (ASTNode::Variable { name, .. }, ASTNode::Literal { value, .. }) => (name, value),
        (ASTNode::Literal { value, .. }, ASTNode::Variable { name, .. }) => (name, value),
        _ => return false,
    };
    matches!(lit_side, LiteralValue::Integer(v) if *v == lit) && var_side == var
}

fn match_substring_literal(
    lhs: &ASTNode,
    rhs: &ASTNode,
    loop_var: &str,
) -> Option<(String, String)> {
    let lit = match rhs {
        ASTNode::Literal { value, .. } => literal_string(value)?,
        _ => return None,
    };
    let (object, method, arguments) = match_method_call(lhs)?;
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let haystack = match_substring_call(object, arguments, loop_var)?;
    Some((lit, haystack))
}

fn match_var_literal(lhs: &ASTNode, rhs: &ASTNode, name: &str) -> Option<String> {
    if !matches!(lhs, ASTNode::Variable { name: var, .. } if var == name) {
        return None;
    }
    let ASTNode::Literal { value, .. } = rhs else {
        return None;
    };
    literal_string(value)
}

fn literal_string(value: &LiteralValue) -> Option<String> {
    match value {
        LiteralValue::String(s) => Some(s.clone()),
        _ => None,
    }
}
