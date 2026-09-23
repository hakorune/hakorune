//! Caller-zero, AST-free join schema for the Main0 in-body-step source map.
//!
//! This product is deliberately only a source map. It owns no Recipe,
//! ValueId, CFG, PHI, Builder route, or physical policy. SyntaxFacts owns
//! neutral source shapes; the ledger owns resolver identity. This box merely
//! co-seals their exact sites before the Recipe draft is opened.
//!
//! The map assigns the loop-carried roles used by Recipe emission: the
//! loop-head left operand is the carrier read, the right operand is an
//! integer literal bound (no binding read), the first body statement rebinds
//! the carrier through the resolver's rebind record, and the second body
//! statement is the write-only `<effect> = <integer>` rebind the callable
//! loop handoff refuses. No explicit transfer exists; the predicate's false
//! edge is the only loop exit.
//!
//! Issuance (the ledger join) lives in
//! `main0_in_body_step_source_map_issue.rs`; this file only owns the row
//! schema and the sealed product type.

use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

use super::main0_continue_source_map::{Main0ContinueMapSiteV1, Main0ContinueMapTargetV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Main0InBodyStepMapRoleV1 {
    CarrierDeclaration,
    EffectDeclaration,
    ConditionCarrierRead,
    ConditionBound,
    ConditionOperator,
    StepRead,
    StepDelta,
    StepOperator,
    StepWrite,
    EffectValue,
    EffectWrite,
    TailReturnRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0InBodyStepMapRowV1 {
    site: Main0ContinueMapSiteV1,
    role: Main0InBodyStepMapRoleV1,
    target: Main0ContinueMapTargetV1,
}

impl Main0InBodyStepMapRowV1 {
    pub(crate) const fn statement(
        site: SourceStmtSiteV1,
        role: Main0InBodyStepMapRoleV1,
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
        role: Main0InBodyStepMapRoleV1,
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

    pub(crate) const fn role(&self) -> Main0InBodyStepMapRoleV1 {
        self.role
    }

    pub(crate) fn target(&self) -> &Main0ContinueMapTargetV1 {
        &self.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Main0InBodyStepSourceMapRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    InvalidInitialLocal,
    MissingSourceSite(Main0InBodyStepMapRoleV1),
    DuplicateEvidence(Main0InBodyStepMapRoleV1),
    MissingVariableReference(Main0InBodyStepMapRoleV1),
    BindingMismatch(Main0InBodyStepMapRoleV1),
    CarrierIsEffect,
    MissingAssignmentTarget(Main0InBodyStepMapRoleV1),
    UnsupportedAssignmentTarget(Main0InBodyStepMapRoleV1),
    CarrierWrittenElsewhere,
    ResidualRebind,
    UnsupportedLiteral(Main0InBodyStepMapRoleV1),
    UnsupportedOperator(Main0InBodyStepMapRoleV1),
    MissingTerminalReturn,
    NonTerminalReturn,
    ResidualCall,
    ResidualExit,
    ResidualVariableRef,
}

/// Owned caller-zero source map. The loop source, frame, and scope/region
/// pair are consumed from the resolver-issued lookup; neither can be minted
/// from a route or AST.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0InBodyStepSourceMapV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_source: VerifiedResolvedLoopSourceV1,
    loop_frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    effect_site: SourceStmtSiteV1,
    rows: Box<[Main0InBodyStepMapRowV1]>,
    _seal: VerifiedMain0InBodyStepSourceMapSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedMain0InBodyStepSourceMapSealV1;

impl VerifiedMain0InBodyStepSourceMapV1 {
    /// Seal a completed map. Only `main0_in_body_step_source_map_issue`
    /// calls this; it is the sole production sealer.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn seal(
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        loop_source: VerifiedResolvedLoopSourceV1,
        loop_frame: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
        effect_site: SourceStmtSiteV1,
        rows: Vec<Main0InBodyStepMapRowV1>,
    ) -> Self {
        Self {
            owner,
            origin,
            source_kind,
            loop_source,
            loop_frame,
            scope_region,
            effect_site,
            rows: rows.into_boxed_slice(),
            _seal: VerifiedMain0InBodyStepSourceMapSealV1,
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

    /// Source site of the write-only `<effect> = <integer>` statement.
    pub(crate) fn effect_site(&self) -> &SourceStmtSiteV1 {
        &self.effect_site
    }

    pub(crate) fn rows(&self) -> &[Main0InBodyStepMapRowV1] {
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
        effect_site: SourceStmtSiteV1,
        rows: Vec<Main0InBodyStepMapRowV1>,
    ) -> Self {
        Self::seal(
            owner,
            origin,
            source_kind,
            loop_source,
            loop_frame,
            scope_region,
            effect_site,
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
        Box<[Main0InBodyStepMapRowV1]>,
    ) {
        (
            self.owner,
            self.origin,
            self.source_kind,
            self.loop_source,
            self.loop_frame,
            self.scope_region,
            self.effect_site,
            self.rows,
        )
    }
}
