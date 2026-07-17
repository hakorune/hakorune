use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourceExprSiteV1, SourcePathV1};
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;
use crate::mir::source_core_receiver::{SourceCoreReceiverFactV1, VerifiedSourceCoreReceiverV1};

use super::call_proof::CallProofContextV1;
use super::call_row::{CallableResultCallRowsV1, VerifiedCallableResultCallSiteV1};
use super::requirements::{union_requirements, RequirementSetV1};
use super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum I64ExpressionFactV1 {
    Exact(RequirementSetV1),
    KnownNonI64,
    Unknown(CallableResultUnavailableReasonV1),
    PendingDependency,
    Conflict,
}

impl I64ExpressionFactV1 {
    pub(super) fn exact_empty() -> Self {
        Self::Exact(BTreeSet::new())
    }

    pub(super) fn merge_paths(left: &Self, right: &Self) -> Self {
        match (left, right) {
            (Self::PendingDependency, _) | (_, Self::PendingDependency) => Self::PendingDependency,
            (Self::Exact(left), Self::Exact(right)) => Self::Exact(union_requirements(left, right)),
            (Self::KnownNonI64, Self::KnownNonI64) => Self::KnownNonI64,
            (Self::Unknown(left), Self::Unknown(right)) if left == right => {
                Self::Unknown(left.clone())
            }
            _ => Self::Conflict,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExpressionEnvironmentV1 {
    i64_bindings: BTreeMap<String, I64ExpressionFactV1>,
    core_receiver_bindings: BTreeMap<String, SourceCoreReceiverFactV1>,
}

impl ExpressionEnvironmentV1 {
    pub(super) fn merge(left: &Self, right: &Self) -> Self {
        let i64_bindings = left
            .i64_bindings
            .iter()
            .filter_map(|(name, left_fact)| {
                right.i64_bindings.get(name).map(|right_fact| {
                    (
                        name.clone(),
                        I64ExpressionFactV1::merge_paths(left_fact, right_fact),
                    )
                })
            })
            .collect();
        let core_receiver_bindings = left
            .core_receiver_bindings
            .iter()
            .filter_map(|(name, left_fact)| {
                right
                    .core_receiver_bindings
                    .get(name)
                    .filter(|right_fact| *right_fact == left_fact)
                    .map(|_| (name.clone(), *left_fact))
            })
            .collect();
        Self {
            i64_bindings,
            core_receiver_bindings,
        }
    }

    pub(super) fn binding_count(&self) -> usize {
        self.i64_bindings
            .len()
            .saturating_add(self.core_receiver_bindings.len())
    }
}

#[derive(Debug, Clone)]
pub(super) struct CallRowDraftStateV1<'targets> {
    rows: CallableResultCallRowsV1<'targets>,
    conflicts: BTreeSet<(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1)>,
}

pub(super) struct ExpressionProofContextV1<'targets, 'catalog, 'rows> {
    current_key: &'catalog CanonicalSameModuleCallableKeyV1,
    environment: ExpressionEnvironmentV1,
    call_proof: CallProofContextV1<'targets, 'catalog, 'rows>,
    call_rows: CallableResultCallRowsV1<'targets>,
    call_row_conflicts: BTreeSet<(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1)>,
}

impl<'targets, 'catalog, 'rows> ExpressionProofContextV1<'targets, 'catalog, 'rows> {
    fn child_path(parent: &ASTNode, path: &SourcePathV1, role: ExprChildRoleV1) -> SourcePathV1 {
        path.child(
            role.segment_for(parent)
                .expect("[freeze:contract][source_path/callable_result_expr_role]"),
        )
    }

    pub(super) fn new(
        current_key: &'catalog CanonicalSameModuleCallableKeyV1,
        params: &[String],
        targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
        result_rows: &'rows BTreeMap<
            CanonicalSameModuleCallableKeyV1,
            VerifiedCallableResultDispositionV1,
        >,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let mut i64_bindings = BTreeMap::new();
        for (ordinal, name) in params.iter().enumerate() {
            let ordinal = u32::try_from(ordinal).map_err(|_| {
                CallableResultCatalogErrorV1::CallArityOverflow {
                    caller: current_key.clone(),
                    arity: params.len(),
                }
            })?;
            i64_bindings.insert(name.clone(), I64ExpressionFactV1::Exact([ordinal].into()));
        }
        Ok(Self {
            current_key,
            environment: ExpressionEnvironmentV1 {
                i64_bindings,
                core_receiver_bindings: BTreeMap::new(),
            },
            call_proof: CallProofContextV1::new(current_key, targets, result_rows),
            call_rows: BTreeMap::new(),
            call_row_conflicts: BTreeSet::new(),
        })
    }

    pub(super) const fn environment(&self) -> &ExpressionEnvironmentV1 {
        &self.environment
    }

    pub(super) fn replace_environment(&mut self, environment: ExpressionEnvironmentV1) {
        self.environment = environment;
    }

    pub(super) fn binding(&self, name: &str) -> Option<&I64ExpressionFactV1> {
        self.environment.i64_bindings.get(name)
    }

    pub(super) fn publish_binding(&mut self, name: &str, fact: I64ExpressionFactV1) {
        self.environment.i64_bindings.insert(name.to_owned(), fact);
    }

    pub(super) fn publish_core_receiver_binding(
        &mut self,
        name: &str,
        fact: Option<SourceCoreReceiverFactV1>,
    ) {
        if let Some(fact) = fact {
            self.environment
                .core_receiver_bindings
                .insert(name.to_owned(), fact);
        } else {
            self.environment.core_receiver_bindings.remove(name);
        }
    }

    pub(super) fn contains_binding(&self, name: &str) -> bool {
        self.environment.i64_bindings.contains_key(name)
    }

    pub(super) fn core_receiver_fact(
        &self,
        expression: &ASTNode,
    ) -> Option<SourceCoreReceiverFactV1> {
        if let Ok(proof) = VerifiedSourceCoreReceiverV1::verify(expression) {
            return Some(proof.fact());
        }
        let ASTNode::Variable { name, .. } = expression else {
            return None;
        };
        self.environment.core_receiver_bindings.get(name).copied()
    }

    pub(super) fn call_row_state(&self) -> CallRowDraftStateV1<'targets> {
        CallRowDraftStateV1 {
            rows: self.call_rows.clone(),
            conflicts: self.call_row_conflicts.clone(),
        }
    }

    pub(super) fn restore_call_row_state(&mut self, state: CallRowDraftStateV1<'targets>) {
        self.call_rows = state.rows;
        self.call_row_conflicts = state.conflicts;
    }

    pub(super) fn into_call_rows(self) -> CallableResultCallRowsV1<'targets> {
        self.call_rows
    }

    pub(super) fn prove_expression(
        &mut self,
        expression: &ASTNode,
        path: &SourcePathV1,
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
            } => self.prove_expression(
                operand,
                &Self::child_path(expression, path, ExprChildRoleV1::UnaryOperand),
            ),
            ASTNode::UnaryOp { .. } => Ok(I64ExpressionFactV1::KnownNonI64),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.prove_expression(
                    left,
                    &Self::child_path(expression, path, ExprChildRoleV1::BinaryLeft),
                )?;
                let right = self.prove_expression(
                    right,
                    &Self::child_path(expression, path, ExprChildRoleV1::BinaryRight),
                )?;
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
                let _ = self.prove_arguments(expression, arguments, path)?;
                Ok(I64ExpressionFactV1::Unknown(
                    CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
                ))
            }
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                let receiver_fact = self.core_receiver_fact(object);
                let arguments = self.prove_arguments(expression, arguments, path)?;
                let outcome = self.call_proof.prove_method_call(
                    path.expr(),
                    method,
                    &arguments,
                    receiver_fact,
                )?;
                if let Some(row) = outcome.row {
                    if !self.record_call_row(path.expr(), row) {
                        return Ok(I64ExpressionFactV1::Conflict);
                    }
                }
                Ok(outcome.fact)
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } if prelude_stmts.is_empty() => self.prove_expression(
                tail_expr,
                &Self::child_path(expression, path, ExprChildRoleV1::BlockExprTail),
            ),
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
        parent: &ASTNode,
        arguments: &[ASTNode],
        path: &SourcePathV1,
    ) -> Result<Vec<I64ExpressionFactV1>, CallableResultCatalogErrorV1> {
        arguments
            .iter()
            .enumerate()
            .map(|(index, argument)| {
                self.prove_expression(
                    argument,
                    &Self::child_path(parent, path, ExprChildRoleV1::CallArgument(index as u32)),
                )
            })
            .collect()
    }

    fn record_call_row(
        &mut self,
        site: SourceExprSiteV1,
        row: VerifiedCallableResultCallSiteV1<'targets>,
    ) -> bool {
        let key = (self.current_key.clone(), site);
        if self.call_row_conflicts.contains(&key) {
            return false;
        }
        match self.call_rows.get(&key) {
            None => {
                self.call_rows.insert(key, row);
                true
            }
            Some(existing) if existing.semantically_matches(&row) => true,
            Some(_) => {
                self.call_rows.remove(&key);
                self.call_row_conflicts.insert(key);
                false
            }
        }
    }
}

fn combine_arithmetic(
    left: I64ExpressionFactV1,
    right: I64ExpressionFactV1,
) -> I64ExpressionFactV1 {
    match (left, right) {
        (I64ExpressionFactV1::PendingDependency, _)
        | (_, I64ExpressionFactV1::PendingDependency) => I64ExpressionFactV1::PendingDependency,
        (I64ExpressionFactV1::Exact(left), I64ExpressionFactV1::Exact(right)) => {
            I64ExpressionFactV1::Exact(union_requirements(&left, &right))
        }
        _ => I64ExpressionFactV1::Unknown(CallableResultUnavailableReasonV1::UnknownExpression),
    }
}
