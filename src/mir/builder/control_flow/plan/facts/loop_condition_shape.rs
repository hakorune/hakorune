//! Condition shape extraction for loop analysis

use super::scan_shapes::{ConditionShape, LengthMethod};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::{BoundExpr, CmpOp};

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
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            };
            let LiteralValue::Integer(bound) = value else {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            };
            Ok(Some(ConditionShape::VarLessLiteral {
                idx_var: idx_var.clone(),
                bound: *bound,
            }))
        }
        BinaryOperator::LessEqual => {
            let ASTNode::Variable { name: idx_var, .. } = left.as_ref() else {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            };

            let ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: minus_left,
                right: minus_right,
                ..
            } = right.as_ref()
            else {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
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
        BinaryOperator::Greater => Ok(numeric_compare_shape(
            operator,
            left.as_ref(),
            right.as_ref(),
        )),
        BinaryOperator::Equal | BinaryOperator::NotEqual => Ok(numeric_compare_shape(
            operator,
            left.as_ref(),
            right.as_ref(),
        )),
        BinaryOperator::GreaterEqual => {
            let ASTNode::Variable { name: idx_var, .. } = left.as_ref() else {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            };
            let ASTNode::Literal { value, .. } = right.as_ref() else {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            };
            if !matches!(value, LiteralValue::Integer(0)) {
                return Ok(numeric_compare_shape(
                    operator,
                    left.as_ref(),
                    right.as_ref(),
                ));
            }

            Ok(Some(ConditionShape::VarGreaterEqualZero {
                idx_var: idx_var.clone(),
            }))
        }
        _ => Ok(None),
    }
}

fn numeric_compare_shape(
    operator: &BinaryOperator,
    left: &ASTNode,
    right: &ASTNode,
) -> Option<ConditionShape> {
    if let ASTNode::Variable { name, .. } = left {
        return bound_from_numeric_expr(right).and_then(|bound| {
            cmp_from_operator(operator).map(|cmp| ConditionShape::VarCompareBound {
                idx_var: name.clone(),
                cmp,
                bound,
            })
        });
    }

    if let ASTNode::Variable { name, .. } = right {
        return bound_from_numeric_expr(left).and_then(|bound| {
            cmp_from_operator(operator).and_then(invert_cmp).map(|cmp| {
                ConditionShape::VarCompareBound {
                    idx_var: name.clone(),
                    cmp,
                    bound,
                }
            })
        });
    }

    None
}

fn bound_from_numeric_expr(expr: &ASTNode) -> Option<BoundExpr> {
    match expr {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Some(BoundExpr::LiteralI64(*value)),
        ASTNode::Variable { name, .. } => Some(BoundExpr::Var(name.clone())),
        _ => None,
    }
}

fn cmp_from_operator(operator: &BinaryOperator) -> Option<CmpOp> {
    match operator {
        BinaryOperator::Less => Some(CmpOp::Lt),
        BinaryOperator::LessEqual => Some(CmpOp::Le),
        BinaryOperator::Greater => Some(CmpOp::Gt),
        BinaryOperator::GreaterEqual => Some(CmpOp::Ge),
        BinaryOperator::Equal => Some(CmpOp::Eq),
        BinaryOperator::NotEqual => Some(CmpOp::Ne),
        _ => None,
    }
}

fn invert_cmp(cmp: CmpOp) -> Option<CmpOp> {
    match cmp {
        CmpOp::Lt => Some(CmpOp::Gt),
        CmpOp::Le => Some(CmpOp::Ge),
        CmpOp::Gt => Some(CmpOp::Lt),
        CmpOp::Ge => Some(CmpOp::Le),
        CmpOp::Eq => Some(CmpOp::Eq),
        CmpOp::Ne => Some(CmpOp::Ne),
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

#[cfg(test)]
mod tests {
    use super::try_extract_condition_shape;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::ConditionShape;
    use crate::mir::policies::{BoundExpr, CmpOp};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn cmp(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    #[test]
    fn condition_shape_accepts_var_le_bound_var() {
        let condition = cmp(BinaryOperator::LessEqual, var("i"), var("n"));
        let shape = try_extract_condition_shape(&condition)
            .expect("no freeze")
            .expect("shape");

        assert_eq!(
            shape,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Le,
                bound: BoundExpr::Var("n".to_string()),
            }
        );
    }

    #[test]
    fn condition_shape_inverts_literal_ge_var() {
        let condition = cmp(BinaryOperator::GreaterEqual, int(3), var("i"));
        let shape = try_extract_condition_shape(&condition)
            .expect("no freeze")
            .expect("shape");

        assert_eq!(
            shape,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Le,
                bound: BoundExpr::LiteralI64(3),
            }
        );
    }

    #[test]
    fn condition_shape_accepts_var_eq_bound_var() {
        let condition = cmp(BinaryOperator::Equal, var("i"), var("n"));
        let shape = try_extract_condition_shape(&condition)
            .expect("no freeze")
            .expect("shape");

        assert_eq!(
            shape,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Eq,
                bound: BoundExpr::Var("n".to_string()),
            }
        );
    }

    #[test]
    fn condition_shape_accepts_var_ne_literal() {
        let condition = cmp(BinaryOperator::NotEqual, var("i"), int(3));
        let shape = try_extract_condition_shape(&condition)
            .expect("no freeze")
            .expect("shape");

        assert_eq!(
            shape,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Ne,
                bound: BoundExpr::LiteralI64(3),
            }
        );
    }

    #[test]
    fn condition_shape_inverts_literal_eq_and_ne_var() {
        let eq = try_extract_condition_shape(&cmp(BinaryOperator::Equal, int(3), var("i")))
            .expect("no freeze")
            .expect("eq shape");
        let ne = try_extract_condition_shape(&cmp(BinaryOperator::NotEqual, int(3), var("i")))
            .expect("no freeze")
            .expect("ne shape");

        assert_eq!(
            eq,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Eq,
                bound: BoundExpr::LiteralI64(3),
            }
        );
        assert_eq!(
            ne,
            ConditionShape::VarCompareBound {
                idx_var: "i".to_string(),
                cmp: CmpOp::Ne,
                bound: BoundExpr::LiteralI64(3),
            }
        );
    }

    #[test]
    fn condition_shape_rejects_constant_numeric_compare() {
        let condition = cmp(BinaryOperator::LessEqual, int(1), int(3));
        assert!(try_extract_condition_shape(&condition)
            .expect("no freeze")
            .is_none());
    }

    #[test]
    fn condition_shape_rejects_constant_eq_compare() {
        let condition = cmp(BinaryOperator::Equal, int(1), int(3));
        assert!(try_extract_condition_shape(&condition)
            .expect("no freeze")
            .is_none());
    }
}
