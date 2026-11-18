//! Split scan facts extraction

use super::loop_types::SplitScanFacts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct SplitScanSourceTopologyV1 {
    matched_if: LoopSourceBodySiteV1,
}

impl SplitScanSourceTopologyV1 {
    pub(in crate::mir::builder) fn has_scope_box_lineage(&self) -> bool {
        !self.matched_if.scope_box_children().is_empty()
    }
}

pub(super) fn try_extract_split_scan_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    _source_projection: &LoopSourceProjectionV1,
) -> Result<Option<SplitScanFacts>, Freeze> {
    let Some((i_var, s_var, sep_var)) = try_extract_split_scan_condition_vars(condition) else {
        return Ok(None);
    };

    let Some((_matched_index, if_stmt)) =
        body.iter()
            .enumerate()
            .find_map(|(index, stmt)| match stmt {
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => Some((index, (condition.as_ref(), then_body, else_body.as_ref()))),
                _ => None,
            })
    else {
        return Ok(None);
    };
    let (match_condition, _then_body, else_body) = if_stmt;
    let Some(_else_body) = else_body else {
        return Ok(None);
    };

    if !is_split_scan_match_condition(match_condition, &s_var, &i_var, &sep_var) {
        return Ok(None);
    }

    Ok(None)
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
