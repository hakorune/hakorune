//! AST-free typed source map for the bounded LoopCond branch profile.
//!
//! This product co-seals the sealed source projection with the typed
//! predicate/effect mapping the S6D D0 named missing: the carrier
//! declaration and both condition compare triples (binding read,
//! compare operator, integer bound) verified against resolver records.
//! It owns no Recipe, route, ValueId, CFG, PHI, or physical identity.

use crate::mir::loop_structural_facts::{
    VerifiedLoopCondBreakContinueSourceProjectionV1, VerifiedLoopRootSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondTypedSourceMapRejectV1 {
    ForeignOwner,
    SourceIdentity,
    LoopContextMismatch,
    SourceBinding,
    ConditionShape(LoopCondTypedMapRoleV1),
    UnsupportedOperator(LoopCondTypedMapRoleV1),
    MissingVariableReference(LoopCondTypedMapRoleV1),
    DuplicateEvidence(LoopCondTypedMapRoleV1),
    BindingMismatch(LoopCondTypedMapRoleV1),
    DistinctConditionBindings,
    CarrierNotDeclared,
    CarrierInitializerNotInteger,
    ResidualVariableRef,
    ResidualExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LoopCondTypedMapRoleV1 {
    CarrierDeclaration,
    LoopConditionRead,
    LoopConditionOperator,
    LoopConditionBound,
    BranchConditionRead,
    BranchConditionOperator,
    BranchConditionBound,
}

/// Typed compare triple for one bounded LoopCond condition:
/// `<binding read> <operator> <integer bound>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopCondTypedCompareV1 {
    pub(crate) read_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) operator: SyntaxBinaryOperatorV1,
    pub(crate) bound_site: SourceExprSiteV1,
    pub(crate) bound: i64,
}

/// The declared local both conditions read; the recipe binds it as the
/// loop carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopCondCarrierDeclarationV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) initializer_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) literal: i64,
}

/// Co-sealed projection + typed predicate/effect map. The projection is
/// retained so the policy demand can re-check the exact frame identity;
/// the root source stays sealed until the verified Recipe claims it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinueTypedSourceMapV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    source_binding: VerifiedLoopRootSourceV1,
    projection: VerifiedLoopCondBreakContinueSourceProjectionV1,
    carrier: LoopCondCarrierDeclarationV1,
    loop_condition: LoopCondTypedCompareV1,
    branch_condition: LoopCondTypedCompareV1,
}

impl VerifiedLoopCondBreakContinueTypedSourceMapV1 {
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_frame_key: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
        source_binding: VerifiedLoopRootSourceV1,
        projection: VerifiedLoopCondBreakContinueSourceProjectionV1,
        carrier: LoopCondCarrierDeclarationV1,
        loop_condition: LoopCondTypedCompareV1,
        branch_condition: LoopCondTypedCompareV1,
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
            loop_condition,
            branch_condition,
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

    pub(crate) fn projection(&self) -> &VerifiedLoopCondBreakContinueSourceProjectionV1 {
        &self.projection
    }

    pub(crate) fn carrier(&self) -> &LoopCondCarrierDeclarationV1 {
        &self.carrier
    }

    pub(crate) fn loop_condition(&self) -> &LoopCondTypedCompareV1 {
        &self.loop_condition
    }

    pub(crate) fn branch_condition(&self) -> &LoopCondTypedCompareV1 {
        &self.branch_condition
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopRootSourceV1,
        VerifiedLoopCondBreakContinueSourceProjectionV1,
        LoopCondCarrierDeclarationV1,
        LoopCondTypedCompareV1,
        LoopCondTypedCompareV1,
        LoopExecutionFrameKeyV1,
    ) {
        (
            self.source_binding,
            self.projection,
            self.carrier,
            self.loop_condition,
            self.branch_condition,
            self.root_frame_key,
        )
    }
}
