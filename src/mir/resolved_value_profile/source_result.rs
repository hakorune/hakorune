//! Source-owned result classes for expression-value admission.
//!
//! The issuer consumes the resolver-sealed callable source ledger plus the
//! branded source call-target catalog.  The sealed declaration body supplies
//! only the statement scaffold (`Local` / `Assignment` / `Return`); every
//! expression classification goes through `CallableSemanticSourceLedgerView`
//! rows — conditional, operator, literal, lexical-reference, and call rows are
//! never re-derived by rescanning the raw AST.
//!
//! Every observed call site publishes one explicit `Static | Dynamic | Absent`
//! disposition.  `Static` exists only where the branded route catalog proves
//! the exact same-module callable; `Dynamic` only where a source-bound
//! dynamic member route exists; every other observed call — including
//! resolver-proven direct-call owners that carry no same-module catalog key —
//! publishes `Absent`.  `Absent` and `Dynamic` rows keep the call result
//! unproven: the class join rejects them wherever a `SourceResultClassV1` is
//! required.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, CallableSemanticSourceLedgerView,
    ResolvedAssignmentTargetV1, ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1,
    ResolvedConditionalExpressionSourceV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedUnaryExpressionSourceV1, ResolvedUnaryOperatorV1, SemanticOwnerSourceKindV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedResolvedBodyShapeInventoryV1,
};
use crate::mir::source_call_target::{
    VerifiedSourceCallTargetCatalogV1, VerifiedSourceCallTargetV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SourceResultClassV1 {
    I64,
    String,
}

impl SourceResultClassV1 {
    fn from_declared_type(name: &str) -> Option<Self> {
        match name {
            "i64" | "Integer" | "Int" => Some(Self::I64),
            "Str" | "String" => Some(Self::String),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceValueOperationV1 {
    Literal {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
    Binding {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
    StringConcat {
        site: SourceExprSiteV1,
    },
    ConditionalValue {
        site: SourceExprSiteV1,
        class: SourceResultClassV1,
    },
}

/// One branded call-route disposition for an observed call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceCallDispositionV1 {
    /// The branded route catalog proved this exact same-module callable.
    Static {
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    /// The branded route catalog proved a source-bound dynamic member route.
    Dynamic { site: SourceExprSiteV1 },
    /// The resolver observed the call but no branded route exists for it.
    Absent { site: SourceExprSiteV1 },
}

impl SourceCallDispositionV1 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Static { site, .. } | Self::Dynamic { site } | Self::Absent { site } => site,
        }
    }

    pub(crate) fn static_target(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::Static { target, .. } => Some(target),
            Self::Dynamic { .. } | Self::Absent { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceBoolFactV1 {
    owner: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    lhs: SourceExprSiteV1,
    null_operand: SourceExprSiteV1,
}

impl SourceBoolFactV1 {
    pub(crate) fn owner(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.owner
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) fn null_operand(&self) -> &SourceExprSiteV1 {
        &self.null_operand
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceResultProductErrorV1 {
    ForeignCallable,
    ForeignCallTargetCatalog,
    ForeignLedger,
    ForeignResolvedInput,
    UnsupportedDeclaredResult,
    MissingReturn,
    UnprovenResultClass(SourceExprSiteV1),
    UnknownExpression(SourceExprSiteV1),
    MissingConditionalSource(SourceExprSiteV1),
    MissingBodyShape,
    MissingInitializerSource(SourceBindingSiteV1),
    MissingAssignmentTarget(SourceExprSiteV1),
    AmbiguousCallTarget(SourceExprSiteV1),
    ConsumerSiteDrift {
        site: SourceExprSiteV1,
        claimed_parent: SourceNodeSiteV1,
        claimed_role: SourcePathSegmentV1,
    },
    MixedReturnClasses,
    DeclaredResultMismatch {
        declared: SourceResultClassV1,
        inferred: SourceResultClassV1,
    },
    UnsupportedStatement,
    NullableOperandNotString(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedSourceResultProductV1 {
    catalog_identity: usize,
    owner: CanonicalSameModuleCallableKeyV1,
    result: SourceResultClassV1,
    operations: Box<[SourceValueOperationV1]>,
    bool_facts: Box<[SourceBoolFactV1]>,
    call_dispositions: Box<[SourceCallDispositionV1]>,
    _seal: SourceResultProductSealV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceResultProductSealV1;

impl VerifiedSourceResultProductV1 {
    pub(crate) const fn catalog_identity(&self) -> usize {
        self.catalog_identity
    }

    pub(crate) fn owner(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.owner
    }

    pub(crate) const fn result(&self) -> SourceResultClassV1 {
        self.result
    }

    pub(crate) fn operations(&self) -> &[SourceValueOperationV1] {
        &self.operations
    }

    pub(crate) fn bool_facts(&self) -> &[SourceBoolFactV1] {
        &self.bool_facts
    }

    /// Every call observed while issuing this product, in source order, each
    /// carrying one explicit `Static | Dynamic | Absent` route disposition.
    pub(crate) fn call_dispositions(&self) -> &[SourceCallDispositionV1] {
        &self.call_dispositions
    }
}

/// Issue one source-result product from the sealed declaration row plus the
/// resolver-issued source ledger and the branded route catalog.
///
/// `input` and `call_targets` must agree with `declarations`/`owner`: the
/// co-sealed input must have been resolved from the exact declaration node
/// this catalog row sealed, the ledger is rebuilt from the same input, the
/// catalog must be branded by this exact declaration catalog, and every
/// parameter binding in the ledger must name the same declaration parameter.
pub(crate) fn issue_source_result_product_v1(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    call_targets: &VerifiedSourceCallTargetCatalogV1<'_>,
) -> Result<VerifiedSourceResultProductV1, SourceResultProductErrorV1> {
    let declaration = declarations
        .declaration(owner)
        .ok_or(SourceResultProductErrorV1::ForeignCallable)?;
    if !call_targets.is_branded_by(declarations) {
        return Err(SourceResultProductErrorV1::ForeignCallTargetCatalog);
    }
    verify_source_input_identity(declaration, input)?;
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| SourceResultProductErrorV1::ForeignLedger)?;
    verify_ledger_correspondence(declaration, &ledger)?;
    // The co-sealed input must carry the body-shape inventory for the same
    // owner: BlockExpr wrapper sites are proven by shape rows, not by AST.
    let body_shape = input
        .body_shape()
        .filter(|shape| shape.owner() == ledger.owner())
        .ok_or(SourceResultProductErrorV1::MissingBodyShape)?;
    let mut state = InferenceState {
        owner,
        declaration,
        catalog: declarations,
        ledger: &ledger,
        body_shape,
        call_targets,
        bindings: BTreeMap::new(),
        operations: Vec::new(),
        bool_facts: Vec::new(),
        call_dispositions: Vec::new(),
        returns: Vec::new(),
    };
    infer_statements(&mut state, declaration.body(), &[])?;
    let inferred = state
        .returns
        .iter()
        .copied()
        .try_fold(None, |class, next| match (class, next) {
            (None, class) => Ok(Some(class)),
            (Some(previous), class) if previous == class => Ok(Some(previous)),
            _ => Err(SourceResultProductErrorV1::MixedReturnClasses),
        })?
        .ok_or(SourceResultProductErrorV1::MissingReturn)?;
    if let Some(declared_name) = declaration.return_type_name() {
        let declared = SourceResultClassV1::from_declared_type(declared_name)
            .ok_or(SourceResultProductErrorV1::UnsupportedDeclaredResult)?;
        if declared != inferred {
            return Err(SourceResultProductErrorV1::DeclaredResultMismatch { declared, inferred });
        }
    }
    Ok(VerifiedSourceResultProductV1 {
        catalog_identity: declarations.brand().identity(),
        owner: owner.clone(),
        result: inferred,
        operations: state.operations.into_boxed_slice(),
        bool_facts: state.bool_facts.into_boxed_slice(),
        call_dispositions: state.call_dispositions.into_boxed_slice(),
        _seal: SourceResultProductSealV1,
    })
}

/// Ties the resolved input to the catalog declaration: the co-sealed lowering
/// input must carry the exact declaration node this catalog row sealed —
/// name, parameters, return type, uses, attributes, and the body including
/// source spans all match. Parameter names/count alone cannot distinguish a
/// same-shaped foreign declaration.
fn verify_source_input_identity(
    declaration: &VerifiedSameModuleCallableDeclarationV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<(), SourceResultProductErrorV1> {
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        attrs,
        ..
    } = input.source().root()
    else {
        return Err(SourceResultProductErrorV1::ForeignResolvedInput);
    };
    let matches = name.as_str() == declaration.key().name()
        && params.as_slice() == declaration.params()
        && param_decls.as_slice() == declaration.param_decls()
        && return_type_name.as_deref() == declaration.return_type_name()
        && body.as_slice() == declaration.body()
        && uses.as_slice() == declaration.uses()
        && attrs == declaration.attrs();
    if !matches {
        return Err(SourceResultProductErrorV1::ForeignResolvedInput);
    }
    Ok(())
}

/// Ties the borrowed ledger to the catalog declaration: same declared-function
/// source kind and identical parameter header names.
fn verify_ledger_correspondence(
    declaration: &VerifiedSameModuleCallableDeclarationV1,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<(), SourceResultProductErrorV1> {
    if ledger.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(SourceResultProductErrorV1::ForeignLedger);
    }
    let mut parameters = 0usize;
    for (_binding, record) in ledger.bindings() {
        let BindingKindV1::Parameter { index } = record.kind() else {
            continue;
        };
        parameters += 1;
        let declared = declaration
            .param_decls()
            .get(index as usize)
            .ok_or(SourceResultProductErrorV1::ForeignLedger)?;
        if declared.name.as_str() != record.diagnostic_name() {
            return Err(SourceResultProductErrorV1::ForeignLedger);
        }
    }
    if parameters != declaration.params().len() {
        return Err(SourceResultProductErrorV1::ForeignLedger);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprClassV1 {
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

struct InferenceState<'a> {
    owner: &'a CanonicalSameModuleCallableKeyV1,
    declaration: &'a VerifiedSameModuleCallableDeclarationV1,
    catalog: &'a VerifiedSameModuleCallableDeclarationCatalogV1,
    ledger: &'a CallableSemanticSourceLedgerView<'a>,
    body_shape: &'a VerifiedResolvedBodyShapeInventoryV1,
    call_targets: &'a VerifiedSourceCallTargetCatalogV1<'a>,
    bindings: BTreeMap<BindingRefV1, ExprClassV1>,
    operations: Vec<SourceValueOperationV1>,
    bool_facts: Vec<SourceBoolFactV1>,
    call_dispositions: Vec<SourceCallDispositionV1>,
    returns: Vec<SourceResultClassV1>,
}

fn infer_statements(
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
        ResolvedLiteralSourceV1::String => ExprClassV1::String,
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
