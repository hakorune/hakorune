//! Outward Fault from an original direct call in one verified Loop body.
//! Home discharge and physical cleanup remain separate obligations.
use super::*;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, LoopExecutionFrameKeyV1, ScopeId, ResolvedLoopPlacementV1, SourcePathSegmentV1, BodyChildRoleV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopCallFaultContinuationV1 {
    call: OwnedExprSiteV1,
    loop_site: SourceStmtSiteV1,
    source_scope: ScopeId,
    target_function: RegionId,
    crossed_scopes: Box<[ScopeId]>,
}
impl LoopCallFaultContinuationV1 {
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 { &self.call }
    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 { &self.loop_site }
    pub(crate) fn target_function(&self) -> RegionId { self.target_function }
    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] { &self.crossed_scopes }
}

pub(crate) fn issue_loop_call_fault_continuation_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, loop_site: &SourceStmtSiteV1,
    frame: &LoopExecutionFrameKeyV1, call: &OwnedExprSiteV1,
) -> Result<LoopCallFaultContinuationV1, &'static str> {
    let function = input.function();
    if input.owner() != function.owner() || call.owner() != input.owner()
        || !input.forest().owner(input.owner()).is_some_and(|owner| std::ptr::eq(owner, function))
    { return Err("loop-call-fault/foreign-owner"); }
    let ledger = input.forest().callable_source_ledger(input.owner())
        .map_err(|_| "loop-call-fault/source-ledger")?;
    let original = ledger.resolved_loop_source(loop_site)
        .map_err(|_| "loop-call-fault/source-loop")?;
    if !original.source().matches_identity(function.function_origin(), function.source_kind(), loop_site)
        || !original.source().frame_key().matches(frame)
        || function.resolved_loop_placement(loop_site, call.site()) != Ok(Some(ResolvedLoopPlacementV1::Body))
        || function.method_call(call.site()).is_none()
    { return Err("loop-call-fault/source-membership"); }
    let statement = SourceStmtSiteV1::from_node(call.site().node().clone());
    let relative = call.site().node().segments().strip_prefix(loop_site.node().segments())
        .ok_or("loop-call-fault/source-path")?;
    let [SourcePathSegmentV1::LoopBody(index)] = relative else { return Err("loop-call-fault/not-direct-body-call"); };
    let source = input.source();
    let root = source.stmt_at(&original).map_err(|_| "loop-call-fault/source-loop-statement")?;
    let body = source.child_body_from_stmt(&root, BodyChildRoleV1::LoopBody)
        .map_err(|_| "loop-call-fault/source-body")?;
    let located = source.body_stmt(&body, *index as usize).map_err(|_| "loop-call-fault/source-call")?;
    if located.site() != &statement || !matches!(located.node(), ASTNode::MethodCall { .. }) {
        return Err("loop-call-fault/not-call");
    }
    let pair = function.loop_region_bundle(loop_site).map_err(|_| "loop-call-fault/source-pair")?.loop_pair();
    let scope = function.exact_scope_containing(call.site().node()).ok_or("loop-call-fault/source-scope")?;
    let parent = function.scope(pair.scope()).and_then(|scope| scope.parent())
        .ok_or("loop-call-fault/parent-scope")?;
    if scope != pair.scope() || function.region(pair.region()).and_then(|region| region.parent())
        != function.scope(parent).map(|scope| scope.owner_region())
    { return Err("loop-call-fault/source-ancestry"); }
    let roots = function.lowering_roots();
    let target = roots.function_pair().region();
    if target != function.function_region() || function.region(roots.body_pair().region())
        .and_then(|region| region.parent()) != Some(target)
    { return Err("loop-call-fault/function-target"); }
    let mut crossed = vec![scope];
    crossed.extend_from_slice(&prove_branch_scope_ancestry(function, parent, roots.body_pair().scope())?);
    Ok(LoopCallFaultContinuationV1 { call: call.clone(), loop_site: loop_site.clone(),
        source_scope: scope, target_function: target, crossed_scopes: crossed.into_boxed_slice() })
}
