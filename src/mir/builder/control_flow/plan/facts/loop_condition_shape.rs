//! Condition shape extraction for loop analysis

use super::scan_shapes::{ConditionShape, LengthMethod};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(in crate::mir::builder) fn try_extract_condition_shape(
    condition: &ASTNode,
) -> Result<Option<ConditionShape>, Freeze> {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = condition
    else {
        return Ok(None);
    };

    match operator {
        BinaryOperator::Less => {
            let ASTNode::Variable { name: idx_var, .. } = left.as_ref() else {
                return Ok(None);
            };

            if let Some((haystack_var, method)) = match_length_call(right.as_ref()) {
                return Ok(Some(ConditionShape::VarLessLength {
                    idx_var: idx_var.clone(),
                    haystack_var,
                    method,
                }));
            }

            let ASTNode::Literal { value, .. } = right.as_ref() else {
                return Ok(None);
            };
            let LiteralValue::Integer(bound) = value else {
                return Ok(None);
            };
            Ok(Some(ConditionShape::VarLessLiteral {
                idx_var: idx_var.clone(),
                bound: *bound,
            }))
        }
        BinaryOperator::LessEqual => {
            let ASTNode::Variable { name: idx_var, .. } = left.as_ref() else {
                return Ok(None);
            };

            let ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: minus_left,
                right: minus_right,
                ..
            } = right.as_ref()
            else {
                return Ok(None);
            };
            let Some((haystack_var, haystack_method)) = match_length_call(minus_left.as_ref())
            else {
                return Ok(None);
            };
            let Some((needle_var, needle_method)) = match_length_call(minus_right.as_ref()) else {
                return Ok(None);
            };

            Ok(Some(ConditionShape::VarLessEqualLengthMinusNeedle {
                idx_var: idx_var.clone(),
                haystack_var,
                needle_var,
                haystack_method,
                needle_method,
            }))
        }
        BinaryOperator::GreaterEqual => {
            let ASTNode::Variable { name: idx_var, .. } = left.as_ref() else {
                return Ok(None);
            };
            let ASTNode::Literal { value, .. } = right.as_ref() else {
                return Ok(None);
            };
            if !matches!(value, LiteralValue::Integer(0)) {
                return Ok(None);
            }

            Ok(Some(ConditionShape::VarGreaterEqualZero {
                idx_var: idx_var.clone(),
            }))
        }
        _ => Ok(None),
    }
}

pub(super) fn match_length_call(expr: &ASTNode) -> Option<(String, LengthMethod)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if !arguments.is_empty() {
        return None;
    }
    let method = match method.as_str() {
        "length" => LengthMethod::Length,
        "size" => LengthMethod::Size,
        _ => return None,
    };
    let ASTNode::Variable { name, .. } = object.as_ref() else {
        return None;
    };
    Some((name.clone(), method))
}
