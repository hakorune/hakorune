//! Close one original CurrentOwner Static forwarded input after signature and Completion.
//! The selected source remains passive until this exact whole cohort is finished.
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
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticOneInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
}

impl PartialEq for StaticOneInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && Rc::ptr_eq(&self.completion, &other.completion)
    }
}
impl Eq for StaticOneInputFinishV1 {}

impl StaticOneInputFinishV1 {
    pub(super) fn corroborates_source_v1(&self, source: &Rc<StaticIncomingSourceV1>) -> bool {
        Rc::ptr_eq(&self.source, source)
            && source.is_current_owner_i64_source_v1()
            && source.argument_sites().len() == 1
            && self.completion.owner() == source.callee_owner()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_one_actuals_v1(
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
        let target = source.target_static.get(&original.callee_owner());
        if !matches!(cohort.len(), 1 | 2)
            || !cohort.iter().any(|row| Rc::ptr_eq(row, original))
            || (cohort.len() == 2 && target.is_none())
            || (cohort.len() == 1 && target.is_some())
        {
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
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(finish) = &row.phase else {
            return Ok(None);
        };
        let completion = self
            .completion_index
            .get(&original.callee_owner())
            .ok_or_else(|| freeze("static-one/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-one/completion")))?;
        if !finish.corroborates_source_v1(original) || !Rc::ptr_eq(&finish.completion, completion) {
            return Err(freeze("static-one/completion-drift"));
        }
        if cohort.len() == 2 {
            for sibling in &cohort {
                let ready = self
                    .borrowed_formal_actuals
                    .get(sibling.call_site())
                    .ok_or_else(|| freeze("static-one/cohort-actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                let BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(proof) = &ready.phase
                else {
                    return Err(freeze("static-one/cohort-unfinished"));
                };
                if !proof.corroborates_source_v1(sibling)
                    || !Rc::ptr_eq(&proof.completion, completion)
                {
                    return Err(freeze("static-one/cohort-completion-drift"));
                }
            }
        }
        let incoming_rows = target.map_or(source.incoming.as_ref(), |row| row.incoming.as_ref());
        let mut incoming = incoming_rows.iter().filter(|call| &call.call == original.call_site());
        let call = incoming
            .next()
            .ok_or_else(|| freeze("static-one/incoming-missing"))?;
        if incoming.next().is_some()
            || call.callee != original.callee_owner()
            || !matches!(&call.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                    if Rc::ptr_eq(retained, original))
        {
            return Err(freeze("static-one/incoming-identity"));
        }
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

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_one_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
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
            cohort.incoming.iter().filter_map(|call| match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.is_current_owner_i64_source_v1() => Some(Rc::clone(original)),
                _ => None,
            })
        }));
        let mut replacements = Vec::new();
        for original in originals {
            let cohort = source.static_incoming_cohort_v1(&original)?;
            let target = source.target_static.get(&original.callee_owner());
            if !matches!(cohort.len(), 1 | 2)
                || !cohort.iter().any(|row| Rc::ptr_eq(row, &original))
                || (cohort.len() == 2 && target.is_none())
                || (cohort.len() == 1 && target.is_some())
            {
                return Err(freeze("static-one/whole-cohort"));
            }
            let owner = original.callee_owner();
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-one/contract-missing"))?;
            if matching.next().is_some()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.batch_slot != original.target_batch_slot()
                || contract.parameters.len() != 1
                || contract.parameters[0].ordinal != 0
                || contract.parameters[0].binding != original.parameters()[0].binding
                || contract.parameters[0].kind != original.parameters()[0].kind
                || !contract.parameters[0].kind.is_ordinary_borrowed_handle()
                || !(source.definitions.contains_key(&owner)
                    || source.target_static.contains_key(&owner))
            {
                return Err(freeze("static-one/contract-identity"));
            }
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-one/selected-identity"))?;
            if !matches!(selected.key_for_batch_slot(contract.batch_slot),
                Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.target())
            {
                return Err(freeze("static-one/selected-target"));
            }
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-one/signature-missing"))?;
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
                return Err(freeze("static-one/signature-identity"));
            }
            let completion = self
                .completion_index
                .get(&owner)
                .ok_or_else(|| freeze("static-one/completion-missing"))?
                .as_ref()
                .map_err(|error| format!("{}: {error:?}", freeze("static-one/completion")))?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-one/result-missing"))?;
            let result = result.borrow();
            if result.owner() != owner
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_i64_source()
                || !completion.returns_value()
                || completion.owner() != owner
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-one/result-completion-identity"));
            }
            let site = original.call_site();
            let incoming_rows = target.map_or(source.incoming.as_ref(), |row| row.incoming.as_ref());
            let mut incoming = incoming_rows.iter().filter(|row| &row.call == site);
            let call = incoming
                .next()
                .ok_or_else(|| freeze("static-one/incoming-missing"))?;
            if incoming.next().is_some()
                || call.callee != owner
                || !matches!(&call.source,
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                        if Rc::ptr_eq(retained, &original))
            {
                return Err(freeze("static-one/incoming-identity"));
            }
            let fact = source
                .static_arguments
                .get(&(site.clone(), 0))
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
            if !Rc::ptr_eq(&source_actual.source, &original)
                || source_actual.integer_evidence.len() != 1
                || staged.opaque_actuals.len() != 0
                || staged.ordered_arguments.as_ref()
                    != [LocalCallArgumentV1::BorrowedActual {
                        ordinal: 0,
                        site: original.argument_sites()[0].clone(),
                    }]
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
                PreparedBorrowedCallActualsV1 {
                    phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(
                        StaticOneInputFinishV1 {
                            source: original,
                            completion: Rc::clone(completion),
                        },
                    ),
                    opaque_actuals: Box::new([issued.actual().clone()]),
                    ordered_arguments: staged.ordered_arguments.clone(),
                },
            ));
        }
        for (site, actuals) in replacements {
            self.borrowed_formal_actuals.insert(site, Ok(actuals));
        }
        Ok(())
    }
}
