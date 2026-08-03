//! AST-free Direct Accum structural vocabulary.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DirectAccumUpdateShapeV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) target_site: SourceExprSiteV1,
    pub(crate) value_site: SourceExprSiteV1,
    pub(crate) lhs_site: SourceExprSiteV1,
    pub(crate) rhs_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) delta: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DirectAccumStructuralShapeV1 {
    pub(crate) condition_site: SourceExprSiteV1,
    pub(crate) condition_lhs_site: SourceExprSiteV1,
    pub(crate) condition_binding: BindingRefV1,
    pub(crate) condition_bound: i64,
    pub(crate) update: DirectAccumUpdateShapeV1,
    pub(crate) step: DirectAccumUpdateShapeV1,
    pub(crate) induction: BindingRefV1,
    pub(crate) accumulator: BindingRefV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DirectAccumObservedShapeV1 {
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) owner_source_kind: SemanticOwnerSourceKindV1,
    pub(crate) loop_site: SourceStmtSiteV1,
    pub(crate) frame_key: LoopExecutionFrameKeyV1,
    pub(crate) shape: DirectAccumStructuralShapeV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopStructuralFactsPayloadV1 {
    IdentityOnly,
    DirectAccum(DirectAccumStructuralShapeV1),
}
