use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;

use super::requirements::{union_requirements, RequirementSetV1};
use super::{CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum I64ExpressionFactV1 {
    Exact(RequirementSetV1),
    KnownNonI64,
    Unknown(CallableResultUnavailableReasonV1),
    Conflict,
}

impl I64ExpressionFactV1 {
    pub(super) fn exact_empty() -> Self {
        Self::Exact(BTreeSet::new())
    }

    pub(super) fn merge_paths(left: &Self, right: &Self) -> Self {
        match (left, right) {
            (Self::Exact(left), Self::Exact(right)) => Self::Exact(union_requirements(left, right)),
            (Self::KnownNonI64, Self::KnownNonI64) => Self::KnownNonI64,
            (Self::Unknown(left), Self::Unknown(right)) if left == right => {
                Self::Unknown(left.clone())
            }
            _ => Self::Conflict,
        }
    }
}

pub(super) struct ExpressionProofContextV1 {
    bindings: BTreeMap<String, I64ExpressionFactV1>,
}

impl ExpressionProofContextV1 {
    pub(super) fn new(
        current_key: &CanonicalSameModuleCallableKeyV1,
        params: &[String],
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let mut bindings = BTreeMap::new();
        for (ordinal, name) in params.iter().enumerate() {
            let ordinal = u32::try_from(ordinal).map_err(|_| {
                CallableResultCatalogErrorV1::CallArityOverflow {
                    caller: current_key.clone(),
                    arity: params.len(),
                }
            })?;
            bindings.insert(name.clone(), I64ExpressionFactV1::Exact([ordinal].into()));
        }
        Ok(Self { bindings })
    }

    pub(super) fn bindings(&self) -> &BTreeMap<String, I64ExpressionFactV1> {
        &self.bindings
    }

    pub(super) fn replace_bindings(&mut self, bindings: BTreeMap<String, I64ExpressionFactV1>) {
        self.bindings = bindings;
    }

    pub(super) fn binding(&self, name: &str) -> Option<&I64ExpressionFactV1> {
        self.bindings.get(name)
    }

    pub(super) fn publish_binding(&mut self, name: impl Into<String>, fact: I64ExpressionFactV1) {
        self.bindings.insert(name.into(), fact);
    }

    pub(super) fn contains_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub(super) fn prove_expression(
        &mut self,
        expression: &ASTNode,
    ) -> Result<I64ExpressionFactV1, CallableResultCatalogErrorV1> {
        match expression {
            ASTNode::Literal { value, .. } => Ok(match value {
                LiteralValue::Integer(_) => I64ExpressionFactV1::exact_empty(),
                LiteralValue::TypedInteger {
                    declared_type_name, ..
                } if crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::classify(
                    declared_type_name,
                )
                .is_some() =>
                {
                    I64ExpressionFactV1::exact_empty()
                }
                LiteralValue::String(_)
                | LiteralValue::TypedInteger { .. }
                | LiteralValue::Float(_)
                | LiteralValue::Bool(_)
                | LiteralValue::Null
                | LiteralValue::Void => I64ExpressionFactV1::KnownNonI64,
            }),
            ASTNode::Variable { name, .. } => {
                Ok(self
                    .binding(name)
                    .cloned()
                    .unwrap_or(I64ExpressionFactV1::Unknown(
                        CallableResultUnavailableReasonV1::UnboundLocal,
                    )))
            }
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand,
                ..
            } => self.prove_expression(operand),
            ASTNode::UnaryOp { .. } => Ok(I64ExpressionFactV1::KnownNonI64),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.prove_expression(left)?;
                let right = self.prove_expression(right)?;
                if matches!(
                    operator,
                    BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Multiply
                        | BinaryOperator::Divide
                        | BinaryOperator::Modulo
                ) {
                    Ok(combine_arithmetic(left, right))
                } else {
                    Ok(I64ExpressionFactV1::KnownNonI64)
                }
            }
            ASTNode::FunctionCall { arguments, .. } => {
                let _ = self.prove_arguments(arguments)?;
                Ok(I64ExpressionFactV1::Unknown(
                    CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
                ))
            }
            ASTNode::MethodCall { arguments, .. } => {
                let _ = self.prove_arguments(arguments)?;
                Ok(I64ExpressionFactV1::Unknown(
                    CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
                ))
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } if prelude_stmts.is_empty() => self.prove_expression(tail_expr),
            ASTNode::New { .. }
            | ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. } => Ok(I64ExpressionFactV1::KnownNonI64),
            _ => Ok(I64ExpressionFactV1::Unknown(
                CallableResultUnavailableReasonV1::UnsupportedExpressionKind,
            )),
        }
    }

    fn prove_arguments(
        &mut self,
        arguments: &[ASTNode],
    ) -> Result<Vec<I64ExpressionFactV1>, CallableResultCatalogErrorV1> {
        arguments
            .iter()
            .map(|argument| self.prove_expression(argument))
            .collect()
    }
}

fn combine_arithmetic(
    left: I64ExpressionFactV1,
    right: I64ExpressionFactV1,
) -> I64ExpressionFactV1 {
    match (left, right) {
        (I64ExpressionFactV1::Exact(left), I64ExpressionFactV1::Exact(right)) => {
            I64ExpressionFactV1::Exact(union_requirements(&left, &right))
        }
        _ => I64ExpressionFactV1::Unknown(CallableResultUnavailableReasonV1::UnknownExpression),
    }
}
