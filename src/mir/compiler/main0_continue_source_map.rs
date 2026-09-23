//! Caller-zero, AST-free join schema for the Main0 Continue source map.
//!
//! This product is deliberately only a source map. It owns no Recipe,
//! ValueId, CFG, PHI, Builder route, or physical policy. SyntaxFacts owns
//! neutral source shapes; the ledger owns resolver identity. This box merely
//! co-seals their exact sites before the Recipe draft is opened.
//!
//! The map assigns the loop-carried roles used by Recipe emission: the
//! loop-head left operand is the carrier read, the right operand is the
//! read-only bound read, and both step assignments rebind the carrier through
//! the resolver's rebind records. The `continue` statement is resolved against
//! the loop scope-region identity: `ResolvedControlTransferV1::Continue` must
//! name the same `RegionId` as the loop membership's scope region.
//!
//! Issuance (the ledger join) lives in `main0_continue_source_map_issue.rs`;
//! this file only owns the row schema and the sealed product type.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, RegionId,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceV1,
};

use super::callable_single_loop_source_shapes::{SourceLiteralShapeV1, SyntaxBinaryOperatorV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Main0ContinueMapRoleV1 {
    CarrierDeclaration,
    BoundDeclaration,
    ConditionCarrierRead,
    ConditionBoundRead,
    ConditionOperator,
    GuardCarrierRead,
    GuardBound,
    GuardOperator,
    ThenStepRead,
    ThenStepDelta,
    ThenStepOperator,
    ThenStepWrite,
    ContinueTransfer,
    NormalStepRead,
    NormalStepDelta,
    NormalStepOperator,
    NormalStepWrite,
    TailReturnRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Main0ContinueMapSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

impl Main0ContinueMapSiteV1 {
    pub(crate) fn expression(&self) -> Option<&SourceExprSiteV1> {
        match self {
            Self::Expression(site) => Some(site),
            Self::Statement(_) => None,
        }
    }

    pub(crate) fn statement(&self) -> Option<&SourceStmtSiteV1> {
        match self {
            Self::Statement(site) => Some(site),
            Self::Expression(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Main0ContinueMapTargetV1 {
    LocalDeclaration {
        binding: BindingRefV1,
        literal: SourceLiteralShapeV1,
    },
    Binding(BindingRefV1),
    Literal(SourceLiteralShapeV1),
    Operator(SyntaxBinaryOperatorV1),
    Continue {
        target_loop: RegionId,
    },
    Tail {
        statement: SourceStmtSiteV1,
        binding: BindingRefV1,
    },
}

impl Main0ContinueMapTargetV1 {
    pub(crate) fn local_declaration(&self) -> Option<(BindingRefV1, &SourceLiteralShapeV1)> {
        match self {
            Self::LocalDeclaration { binding, literal } => Some((*binding, literal)),
            _ => None,
        }
    }

    pub(crate) fn binding(&self) -> Option<BindingRefV1> {
        match self {
            Self::Binding(binding) => Some(*binding),
            Self::LocalDeclaration { binding, .. } => Some(*binding),
            Self::Tail { binding, .. } => Some(*binding),
            _ => None,
        }
    }

    pub(crate) fn literal(&self) -> Option<&SourceLiteralShapeV1> {
        match self {
            Self::Literal(literal) => Some(literal),
            Self::LocalDeclaration { literal, .. } => Some(literal),
            _ => None,
        }
    }

    pub(crate) fn operator(&self) -> Option<SyntaxBinaryOperatorV1> {
        match self {
            Self::Operator(operator) => Some(*operator),
            _ => None,
        }
    }

    pub(crate) fn continue_target(&self) -> Option<RegionId> {
        match self {
            Self::Continue { target_loop } => Some(*target_loop),
            _ => None,
        }
    }

    pub(crate) fn tail(&self) -> Option<(&SourceStmtSiteV1, BindingRefV1)> {
        match self {
            Self::Tail { statement, binding } => Some((statement, *binding)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0ContinueMapRowV1 {
    site: Main0ContinueMapSiteV1,
    role: Main0ContinueMapRoleV1,
    target: Main0ContinueMapTargetV1,
}

impl Main0ContinueMapRowV1 {
    pub(crate) const fn statement(
        site: SourceStmtSiteV1,
        role: Main0ContinueMapRoleV1,
        target: Main0ContinueMapTargetV1,
    ) -> Self {
        Self {
            site: Main0ContinueMapSiteV1::Statement(site),
            role,
            target,
        }
    }

    pub(crate) const fn expression(
        site: SourceExprSiteV1,
        role: Main0ContinueMapRoleV1,
        target: Main0ContinueMapTargetV1,
    ) -> Self {
        Self {
            site: Main0ContinueMapSiteV1::Expression(site),
            role,
            target,
        }
    }

    pub(crate) fn site(&self) -> &Main0ContinueMapSiteV1 {
        &self.site
    }

    pub(crate) const fn role(&self) -> Main0ContinueMapRoleV1 {
        self.role
    }

    pub(crate) fn target(&self) -> &Main0ContinueMapTargetV1 {
        &self.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Main0ContinueSourceMapRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    InvalidInitialLocal,
    MissingSourceSite(Main0ContinueMapRoleV1),
    DuplicateEvidence(Main0ContinueMapRoleV1),
    MissingVariableReference(Main0ContinueMapRoleV1),
    BindingMismatch(Main0ContinueMapRoleV1),
    CarrierIsBound,
    MissingAssignmentTarget(Main0ContinueMapRoleV1),
    UnsupportedAssignmentTarget(Main0ContinueMapRoleV1),
    BoundWritten,
    ResidualRebind,
    UnsupportedLiteral(Main0ContinueMapRoleV1),
    UnsupportedOperator(Main0ContinueMapRoleV1),
    MissingContinueTransfer,
    NonLoopContinueTransfer,
    MissingTerminalReturn,
    NonTerminalReturn,
    ResidualCall,
    ResidualExit,
    ResidualVariableRef,
}

/// Owned caller-zero source map. The loop source, frame, and scope/region pair
/// are consumed from the resolver-issued lookup; neither can be minted from a
/// route or AST.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0ContinueSourceMapV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_source: VerifiedResolvedLoopSourceV1,
    loop_frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    if_site: SourceStmtSiteV1,
    continue_site: SourceStmtSiteV1,
    rows: Box<[Main0ContinueMapRowV1]>,
    _seal: VerifiedMain0ContinueSourceMapSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedMain0ContinueSourceMapSealV1;

impl VerifiedMain0ContinueSourceMapV1 {
    /// Seal a completed map. Only `main0_continue_source_map_issue` calls
    /// this; it is the sole production sealer.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn seal(
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        loop_source: VerifiedResolvedLoopSourceV1,
        loop_frame: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
        if_site: SourceStmtSiteV1,
        continue_site: SourceStmtSiteV1,
        rows: Vec<Main0ContinueMapRowV1>,
    ) -> Self {
        Self {
            owner,
            origin,
            source_kind,
            loop_source,
            loop_frame,
            scope_region,
            if_site,
            continue_site,
            rows: rows.into_boxed_slice(),
            _seal: VerifiedMain0ContinueSourceMapSealV1,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.loop_source
    }

    pub(crate) fn loop_frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.loop_frame
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    /// Source site of the in-loop `if` statement that owns the continue arm.
    pub(crate) fn if_site(&self) -> &SourceStmtSiteV1 {
        &self.if_site
    }

    /// Source site of the `continue` statement.
    pub(crate) fn continue_site(&self) -> &SourceStmtSiteV1 {
        &self.continue_site
    }

    pub(crate) fn rows(&self) -> &[Main0ContinueMapRowV1] {
        &self.rows
    }

    /// Test-only rebuild used to exercise missing/extra-row rejection at the
    /// co-seal boundary. The production issuer is the only sealer.
    #[cfg(test)]
    pub(crate) fn rebuild_for_test(
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        loop_source: VerifiedResolvedLoopSourceV1,
        loop_frame: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
        if_site: SourceStmtSiteV1,
        continue_site: SourceStmtSiteV1,
        rows: Vec<Main0ContinueMapRowV1>,
    ) -> Self {
        Self::seal(
            owner,
            origin,
            source_kind,
            loop_source,
            loop_frame,
            scope_region,
            if_site,
            continue_site,
            rows,
        )
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        FunctionOriginV1,
        SemanticOwnerSourceKindV1,
        VerifiedResolvedLoopSourceV1,
        LoopExecutionFrameKeyV1,
        ResolvedScopeRegionPairV1,
        SourceStmtSiteV1,
        SourceStmtSiteV1,
        Box<[Main0ContinueMapRowV1]>,
    ) {
        (
            self.owner,
            self.origin,
            self.source_kind,
            self.loop_source,
            self.loop_frame,
            self.scope_region,
            self.if_site,
            self.continue_site,
            self.rows,
        )
    }
}
