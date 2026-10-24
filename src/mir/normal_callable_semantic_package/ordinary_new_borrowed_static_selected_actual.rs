//! Source-only forwarding and the stronger selected-loop caller witness.
//! Neither source identity nor a forwarded root grants physical entry permission.

use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct VerifiedStaticForwardedActualV1 {
    original: Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
    target_formal: BindingRefV1,
    site: SourceExprSiteV1,
    actual: PreparedBorrowedFormalActualV1,
}

impl VerifiedStaticForwardedActualV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn caller_formal(&self) -> BindingRefV1 {
        self.caller_formal
    }

    pub(in crate::mir::normal_callable_semantic_package) fn actual(
        &self,
    ) -> &PreparedBorrowedFormalActualV1 {
        &self.actual
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
            && self.actual.ordinal == 0
            && self.actual.site == self.site
            && self.actual.formal == self.target_formal
            && matches!(&self.actual.source,
                BorrowedFormalActualSourceV1::Forwarded { binding, formal }
                    if *formal == self.caller_formal && binding.owner() == self.caller_formal.owner())
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
    if !(original.is_current_owner_i64_source_v1() || original.is_qualified_i64_source_v1())
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
            .source_definition_for(original.call_site().owner())
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
        actual: PreparedBorrowedFormalActualV1 {
            ordinal: 0,
            site: site.clone(),
            formal: formal.binding,
            source: BorrowedFormalActualSourceV1::Forwarded {
                binding,
                formal: caller_formal,
            },
        },
    })
}

/// Adapt the retained source-use fact, without granting a checked input domain.
/// Revalidation uses the saved SourceStatic snapshot, not the live finished phase.
pub(super) fn original_static_forwarded_source_actual_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    snapshot: &PreparedBorrowedCallActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    ordinal: u32,
) -> Result<PreparedBorrowedFormalActualV1, String> {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
    let reject = || freeze("borrowed-static/source-identity");
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &snapshot.phase else {
        return Err(reject());
    };
    let site = original
        .argument_sites()
        .get(ordinal as usize)
        .ok_or_else(reject)?;
    let formal = original
        .parameters()
        .get(ordinal as usize)
        .ok_or_else(reject)?;
    let fact = prepared
        .static_arguments
        .get(&(original.call_site().clone(), ordinal))
        .ok_or_else(reject)?;
    let mut candidates = identity
        .candidates
        .iter()
        .filter(|row| row.ordinal == ordinal);
    let candidate = candidates.next().ok_or_else(reject)?;
    let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value else {
        return Err(reject());
    };
    let draft = prepared
        .source_definition_for(original.call_site().owner())
        .ok_or_else(reject)?;
    let mut uses = draft.uses.iter().filter(|row| {
        matches!(&row.kind,
        BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal: observed }
            if call == original.call_site() && *observed == ordinal)
    });
    let use_row = uses.next().ok_or_else(reject)?;
    if candidates.next().is_some()
        || uses.next().is_some()
        || !Rc::ptr_eq(&identity.source, original)
        || !Rc::ptr_eq(fact.retained_call_source(), original)
        || !formal.kind.is_ordinary_borrowed_handle()
        || formal.ordinal != ordinal
        || fact.call() != original.call_site()
        || fact.ordinal() != ordinal
        || fact.use_site().owner() != original.call_site().owner()
        || fact.use_site().site() != site
        || candidate.site != *site
        || fact.binding() != binding
        || fact.formal() != root
        || fact.target_formal() != formal.binding
        || binding.owner() != original.call_site().owner()
        || root.owner() != original.call_site().owner()
        || use_row.site != *fact.use_site()
        || use_row.binding != binding
        || use_row.formal != root
        || draft.origins.get(&binding) != Some(&root)
        || !matches!(snapshot.ordered_arguments.get(ordinal as usize),
            Some(LocalCallArgumentV1::BorrowedActual { ordinal: observed, site: observed_site })
                if *observed == ordinal && observed_site == site)
        || !snapshot.opaque_actuals.is_empty()
    {
        return Err(reject());
    }
    Ok(PreparedBorrowedFormalActualV1 {
        ordinal,
        site: site.clone(),
        formal: formal.binding,
        source: BorrowedFormalActualSourceV1::Forwarded {
            binding,
            formal: root,
        },
    })
}
