//! Source-purpose connection at the original actual observation. No Home rights.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};

pub(super) fn corroborates_argument_source_v1(
    plan: &ExpressionSourcePlanV1,
    source: &StaticIncomingSourceV1,
    ordinal: u32,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
) -> bool {
    matches!(plan.root(), ExpressionPlanRootV1::CallArgument { call, ordinal: original, .. }
        if *original == ordinal && std::sync::Arc::ptr_eq(call, source.method_source()))
        && plan.owner() == source.call_site().owner() && method.owner() == plan.owner()
        && source.argument_sites().get(ordinal as usize) == Some(method.site())
        && plan.nodes().first().is_some_and(|node| node.site() == method.site()
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall))
}

/// Re-observation borrows the existing original Rc; structural equality cannot
/// replace that loan or relax the existing repeated-walk-drift boundary.
pub(in crate::mir::normal_callable_semantic_package) fn bind_static_argument_sources_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
    rows: Option<&mut PreparedBorrowedCallActualsV1>,
) -> Result<(), String> {
    let Some(rows) = rows else { return Ok(()) };
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else { return Ok(()) };
    if identity.core_arguments.is_empty() { return Ok(()); }
    let reject = || freeze("borrowed-static/core-source-identity");
    let old = previous.map(|rows| {
        // The new preparation is already Ok here. A preceding Err differs
        // exactly as it did at the existing staging boundary; never replay it.
        let rows = rows.as_ref().map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
        let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &rows.phase else { return Err(reject()) };
        if !Rc::ptr_eq(&old.source, &identity.source) || old.candidates != identity.candidates
            || old.core_arguments.len() != identity.core_arguments.len()
            || old.argument_sources.len() != old.core_arguments.len()
        { return Err(reject()); }
        Ok(old)
    }).transpose()?;
    if input.owner() != site.owner() || identity.source.call_site() != site
        || !input.function().method_call_source_v1(site.site()).is_some_and(|call|
            std::sync::Arc::ptr_eq(call, identity.source.method_source()))
    { return Err(reject()); }
    let mut plans = BTreeMap::new();
    for (ordinal, receipt) in &identity.core_arguments {
        let candidate = identity.candidates.get(*ordinal as usize).filter(|row| row.ordinal == *ordinal).ok_or_else(reject)?;
        let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value else { return Err(reject()) };
        if !std::sync::Arc::ptr_eq(method, receipt.source().source())
            || !input.function().method_call_source_v1(method.site()).is_some_and(|call| std::sync::Arc::ptr_eq(call, method))
        { return Err(reject()); }
        let plan = if let Some(old) = old {
            if !old.core_arguments.get(ordinal).is_some_and(|original| Rc::ptr_eq(original, receipt)) { return Err(reject()); }
            Rc::clone(old.argument_sources.get(ordinal).ok_or_else(reject)?)
        } else {
            call_argument_expression_source_plan_v1(input, identity.source.method_source(), *ordinal)
                .map_err(|_| reject())?
        };
        if !corroborates_argument_source_v1(&plan, &identity.source, *ordinal, method)
            || input.function().source_site_inventory().containing_statement(site.site()) != Some(plan.root().statement())
        { return Err(reject()); }
        plans.insert(*ordinal, plan);
    }
    identity.argument_sources = plans;
    Ok(())
}
