//! Source-only forwarding and the stronger selected-loop caller witness.
//! Neither source identity nor a forwarded root grants physical entry permission.

use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::collections::BTreeSet;
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
    if !(original.is_current_owner_i64_source_v1() || original.qualified_scalar_result_v1().is_some())
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

/// Source-only decision 27. The selected read uses the resolver's reaching
/// definition; neither the caller's incoming domains nor its legacy draft are
/// an authority on the tag. The packet still checks the actual physical read.
pub(super) fn original_tagged_forward_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
    candidate: &BorrowedCallActualCandidateV1,
) -> Option<BorrowedFormalActualSourceV1> {
    let (binding, formal) =
        original_tagged_forward_at_v1(prepared, original, candidate.ordinal, &candidate.site)?;
    let matches_read = match candidate.value {
        BorrowedCallActualValueV1::SelfRooted { binding: observed, root } => {
            observed == binding && root == formal
        }
        BorrowedCallActualValueV1::Scalar(observed, _) | BorrowedCallActualValueV1::Binding(observed) => {
            observed == binding
        }
        _ => false,
    };
    matches_read.then_some(BorrowedFormalActualSourceV1::Forwarded { binding, formal })
}

pub(in crate::mir::normal_callable_semantic_package) fn original_tagged_forward_at_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, BindingRefV1)> {
    if !prepared.has_tagged_static_input_v1(original.callee_owner(), original.parameters()) {
        return None;
    }
    let (binding, formal) = original_tagged_argument_read_v1(prepared, original, ordinal, site)?;
    Some((binding, formal?))
}

fn original_tagged_argument_read_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
) -> Option<(BindingRefV1, Option<BindingRefV1>)> {
    let callee = original.parameters().get(ordinal as usize)?;
    if callee.kind != CallableParameterContractKindV1::OpaqueHandle
        || callee.demand != crate::mir::resolved_semantics::HomeDemandV1::Handle
        || callee.ordinal != ordinal
        || callee.binding.owner() != original.callee_owner()
        || original.argument_sites().get(ordinal as usize) != Some(site)
    {
        return None;
    }
    let caller = original.call_site().owner();
    let (binding, formal) = prepared
        .original_formal_uses_v1(caller)
        .ok()?
        .tagged_argument_read_v1(site)?;
    (binding.owner() == caller && formal.is_none_or(|formal| formal.owner() == caller))
        .then_some((binding, formal))
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn static_scalar_actual_rebound_v1(
        &self,
        call: &OwnedExprSiteV1,
        actual: &PreparedBorrowedFormalActualV1,
        binding: BindingRefV1,
    ) -> Result<bool, String> {
        let Some(Ok(source)) = &self.borrowed_formal_source else {
            return Ok(false);
        };
        let Some(original) = super::static_source::original_static_source_v1(source, call)? else {
            return Ok(false);
        };
        if original.parameters().get(actual.ordinal as usize)
            .is_none_or(|formal| formal.binding != actual.formal)
            || !matches!(original_tagged_argument_read_v1(source, original, actual.ordinal, &actual.site),
                Some((current, None)) if current == binding)
        {
            return Ok(false);
        }
        let uses = source.original_formal_uses_v1(call.owner())?;
        Ok(uses.corroborates_non_entry_source_read_v1(
            &actual.site, binding, uses.roots(), &|_| false,
        ))
    }
}

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_tagged_static_loop_sites_v1(
        &mut self,
        sites: Rc<BTreeSet<(FunctionOwnerIdV1, crate::mir::resolved_semantics::SourceStmtSiteV1)>>,
    ) {
        self.tagged_static_loop_sites = sites;
        *self.tagged_static_owners.get_mut() = Default::default();
    }

    pub(in crate::mir::normal_callable_semantic_package) fn has_tagged_static_input_v1(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1],
    ) -> bool {
        let mut borrowed = parameters.iter().filter(|formal| formal.kind.is_ordinary_borrowed_handle());
        borrowed.clone().next().is_some()
            && borrowed.all(|formal| {
                formal.kind == CallableParameterContractKindV1::OpaqueHandle
                    && formal.demand == crate::mir::resolved_semantics::HomeDemandV1::Handle
            })
            && self.static_tagged_forward_owner_v1(owner)
            // A declared destination must own a checked conversion of the
            // entry read. The general transport supplies no such conversion.
            && self.original_formal_uses_v1(owner)
                .is_ok_and(|uses| !uses.has_declared_entry_initializer_v1())
            // An incoming entry already selected from the original source
            // has its own transport proof. Keep that proof and its writer.
            && !self.incoming.iter().any(|row| {
                row.callee == owner
                    && matches!(&row.source, super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(_))
            })
            // Keep the original finite Text entry proof when it owns the
            // complete cohort. Its per-operation facts remain needed even
            // though an argument still travels in the existing tagged form.
            && !self.lend_original_input_sums_v1(parameters).is_some_and(|sums| {
                sums.iter().all(|sum| sum.finite_input())
                    && sums.iter().any(|sum| sum.alternatives().contains(
                        &super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                    ))
            })
    }

    /// A tagged argument requires a borrowed entry at both ends. Reuse the
    /// participation query until the original inventory is changed.
    fn static_tagged_forward_owner_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;

        if !self.corroborated_original_v1() {
            return false;
        }
        let revision = self.source_incoming.revision_v1();
        let mut participation = self.tagged_static_owners.borrow_mut();
        if participation.0 != revision {
            let incoming_owners = self.source_incoming.source_rows_v1()
                .map(|row| row.callee)
                .collect::<BTreeSet<_>>();
            let selected_calls = self.incoming.iter().map(|row| &row.call).collect::<BTreeSet<_>>();
            let owners = self.source_incoming.source_rows_v1().filter_map(|row| {
                let BorrowedIncomingSourceV1::Static(original) = &row.source else {
                    return None;
                };
                // A declaration without an incoming source can observe its
                // arguments passively. A selected call keeps its own proof.
                if !incoming_owners.contains(&row.call.owner())
                    || selected_calls.contains(&row.call)
                    || self.tagged_static_loop_sites.iter().any(|(callee, _)| *callee == row.callee)
                {
                    return None;
                }
                original.parameters().iter().zip(original.argument_sites()).any(|(formal, site)| {
                    original_tagged_argument_read_v1(self, original, formal.ordinal, site).is_some()
                }).then_some([row.callee, row.call.owner()])
            }).flatten().collect();
            *participation = (revision, owners);
        }
        participation.1.contains(&owner)
    }
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
