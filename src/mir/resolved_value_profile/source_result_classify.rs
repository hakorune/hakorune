//! Row-driven expression classification for the source-result product.
//!
//! Each site is claimed by at most one sealed row family — conditional,
//! operator, literal, lexical-reference, and call rows from
//! `CallableSemanticSourceLedgerView` plus `body_shape` BlockExpr wrappers.
//! An unclaimed site is a named `UnknownExpression` rejection, never an AST
//! re-scan.

use super::*;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, ResolvedAssignmentTargetV1,
    ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1,
    ResolvedConditionalExpressionSourceV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedUnaryExpressionSourceV1, ResolvedUnaryOperatorV1, SourcePathV1,
    VerifiedResolvedBodyShapeInventoryV1,
};
use crate::mir::source_call_target::VerifiedSourceCallTargetV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExprClassV1 {
    I64,
    String,
    Bool,
    Null,
    /// Sealed rows exist but no source class is proven: unannotated
    /// parameters, unproven call results, or non-value bindings.
    Opaque,
}

impl ExprClassV1 {
    fn result(self) -> Option<SourceResultClassV1> {
        match self {
            Self::I64 => Some(SourceResultClassV1::I64),
            Self::String => Some(SourceResultClassV1::String),
            Self::Bool | Self::Null | Self::Opaque => None,
        }
    }

    fn from_declared_type(name: &str) -> Self {
        SourceResultClassV1::from_declared_type(name)
            .map(Self::from_result)
            .unwrap_or(Self::Opaque)
    }

    fn from_result(class: SourceResultClassV1) -> Self {
        match class {
            SourceResultClassV1::I64 => Self::I64,
            SourceResultClassV1::String => Self::String,
        }
    }
}

pub(super) struct InferenceState<'a> {
    pub(super) owner: &'a CanonicalSameModuleCallableKeyV1,
    pub(super) declaration: &'a VerifiedSameModuleCallableDeclarationV1,
    pub(super) catalog: &'a VerifiedSameModuleCallableDeclarationCatalogV1,
    pub(super) ledger: &'a CallableSemanticSourceLedgerView<'a>,
    pub(super) body_shape: &'a VerifiedResolvedBodyShapeInventoryV1,
    pub(super) call_targets: &'a VerifiedSourceCallTargetCatalogV1<'a>,
    pub(super) bindings: BTreeMap<BindingRefV1, ExprClassV1>,
    pub(super) operations: Vec<SourceValueOperationV1>,
    pub(super) bool_facts: Vec<SourceBoolFactV1>,
    pub(super) call_dispositions: Vec<SourceCallDispositionV1>,
    pub(super) returns: Vec<SourceResultClassV1>,
}

pub(super) fn infer_statements(
    state: &mut InferenceState<'_>,
    statements: &[ASTNode],
    prefix: &[SourcePathSegmentV1],
) -> Result<(), SourceResultProductErrorV1> {
    for (index, statement) in statements.iter().enumerate() {
        let statement_path =
            SourcePathV1::from_node(&SourceNodeSiteV1::from_segments(prefix.to_vec()))
                .child(SourcePathSegmentV1::Body(index as u32));
        match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } if variables.len() == initial_values.len() => {
                for ordinal in 0..variables.len() {
                    let declaration_site = SourceBindingSiteV1::Local {
                        statement: statement_path.stmt(),
                        ordinal: ordinal as u32,
                    };
                    let relation =
                        state.ledger.initializer(&declaration_site).ok_or_else(|| {
                            SourceResultProductErrorV1::MissingInitializerSource(
                                declaration_site.clone(),
                            )
                        })?;
                    let class = match (
                        initial_values.get(ordinal).and_then(Option::as_ref),
                        relation.initializer_site(),
                    ) {
                        (Some(initial), Some(initializer_site)) => {
                            classify_entry(state, initializer_site, initial)?
                        }
                        (None, None) => relation
                            .declared_type_name()
                            .map(ExprClassV1::from_declared_type)
                            .unwrap_or(ExprClassV1::Opaque),
                        _ => {
                            return Err(SourceResultProductErrorV1::MissingInitializerSource(
                                declaration_site.clone(),
                            ));
                        }
                    };
                    state.bindings.insert(relation.binding(), class);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { .. } = target.as_ref() else {
                    return Err(SourceResultProductErrorV1::UnsupportedStatement);
                };
                let target_site = statement_path.child(SourcePathSegmentV1::Target).expr();
                let Some(&ResolvedAssignmentTargetV1::BindingRebind(binding)) =
                    state.ledger.assignment_target(&target_site)
                else {
                    return Err(match state.ledger.assignment_target(&target_site) {
                        Some(_) => SourceResultProductErrorV1::UnsupportedStatement,
                        None => SourceResultProductErrorV1::MissingAssignmentTarget(target_site),
                    });
                };
                let value_site = statement_path.child(SourcePathSegmentV1::Value).expr();
                let class = classify_entry(state, &value_site, value)?;
                state.bindings.insert(binding, class);
            }
            ASTNode::Return {
                value: Some(value), ..
            } => {
                let value_site = statement_path.child(SourcePathSegmentV1::Value).expr();
                let class = classify_entry(state, &value_site, value)?;
                state.returns.push(class.result().ok_or_else(|| {
                    SourceResultProductErrorV1::UnprovenResultClass(value_site.clone())
                })?);
            }
            ASTNode::Return { value: None, .. } => {
                return Err(SourceResultProductErrorV1::UnknownExpression(
                    statement_path.expr(),
                ));
            }
            _ => return Err(SourceResultProductErrorV1::UnsupportedStatement),
        }
    }
    Ok(())
}

/// Statement-level entry: names the missing-conditional rejection before the
/// row-driven classifier is entered.
fn classify_entry(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    node: &ASTNode,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    if matches!(node, ASTNode::If { .. }) && state.ledger.conditional_source(site).is_none() {
        return Err(SourceResultProductErrorV1::MissingConditionalSource(
            site.clone(),
        ));
    }
    classify(state, site)
}

/// Row-driven expression classification.  Each site is claimed by at most one
/// sealed row family; an unclaimed site is a named `UnknownExpression`
/// rejection, never an AST re-scan.
fn classify(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    if let Some(row) = state.ledger.conditional_source(site) {
        classify_conditional(state, site, row)
    } else if let Some(row) = state.ledger.binary_source(site) {
        classify_binary(state, site, row)
    } else if let Some(row) = state.ledger.unary_source(site) {
        classify_unary(state, site, row)
    } else if let Some(literal) = state.ledger.literal_source(site) {
        let class = literal_class(site, literal)?;
        if let Some(result) = class.result() {
            state.operations.push(SourceValueOperationV1::Literal {
                site: site.clone(),
                class: result,
            });
        }
        Ok(class)
    } else if let Some(reference) = state.ledger.variable_ref(site) {
        classify_reference(state, site, reference)
    } else if let Some(row) = state.ledger.method_call(site) {
        classify_method_call(state, site, row)
    } else if state.ledger.direct_call_target(site).is_some()
        || state.ledger.direct_call_observation(site).is_some()
    {
        classify_direct_call(state, site)
    } else if let Some(BodyExpressionShapeV1::BlockExpr { .. }) =
        state.body_shape.expression_shape(site)
    {
        // A BlockExpr wrapper is transparent only when its prelude is empty:
        // `{ stmt; tail }` statement effects sit outside the result product's
        // model, so a non-empty prelude rejects instead of being dropped.
        let site_segments = site.node().segments();
        let has_prelude_item = state.body_shape.statements().iter().any(|statement| {
            matches!(
                statement.site().node().segments().split_last(),
                Some((SourcePathSegmentV1::BlockExprPrelude(_), prefix))
                    if prefix == site_segments
            )
        });
        if has_prelude_item {
            return Err(SourceResultProductErrorV1::NonEmptyBlockExprPrelude(
                site.clone(),
            ));
        }
        // Transparent BlockExpr wrapper (e.g. an expression-If branch tail):
        // the sealed shape row proves the wrapper, so the only child to
        // consume is the resolver-published `BlockExprTail` expression site.
        let tail = SourcePathV1::from_node(site.node())
            .child(SourcePathSegmentV1::BlockExprTail)
            .expr();
        classify(state, &tail)
    } else {
        Err(SourceResultProductErrorV1::UnknownExpression(site.clone()))
    }
}

fn classify_conditional(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    row: &ResolvedConditionalExpressionSourceV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let consumer = row.consumer();
    let Some((role, parent)) = site.node().segments().split_last() else {
        return Err(SourceResultProductErrorV1::ConsumerSiteDrift {
            site: site.clone(),
            claimed_parent: consumer.parent().clone(),
            claimed_role: consumer.role().clone(),
        });
    };
    if consumer.role() != role || consumer.parent().segments() != parent {
        return Err(SourceResultProductErrorV1::ConsumerSiteDrift {
            site: site.clone(),
            claimed_parent: consumer.parent().clone(),
            claimed_role: consumer.role().clone(),
        });
    }
    classify(state, row.condition())?;
    let then_class = classify(state, row.then_tail())?;
    let else_class = classify(state, row.else_tail())?;
    if then_class != else_class {
        return Err(SourceResultProductErrorV1::MixedReturnClasses);
    }
    if let Some(result) = then_class.result() {
        state
            .operations
            .push(SourceValueOperationV1::ConditionalValue {
                site: site.clone(),
                class: result,
            });
    }
    Ok(then_class)
}

fn classify_binary(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    row: &ResolvedBinaryExpressionSourceV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let lhs = classify(state, row.lhs())?;
    let rhs = classify(state, row.rhs())?;
    match row.operator() {
        ResolvedBinaryOperatorV1::Add
            if lhs == ExprClassV1::String || rhs == ExprClassV1::String =>
        {
            state
                .operations
                .push(SourceValueOperationV1::StringConcat { site: site.clone() });
            Ok(ExprClassV1::String)
        }
        ResolvedBinaryOperatorV1::Add
        | ResolvedBinaryOperatorV1::Subtract
        | ResolvedBinaryOperatorV1::Multiply
        | ResolvedBinaryOperatorV1::Divide
        | ResolvedBinaryOperatorV1::Modulo
        | ResolvedBinaryOperatorV1::BitAnd
        | ResolvedBinaryOperatorV1::BitOr
        | ResolvedBinaryOperatorV1::BitXor
        | ResolvedBinaryOperatorV1::Shl
        | ResolvedBinaryOperatorV1::Shr
            if lhs == ExprClassV1::I64 && rhs == ExprClassV1::I64 =>
        {
            Ok(ExprClassV1::I64)
        }
        ResolvedBinaryOperatorV1::Equal => Ok(ExprClassV1::Bool),
        ResolvedBinaryOperatorV1::NotEqual => {
            if lhs == ExprClassV1::Null || rhs == ExprClassV1::Null {
                let (string_operand, null_operand) = if lhs == ExprClassV1::String {
                    (row.lhs(), row.rhs())
                } else if rhs == ExprClassV1::String {
                    (row.rhs(), row.lhs())
                } else {
                    return Err(SourceResultProductErrorV1::NullableOperandNotString(
                        site.clone(),
                    ));
                };
                state.bool_facts.push(SourceBoolFactV1 {
                    owner: state.owner.clone(),
                    site: site.clone(),
                    lhs: string_operand.clone(),
                    null_operand: null_operand.clone(),
                });
            }
            Ok(ExprClassV1::Bool)
        }
        ResolvedBinaryOperatorV1::Less
        | ResolvedBinaryOperatorV1::Greater
        | ResolvedBinaryOperatorV1::LessEqual
        | ResolvedBinaryOperatorV1::GreaterEqual
            if lhs == ExprClassV1::I64 && rhs == ExprClassV1::I64 =>
        {
            Ok(ExprClassV1::Bool)
        }
        ResolvedBinaryOperatorV1::And | ResolvedBinaryOperatorV1::Or
            if lhs == ExprClassV1::Bool && rhs == ExprClassV1::Bool =>
        {
            Ok(ExprClassV1::Bool)
        }
        _ => Err(SourceResultProductErrorV1::UnknownExpression(site.clone())),
    }
}

fn classify_unary(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    row: &ResolvedUnaryExpressionSourceV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let operand = classify(state, row.operand())?;
    match row.operator() {
        ResolvedUnaryOperatorV1::Minus | ResolvedUnaryOperatorV1::BitNot
            if operand == ExprClassV1::I64 =>
        {
            Ok(ExprClassV1::I64)
        }
        ResolvedUnaryOperatorV1::Not if operand == ExprClassV1::Bool => Ok(ExprClassV1::Bool),
        _ => Err(SourceResultProductErrorV1::UnknownExpression(site.clone())),
    }
}

fn literal_class(
    site: &SourceExprSiteV1,
    literal: &ResolvedLiteralSourceV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    Ok(match literal {
        ResolvedLiteralSourceV1::Integer(_) | ResolvedLiteralSourceV1::TypedInteger { .. } => {
            ExprClassV1::I64
        }
        ResolvedLiteralSourceV1::String(_) => ExprClassV1::String,
        ResolvedLiteralSourceV1::Bool(_) => ExprClassV1::Bool,
        ResolvedLiteralSourceV1::Null => ExprClassV1::Null,
        ResolvedLiteralSourceV1::Float | ResolvedLiteralSourceV1::Void => {
            return Err(SourceResultProductErrorV1::UnknownExpression(site.clone()));
        }
    })
}

fn classify_reference(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    reference: ResolvedLexicalRefV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let class = match reference {
        ResolvedLexicalRefV1::Local(binding) => {
            let record = state
                .ledger
                .binding(binding)
                .ok_or(SourceResultProductErrorV1::ForeignLedger)?;
            match record.kind() {
                BindingKindV1::Parameter { index } => state
                    .declaration
                    .param_decls()
                    .get(index as usize)
                    .and_then(|decl| decl.declared_type_name.as_deref())
                    .map(ExprClassV1::from_declared_type)
                    .unwrap_or(ExprClassV1::Opaque),
                BindingKindV1::Local { .. } | BindingKindV1::Outbox { .. } => state
                    .bindings
                    .get(&binding)
                    .copied()
                    .unwrap_or(ExprClassV1::Opaque),
                _ => ExprClassV1::Opaque,
            }
        }
        ResolvedLexicalRefV1::Upvar(_) => ExprClassV1::Opaque,
    };
    if let Some(result) = class.result() {
        state.operations.push(SourceValueOperationV1::Binding {
            site: site.clone(),
            class: result,
        });
    }
    Ok(class)
}

fn classify_method_call(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
    row: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    let class = match state.call_targets.route_target(state.owner, site) {
        Some(VerifiedSourceCallTargetV1::Static(target)) => {
            let key = target.target().clone();
            state
                .call_dispositions
                .push(SourceCallDispositionV1::Static {
                    site: site.clone(),
                    target: key.clone(),
                });
            state
                .catalog
                .declaration(&key)
                .and_then(|decl| decl.return_type_name())
                .map(ExprClassV1::from_declared_type)
                .unwrap_or(ExprClassV1::Opaque)
        }
        Some(VerifiedSourceCallTargetV1::DynamicMember(_)) => {
            state
                .call_dispositions
                .push(SourceCallDispositionV1::Dynamic { site: site.clone() });
            ExprClassV1::Opaque
        }
        Some(VerifiedSourceCallTargetV1::CoreMethod(_)) => {
            state
                .call_dispositions
                .push(SourceCallDispositionV1::Absent { site: site.clone() });
            ExprClassV1::Opaque
        }
        None => {
            state
                .call_dispositions
                .push(SourceCallDispositionV1::Absent { site: site.clone() });
            ExprClassV1::Opaque
        }
    };
    for argument in row.arguments() {
        classify(state, argument.site())?;
    }
    Ok(class)
}

fn classify_direct_call(
    state: &mut InferenceState<'_>,
    site: &SourceExprSiteV1,
) -> Result<ExprClassV1, SourceResultProductErrorV1> {
    // ObserveOnly direct calls carry no target row by design; a target row
    // without its observation is verifier-invalid.  Either way the branded
    // route catalog has no same-module catalog key for a free-function call,
    // so the observed call publishes Absent and its result stays unproven.
    let Some(observation) = state.ledger.direct_call_observation(site) else {
        return Err(SourceResultProductErrorV1::AmbiguousCallTarget(
            site.clone(),
        ));
    };
    for argument_site in observation.argument_sites() {
        classify(state, argument_site)?;
    }
    state
        .call_dispositions
        .push(SourceCallDispositionV1::Absent { site: site.clone() });
    Ok(ExprClassV1::Opaque)
}
