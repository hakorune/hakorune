//! Complete one original CurrentOwner Static scalar cohort in the existing ledger.
//! The later physical caller read remains an independent publication obligation.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticScalarInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    pub(super) completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    pub(super) binding: BindingRefV1,
}

impl PartialEq for StaticScalarInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && self.binding == other.binding
    }
}
impl Eq for StaticScalarInputFinishV1 {}

impl StaticScalarInputFinishV1 {
    pub(super) fn corroborates(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    ) -> bool {
        Rc::ptr_eq(&self.source, original)
            && Rc::ptr_eq(&self.completion, completion)
            && original.is_current_owner_i64_source_v1()
            && (original.required_i64_arguments() == [0]
                || original.required_i64_arguments().is_empty())
            && original.parameters().len() == 1
            && original.argument_sites().len() == 1
            && self.binding.owner() == original.call_site().owner()
            && completion.owner() == original.callee_owner()
    }
}

pub(super) fn check_exact_usize_scalar_caller_v1(
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    original: &Rc<StaticIncomingSourceV1>,
    binding: BindingRefV1,
) -> Result<(), String> {
    let mut callers = contracts.iter().filter(|row| row.owner == original.call_site().owner());
    let caller = callers.next()
        .ok_or_else(|| freeze("static-scalar/caller-contract-missing"))?;
    let caller_signature = signatures.row(caller.batch_slot)
        .ok_or_else(|| freeze("static-scalar/caller-signature-missing"))?;
    if callers.next().is_some()
        || caller.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || caller.parameters.len() != 1
        || caller.parameters[0].ordinal != 0
        || caller.parameters[0].binding != binding
        || caller.parameters[0].kind != CallableParameterContractKindV1::ExactTrivial(
            ExactTrivialParameterAbiV1::USIZE)
        || !matches!(selected.key_for_batch_slot(caller.batch_slot),
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.caller())
        || caller_signature.owner() != caller.owner
        || caller_signature.mode() != caller.mode
        || caller_signature.source_logical_arity() != 1
        || caller_signature.lanes().len() != 1
        || caller_signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
        || caller_signature.lanes()[0].binding() != binding
    {
        return Err(freeze("static-scalar/caller-formal-drift"));
    }
    Ok(())
}

fn scalar_source_only_use_v1(
    draft: &super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1,
) -> bool {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
    let mul = draft.uses.iter().any(|row| matches!(row.kind, Use::MulOperand { .. }));
    let forwards = draft.uses.iter().filter(|row| matches!(row.kind,
        Use::UnresolvedArgument { ordinal: 0, .. })).count();
    mul || (forwards == 1 && draft.uses.iter().all(|row|
        matches!(row.kind, Use::Copy { .. } | Use::UnresolvedArgument { ordinal: 0, .. })))
}

impl OrdinaryNewClaimLedgerV1 {
    fn source_only_usize_scalar_cohort_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> bool {
        let Ok(rows) = source.source_incoming.project(&BTreeSet::from([owner])) else {
            return false;
        };
        !rows.is_empty() && rows.iter().all(|row| {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source else { return false };
            let Some(caller) = contracts.iter().find(|contract| contract.owner == row.call.owner()) else {
                return false;
            };
            let Some(Ok(staged)) = self.borrowed_formal_actuals.get(&row.call) else {
                return false;
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(actual) = &staged.phase else {
                return false;
            };
            if original.call_site() != &row.call
                || original.callee_owner() != owner
                || original.argument_sites().len() != 1
                || original.parameters().len() != 1
                || row.arguments.as_ref() != [(
                    0,
                    original.argument_sites()[0].clone(),
                    original.parameters()[0].binding,
                )]
                || !Rc::ptr_eq(&actual.source, original)
            {
                return false;
            }
            matches!((&caller.parameters[..], &actual.candidates[..]),
                ([formal], [BorrowedCallActualCandidateV1 {
                    value: BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer),
                    ..
                }]) if formal.ordinal == 0
                    && formal.binding == *binding
                    && formal.kind == CallableParameterContractKindV1::ExactTrivial(
                        ExactTrivialParameterAbiV1::USIZE))
                && original.is_current_owner_i64_source_v1()
                && original.required_i64_arguments().is_empty()
                && original.parameters().len() == 1
                && original.parameters()[0].kind.is_ordinary_borrowed_handle()
        })
    }

    /// Borrow only an entirely finished original scalar cohort; never infer
    /// entry permission from one successful caller or from a SourceStatic row.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_scalar_cohort_v1<
        'a,
    >(
        &self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<
        Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
        String,
    > {
        if !source
            .source_only_definitions
            .get(&owner)
            .is_some_and(scalar_source_only_use_v1)
            || source.definitions.contains_key(&owner)
            || source.target_static.contains_key(&owner)
        {
            return Ok(None);
        }
        let rows = source
            .source_incoming
            .project(&BTreeSet::from([owner]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/incoming-veto")))?;
        // A forwarding-only source does not become an entry route merely
        // because another scalar cohort was completed in this package.
        if !source.source_only_definitions[&owner].uses.iter().any(|row| matches!(
            row.kind,
            super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
        )) && !rows.iter().all(|row| matches!(
            self.borrowed_formal_actuals.get(&row.call),
            Some(Ok(actual)) if matches!(
                actual.phase,
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_)
            )
        )) {
            return Ok(None);
        }
        let Some(first) = rows.first() else {
            return Ok(None);
        };
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &first.source
        else {
            return Ok(None);
        };
        let cohort = source.static_incoming_cohort_v1(original)?;
        if cohort.len() != rows.len() {
            return Err(freeze("static-scalar/whole-cohort"));
        }
        let completion = self
            .completion_index
            .get(&owner)
            .ok_or_else(|| freeze("static-scalar/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/completion")))?;
        let mut retained = Vec::with_capacity(rows.len());
        for original in &cohort {
            let mut incoming = source
                .source_incoming
                .exact_rows()
                .filter(|call| call.callee == owner && call.call == *original.call_site());
            let call = incoming
                .next()
                .ok_or_else(|| freeze("static-scalar/incoming-missing"))?;
            if incoming.next().is_some()
                || !matches!(&call.source,
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                        if Rc::ptr_eq(source, original))
                || !original.is_current_owner_i64_source_v1()
                || !(original.required_i64_arguments() == [0]
                    || original.required_i64_arguments().is_empty())
                || original.parameters().len() != 1
                || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
                || call.arguments.as_ref()
                    != [(
                        0,
                        original.argument_sites()[0].clone(),
                        original.parameters()[0].binding,
                    )]
            {
                return Err(freeze("static-scalar/incoming-identity"));
            }
            let Some(actual) = self.borrowed_formal_actuals.get(&call.call) else {
                return Ok(None);
            };
            let actual = actual.as_ref().map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &actual.phase
            else {
                return Ok(None);
            };
            if !proof.corroborates(original, completion)
                || actual.ordered_arguments.as_ref()
                    != [LocalCallArgumentV1::BorrowedActual {
                        ordinal: 0,
                        site: original.argument_sites()[0].clone(),
                    }]
                || actual.opaque_actuals.len() != 1
                || actual.opaque_actuals[0].ordinal != 0
                || actual.opaque_actuals[0].site != original.argument_sites()[0]
                || actual.opaque_actuals[0].formal != original.parameters()[0].binding
                || !matches!(&actual.opaque_actuals[0].source,
                    BorrowedFormalActualSourceV1::Scalar { binding, kind: SourceScalarKind::Integer }
                        if *binding == proof.binding && binding.owner() == call.call.owner())
            {
                return Err(freeze("static-scalar/actual-identity"));
            }
            retained.push(call);
        }
        Ok(Some(retained.into_boxed_slice()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_scalar_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let owners: Vec<_> = source.source_only_definitions.iter()
            .filter(|(owner, draft)| !source.target_static.contains_key(owner)
                && scalar_source_only_use_v1(draft)
                && (draft.uses.iter().any(|row| matches!(row.kind,
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }))
                    || self.source_only_usize_scalar_cohort_v1(source, **owner, contracts)))
            .map(|(owner, _)| *owner).collect();
        for owner in owners {
            let projected = match source.source_incoming.project(&BTreeSet::from([owner])) {
                Ok(rows) if !rows.is_empty() => rows,
                _ => continue, // A veto or absent caller cannot enable this entry.
            };
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(first) =
                &projected[0].source
            else {
                continue;
            };
            let cohort = match source.static_incoming_cohort_v1(first) {
                Ok(rows) if rows.len() == projected.len() => rows,
                _ => continue,
            };
            let forward_only = !source.source_only_definitions[&owner].uses.iter().any(|row| matches!(
                row.kind,
                super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
            ));
            if cohort.iter().any(|original| {
                !original.is_current_owner_i64_source_v1()
                    || (if forward_only {
                        !original.required_i64_arguments().is_empty()
                    } else {
                        original.required_i64_arguments() != [0]
                    })
                    || original.parameters().len() != 1
                    || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            }) {
                continue;
            }
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-scalar/contract-missing"))?;
            if matching.next().is_some()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.batch_slot != first.target_batch_slot()
                || contract.parameters.len() != 1
                || contract.parameters[0].ordinal != 0
                || contract.parameters[0].binding != first.parameters()[0].binding
                || contract.parameters[0].kind != first.parameters()[0].kind
                || !matches!(selected.key_for_batch_slot(contract.batch_slot),
                    Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == first.target())
            {
                return Err(freeze("static-scalar/contract-identity"));
            }
            let draft = &source.source_only_definitions[&owner];
            let scalar_formal_forward = !draft.uses.iter().any(|row| matches!(
                row.kind,
                super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
            ));
            if scalar_formal_forward {
                let mut uses = draft.uses.iter().filter_map(|row| match &row.kind {
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument {
                        call, ordinal: 0,
                    } => Some((row, call)),
                    _ => None,
                });
                let (use_row, call) = uses.next().ok_or_else(|| freeze("static-scalar/forward-use-missing"))?;
                let fact = source.static_arguments.get(&(call.clone(), 0))
                    .ok_or_else(|| freeze("static-scalar/forward-fact-missing"))?;
                if uses.next().is_some()
                    || !source.checked_static_input(contract.parameters[0].binding)
                    || fact.call() != call
                    || fact.ordinal() != 0
                    || fact.use_site() != &use_row.site
                    || fact.formal() != contract.parameters[0].binding
                    || draft.origins.get(&fact.binding()) != Some(&contract.parameters[0].binding)
                {
                    return Err(freeze("static-scalar/forward-chain-drift"));
                }
            }
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/selected-identity"))?;
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/signature-missing"))?;
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
                return Err(freeze("static-scalar/signature-identity"));
            }
            let completion = self
                .completion_index
                .get(&owner)
                .ok_or_else(|| freeze("static-scalar/completion-missing"))?
                .as_ref()
                .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/completion")))?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/result-missing"))?
                .borrow();
            if result.owner() != owner
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_i64_source()
                || !completion.returns_value()
                || completion.owner() != owner
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-scalar/result-completion-identity"));
            }
            let mut replacements = Vec::with_capacity(cohort.len());
            for original in &cohort {
                let site = original.call_site();
                let Some(staged) = self.borrowed_formal_actuals.get(site) else {
                    replacements.clear();
                    break;
                };
                let staged = staged.as_ref().map_err(Clone::clone)?;
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(source_actual) = &staged.phase
                else {
                    replacements.clear();
                    break;
                };
                let [candidate] = source_actual.candidates.as_ref() else {
                    replacements.clear();
                    break;
                };
                let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) =
                    candidate.value
                else {
                    replacements.clear();
                    break;
                };
                if scalar_formal_forward {
                    check_exact_usize_scalar_caller_v1(
                        selected, contracts, signatures, original, binding,
                    )?;
                }
                if !Rc::ptr_eq(&source_actual.source, original)
                    || source_actual.integer_evidence.as_ref() != [true]
                    || candidate.ordinal != 0
                    || candidate.site != original.argument_sites()[0]
                    || binding.owner() != site.owner()
                    || !staged.opaque_actuals.is_empty()
                    || staged.ordered_arguments.as_ref()
                        != [LocalCallArgumentV1::BorrowedActual {
                            ordinal: 0,
                            site: candidate.site.clone(),
                        }]
                    || projected.iter().filter(|row| row.call == *site).count() != 1
                {
                    replacements.clear();
                    break;
                }
                replacements.push((
                    site.clone(),
                    PreparedBorrowedCallActualsV1 {
                        phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(
                            StaticScalarInputFinishV1 {
                                source: Rc::clone(original),
                                completion: Rc::clone(completion),
                                binding,
                            },
                        ),
                        opaque_actuals: Box::new([PreparedBorrowedFormalActualV1 {
                            ordinal: 0,
                            site: candidate.site.clone(),
                            formal: original.parameters()[0].binding,
                            source: BorrowedFormalActualSourceV1::Scalar {
                                binding,
                                kind: SourceScalarKind::Integer,
                            },
                        }]),
                        ordered_arguments: staged.ordered_arguments.clone(),
                    },
                ));
            }
            if replacements.len() == cohort.len() {
                for (site, actuals) in replacements {
                    self.borrowed_formal_actuals.insert(site, Ok(actuals));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_scalar_input_finish_tests.rs"]
mod tests;
