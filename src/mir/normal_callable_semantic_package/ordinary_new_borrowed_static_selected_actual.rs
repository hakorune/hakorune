//! Source-only forwarding and the stronger selected-loop caller witness.
//! Neither source identity nor a forwarded root grants physical entry permission.

use super::static_input_finish::{StaticInputEvidenceV1, StaticInputFinishV1, StaticInputRowsV1};
use super::*;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct VerifiedStaticForwardedActualV1 {
    original: Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
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
            && original.argument_sites() == [self.actual.site.clone()]
            && original.parameters().len() == 1
            && original.parameters()[0].binding == self.actual.formal
            && self.actual.ordinal == 0
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
    let reason = "borrowed-static/selected-forwarded-actual-unavailable";
    let reject = || freeze(reason);
    let [formal] = original.parameters() else {
        return Err(reject());
    };
    let rows = pending
        .get(original.call_site())
        .ok_or_else(reject)?
        .as_ref()
        .map_err(Clone::clone)?;
    let actual =
        checked_original_static_forward_v1(prepared, rows, original, formal.ordinal, reason)?;
    if !(original.is_current_owner_i64_source_v1()
        || original.qualified_scalar_result_v1().is_some())
        || formal.ordinal != 0
        || rows.ordered_arguments.len() != original.parameters().len()
        || !matches!(actual.source, BorrowedFormalActualSourceV1::Forwarded { formal, .. } if formal == caller_formal)
        || !prepared.checked_static_input(caller_formal)
        || !prepared.checked_static_input(formal.binding)
    {
        return Err(reject());
    }
    Ok(VerifiedStaticForwardedActualV1 {
        original: Rc::clone(original),
        caller_formal,
        actual,
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

impl PreparedBorrowedFormalIngressV1 {
    /// Ownership proof of one synchronous, non-consuming read. The lowerer
    /// still checks the actual call's original tag, SSA payload and order.
    pub(in crate::mir::normal_callable_semantic_package) fn proves_tagged_static_argument_use_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        read: &OwnedExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> Result<bool, String> {
        use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};

        let Some((SourcePathSegmentV1::Argument(ordinal), parent)) =
            read.site().node().segments().split_last()
        else {
            return Ok(false);
        };
        let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(parent.to_vec()));
        let Some(method) = input.function().method_call_source_v1(&site) else {
            return Ok(false);
        };
        let call = OwnedExprSiteV1::new(input.owner(), site);
        let original = super::static_source::original_static_source_v1(self, &call)?;
        Ok(original.is_some_and(|original| {
            std::sync::Arc::ptr_eq(method, original.method_source())
                && original_tagged_forward_at_v1(self, original, *ordinal, read.site())
                    == Some((binding, formal))
        }))
    }
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
    /// Resolve the source origin before Home walks can demand a borrowed entry.
    pub(in crate::mir::normal_callable_semantic_package) fn select_static_tagged_sources_v1(&mut self) {
        let owners = self.static_tagged_forward_owners_v1();
        *self.tagged_static_owners.get_mut() = (self.source_incoming.revision_v1(), owners);
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
            // The general lane can replace an older scalar entry when all
            // source uses already have a checked, read-only tagged meaning.
            && (!self.incoming.iter().any(|row| {
                row.callee == owner
                    && matches!(&row.source, super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(_))
            }) || self.legacy_static_input_has_tagged_uses_v1(owner))
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

    fn legacy_static_input_has_tagged_uses_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;

        // An original instance caller still uses its receiver-bound entry
        // contract. The source-only tagged lane cannot replace that contract.
        if self.incoming.iter().filter(|row| row.callee == owner).any(|row| {
            !matches!(&row.source, BorrowedIncomingSourceV1::Static(original)
                if original.caller().namespace()
                    == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod)
        }) {
            return false;
        }
        // An unresolved argument is only an observed read. Its original
        // destination must also support tagged reads before displacing an
        // existing whole-callee entry (for example, a signed provider).
        fn closes(
            source: &PreparedBorrowedFormalIngressV1,
            owner: FunctionOwnerIdV1,
            open: &mut BTreeSet<FunctionOwnerIdV1>,
        ) -> bool {
            use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;

            if !open.insert(owner) {
                return false;
            }
            let closed = source.source_definition_for(owner).is_some_and(|definition| {
                definition.uses.iter().all(|row| match &row.kind {
                    Use::Copy { .. }
                    | Use::DynamicComparisonOperand { .. }
                    | Use::CompareOperand { .. }
                    | Use::IntegerReturn { .. } => true,
                    Use::UnresolvedArgument { call, ordinal } => {
                        super::static_source::original_static_source_v1(source, call)
                            .ok()
                            .flatten()
                            .is_some_and(|original| {
                                original_tagged_argument_read_v1(
                                    source, original, *ordinal, row.site.site(),
                                ) == Some((row.binding, Some(row.formal)))
                                    && closes(source, original.callee_owner(), open)
                            })
                    }
                    _ => false,
                })
            });
            open.remove(&owner);
            closed
        }
        closes(self, owner, &mut BTreeSet::new())
    }

    /// A tagged argument requires a borrowed entry at both ends. Reuse the
    /// participation query until the original inventory is changed.
    fn static_tagged_forward_owner_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        if !self.corroborated_original_v1() {
            return false;
        }
        let revision = self.source_incoming.revision_v1();
        let mut participation = self.tagged_static_owners.borrow_mut();
        if participation.0 == 0 {
            *participation = (revision, self.static_tagged_forward_owners_v1());
        }
        participation.0 == revision && participation.1.contains(&owner)
    }

    fn static_tagged_forward_owners_v1(&self) -> BTreeSet<FunctionOwnerIdV1> {
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;

        let incoming_owners = self
            .source_incoming
            .source_rows_v1()
            .map(|row| row.callee)
            .collect::<BTreeSet<_>>();
        let selected_calls = self
            .incoming
            .iter()
            .map(|row| &row.call)
            .collect::<BTreeSet<_>>();
        self.source_incoming
            .source_rows_v1()
            .filter_map(|row| {
                let BorrowedIncomingSourceV1::Static(original) = &row.source else {
                    return None;
                };
                // Retain source participation independently of an older
                // signed entry when the same callee has checked tagged uses.
                if !incoming_owners.contains(&row.call.owner())
                    || (selected_calls.contains(&row.call)
                        && !self.legacy_static_input_has_tagged_uses_v1(row.callee))
                {
                    return None;
                }
                original
                    .parameters()
                    .iter()
                    .zip(original.argument_sites())
                    .any(|(formal, site)| {
                        original_tagged_argument_read_v1(
                                self,
                                original,
                                formal.ordinal,
                                site,
                            )
                            .is_some()
                    })
                    .then_some([row.callee, row.call.owner()])
            })
            .flatten()
            .collect()
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
    let reason = "borrowed-static/source-identity";
    let reject = || freeze(reason);
    let actual = checked_original_static_forward_v1(prepared, snapshot, original, ordinal, reason)?;
    let BorrowedFormalActualSourceV1::Forwarded { binding, formal } = actual.source else {
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
    if uses.next().is_some()
        || use_row.site.owner() != original.call_site().owner()
        || use_row.site.site() != &actual.site
        || use_row.binding != binding
        || use_row.formal != formal
    {
        return Err(reject());
    }
    Ok(actual)
}

/// Match the original forwarded argument before a consumer applies its own use
/// or entry obligation. Neither the callee ordinal nor the caller root is inferred.
fn checked_original_static_forward_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    snapshot: &PreparedBorrowedCallActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    ordinal: u32,
    reason: &str,
) -> Result<PreparedBorrowedFormalActualV1, String> {
    let reject = || freeze(reason);
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &snapshot.phase else {
        return Err(reject());
    };
    let (formal, candidate) = static_source::original_static_borrowed_argument_v1(
        original, identity, snapshot, ordinal,
    )
    .ok_or_else(reject)?;
    let fact = prepared
        .static_arguments
        .get(&(original.call_site().clone(), ordinal))
        .ok_or_else(reject)?;
    let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value else {
        return Err(reject());
    };
    let caller = original.call_site().owner();
    if !Rc::ptr_eq(fact.retained_call_source(), original)
        || !formal.kind.is_ordinary_borrowed_handle()
        || fact.call() != original.call_site()
        || fact.ordinal() != ordinal
        || fact.use_site().owner() != caller
        || fact.use_site().site() != &candidate.site
        || fact.binding() != binding
        || fact.formal() != root
        || fact.target_formal() != formal.binding
        || binding.owner() != caller
        || root.owner() != caller
        || prepared
            .source_definition_for(caller)
            .and_then(|draft| draft.origins.get(&binding))
            != Some(&root)
        || !snapshot.opaque_actuals.is_empty()
    {
        return Err(reject());
    }
    Ok(super::static_input_validation::static_argument_actual_v1(
        formal.binding,
        candidate,
        BorrowedFormalActualSourceV1::Forwarded {
            binding,
            formal: root,
        },
    ))
}

fn forwarded_static_incoming_v1(
    source: &PreparedBorrowedFormalIngressV1,
    owner: FunctionOwnerIdV1,
) -> &[super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1] {
    source.target_static.get(&owner)
        .map_or(source.incoming.as_ref(), |target| target.incoming.as_ref())
}

pub(super) fn forwarded_static_incoming_row_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    original: &Rc<StaticIncomingSourceV1>,
    missing: &'static str,
    drift: &'static str,
) -> Result<&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1, String> {
    use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
    let mut matches = forwarded_static_incoming_v1(source, original.callee_owner())
        .iter()
        .filter(|row| &row.call == original.call_site());
    let call = matches.next().ok_or_else(|| freeze(missing))?;
    if matches.next().is_some()
        || call.callee != original.callee_owner()
        || !matches!(&call.source, BorrowedIncomingSourceV1::Static(row) if Rc::ptr_eq(row, original))
    {
        return Err(freeze(drift));
    }
    Ok(call)
}

pub(super) fn retains_forwarded_static_cohort_v1(
    source: &PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
    cohort: &[Rc<StaticIncomingSourceV1>],
) -> bool {
    use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
    let owner = original.callee_owner();
    // Only a registered target authorizes a wider forwarded input scope.
    // Without one, the source inventory has already confirmed this exact site.
    let Some(target) = source.target_static.get(&owner) else {
        return cohort.iter().all(|member| std::ptr::eq(member.as_ref(), original));
    };
    let incoming = target.incoming.iter().filter(|call| call.callee == owner);
    incoming.clone().count() == cohort.len()
        && cohort.iter().all(|member| {
            incoming.clone().filter(|call| {
                call.call == *member.call_site()
                    && matches!(&call.source, BorrowedIncomingSourceV1::Static(source)
                        if Rc::ptr_eq(member, source))
            }).count() == 1
        })
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_forwarded_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&PreparedBorrowedCallActualsV1>, String> {
        if !original.is_current_owner_i64_source_v1() || original.argument_sites().len() != 1 {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("static-one/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let cohort = source.static_incoming_cohort_v1(original)?;
        if !retains_forwarded_static_cohort_v1(source, original, &cohort) {
            return Err(freeze("static-one/whole-cohort"));
        }
        let retained = source
            .source_incoming
            .static_observations()
            .get(original.call_site())
            .ok_or_else(|| freeze("static-one/original-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if !Rc::ptr_eq(retained, original) {
            return Err(freeze("static-one/original-drift"));
        }
        let Some(row) = self.borrowed_formal_actuals.get(original.call_site()) else {
            return Ok(None);
        };
        let row = row.as_ref().map_err(Clone::clone)?;
        let Some(finish) = row.forwarded_static_finish_v1() else {
            return Ok(None);
        };
        let completion = self
            .completion_index
            .get(&original.callee_owner())
            .ok_or_else(|| freeze("static-one/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-one/completion")))?;
        if !finish.corroborates_forwarded_v1(original) || !Rc::ptr_eq(&finish.completion, completion) {
            return Err(freeze("static-one/completion-drift"));
        }
        if source.target_static.contains_key(&original.callee_owner()) {
            for sibling in &cohort {
                let ready = self
                    .borrowed_formal_actuals
                    .get(sibling.call_site())
                    .ok_or_else(|| freeze("static-one/cohort-actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                let Some(proof) = ready.forwarded_static_finish_v1() else {
                    return Err(freeze("static-one/cohort-unfinished"));
                };
                if !proof.corroborates_forwarded_v1(sibling)
                    || !Rc::ptr_eq(&proof.completion, completion)
                {
                    return Err(freeze("static-one/cohort-completion-drift"));
                }
            }
        }
        let call = forwarded_static_incoming_row_v1(
            source, original, "static-one/incoming-missing", "static-one/incoming-identity",
        )?;
        let arguments = row.ordered_arguments_for_v1(call)?;
        if row.opaque_actuals.len() != 1
            || !matches!(
                &row.opaque_actuals[0].source,
                BorrowedFormalActualSourceV1::Forwarded { .. }
            )
        {
            return Err(freeze("static-one/actual-identity"));
        }
        let observation = self
            .local_call_for_owner(original.call_site().owner(), original.call_site().site())
            .ok_or_else(|| freeze("static-one/home-observation-missing"))?;
        if observation.arguments() != arguments
            || observation.result() != LocalCallResultClassV1::I64
        {
            return Err(freeze("static-one/home-observation-drift"));
        }
        Ok(Some(row))
    }

    pub(super) fn prepare_static_forwarded_inputs_v1(
        &self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<StaticInputRowsV1, String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(Vec::new());
        };
        let mut originals: Vec<_> = source
            .incoming
            .iter()
            .filter_map(|call| match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.is_current_owner_i64_source_v1()
                        && source.definitions.contains_key(&original.callee_owner()) =>
                {
                    Some(Rc::clone(original))
                }
                _ => None,
            })
            .collect();
        originals.extend(source.target_static.values().flat_map(|cohort| {
            cohort
                .incoming
                .iter()
                .filter_map(|call| match &call.source {
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                        original,
                    ) if original.is_current_owner_i64_source_v1() => Some(Rc::clone(original)),
                    _ => None,
                })
        }));
        let mut replacements = Vec::new();
        for original in originals {
            let cohort = source.static_incoming_cohort_v1(&original)?;
            if !cohort.iter().any(|row| Rc::ptr_eq(row, &original))
                || !retains_forwarded_static_cohort_v1(source, &original, &cohort)
            {
                return Err(freeze("static-one/whole-cohort"));
            }
            let owner = original.callee_owner();
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-one/contract-missing"))?;
            if matching.next().is_some()
                || !original.matches_static_input_contract_v1(selected, contract)
                || !(source.definitions.contains_key(&owner)
                    || source.target_static.contains_key(&owner))
            {
                return Err(freeze("static-one/contract-identity"));
            }
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-one/selected-identity"))?;
            let completion = self.require_static_input_completion_v1(
                identity, contract, signatures, results, "static-one",
            )?;
            let site = original.call_site();
            let call = forwarded_static_incoming_row_v1(
                source, &original, "static-one/incoming-missing", "static-one/incoming-identity",
            )?;
            let [(ordinal, _, _)] = call.arguments.as_ref() else {
                return Err(freeze("static-one/incoming-identity"));
            };
            let fact = source
                .static_arguments
                .get(&(site.clone(), *ordinal))
                .ok_or_else(|| freeze("static-one/forward-source-missing"))?;
            let issued = super::static_selected_actual::issue_original_static_forwarded_actual_v1(
                source,
                &self.borrowed_formal_actuals,
                &original,
                fact.formal(),
            )?;
            if !issued.corroborates(&original, fact.formal()) {
                return Err(freeze("static-one/forward-identity"));
            }
            let staged = self
                .borrowed_formal_actuals
                .get(site)
                .ok_or_else(|| freeze("static-one/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(source_actual) = &staged.phase
            else {
                return Err(freeze("static-one/source-phase"));
            };
            if source_actual.integer_evidence.len() != original.parameters().len()
            {
                return Err(freeze("static-one/source-actual-drift"));
            }
            let observation = self
                .local_call_for_owner(site.owner(), site.site())
                .ok_or_else(|| freeze("static-one/home-observation-missing"))?;
            if observation.site() != site
                || observation.owner() != site.owner()
                || observation.result() != LocalCallResultClassV1::I64
                || observation.arguments() != staged.ordered_arguments.as_ref()
            {
                return Err(freeze("static-one/home-observation-drift"));
            }
            replacements.push((
                site.clone(),
                StaticInputFinishV1::finish_arguments_v1(
                    &original,
                    completion,
                    staged,
                    StaticInputEvidenceV1::Forwarded,
                    Box::new([issued.actual().clone()]),
                    None,
                ),
            ));
        }
        Ok(replacements)
    }
}
