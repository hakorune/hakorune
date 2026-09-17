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
    BindingKindV1, CallableSemanticSourceLedgerView, SemanticOwnerSourceKindV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};
use crate::mir::source_call_target::VerifiedSourceCallTargetCatalogV1;
use classify::{infer_statements, InferenceState};

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
    NonEmptyBlockExprPrelude(SourceExprSiteV1),
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

#[path = "source_result_classify.rs"]
mod classify;
