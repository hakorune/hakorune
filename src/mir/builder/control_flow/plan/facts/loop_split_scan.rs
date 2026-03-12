//! Split scan facts extraction

use super::loop_types::SplitScanFacts;
use super::scan_shapes::SplitScanShape;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(super) fn try_extract_split_scan_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<SplitScanFacts>, Freeze> {
    let Some((i_var, s_var, sep_var)) = try_extract_split_scan_condition_vars(condition) else {
        return Ok(None);
    };

    let Some(if_stmt) = body.iter().find_map(|stmt| match stmt {
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => Some((condition.as_ref(), then_body, else_body.as_ref())),
        _ => None,
    }) else {
        return Ok(None);
    };
    let (match_condition, then_body, else_body) = if_stmt;
    let Some(else_body) = else_body else {
        return Ok(None);
    };

    if !is_split_scan_match_condition(match_condition, &s_var, &i_var, &sep_var) {
        return Ok(None);
    }

    let Some((result_var, start_var)) = then_body
        .iter()
        .find_map(|stmt| extract_split_scan_then_push(stmt, &s_var, &i_var))
    else {
        return Err(Freeze::contract(
            "[joinir/phase29ab/split_scan/contract] splitscan contract: missing result push",
        ));
    };

    let has_start_update = then_body
        .iter()
        .any(|stmt| is_start_update(stmt, &start_var, &i_var, &sep_var));
    if !has_start_update {
        return Err(Freeze::contract(
            "[joinir/phase29ab/split_scan/contract] splitscan contract: missing start update",
        ));
    }

    let has_i_set_to_start = then_body
        .iter()
        .any(|stmt| is_i_set_to_start(stmt, &i_var, &start_var));
    if !has_i_set_to_start {
        return Err(Freeze::contract(
            "[joinir/phase29ab/split_scan/contract] splitscan contract: missing i = start",
        ));
    }

    let has_else_increment = else_body
        .iter()
        .any(|stmt| is_i_increment_by_one(stmt, &i_var));
    if !has_else_increment {
        return Err(Freeze::contract(
            "[joinir/phase29ab/split_scan/contract] splitscan contract: else i = i + 1 required",
        ));
    }

    Ok(Some(SplitScanFacts {
        s_var,
        sep_var,
        result_var,
        i_var,
        start_var,
        shape: SplitScanShape::Minimal,
    }))
}

fn try_extract_split_scan_condition_vars(condition: &ASTNode) -> Option<(String, String, String)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    let ASTNode::Variable { name: i_var, .. } = left.as_ref() else {
        return None;
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left: minus_left,
        right: minus_right,
        ..
    } = right.as_ref()
    else {
        return None;
    };
    let s_var = match minus_left.as_ref() {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } if method == "length" && arguments.is_empty() => match object.as_ref() {
            ASTNode::Variable { name, .. } => name.clone(),
            _ => return None,
        },
        _ => return None,
    };
    let sep_var = match minus_right.as_ref() {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } if method == "length" && arguments.is_empty() => match object.as_ref() {
            ASTNode::Variable { name, .. } => name.clone(),
            _ => return None,
        },
        _ => return None,
    };

    Some((i_var.clone(), s_var, sep_var))
}

fn is_split_scan_match_condition(
    condition: &ASTNode,
    s_var: &str,
    i_var: &str,
    sep_var: &str,
) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };

    let matches_left = is_substring_i_plus_sep_len(left.as_ref(), s_var, i_var, sep_var)
        && matches!(
            right.as_ref(),
            ASTNode::Variable { name, .. } if name == sep_var
        );
    let matches_right = is_substring_i_plus_sep_len(right.as_ref(), s_var, i_var, sep_var)
        && matches!(
            left.as_ref(),
            ASTNode::Variable { name, .. } if name == sep_var
        );
    matches_left || matches_right
}

fn is_substring_i_plus_sep_len(expr: &ASTNode, s_var: &str, i_var: &str, sep_var: &str) -> bool {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return false;
    };
    if method != "substring" || arguments.len() != 2 {
        return false;
    }
    let ASTNode::Variable { name: obj, .. } = object.as_ref() else {
        return false;
    };
    if obj != s_var {
        return false;
    }

    match &arguments[0] {
        ASTNode::Variable { name, .. } if name == i_var => {}
        _ => return false,
    }

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = &arguments[1]
    else {
        return false;
    };
    match left.as_ref() {
        ASTNode::Variable { name, .. } if name == i_var => {}
        _ => return false,
    }
    matches!(
        right.as_ref(),
        ASTNode::MethodCall { object, method, arguments, .. }
            if method == "length"
                && arguments.is_empty()
                && matches!(object.as_ref(), ASTNode::Variable { name, .. } if name == sep_var)
    )
}

fn extract_split_scan_then_push(
    stmt: &ASTNode,
    s_var: &str,
    i_var: &str,
) -> Option<(String, String)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = stmt
    else {
        return None;
    };
    if method != "push" || arguments.len() != 1 {
        return None;
    }
    let ASTNode::Variable {
        name: result_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    let ASTNode::MethodCall {
        object: substr_object,
        method: substr_method,
        arguments: substr_args,
        ..
    } = &arguments[0]
    else {
        return None;
    };
    if substr_method != "substring" || substr_args.len() != 2 {
        return None;
    }
    let ASTNode::Variable {
        name: substr_obj, ..
    } = substr_object.as_ref()
    else {
        return None;
    };
    if substr_obj != s_var {
        return None;
    }
    let ASTNode::Variable {
        name: start_var, ..
    } = &substr_args[0]
    else {
        return None;
    };
    match &substr_args[1] {
        ASTNode::Variable { name, .. } if name == i_var => {}
        _ => return None,
    }

    Some((result_var.clone(), start_var.clone()))
}

fn is_start_update(stmt: &ASTNode, start_var: &str, i_var: &str, sep_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    match target.as_ref() {
        ASTNode::Variable { name, .. } if name == start_var => {}
        _ => return false,
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return false;
    };
    match left.as_ref() {
        ASTNode::Variable { name, .. } if name == i_var => {}
        _ => return false,
    }
    matches!(
        right.as_ref(),
        ASTNode::MethodCall { object, method, arguments, .. }
            if method == "length"
                && arguments.is_empty()
                && matches!(object.as_ref(), ASTNode::Variable { name, .. } if name == sep_var)
    )
}

fn is_i_set_to_start(stmt: &ASTNode, i_var: &str, start_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == i_var)
        && matches!(value.as_ref(), ASTNode::Variable { name, .. } if name == start_var)
}

fn is_i_increment_by_one(stmt: &ASTNode, i_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let target_ok = matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == i_var);
    let value_ok = matches!(
        value.as_ref(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == i_var)
            && matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::Integer(1), .. })
    );
    target_ok && value_ok
}
