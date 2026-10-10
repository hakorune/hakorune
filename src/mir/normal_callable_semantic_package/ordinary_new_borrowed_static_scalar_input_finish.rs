//! Complete one original CurrentOwner Static scalar cohort in the existing ledger.
//! The later physical caller read remains an independent publication obligation.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
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
            && original.required_i64_arguments() == [0]
            && original.parameters().len() == 1
            && original.argument_sites().len() == 1
            && self.binding.owner() == original.call_site().owner()
            && completion.owner() == original.callee_owner()
    }
}

impl OrdinaryNewClaimLedgerV1 {
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
            .is_some_and(|draft| {
                draft.uses.iter().any(|row| {
                    matches!(
                row.kind,
                super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
            )
                })
            })
            || source.definitions.contains_key(&owner)
            || source.target_static.contains_key(&owner)
        {
            return Ok(None);
        }
        let rows = source
            .source_incoming
            .project(&BTreeSet::from([owner]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/incoming-veto")))?;
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
                || original.required_i64_arguments() != [0]
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
                && draft.uses.iter().any(|row| matches!(row.kind,
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. })))
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
            if cohort.iter().any(|original| {
                !original.is_current_owner_i64_source_v1()
                    || original.required_i64_arguments() != [0]
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
