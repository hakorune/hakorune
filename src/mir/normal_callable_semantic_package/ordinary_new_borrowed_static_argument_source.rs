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

/// One original source loan, never an evaluation, entry or ownership grant.
/// Rc identity is retained across the callback; no source plan is reissued.
#[derive(Debug, Clone)]
pub(crate) struct ArgumentExpressionSourceLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    core: Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>,
}
impl PartialEq for ArgumentExpressionSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan) && Rc::ptr_eq(&self.core, &other.core)
    }
}
impl Eq for ArgumentExpressionSourceLoanV1 {}
impl ArgumentExpressionSourceLoanV1 {
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> { &self.plan }
    pub(crate) fn resolver_source(&self) -> &crate::mir::resolved_semantics::VerifiedResolverOrdinaryCoreMethodSourceV1 {
        self.core.source()
    }
}

/// Shared by the passive projection and source lender. In particular this
/// checks the complete original tuple, not just the requested ordinal.
pub(super) fn validate_static_source_actuals_v1<'a>(
    prepared: &PreparedBorrowedFormalIngressV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
) -> Result<&'a StaticSourceActualIdentityV1, String> {
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        return Err(freeze("borrowed-static/source-phase-required"));
    };
    let has_core_source = !identity.core_arguments.is_empty() || identity.candidates.iter().any(|actual| matches!(&actual.value, BorrowedCallActualValueV1::MethodCall(_)));
    let retained = if has_core_source { original_core_static_source_v1(prepared, site)?
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))? }
        else { prepared.source_incoming.static_observations().get(site)
            .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?.as_ref().map_err(Clone::clone)? };
    if !Rc::ptr_eq(retained, &identity.source)
        || identity.source.call_site() != site
        || required_i64_arguments != identity.source.required_i64_arguments()
        || rows.ordered_arguments.len() != identity.source.argument_sites().len()
        || identity.candidates.len() != rows.ordered_arguments.len()
        || identity.integer_evidence.len() != rows.ordered_arguments.len()
        || !rows.opaque_actuals.is_empty()
    {
        return Err(freeze("borrowed-static/source-projection-identity"));
    }
    for (index, ((formal, candidate), argument)) in identity
        .source
        .parameters()
        .iter()
        .zip(identity.candidates.iter())
        .zip(rows.ordered_arguments.iter())
        .enumerate()
    {
        if formal.ordinal as usize != index
            || candidate.ordinal != formal.ordinal
            || formal.binding.owner() != identity.source.callee_owner()
            || identity.source.argument_sites().get(index) != Some(&candidate.site)
        {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
        match (&candidate.value, identity.core_arguments.get(&candidate.ordinal)) {
            (BorrowedCallActualValueV1::MethodCall(method), Some(receipt)) => {
                let site = OwnedExprSiteV1::new(site.owner(), candidate.site.clone());
                if !identity.argument_sources.get(&candidate.ordinal).is_some_and(|plan|
                    corroborates_argument_source_v1(plan, identity.source.as_ref(), candidate.ordinal, method))
                    || !prepared.ordinary_core_source_v1(&site).is_some_and(|original| Rc::ptr_eq(original, receipt))
                    || !std::sync::Arc::ptr_eq(method, receipt.source().source())
                    || method.owner() != site.owner() || method.site() != site.site()
                    || identity.integer_evidence.get(index) != Some(&false)
                    || receipt.source().target().schema() != crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                    || receipt.source().target().result() != crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
                { return Err(freeze("borrowed-static/core-source-identity")); }
            }
            (BorrowedCallActualValueV1::MethodCall(_), None) | (_, Some(_)) => return Err(freeze("borrowed-static/core-source-identity")),
            (_, None) => {}
        }
        let matches = match (&formal.kind, &candidate.value, argument) {
            (kind, _, LocalCallArgumentV1::BorrowedActual { ordinal, site })
                if kind.is_ordinary_borrowed_handle() =>
            {
                *ordinal == formal.ordinal && site == &candidate.site
            }
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Integer(expected),
                LocalCallArgumentV1::Integer(actual),
            ) if abi.is_i64() => expected == actual,
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Scalar(expected, SourceScalarKind::Integer),
                LocalCallArgumentV1::Scalar(actual),
            ) if abi.is_i64() => expected == actual && actual.owner() == site.owner(),
            _ => false,
        };
        if !matches {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
    }
    if identity.argument_sources.len() != identity.core_arguments.len()
        || identity.argument_sources.keys().any(|ordinal| !identity.core_arguments.contains_key(ordinal))
        || identity.core_arguments.keys().any(|ordinal| *ordinal as usize >= identity.candidates.len()) {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(identity)
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_argument_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    let rows = pending.get(site).ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref().map_err(Clone::clone)?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { return Ok(None) };
    let identity = validate_static_source_actuals_v1(prepared, rows, site, identity.source.required_i64_arguments())?;
    let candidate = identity.candidates.get(ordinal as usize)
        .filter(|row| row.ordinal == ordinal).ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    if !matches!(candidate.value, BorrowedCallActualValueV1::MethodCall(_)) { return Ok(None); }
    // A selected Core candidate with missing evidence is a refusal, never an
    // absent loan that invites retry through another source or execution route.
    let core = identity.core_arguments.get(&ordinal).ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    let plan = identity.argument_sources.get(&ordinal).ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    Ok(Some(ArgumentExpressionSourceLoanV1 { plan: Rc::clone(plan), core: Rc::clone(core) }))
}
