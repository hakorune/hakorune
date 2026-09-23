//! AST-free typed source map for the bounded Generic residual profile.
//!
//! This product co-seals the sealed source projection with the typed
//! predicate/effect mapping: the condition compare triple (carrier read,
//! compare operator, integer or binding bound), the body-local integer
//! declarations, the integer rebinds, and the terminal carrier step —
//! all verified against resolver records. It owns no Recipe, route,
//! ValueId, CFG, PHI, or physical identity.

use crate::mir::loop_structural_facts::{
    VerifiedGenericResidualSourceProjectionV1, VerifiedLoopRootSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericResidualTypedSourceMapRejectV1 {
    ForeignOwner,
    SourceIdentity,
    LoopContextMismatch,
    SourceBinding,
    ConditionShape,
    UnsupportedOperator,
    ConditionBoundShape,
    MissingVariableReference(GenericResidualTypedMapRoleV1),
    DuplicateEvidence(GenericResidualTypedMapRoleV1),
    BindingMismatch(GenericResidualTypedMapRoleV1),
    DeclarationArity,
    InitializerNotInteger,
    RebindTargetMismatch,
    RebindShape,
    StepOperator,
    StepDeltaNotInteger,
    StepReadMismatch,
    RebindTargetForeign,
    CarrierStepMissing,
    CarrierNotDeclared,
    CarrierDeclaredInBody,
    ResidualVariableRef,
    ResidualAssignmentTarget,
    ResidualExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericResidualTypedMapRoleV1 {
    ConditionRead,
    ConditionBound,
    BodyRebind,
    CarrierStepRead,
}

/// The right operand of the bounded condition compare: an integer
/// literal or a second declared binding read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericResidualBoundV1 {
    Integer { site: SourceExprSiteV1, value: i64 },
    Binding { site: SourceExprSiteV1, binding: BindingRefV1 },
}

/// Typed compare triple for the bounded loop condition:
/// `<carrier read> <operator> <bound>` with the operator in the
/// `LoopCompareI64OpV1` vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericResidualTypedCompareV1 {
    pub(crate) read_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) operator: SyntaxBinaryOperatorV1,
    pub(crate) bound: GenericResidualBoundV1,
}

/// One `local <name> = <integer>` declaration inside the loop body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericResidualBodyDeclV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) ordinal: u32,
    pub(crate) initializer_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) literal: i64,
}

/// One `<var> = <same var> <+|-> <integer>` assignment row. Used for
/// body-local rebinds and for the terminal carrier step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericResidualBodyStepV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) target_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) read_site: SourceExprSiteV1,
    pub(crate) operator: SyntaxBinaryOperatorV1,
    pub(crate) delta_site: SourceExprSiteV1,
    pub(crate) delta: i64,
}

/// One body statement before the terminal carrier step, in source
/// order. Body-local declarations and rebinds stay ordered so the
/// producer can replay per-iteration SSA value flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericResidualBodyRowV1 {
    Declaration(GenericResidualBodyDeclV1),
    Rebind(GenericResidualBodyStepV1),
}

/// Co-sealed projection + typed predicate/effect map. The projection is
/// retained so the policy demand can re-check the exact frame identity;
/// the root source stays sealed until the verified Recipe claims it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResidualTypedSourceMapV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    source_binding: VerifiedLoopRootSourceV1,
    projection: VerifiedGenericResidualSourceProjectionV1,
    carrier: BindingRefV1,
    carrier_declaration: SourceBindingSiteV1,
    condition: GenericResidualTypedCompareV1,
    body_rows: Box<[GenericResidualBodyRowV1]>,
    carrier_step: GenericResidualBodyStepV1,
}

impl VerifiedGenericResidualTypedSourceMapV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_frame_key: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
        source_binding: VerifiedLoopRootSourceV1,
        projection: VerifiedGenericResidualSourceProjectionV1,
        carrier: BindingRefV1,
        carrier_declaration: SourceBindingSiteV1,
        condition: GenericResidualTypedCompareV1,
        body_rows: Box<[GenericResidualBodyRowV1]>,
        carrier_step: GenericResidualBodyStepV1,
    ) -> Self {
        Self {
            owner,
            function_origin,
            source_kind,
            root_frame_key,
            scope_region,
            source_binding,
            projection,
            carrier,
            carrier_declaration,
            condition,
            body_rows,
            carrier_step,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(crate) fn projection(&self) -> &VerifiedGenericResidualSourceProjectionV1 {
        &self.projection
    }

    pub(crate) const fn carrier(&self) -> BindingRefV1 {
        self.carrier
    }

    pub(crate) const fn carrier_declaration(&self) -> &SourceBindingSiteV1 {
        &self.carrier_declaration
    }

    pub(crate) fn condition(&self) -> &GenericResidualTypedCompareV1 {
        &self.condition
    }

    pub(crate) fn body_rows(&self) -> &[GenericResidualBodyRowV1] {
        &self.body_rows
    }

    pub(crate) fn carrier_step(&self) -> &GenericResidualBodyStepV1 {
        &self.carrier_step
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopRootSourceV1,
        VerifiedGenericResidualSourceProjectionV1,
        BindingRefV1,
        GenericResidualTypedCompareV1,
        Box<[GenericResidualBodyRowV1]>,
        GenericResidualBodyStepV1,
        LoopExecutionFrameKeyV1,
    ) {
        (
            self.source_binding,
            self.projection,
            self.carrier,
            self.condition,
            self.body_rows,
            self.carrier_step,
            self.root_frame_key,
        )
    }
}
