use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};

pub(in crate::mir::builder) fn collect_candidates_from_top_level_comparison(
    condition: &ASTNode,
) -> Option<Vec<String>> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    if !is_supported_comparison_operator(operator) {
        return None;
    }
    let mut candidates = Vec::new();
    if let Some(name) = extract_var_candidate(left) {
        push_candidate(&mut candidates, name);
    }
    if let Some(name) = extract_var_candidate(right) {
        push_candidate(&mut candidates, name);
    }
    Some(candidates)
}

pub(in crate::mir::builder) fn collect_candidates_from_condition(
    expr: &ASTNode,
    candidates: &mut Vec<String>,
) -> bool {
    match expr {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => collect_candidates_from_condition(operand, candidates),
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } => true,
        ASTNode::BinaryOp {
            operator: BinaryOperator::And | BinaryOperator::Or,
            left,
            right,
            ..
        } => {
            collect_candidates_from_condition(left, candidates)
                && collect_candidates_from_condition(right, candidates)
        }
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            if !is_supported_comparison_operator(operator) {
                return false;
            }
            if let Some(name) = extract_var_candidate(left) {
                push_candidate(candidates, name);
            }
            if let Some(name) = extract_var_candidate(right) {
                push_candidate(candidates, name);
            }
            true
        }
        _ => false,
    }
}

pub(in crate::mir::builder) fn extract_var_candidate(expr: &ASTNode) -> Option<String> {
    match expr {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::Add | BinaryOperator::Subtract => {
                if let ASTNode::Variable { name, .. } = left.as_ref() {
                    return Some(name.clone());
                }
                if let ASTNode::Variable { name, .. } = right.as_ref() {
                    return Some(name.clone());
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

pub(in crate::mir::builder) fn is_supported_comparison_operator(
    operator: &BinaryOperator,
) -> bool {
    matches!(
        operator,
        BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
    )
}

fn push_candidate(candidates: &mut Vec<String>, name: String) {
    if !candidates.iter().any(|existing| existing == &name) {
        candidates.push(name);
    }
}
