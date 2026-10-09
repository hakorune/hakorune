//! Exact forwarded actuals for one selected Static loop's original callers.
//! The source-only phase stays closed for every other Static route.

use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct VerifiedStaticForwardedActualV1 {
    original: Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
    target_formal: BindingRefV1,
    site: SourceExprSiteV1,
}

impl VerifiedStaticForwardedActualV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn caller_formal(&self) -> BindingRefV1 {
        self.caller_formal
    }

    pub(in crate::mir::normal_callable_semantic_package) fn corroborates(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        caller_formal: BindingRefV1,
    ) -> bool {
        Rc::ptr_eq(&self.original, original)
            && self.caller_formal == caller_formal
            && original.argument_sites() == [self.site.clone()]
            && original.parameters().len() == 1
            && original.parameters()[0].binding == self.target_formal
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_original_static_forwarded_actual_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
) -> Result<VerifiedStaticForwardedActualV1, String> {
    let reject = || freeze("borrowed-static/selected-forwarded-actual-unavailable");
    let [site] = original.argument_sites() else {
        return Err(reject());
    };
    let [formal] = original.parameters() else {
        return Err(reject());
    };
    let fact = prepared
        .static_arguments
        .get(&(original.call_site().clone(), 0))
        .ok_or_else(reject)?;
    let rows = pending
        .get(original.call_site())
        .ok_or_else(reject)?
        .as_ref()
        .map_err(Clone::clone)?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        return Err(reject());
    };
    let [candidate] = identity.candidates.as_ref() else {
        return Err(reject());
    };
    let [argument] = rows.ordered_arguments.as_ref() else {
        return Err(reject());
    };
    let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value else {
        return Err(reject());
    };
    if !(original.is_current_owner_i64_source_v1() || original.is_qualified())
        || formal.ordinal != 0
        || !formal.kind.is_ordinary_borrowed_handle()
        || !Rc::ptr_eq(&identity.source, original)
        || !Rc::ptr_eq(fact.retained_call_source(), original)
        || fact.call() != original.call_site()
        || fact.ordinal() != 0
        || fact.use_site().site() != site
        || fact.binding() != binding
        || fact.formal() != caller_formal
        || fact.target_formal() != formal.binding
        || candidate.ordinal != 0
        || candidate.site != *site
        || root != caller_formal
        || binding.owner() != original.call_site().owner()
        || caller_formal.owner() != original.call_site().owner()
        || !prepared.checked_static_input(caller_formal)
        || !prepared.checked_static_input(formal.binding)
        || prepared
            .source_only_definitions
            .get(&original.call_site().owner())
            .and_then(|draft| draft.origins.get(&binding))
            != Some(&caller_formal)
        || !matches!(argument, LocalCallArgumentV1::BorrowedActual { ordinal: 0, site: observed } if observed == site)
        || !rows.opaque_actuals.is_empty()
    {
        return Err(reject());
    }
    Ok(VerifiedStaticForwardedActualV1 {
        original: Rc::clone(original),
        caller_formal,
        target_formal: formal.binding,
        site: site.clone(),
    })
}
