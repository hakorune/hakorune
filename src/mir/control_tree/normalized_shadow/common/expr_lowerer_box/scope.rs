use super::binary::{binary_kind, BinaryKind};
use super::NormalizedExprLowererBox;
use super::super::expr_lowering_contract::{
    ExprLoweringScope, ImpurePolicy, OutOfScopeReason,
};
use super::super::known_intrinsics::KnownIntrinsicRegistryBox;
use crate::ast::{ASTNode, LiteralValue, UnaryOperator};
use crate::mir::ValueId;
use std::collections::BTreeMap;

impl NormalizedExprLowererBox {
    /// Classify out-of-scope reasons for diagnostics/tests without changing the core API.
    pub fn out_of_scope_reason(
        scope: ExprLoweringScope,
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
    ) -> Option<OutOfScopeReason> {
        match scope {
            ExprLoweringScope::PureOnly => Self::out_of_scope_reason_pure(ast, env),
            ExprLoweringScope::WithImpure(policy) => {
                Self::out_of_scope_reason_with_impure(policy, ast, env)
            }
        }
    }

    fn out_of_scope_reason_pure(
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
    ) -> Option<OutOfScopeReason> {
        match ast {
            ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => Some(OutOfScopeReason::Call),
            ASTNode::MethodCall { .. } => Some(OutOfScopeReason::MethodCall),

            ASTNode::Variable { name, .. } => {
                if env.contains_key(name) {
                    None
                } else {
                    Some(OutOfScopeReason::MissingEnvVar)
                }
            }

            ASTNode::Literal { value, .. } => match value {
                LiteralValue::Integer(_) | LiteralValue::Bool(_) => None,
                _ => Some(OutOfScopeReason::UnsupportedLiteral),
            },

            ASTNode::UnaryOp {
                operator, operand, ..
            } => match operator {
                UnaryOperator::Minus => {
                    if Self::is_supported_int_expr(operand, env) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                }
                UnaryOperator::Not => {
                    if Self::is_supported_bool_expr(operand, env) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                }
                UnaryOperator::BitNot => Some(OutOfScopeReason::UnsupportedOperator),
                UnaryOperator::Weak => Some(OutOfScopeReason::UnsupportedOperator),
            },

            ASTNode::BinaryOp {
                operator, left, right, ..
            } => {
                if binary_kind(operator).is_some() {
                    if Self::is_supported_int_expr(left, env) && Self::is_supported_int_expr(right, env) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                } else {
                    Some(OutOfScopeReason::UnsupportedOperator)
                }
            }

            _ => Some(OutOfScopeReason::ImpureExpression),
        }
    }

    fn out_of_scope_reason_with_impure(
        policy: ImpurePolicy,
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
    ) -> Option<OutOfScopeReason> {
        match ast {
            ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => Some(OutOfScopeReason::Call),
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                if Self::match_known_intrinsic_method_call(policy, object, method, arguments, env)
                    .is_some()
                {
                    None
                } else {
                    let receiver_ok =
                        matches!(object.as_ref(), ASTNode::Variable { name, .. } if env.contains_key(name));
                    if receiver_ok && KnownIntrinsicRegistryBox::lookup(method, arguments.len()).is_some()
                    {
                        Some(OutOfScopeReason::IntrinsicNotWhitelisted)
                    } else {
                        Some(OutOfScopeReason::MethodCall)
                    }
                }
            }

            ASTNode::Variable { name, .. } => {
                if env.contains_key(name) {
                    None
                } else {
                    Some(OutOfScopeReason::MissingEnvVar)
                }
            }

            ASTNode::Literal { value, .. } => match value {
                LiteralValue::Integer(_) | LiteralValue::Bool(_) => None,
                _ => Some(OutOfScopeReason::UnsupportedLiteral),
            },

            ASTNode::UnaryOp {
                operator, operand, ..
            } => match operator {
                UnaryOperator::Minus => {
                    if Self::is_supported_int_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        operand,
                        env,
                    ) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                }
                UnaryOperator::Not => {
                    if Self::is_supported_bool_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        operand,
                        env,
                    ) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                }
                UnaryOperator::BitNot => Some(OutOfScopeReason::UnsupportedOperator),
                UnaryOperator::Weak => Some(OutOfScopeReason::UnsupportedOperator),
            },

            ASTNode::BinaryOp {
                operator, left, right, ..
            } => {
                if binary_kind(operator).is_some() {
                    if Self::is_supported_int_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        left,
                        env,
                    ) && Self::is_supported_int_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        right,
                        env,
                    ) {
                        None
                    } else {
                        Some(OutOfScopeReason::UnsupportedOperator)
                    }
                } else {
                    Some(OutOfScopeReason::UnsupportedOperator)
                }
            }

            _ => Some(OutOfScopeReason::ImpureExpression),
        }
    }

    fn is_supported_int_expr(ast: &ASTNode, env: &BTreeMap<String, ValueId>) -> bool {
        match ast {
            ASTNode::Variable { name, .. } => env.contains_key(name),
            ASTNode::Literal {
                value: LiteralValue::Integer(_),
                ..
            } => true,
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand,
                ..
            } => Self::is_supported_int_expr(operand, env),
            ASTNode::BinaryOp {
                operator, left, right, ..
            } => {
                matches!(binary_kind(operator), Some(BinaryKind::Arith(_)))
                    && Self::is_supported_int_expr(left, env)
                    && Self::is_supported_int_expr(right, env)
            }
            _ => false,
        }
    }

    fn is_supported_bool_expr(ast: &ASTNode, env: &BTreeMap<String, ValueId>) -> bool {
        match ast {
            ASTNode::Variable { name, .. } => env.contains_key(name),
            ASTNode::Literal {
                value: LiteralValue::Bool(_),
                ..
            } => true,
            ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } => Self::is_supported_bool_expr(operand, env),
            _ => false,
        }
    }

    fn is_supported_int_expr_with_scope(
        scope: ExprLoweringScope,
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
    ) -> bool {
        match scope {
            ExprLoweringScope::PureOnly => Self::is_supported_int_expr(ast, env),
            ExprLoweringScope::WithImpure(policy) => match ast {
                ASTNode::MethodCall {
                    object,
                    method,
                    arguments,
                    ..
                } => Self::match_known_intrinsic_method_call(policy, object, method, arguments, env)
                    .is_some(),
                ASTNode::Variable { name, .. } => env.contains_key(name),
                ASTNode::Literal {
                    value: LiteralValue::Integer(_),
                    ..
                } => true,
                ASTNode::UnaryOp {
                    operator: UnaryOperator::Minus,
                    operand,
                    ..
                } => Self::is_supported_int_expr_with_scope(
                    ExprLoweringScope::WithImpure(policy),
                    operand,
                    env,
                ),
                ASTNode::BinaryOp {
                    operator, left, right, ..
                } => {
                    matches!(
                        binary_kind(operator),
                        Some(BinaryKind::Arith(_)) | Some(BinaryKind::Compare(_))
                    ) && Self::is_supported_int_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        left,
                        env,
                    ) && Self::is_supported_int_expr_with_scope(
                        ExprLoweringScope::WithImpure(policy),
                        right,
                        env,
                    )
                }
                _ => false,
            },
        }
    }

    fn is_supported_bool_expr_with_scope(
        scope: ExprLoweringScope,
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
    ) -> bool {
        match scope {
            ExprLoweringScope::PureOnly => Self::is_supported_bool_expr(ast, env),
            ExprLoweringScope::WithImpure(policy) => match ast {
                ASTNode::Variable { name, .. } => env.contains_key(name),
                ASTNode::Literal {
                    value: LiteralValue::Bool(_),
                    ..
                } => true,
                ASTNode::UnaryOp {
                    operator: UnaryOperator::Not,
                    operand,
                    ..
                } => Self::is_supported_bool_expr_with_scope(
                    ExprLoweringScope::WithImpure(policy),
                    operand,
                    env,
                ),
                _ => {
                    let _ = policy;
                    false
                }
            },
        }
    }
}
