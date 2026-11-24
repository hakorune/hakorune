//! Outward Fault from an original direct call in one verified Loop body.
//! Home discharge and physical cleanup remain separate obligations.
use super::*;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, LoopExecutionFrameKeyV1, ScopeId, SourcePathSegmentV1};

/// Genuine nested expression Fault, independently of direct-call statement
/// morphology. This proves only outward control; discharge is product-owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopExpressionFaultContinuationV1 {
    site: OwnedExprSiteV1,
    loop_site: SourceStmtSiteV1,
    source_scope: ScopeId,
    target_function: RegionId,
    crossed_scopes: Box<[ScopeId]>,
}
impl LoopExpressionFaultContinuationV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 { &self.site }
    pub(crate) fn target_function(&self) -> RegionId { self.target_function }
    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] { &self.crossed_scopes }
}
pub(crate) fn issue_loop_expression_fault_continuation_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, loop_site: &SourceStmtSiteV1,
    frame: &LoopExecutionFrameKeyV1, site: &OwnedExprSiteV1,
) -> Result<LoopExpressionFaultContinuationV1, &'static str> {
    let function = input.function();
    if site.owner() != input.owner() || function.owner() != input.owner()
        || !input.forest().owner(input.owner()).is_some_and(|owner| std::ptr::eq(owner, function)) {
        return Err("loop-expression-fault/foreign-owner");
    }
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| "loop-expression-fault/source-ledger")?;
    let original = ledger.resolved_loop_source(loop_site).map_err(|_| "loop-expression-fault/source-loop")?;
    if !original.source().matches_identity(function.function_origin(), function.source_kind(), loop_site)
        || !original.source().frame_key().matches(frame)
        || !ledger.source_site_inventory().expression_sites().any(|original| original == site.site()) {
        return Err("loop-expression-fault/source-membership");
    }
    let relative = site.site().node().segments().strip_prefix(loop_site.node().segments())
        .ok_or("loop-expression-fault/source-path")?;
    let is_body = match relative.first() {
        Some(SourcePathSegmentV1::LoopBody(_)) => true,
        Some(SourcePathSegmentV1::LoopCondition) => false,
        _ => return Err("loop-expression-fault/source-expression"),
    };
    let pair = function.loop_region_bundle(loop_site).map_err(|_| "loop-expression-fault/source-pair")?.loop_pair();
    let scope = function.exact_scope_containing(site.site().node()).ok_or("loop-expression-fault/source-scope")?;
    let parent = function.scope(pair.scope()).and_then(|scope| scope.parent()).ok_or("loop-expression-fault/parent-scope")?;
    if function.region(pair.region()).and_then(|region| region.parent())
        != function.scope(parent).map(|scope| scope.owner_region()) { return Err("loop-expression-fault/source-ancestry"); }
    let roots = function.lowering_roots();
    let target = roots.function_pair().region();
    if target != function.function_region() || function.region(roots.body_pair().region())
        .and_then(|region| region.parent()) != Some(target) { return Err("loop-expression-fault/function-target"); }
    let mut crossed = if is_body {
        let mut scopes = prove_outward_scope_ancestry(function, scope, pair.scope())?.into_vec();
        scopes.push(pair.scope());
        scopes.extend_from_slice(&prove_outward_scope_ancestry(function, parent, roots.body_pair().scope())?);
        scopes
    } else { prove_outward_scope_ancestry(function, scope, roots.body_pair().scope())?.into_vec() };
    // No repeated scope can masquerade as an outward path.
    let mut unique = std::collections::BTreeSet::new();
    if crossed.iter().any(|scope| !unique.insert(*scope)) { return Err("loop-expression-fault/scope-cycle"); }
    Ok(LoopExpressionFaultContinuationV1 { site: site.clone(), loop_site: loop_site.clone(),
        source_scope: scope, target_function: target, crossed_scopes: std::mem::take(&mut crossed).into_boxed_slice() })
}
