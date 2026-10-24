//! Complete one original Bool CurrentOwner input for the borrowed entry only.
//! This proof never selects a Static packet or a Bool Invoke.
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
use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticBoolEntryFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    binding: BindingRefV1,
}

impl PartialEq for StaticBoolEntryFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && self.binding == other.binding
    }
}
impl Eq for StaticBoolEntryFinishV1 {}

impl PreparedBorrowedCallActualsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn is_entry_only_static_bool_v1(
        &self,
    ) -> bool {
        matches!(
            self.phase,
            BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(_)
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn ordered_arguments_for_entry_v1(
        &self,
        call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> Result<&[LocalCallArgumentV1], String> {
        if !self.is_entry_only_static_bool_v1() {
            self.require_executable_v1()?;
        }
        self.checked_ordered_arguments_v1(call)
    }
}

impl StaticBoolEntryFinishV1 {
    fn corroborates(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        actual: &PreparedBorrowedCallActualsV1,
    ) -> bool {
        Rc::ptr_eq(&self.source, original)
            && Rc::ptr_eq(&self.completion, completion)
            && completion.owner() == original.callee_owner()
            && original.is_current_owner_bool_source_v1()
            && original.argument_sites().len() == 1
            && original.parameters().len() == 1
            && original.parameters()[0].kind.is_ordinary_borrowed_handle()
            && self.binding.owner() == original.call_site().owner()
            && actual.ordered_arguments.as_ref()
                == [LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: original.argument_sites()[0].clone(),
                }]
            && matches!(actual.opaque_actuals.as_ref(), [PreparedBorrowedFormalActualV1 {
                ordinal: 0,
                site,
                formal,
                source: BorrowedFormalActualSourceV1::Scalar {
                    binding,
                    kind: SourceScalarKind::Integer,
                },
            }] if site == &original.argument_sites()[0]
                && formal == &original.parameters()[0].binding
                && binding == &self.binding)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_bool_entry_v1<
        'a,
    >(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<
        Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
        String,
    > {
        let has_proof = self.borrowed_formal_actuals.values().any(|row| {
            matches!(row,
            Ok(actual) if matches!(&actual.phase,
                BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(proof)
                    if proof.source.callee_owner() == owner))
        });
        if !has_proof {
            return Ok(None);
        }
        if !source.source_only_definitions.contains_key(&owner)
            || source.definitions.contains_key(&owner)
            || source.target_static.contains_key(&owner)
            || source.source_incoming.has_unsupported_static_context(owner)
            || source
                .source_incoming
                .has_unsupported_static_spelling(owner)
        {
            return Err(freeze("static-bool-entry/source-authority"));
        }
        let rows = source
            .source_incoming
            .project(&BTreeSet::from([owner]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-bool-entry/incoming-veto")))?;
        let [row] = rows.as_ref() else {
            return Err(freeze("static-bool-entry/whole-cohort"));
        };
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &row.source
        else {
            return Err(freeze("static-bool-entry/source-kind"));
        };
        let completion = self
            .completion_index
            .get(&owner)
            .ok_or_else(|| freeze("static-bool-entry/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-bool-entry/completion")))?;
        let actual = self
            .borrowed_formal_actuals
            .get(&row.call)
            .ok_or_else(|| freeze("static-bool-entry/actual-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(proof) = &actual.phase else {
            return Err(freeze("static-bool-entry/phase-drift"));
        };
        if row.callee != owner
            || original.call_site() != &row.call
            || row.arguments.as_ref()
                != [(
                    0,
                    original.argument_sites()[0].clone(),
                    original.parameters()[0].binding,
                )]
            || !proof.corroborates(original, completion, actual)
            || !source.checked_static_input(original.parameters()[0].binding)
        {
            return Err(freeze("static-bool-entry/identity"));
        }
        Ok(Some(Box::new([source
            .source_incoming
            .exact_rows()
            .find(|candidate| candidate.callee == owner && candidate.call == row.call)
            .ok_or_else(|| {
                freeze("static-bool-entry/retained-row-missing")
            })?])))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_bool_entry_actual_v1(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let mut replacements = Vec::new();
        for owner in source.source_only_definitions.keys().copied() {
            let candidates: Vec<_> = source
                .source_incoming
                .exact_rows()
                .filter(|row| {
                    row.callee == owner
                        && matches!(&row.source,
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                        if original.is_current_owner_bool_source_v1())
                })
                .collect();
            if candidates.is_empty() {
                continue;
            }
            if source.source_incoming.has_unsupported_static_context(owner)
                || source
                    .source_incoming
                    .has_unsupported_static_spelling(owner)
            {
                return Err(freeze("static-bool-entry/source-context"));
            }
            let rows = source
                .source_incoming
                .project(&BTreeSet::from([owner]))
                .map_err(|error| {
                    format!("{}: {error:?}", freeze("static-bool-entry/incoming-veto"))
                })?;
            let [row] = rows.as_ref() else {
                return Err(freeze("static-bool-entry/whole-cohort"));
            };
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                return Err(freeze("static-bool-entry/source-kind"));
            };
            if !original.is_current_owner_bool_source_v1()
                || original.parameters().len() != 1
                || original.argument_sites().len() != 1
                || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
                || row.call != *original.call_site()
                || row.arguments.as_ref()
                    != [(
                        0,
                        original.argument_sites()[0].clone(),
                        original.parameters()[0].binding,
                    )]
                || !source.checked_static_input(original.parameters()[0].binding)
            {
                return Err(freeze("static-bool-entry/source-identity"));
            }
            let mut callers = contracts
                .iter()
                .filter(|caller| caller.owner == row.call.owner());
            let caller = callers
                .next()
                .ok_or_else(|| freeze("static-bool-entry/caller-missing"))?;
            let [formal] = caller.parameters.as_ref() else {
                return Err(freeze("static-bool-entry/caller-arity"));
            };
            if callers.next().is_some() || formal.ordinal != 0 {
                return Err(freeze("static-bool-entry/caller-identity"));
            }
            let (exact_return, binding) = batch
                .with_lowering_input(caller.batch_slot, |input| {
                    (
                    super::super::borrowed_formal_uses::exact_bool_value_return_site_v1(
                        input, row.call.site(),
                    ),
                    (input.owner() == row.call.owner()
                        && matches!(input.function().variable_ref(&original.argument_sites()[0]),
                            Some(ResolvedLexicalRefV1::Local(found)) if found == formal.binding))
                    .then_some(formal.binding),
                )
                })
                .map_err(|_| freeze("static-bool-entry/caller-source-loan"))?;
            if !exact_return {
                return Err(freeze("static-bool-entry/source-context"));
            }
            let binding =
                binding.ok_or_else(|| freeze("static-bool-entry/caller-source-binding"))?;
            // A direct value-return call does not enter the Home expression
            // walk. Issue its source-only candidate from the same incoming
            // row and caller formal through the existing Static source issuer.
            let staged = match self.borrowed_formal_actuals.get(&row.call) {
                Some(Ok(staged)) => staged.clone(),
                Some(Err(error)) => return Err(error.clone()),
                None => {
                    let candidate = BorrowedCallActualCandidateV1 {
                        ordinal: 0,
                        site: original.argument_sites()[0].clone(),
                        value: BorrowedCallActualValueV1::Scalar(
                            binding,
                            SourceScalarKind::Integer,
                        ),
                    };
                    super::static_source::prepare_static_source_actuals_v1(
                        source,
                        contracts,
                        &row.call,
                        &[candidate],
                    )?
                    .ok_or_else(|| freeze("static-bool-entry/source-actual-missing"))?
                }
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(source_actual) = &staged.phase
            else {
                return Err(freeze("static-bool-entry/source-actual-phase"));
            };
            let [candidate] = source_actual.candidates.as_ref() else {
                return Err(freeze("static-bool-entry/source-actual-cardinality"));
            };
            let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) =
                candidate.value
            else {
                return Err(freeze("static-bool-entry/source-actual-kind"));
            };
            if !Rc::ptr_eq(&source_actual.source, original)
                || source_actual.integer_evidence.as_ref() != [true]
                || candidate.ordinal != 0
                || candidate.site != original.argument_sites()[0]
                || !staged.opaque_actuals.is_empty()
                || staged.ordered_arguments.as_ref()
                    != [LocalCallArgumentV1::BorrowedActual {
                        ordinal: 0,
                        site: candidate.site.clone(),
                    }]
            {
                return Err(freeze("static-bool-entry/source-actual-identity"));
            }
            super::static_scalar_input_finish::check_exact_usize_scalar_caller_v1(
                selected, contracts, signatures, original, binding,
            )?;
            let mut matching = contracts.iter().filter(|contract| contract.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-bool-entry/contract-missing"))?;
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-bool-entry/selected-identity"))?;
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-bool-entry/signature-missing"))?;
            if matching.next().is_some()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.batch_slot != original.target_batch_slot()
                || contract.parameters.len() != 1
                || contract.parameters[0].binding != original.parameters()[0].binding
                || contract.parameters[0].kind != original.parameters()[0].kind
                || !matches!(selected.key_for_batch_slot(contract.batch_slot),
                    Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.target())
                || signature.owner() != owner
                || !signature.identity().same_as(identity)
                || signature.lanes().len() != 1
                || signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                || signature.lanes()[0].binding() != original.parameters()[0].binding
            {
                return Err(freeze("static-bool-entry/contract-signature"));
            }
            let completion = self
                .completion_index
                .get(&owner)
                .ok_or_else(|| freeze("static-bool-entry/completion-missing"))?
                .as_ref()
                .map_err(|error| {
                    format!("{}: {error:?}", freeze("static-bool-entry/completion"))
                })?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-bool-entry/result-missing"))?
                .borrow();
            if result.owner() != owner
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_bool_source()
                || !completion.returns_value()
                || completion.owner() != owner
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-bool-entry/result-completion"));
            }
            replacements.push((
                row.call.clone(),
                PreparedBorrowedCallActualsV1 {
                    phase: BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(
                        StaticBoolEntryFinishV1 {
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
                    dynamic_invocation_arguments: None,
                },
            ));
        }
        for (site, actual) in replacements {
            self.borrowed_formal_actuals.insert(site, Ok(actual));
        }
        Ok(())
    }
}
