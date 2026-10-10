//! Finish one source-only Static target with one exact scalar and two forwards.
//! Every caller is checked against the same original incoming cohort.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) enum StaticMixedInputKindV1 {
    Scalar(BindingRefV1),
    Forwarded {
        formal: BindingRefV1,
        issued: Rc<VerifiedStaticForwardedActualV1>,
    },
}

#[derive(Debug, Clone)]
pub(super) struct StaticMixedInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    kind: StaticMixedInputKindV1,
}

impl PartialEq for StaticMixedInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && match (&self.kind, &other.kind) {
                (StaticMixedInputKindV1::Scalar(a), StaticMixedInputKindV1::Scalar(b)) => a == b,
                (
                    StaticMixedInputKindV1::Forwarded {
                        formal: a,
                        issued: x,
                    },
                    StaticMixedInputKindV1::Forwarded {
                        formal: b,
                        issued: y,
                    },
                ) => a == b && Rc::ptr_eq(x, y),
                _ => false,
            }
    }
}
impl Eq for StaticMixedInputFinishV1 {}

impl StaticMixedInputFinishV1 {
    fn corroborates(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        actual: &PreparedBorrowedFormalActualV1,
    ) -> bool {
        if !Rc::ptr_eq(&self.source, original)
            || !Rc::ptr_eq(&self.completion, completion)
            || !original.is_current_owner_i64_source_v1()
            || !original.required_i64_arguments().is_empty()
            || original.argument_sites().len() != 1
            || original.parameters().len() != 1
            || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            || actual.ordinal != 0
            || actual.site != original.argument_sites()[0]
            || actual.formal != original.parameters()[0].binding
        {
            return false;
        }
        match (&self.kind, &actual.source) {
            (
                StaticMixedInputKindV1::Scalar(binding),
                BorrowedFormalActualSourceV1::Scalar {
                    binding: actual,
                    kind: SourceScalarKind::Integer,
                },
            ) => binding == actual && binding.owner() == original.call_site().owner(),
            (
                StaticMixedInputKindV1::Forwarded { formal, issued },
                BorrowedFormalActualSourceV1::Forwarded {
                    binding,
                    formal: actual_formal,
                },
            ) => {
                formal == actual_formal
                    && binding.owner() == original.call_site().owner()
                    && issued.corroborates(original, *formal)
                    && issued.actual() == actual
            }
            _ => false,
        }
    }
}

/// The canonical whole-callee inventory stays authoritative at demand time.
/// An entirely pending cohort is passive; a partly promoted cohort is an error.
pub(super) fn checked_mixed_rows_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    pending: &'a PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
) -> Result<
    Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
    String,
> {
    let owner = original.callee_owner();
    let selected = source.source_incoming.exact_rows().any(|row| {
        row.callee == owner
            && matches!(pending.get(&row.call),
                Some(Ok(actual)) if matches!(actual.phase,
                    BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(_)))
    });
    if !selected {
        // No selected mixed proof: other source-only cohorts stay passive.
        return Ok(None);
    }
    if !source.source_only_definitions.contains_key(&owner)
        || source.definitions.contains_key(&owner)
        || source.target_static.contains_key(&owner)
    {
        return Err(freeze("static-mixed/source-authority-drift"));
    }
    let rows = source
        .source_incoming
        .project(&BTreeSet::from([owner]))
        .map_err(|error| format!("{}: {error:?}", freeze("static-mixed/incoming-veto")))?;
    if rows.len() != 3 {
        return Err(freeze("static-mixed/whole-cohort"));
    }
    let cohort = source.static_incoming_cohort_v1(original)?;
    if cohort.len() != rows.len() {
        return Err(freeze("static-mixed/whole-cohort"));
    }
    let Some(first) = cohort.first() else {
        return Ok(None);
    };
    let mut scalar = 0;
    let mut forwarded = 0;
    let mut completion = None;
    let mut ready = false;
    for member in &cohort {
        if !member.is_current_owner_i64_source_v1()
            || !member.required_i64_arguments().is_empty()
            || member.target() != first.target()
            || member.target_batch_slot() != first.target_batch_slot()
            || member.parameters().len() != 1
            || member.parameters()[0].binding != first.parameters()[0].binding
            || member.argument_sites().len() != 1
        {
            return Err(freeze("static-mixed/source-identity"));
        }
        let row = rows
            .iter()
            .find(|row| row.call == *member.call_site())
            .ok_or_else(|| freeze("static-mixed/incoming-missing"))?;
        if row.callee != owner
            || row.arguments.as_ref()
                != [(
                    0,
                    member.argument_sites()[0].clone(),
                    member.parameters()[0].binding,
                )]
            || !matches!(&row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                    if Rc::ptr_eq(retained, member))
        {
            return Err(freeze("static-mixed/incoming-identity"));
        }
        let Some(actual) = pending.get(member.call_site()) else {
            // A selected cohort cannot silently become passive when one
            // sibling disappears; the final count below rejects the gap.
            continue;
        };
        let actual = actual.as_ref().map_err(Clone::clone)?;
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(proof) = &actual.phase else {
            if ready {
                return Err(freeze("static-mixed/partial-cohort"));
            }
            continue;
        };
        ready = true;
        let first_completion = completion.get_or_insert(&proof.completion);
        if !Rc::ptr_eq(first_completion, &proof.completion)
            || actual.opaque_actuals.len() != 1
            || !proof.corroborates(member, &proof.completion, &actual.opaque_actuals[0])
            || actual.ordered_arguments.as_ref()
                != [LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: member.argument_sites()[0].clone(),
                }]
        {
            return Err(freeze("static-mixed/actual-identity"));
        }
        match &proof.kind {
            StaticMixedInputKindV1::Scalar(_) => scalar += 1,
            StaticMixedInputKindV1::Forwarded { .. } => forwarded += 1,
        }
    }
    if !ready {
        return Ok(None);
    }
    if scalar != 1 || forwarded != 2 {
        return Err(freeze("static-mixed/partial-cohort"));
    }
    let retained: Vec<_> = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.callee == owner)
        .collect();
    if retained.len() != rows.len()
        || retained
            .iter()
            .any(|row| !rows.iter().any(|candidate| candidate.call == row.call))
    {
        return Err(freeze("static-mixed/retained-incoming-drift"));
    }
    Ok(Some(retained.into_boxed_slice()))
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn select_static_mixed_local_routes_v1(
        &mut self,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let mut sites = Vec::new();
        for owner in source.source_only_definitions.keys() {
            let Ok(rows) = source.source_incoming.project(&BTreeSet::from([*owner])) else {
                continue;
            };
            let Some(first) = rows.first() else { continue };
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &first.source
            else {
                continue;
            };
            if let Some(cohort) = self.checked_completed_static_mixed_cohort_v1(source, original)? {
                sites.extend(cohort.iter().filter_map(|row| {
                    self.local_call_for_owner(row.call.owner(), row.call.site())
                        .map(|_| row.call.clone())
                }));
            }
        }
        if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
            selected.extend(sites);
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_static_mixed_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(None);
        };
        let Some(cohort) = self.checked_completed_static_mixed_cohort_v1(source, original)? else {
            return Ok(None);
        };
        let call = cohort
            .iter()
            .find(|row| {
                row.call == *original.call_site()
                    && matches!(&row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                    if Rc::ptr_eq(retained, original))
            })
            .ok_or_else(|| freeze("static-mixed/packet-incoming-missing"))?;
        let actual = self
            .borrowed_formal_actuals
            .get(original.call_site())
            .ok_or_else(|| freeze("static-mixed/packet-actuals-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        actual.require_executable_v1()?;
        let ordered = actual.ordered_arguments_for_v1(call)?;
        let home = self
            .local_call_for_owner(call.call.owner(), call.call.site())
            .ok_or_else(|| freeze("static-mixed/packet-home-missing"))?;
        if home.arguments() != ordered {
            return Err(freeze("static-mixed/packet-ordered-arguments-drift"));
        }
        Ok(Some(actual.opaque_actuals.as_ref()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_mixed_cohort_v1<
        'a,
    >(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<
        Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
        String,
    > {
        let Some(rows) = checked_mixed_rows_v1(source, &self.borrowed_formal_actuals, original)?
        else {
            return Ok(None);
        };
        let completion = self
            .completion_index
            .get(&original.callee_owner())
            .ok_or_else(|| freeze("static-mixed/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-mixed/completion")))?;
        for row in &rows {
            let actual = self
                .borrowed_formal_actuals
                .get(&row.call)
                .ok_or_else(|| freeze("static-mixed/actual-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(proof) = &actual.phase
            else {
                return Err(freeze("static-mixed/partial-cohort"));
            };
            if !Rc::ptr_eq(&proof.completion, completion) {
                return Err(freeze("static-mixed/completion-drift"));
            }
            let home = self
                .local_call_for_owner(row.call.owner(), row.call.site())
                .ok_or_else(|| freeze("static-mixed/home-missing"))?;
            if home.site() != &row.call
                || home.owner() != row.call.owner()
                || home.result() != LocalCallResultClassV1::I64
                || home.arguments() != actual.ordered_arguments.as_ref()
            {
                return Err(freeze("static-mixed/home-drift"));
            }
        }
        Ok(Some(rows))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_mixed_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let owners: Vec<_> = source.source_only_definitions.keys().copied().collect();
        for owner in owners {
            if source.definitions.contains_key(&owner) || source.target_static.contains_key(&owner)
            {
                continue;
            }
            let rows = match source.source_incoming.project(&BTreeSet::from([owner])) {
                Ok(rows) if rows.len() == 3 => rows,
                _ => continue,
            };
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(first) =
                &rows[0].source
            else {
                continue;
            };
            let cohort = match source.static_incoming_cohort_v1(first) {
                Ok(cohort) if cohort.len() == 3 => cohort,
                _ => continue,
            };
            if cohort.iter().any(|member| {
                !member.is_current_owner_i64_source_v1()
                    || !member.required_i64_arguments().is_empty()
                    || member.parameters().len() != 1
                    || !member.parameters()[0].kind.is_ordinary_borrowed_handle()
                    || member.argument_sites().len() != 1
                    || member.target() != first.target()
                    || member.target_batch_slot() != first.target_batch_slot()
            }) {
                continue;
            }
            let mut kinds = Vec::with_capacity(3);
            for member in &cohort {
                let Some(Ok(staged)) = self.borrowed_formal_actuals.get(member.call_site()) else {
                    break;
                };
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &staged.phase
                else {
                    break;
                };
                let [candidate] = identity.candidates.as_ref() else {
                    break;
                };
                kinds.push(candidate.value.clone());
            }
            if kinds.len() != 3
                || kinds
                    .iter()
                    .filter(|kind| {
                        matches!(
                            kind,
                            BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer)
                        )
                    })
                    .count()
                    != 1
                || kinds
                    .iter()
                    .filter(|kind| matches!(kind, BorrowedCallActualValueV1::SelfRooted { .. }))
                    .count()
                    != 2
            {
                continue;
            }
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-mixed/contract-missing"))?;
            if matching.next().is_some()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.batch_slot != first.target_batch_slot()
                || contract.parameters.len() != 1
                || contract.parameters[0].ordinal != 0
                || contract.parameters[0].binding != first.parameters()[0].binding
                || contract.parameters[0].kind != first.parameters()[0].kind
                || !source.checked_static_input(contract.parameters[0].binding)
                || !matches!(selected.key_for_batch_slot(contract.batch_slot),
                    Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == first.target())
            {
                return Err(freeze("static-mixed/contract-identity"));
            }
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-mixed/selected-identity"))?;
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-mixed/signature-missing"))?;
            if !signature.identity().same_as(identity)
                || signature.owner() != owner
                || signature.mode() != contract.mode
                || signature.source_logical_arity() != 1
                || signature.receiver_lane_count() != 0
                || signature.physical_formal_lane_count() != 1
                || signature.physical_callable_lane_count() != 1
                || signature.lanes().len() != 1
                || signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                || signature.lanes()[0].logical_ordinal() != Some(0)
                || signature.lanes()[0].binding() != contract.parameters[0].binding
            {
                return Err(freeze("static-mixed/signature-identity"));
            }
            let completion = self
                .completion_index
                .get(&owner)
                .ok_or_else(|| freeze("static-mixed/completion-missing"))?
                .as_ref()
                .map_err(|error| format!("{}: {error:?}", freeze("static-mixed/completion")))?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-mixed/result-missing"))?
                .borrow();
            if result.owner() != owner
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_i64_source()
                || !completion.returns_value()
                || completion.owner() != owner
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-mixed/result-completion-identity"));
            }
            let mut replacements = Vec::with_capacity(3);
            for (member, kind) in cohort.iter().zip(kinds) {
                let site = member.call_site();
                let staged = self
                    .borrowed_formal_actuals
                    .get(site)
                    .unwrap()
                    .as_ref()
                    .map_err(Clone::clone)?;
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(source_actual) = &staged.phase
                else {
                    break;
                };
                let [candidate] = source_actual.candidates.as_ref() else {
                    break;
                };
                if !Rc::ptr_eq(&source_actual.source, member)
                    || source_actual.integer_evidence.as_ref() != [true]
                    || candidate.ordinal != 0
                    || candidate.site != member.argument_sites()[0]
                    || !staged.opaque_actuals.is_empty()
                    || staged.ordered_arguments.as_ref()
                        != [LocalCallArgumentV1::BorrowedActual {
                            ordinal: 0,
                            site: candidate.site.clone(),
                        }]
                    || rows.iter().filter(|row| row.call == *site).count() != 1
                {
                    return Err(freeze("static-mixed/source-actual-drift"));
                }
                let (kind, actual) = match kind {
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) => {
                        super::static_scalar_input_finish::check_exact_usize_scalar_caller_v1(
                            selected, contracts, signatures, member, binding,
                        )?;
                        (
                            StaticMixedInputKindV1::Scalar(binding),
                            PreparedBorrowedFormalActualV1 {
                                ordinal: 0,
                                site: candidate.site.clone(),
                                formal: member.parameters()[0].binding,
                                source: BorrowedFormalActualSourceV1::Scalar {
                                    binding,
                                    kind: SourceScalarKind::Integer,
                                },
                            },
                        )
                    }
                    BorrowedCallActualValueV1::SelfRooted { root, .. } => {
                        let issued = super::static_selected_actual::issue_original_static_forwarded_actual_v1(
                            source, &self.borrowed_formal_actuals, member, root,
                        )?;
                        if !issued.corroborates(member, root) {
                            return Err(freeze("static-mixed/forward-drift"));
                        }
                        let actual = issued.actual().clone();
                        (
                            StaticMixedInputKindV1::Forwarded {
                                formal: root,
                                issued: Rc::new(issued),
                            },
                            actual,
                        )
                    }
                    _ => return Err(freeze("static-mixed/candidate-kind")),
                };
                let home = self
                    .local_call_for_owner(site.owner(), site.site())
                    .ok_or_else(|| freeze("static-mixed/home-missing"))?;
                if home.site() != site
                    || home.owner() != site.owner()
                    || home.result() != LocalCallResultClassV1::I64
                    || home.arguments() != staged.ordered_arguments.as_ref()
                {
                    return Err(freeze("static-mixed/home-drift"));
                }
                replacements.push((
                    site.clone(),
                    PreparedBorrowedCallActualsV1 {
                        phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(
                            StaticMixedInputFinishV1 {
                                source: Rc::clone(member),
                                completion: Rc::clone(completion),
                                kind,
                            },
                        ),
                        opaque_actuals: Box::new([actual]),
                        ordered_arguments: staged.ordered_arguments.clone(),
                    },
                ));
            }
            if replacements.len() == cohort.len() {
                for (site, actual) in replacements {
                    self.borrowed_formal_actuals.insert(site, Ok(actual));
                }
            }
        }
        Ok(())
    }
}
