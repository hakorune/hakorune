//! AST-free, Home-class-backed Facts for one variable-bound multiply Loop.
//!
//! The compiler source observer owns shape; the running Home prefix owns the
//! three input classes. This issuer only joins their exact identities and
//! retains the resolver's move-only Loop source for the Recipe producer.

use crate::mir::resolved_semantics::{
    home_new_prefix::LoopI64PreStateRequestV1, BindingRefV1, FunctionOwnerIdV1,
    LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableBoundMulFactsIssueV1 {
    ForeignOwner,
    SourceSiteConflict,
    ForeignFrame,
    InputClassMissing,
    DuplicateOperation,
}

/// No Recipe key, physical identity, or post-loop class is issued here.
#[derive(Debug)]
pub(crate) struct VerifiedVariableBoundMulFactsV1 {
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    condition: SourceExprSiteV1,
    condition_operands: [SourceExprSiteV1; 2],
    operations: [(SourceStmtSiteV1, SourceExprSiteV1); 2],
    operation_operands: [[SourceExprSiteV1; 3]; 2],
    bindings: [BindingRefV1; 3],
    inputs: [(SourceBindingSiteV1, SourceExprSiteV1); 3],
    _seal: VariableBoundMulFactsSealV1,
}

#[derive(Debug)]
struct VariableBoundMulFactsSealV1;

impl VerifiedVariableBoundMulFactsV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.source
    }
    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
    pub(crate) fn condition(&self) -> &SourceExprSiteV1 {
        &self.condition
    }
    pub(crate) fn operations(&self) -> &[(SourceStmtSiteV1, SourceExprSiteV1); 2] {
        &self.operations
    }
    pub(crate) fn condition_operands(&self) -> &[SourceExprSiteV1; 2] {
        &self.condition_operands
    }
    pub(crate) fn operation_operands(&self) -> &[[SourceExprSiteV1; 3]; 2] {
        &self.operation_operands
    }
    pub(crate) fn inputs(&self) -> &[(SourceBindingSiteV1, SourceExprSiteV1); 3] {
        &self.inputs
    }
    pub(crate) const fn bindings(&self) -> [BindingRefV1; 3] {
        self.bindings
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        VerifiedResolvedLoopSourceV1,
        LoopExecutionFrameKeyV1,
        ResolvedScopeRegionPairV1,
        SourceExprSiteV1,
        [SourceExprSiteV1; 2],
        [(SourceStmtSiteV1, SourceExprSiteV1); 2],
        [[SourceExprSiteV1; 3]; 2],
        [BindingRefV1; 3],
        [(SourceBindingSiteV1, SourceExprSiteV1); 3],
    ) {
        (
            self.owner,
            self.source,
            self.frame,
            self.scope_region,
            self.condition,
            self.condition_operands,
            self.operations,
            self.operation_operands,
            self.bindings,
            self.inputs,
        )
    }
}

pub(crate) fn issue_variable_bound_mul_facts_v1(
    owner: FunctionOwnerIdV1,
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    condition: SourceExprSiteV1,
    condition_operands: [SourceExprSiteV1; 2],
    operations: [(SourceStmtSiteV1, SourceExprSiteV1); 2],
    operation_operands: [[SourceExprSiteV1; 3]; 2],
    bindings: [BindingRefV1; 3],
    inputs: [(SourceBindingSiteV1, SourceExprSiteV1); 3],
    prestate: LoopI64PreStateRequestV1,
) -> Result<VerifiedVariableBoundMulFactsV1, VariableBoundMulFactsIssueV1> {
    use VariableBoundMulFactsIssueV1 as Issue;
    if prestate.owner() != owner || bindings.iter().any(|binding| binding.owner() != owner) {
        return Err(Issue::ForeignOwner);
    }
    if scope_region.scope().owner() != owner || scope_region.region().owner() != owner {
        return Err(Issue::ForeignFrame);
    }
    if !frame.matches(&source.frame_key()) {
        return Err(Issue::ForeignFrame);
    }
    if source.site() != prestate.site() {
        return Err(Issue::SourceSiteConflict);
    }
    if bindings[0] == bindings[1]
        || bindings[0] == bindings[2]
        || bindings[1] == bindings[2]
        || operations[0].0 == operations[1].0
    {
        return Err(Issue::DuplicateOperation);
    }
    if !bindings
        .into_iter()
        .all(|binding| prestate.proves_integer(binding))
    {
        return Err(Issue::InputClassMissing);
    }
    Ok(VerifiedVariableBoundMulFactsV1 {
        owner,
        source,
        frame,
        scope_region,
        condition,
        condition_operands,
        operations,
        operation_operands,
        bindings,
        inputs,
        _seal: VariableBoundMulFactsSealV1,
    })
}
