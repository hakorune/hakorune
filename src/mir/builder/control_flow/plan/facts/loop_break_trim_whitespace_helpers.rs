//! Helper functions for trim-whitespace loop break extraction.

use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_common::{
    add, lit_bool, lit_int, lit_str, var,
};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::control_flow::plan::facts) enum SubstringDirection {
    Forward,
    Backward,
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_trim_header_condition(
    condition: &ASTNode,
) -> Option<(String, ASTNode, String, SubstringDirection, Vec<String>)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let (loop_var, bound, whitespace_expr) =
        if let Some(loop_var) = match_bound_condition(left.as_ref()) {
            (loop_var, left.as_ref().clone(), right.as_ref())
        } else if let Some(loop_var) = match_bound_condition(right.as_ref()) {
            (loop_var, right.as_ref().clone(), left.as_ref())
        } else {
            return None;
        };

    let mut haystack_var: Option<String> = None;
    let mut direction: Option<SubstringDirection> = None;
    let mut delimiters = Vec::new();
    if !collect_whitespace_terms(
        whitespace_expr,
        loop_var.as_str(),
        &mut haystack_var,
        &mut direction,
        &mut delimiters,
    ) {
        return None;
    }

    let haystack_var = haystack_var?;
    let direction = direction?;

    Some((loop_var, bound, haystack_var, direction, delimiters))
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_var(
    condition: &ASTNode,
) -> Option<String> {
    let ASTNode::BinaryOp {
        operator,
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

    match operator {
        BinaryOperator::Less | BinaryOperator::LessEqual => {
            if matches!(
                right.as_ref(),
                ASTNode::MethodCall { object, method, arguments, .. }
                    if method == "length"
                        && arguments.is_empty()
                        && matches!(object.as_ref(), ASTNode::Variable { .. })
            ) {
                return Some(loop_var.clone());
            }
        }
        BinaryOperator::GreaterEqual => {
            if matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    ..
                }
            ) {
                return Some(loop_var.clone());
            }
        }
        _ => {}
    }

    None
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_bound_condition(
    condition: &ASTNode,
) -> Option<String> {
    let ASTNode::BinaryOp { operator, left, .. } = condition else {
        return None;
    };
    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };
    match operator {
        BinaryOperator::Less
        | BinaryOperator::LessEqual
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEqual => Some(loop_var.clone()),
        _ => None,
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn collect_whitespace_terms(
    expr: &ASTNode,
    loop_var: &str,
    haystack_var: &mut Option<String>,
    direction: &mut Option<SubstringDirection>,
    delimiters: &mut Vec<String>,
) -> bool {
    match expr {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left,
            right,
            ..
        } => {
            collect_whitespace_terms(left.as_ref(), loop_var, haystack_var, direction, delimiters)
                && collect_whitespace_terms(
                    right.as_ref(),
                    loop_var,
                    haystack_var,
                    direction,
                    delimiters,
                )
        }
        _ => {
            let Some((delim, haystack, term_dir)) = match_substring_equals_literal(expr, loop_var)
            else {
                return false;
            };
            if let Some(existing) = haystack_var.as_ref() {
                if existing != &haystack {
                    return false;
                }
            } else {
                *haystack_var = Some(haystack);
            }
            if let Some(existing) = direction.as_ref() {
                if existing != &term_dir {
                    return false;
                }
            } else {
                *direction = Some(term_dir);
            }
            delimiters.push(delim);
            true
        }
    }
}

fn match_substring_equals_literal(
    expr: &ASTNode,
    loop_var: &str,
) -> Option<(String, String, SubstringDirection)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left,
        right,
        ..
    } = expr
    else {
        return None;
    };

    if let Some((lit, haystack, dir)) =
        match_substring_side(left.as_ref(), right.as_ref(), loop_var)
    {
        return Some((lit, haystack, dir));
    }
    if let Some((lit, haystack, dir)) =
        match_substring_side(right.as_ref(), left.as_ref(), loop_var)
    {
        return Some((lit, haystack, dir));
    }
    None
}

fn match_substring_side(
    substring_expr: &ASTNode,
    literal_expr: &ASTNode,
    loop_var: &str,
) -> Option<(String, String, SubstringDirection)> {
    let ASTNode::Literal { value, .. } = literal_expr else {
        return None;
    };
    let lit = literal_string(value)?;
    let (haystack, direction) = match_substring_at_loop_edge(substring_expr, loop_var)?;
    Some((lit, haystack, direction))
}

fn match_substring_at_loop_edge(
    expr: &ASTNode,
    loop_var: &str,
) -> Option<(String, SubstringDirection)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }
    let ASTNode::Variable { name: haystack, .. } = object.as_ref() else {
        return None;
    };
    if matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var)
        && matches_add_one(&arguments[1], loop_var)
    {
        return Some((haystack.clone(), SubstringDirection::Forward));
    }
    if matches_sub_one(&arguments[0], loop_var)
        && matches!(&arguments[1], ASTNode::Variable { name, .. } if name == loop_var)
    {
        return Some((haystack.clone(), SubstringDirection::Backward));
    }
    None
}

fn matches_add_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
        && matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        )
}

fn matches_sub_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
        && matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        )
}

pub(in crate::mir::builder::control_flow::plan::facts) fn build_not_whitespace_condition(
    loop_var: &str,
    haystack_var: &str,
    direction: SubstringDirection,
    delimiters: &[String],
) -> ASTNode {
    let mut iter = delimiters.iter();
    let first = iter
        .next()
        .map(|delim| build_mismatch_expr(loop_var, haystack_var, direction, delim))
        .unwrap_or_else(|| lit_bool(true));
    iter.fold(first, |acc, delim| ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left: Box::new(acc),
        right: Box::new(build_mismatch_expr(
            loop_var,
            haystack_var,
            direction,
            delim,
        )),
        span: Span::unknown(),
    })
}

fn build_mismatch_expr(
    loop_var: &str,
    haystack_var: &str,
    direction: SubstringDirection,
    delim: &str,
) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::NotEqual,
        left: Box::new(build_substring_expr(loop_var, haystack_var, direction)),
        right: Box::new(lit_str(delim)),
        span: Span::unknown(),
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_break_condition(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<ASTNode> {
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
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }

    let whitespace_call = match condition.as_ref() {
        ASTNode::UnaryOp {
            operator, operand, ..
        } => {
            use crate::ast::UnaryOperator;
            if !matches!(operator, UnaryOperator::Not) {
                return None;
            }
            match_is_whitespace_call(operand.as_ref(), loop_var)?
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            if matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    ..
                }
            ) {
                match_is_whitespace_call(left.as_ref(), loop_var)?
            } else if matches!(
                left.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    ..
                }
            ) {
                match_is_whitespace_call(right.as_ref(), loop_var)?
            } else {
                return None;
            }
        }
        _ => return None,
    };

    Some(ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(whitespace_call),
        right: Box::new(lit_bool(false)),
        span: Span::unknown(),
    })
}

pub(in crate::mir::builder::control_flow::plan::facts) fn extract_trim_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add | BinaryOperator::Subtract,
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

fn build_substring_expr(
    loop_var: &str,
    haystack_var: &str,
    direction: SubstringDirection,
) -> ASTNode {
    let (start, end) = match direction {
        SubstringDirection::Forward => (var(loop_var), add(var(loop_var), lit_int(1))),
        SubstringDirection::Backward => (sub(var(loop_var), lit_int(1)), var(loop_var)),
    };
    ASTNode::MethodCall {
        object: Box::new(var(haystack_var)),
        method: "substring".to_string(),
        arguments: vec![start, end],
        span: Span::unknown(),
    }
}

fn sub(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn literal_string(value: &LiteralValue) -> Option<String> {
    match value {
        LiteralValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

pub(in crate::mir::builder::control_flow::plan::facts) fn match_is_whitespace_call(
    expr: &ASTNode,
    loop_var: &str,
) -> Option<ASTNode> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "is_whitespace" || arguments.len() != 1 {
        return None;
    }
    let normalized_object = match object.as_ref() {
        ASTNode::This { .. } => ASTNode::This {
            span: Span::unknown(),
        },
        ASTNode::Me { .. } => ASTNode::This {
            span: Span::unknown(),
        },
        _ => return None,
    };
    if !matches_substring_at_loop_var(&arguments[0], loop_var) {
        return None;
    }
    Some(ASTNode::MethodCall {
        object: Box::new(normalized_object),
        method: method.clone(),
        arguments: arguments.clone(),
        span: Span::unknown(),
    })
}

pub(in crate::mir::builder::control_flow::plan::facts) fn matches_substring_at_loop_var(
    expr: &ASTNode,
    loop_var: &str,
) -> bool {
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
    if !matches!(object.as_ref(), ASTNode::Variable { .. }) {
        return false;
    }
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }
    match &arguments[1] {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
                && matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                )
        }
        _ => false,
    }
}
