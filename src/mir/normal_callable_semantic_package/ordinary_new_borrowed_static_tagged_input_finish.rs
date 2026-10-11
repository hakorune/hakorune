//! Finish Main literal actuals against the selected existing tagged entry.
//! Forwarded siblings keep their original SourceStatic witness and consumers.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticTaggedInputFinishV1 {
    identity: static_source::StaticSourceActualIdentityV1,
    formal: BindingRefV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
}

impl PartialEq for StaticTaggedInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
            && self.formal == other.formal
            && Rc::ptr_eq(&self.completion, &other.completion)
    }
}
impl Eq for StaticTaggedInputFinishV1 {}

fn literal_identity_v1(
    source: &PreparedBorrowedFormalIngressV1,
    original: &Rc<StaticIncomingSourceV1>,
    identity: &static_source::StaticSourceActualIdentityV1,
    rows: &PreparedBorrowedCallActualsV1,
    formal: BindingRefV1,
) -> Result<Option<i64>, String> {
    if !source.is_main_static_terminal_source_v1(original) {
        return Ok(None);
    }
    let [candidate] = identity.candidates.as_ref() else {
        return Err(freeze("static-tagged/literal-cardinality"));
    };
    let BorrowedCallActualValueV1::Integer(value) = candidate.value else {
        return Ok(None);
    };
    if !Rc::ptr_eq(&identity.source, original)
        || identity.integer_evidence.as_ref() != [true]
        || formal.owner() != original.callee_owner()
        || original.parameters()[0].binding != formal
        || original.parameters()[0].ordinal != 0
        || candidate.ordinal != 0
        || original.argument_sites() != [candidate.site.clone()]
        || rows.ordered_arguments.as_ref()
            != [LocalCallArgumentV1::BorrowedActual {
                ordinal: 0,
                site: candidate.site.clone(),
            }]
    {
        return Err(freeze("static-tagged/literal-identity"));
    }
    Ok(Some(value))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Validate the complete original incoming set; readiness of one Main row
    /// cannot erase an unfinished, foreign, duplicate or unresolved sibling.
    fn checked_tagged_cohort_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        seed: &Rc<StaticIncomingSourceV1>,
        formal: BindingRefV1,
        completion: &Rc<VerifiedFunctionCompletionV1>,
        finished: bool,
    ) -> Result<bool, String> {
        let cohort = source.static_incoming_cohort_v1(seed)?;
        if completion.owner() != formal.owner() || !completion.returns_value() {
            return Err(freeze("static-tagged/completion-identity"));
        }
        for original in &cohort {
            if original.callee_owner() != formal.owner()
                || original.parameters().len() != 1
                || original.parameters()[0].binding != formal
                || original.parameters()[0].ordinal != 0
                || original.parameters()[0].kind != CallableParameterContractKindV1::OpaqueHandle
                || original.argument_sites().len() != 1
            {
                return Err(freeze("static-tagged/cohort-formal"));
            }
            let Some(rows) = self.borrowed_formal_actuals.get(original.call_site()) else {
                return Ok(false);
            };
            let rows = rows.as_ref().map_err(Clone::clone)?;
            if source.is_main_static_terminal_source_v1(original) {
                let identity = match &rows.phase {
                    BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) if !finished => {
                        if !rows.opaque_actuals.is_empty() {
                            return Err(freeze("static-tagged/source-has-actuals"));
                        }
                        identity
                    }
                    BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(proof)
                        if finished =>
                    {
                        if proof.formal != formal || !Rc::ptr_eq(&proof.completion, completion) {
                            return Err(freeze("static-tagged/finished-identity"));
                        }
                        &proof.identity
                    }
                    _ => return Ok(false),
                };
                let Some(value) = literal_identity_v1(source, original, identity, rows, formal)?
                else {
                    return Ok(false);
                };
                if finished
                    && rows.opaque_actuals.as_ref()
                        != [PreparedBorrowedFormalActualV1 {
                            ordinal: 0,
                            site: original.argument_sites()[0].clone(),
                            formal,
                            source: BorrowedFormalActualSourceV1::Integer(value),
                        }]
                {
                    return Err(freeze("static-tagged/finished-payload"));
                }
            } else {
                let Some(fact) = source
                    .static_arguments
                    .get(&(original.call_site().clone(), 0))
                else {
                    return Ok(false);
                };
                // This issuer checks Rc/site/ordinal/formal/origin and retains
                // SourceStatic. Do not promote or overwrite forwarded rows.
                let forwarded = issue_original_static_forwarded_actual_v1(
                    source,
                    &self.borrowed_formal_actuals,
                    original,
                    fact.formal(),
                )?;
                if !forwarded.corroborates(original, fact.formal()) {
                    return Err(freeze("static-tagged/forwarded-identity"));
                }
            }
        }
        Ok(true)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_static_tagged_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(None);
        };
        let Some(Ok(rows)) = self.borrowed_formal_actuals.get(original.call_site()) else {
            return Ok(None);
        };
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(proof) = &rows.phase else {
            return Ok(None);
        };
        let completion = self
            .completion_index
            .get(&original.callee_owner())
            .ok_or_else(|| freeze("static-tagged/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-tagged/completion")))?;
        if !Rc::ptr_eq(&proof.identity.source, original)
            || !self.checked_tagged_cohort_v1(source, original, proof.formal, completion, true)?
        {
            return Err(freeze("static-tagged/cohort-unfinished"));
        }
        let observation = self
            .local_call_for_owner(original.call_site().owner(), original.call_site().site())
            .ok_or_else(|| freeze("static-tagged/local-observation-missing"))?;
        if observation.arguments() != rows.ordered_arguments.as_ref()
            || observation.local_binding().is_some()
        {
            return Err(freeze("static-tagged/local-observation-drift"));
        }
        rows.require_executable_v1()?;
        Ok(Some(rows.opaque_actuals.as_ref()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_tagged_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let requested: BTreeSet<_> = source
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .filter(|original| source.is_main_static_terminal_source_v1(original))
            .map(|original| original.parameters()[0].binding)
            .collect();
        if requested.is_empty() {
            return Ok(());
        }
        let mut formals = BTreeSet::new();
        // The affine source may still be residual or already owned by the
        // preissued V2 product. Borrow both, never take or rebuild either.
        let entries = self.loop_entry_static_i64_source_loans.borrow();
        for (key, tagged) in self.loop_static_tagged_entry.borrow().iter() {
            let (Ok(tagged), Some(Ok(entry))) = (tagged, entries.get(key)) else {
                continue;
            };
            if !requested.contains(&tagged.formal()) {
                continue;
            }
            if !entry.publication_brand().is_same(selected.catalog_brand())
                || !tagged.corroborates(entry)
                || !formals.insert(tagged.formal())
            {
                return Err(freeze("static-tagged/entry-source-drift"));
            }
        }
        for (owner, product) in self.preissued_static_i64_products.borrow().iter() {
            let Ok(product) = product else { continue };
            let (entry, tagged) = product.packet_source_inputs();
            if !requested.contains(&tagged.formal()) {
                continue;
            }
            if tagged.formal().owner() != *owner
                || !entry.publication_brand().is_same(selected.catalog_brand())
                || !tagged.corroborates(entry)
                || !formals.insert(tagged.formal())
            {
                return Err(freeze("static-tagged/product-source-drift"));
            }
        }
        let mut replacements = Vec::new();
        for original in source
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
        {
            if !source.is_main_static_terminal_source_v1(original) {
                continue;
            }
            let formal = original.parameters()[0].binding;
            if !formals.contains(&formal) {
                continue;
            }
            let mut contracts = contracts.iter().filter(|row| row.owner == formal.owner());
            let contract = contracts
                .next()
                .ok_or_else(|| freeze("static-tagged/contract-missing"))?;
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-tagged/selected-identity"))?;
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-tagged/signature-missing"))?;
            let lanes = signature.lanes();
            if contracts.next().is_some()
                || contract.batch_slot != original.target_batch_slot()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.parameters.len() != 1
                || contract.parameters[0].ordinal != 0
                || contract.parameters[0].binding != formal
                || contract.parameters[0].kind != CallableParameterContractKindV1::OpaqueHandle
                || !matches!(selected.key_for_batch_slot(contract.batch_slot), Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.target())
                || !signature.identity().same_as(identity)
                || signature.owner() != formal.owner()
                || signature.mode() != contract.mode
                || signature.source_logical_arity() != 1
                || signature.receiver_lane_count() != 0
                || signature.physical_formal_lane_count() != 1
                || signature.physical_callable_lane_count() != 1
                || lanes.len() != 1
                || lanes[0].index() != 0
                || lanes[0].logical_ordinal() != Some(0)
                || lanes[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                || lanes[0].binding() != formal
            {
                return Err(freeze("static-tagged/signature-contract-drift"));
            }
            let completion = self
                .completion_index
                .get(&formal.owner())
                .ok_or_else(|| freeze("static-tagged/completion-missing"))?
                .as_ref()
                .map_err(|error| format!("{}: {error:?}", freeze("static-tagged/completion")))?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-tagged/result-missing"))?
                .borrow();
            if result.owner() != formal.owner()
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_i64_source()
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-tagged/result-completion-drift"));
            }
            if !self.checked_tagged_cohort_v1(source, original, formal, completion, false)? {
                continue;
            }
            let rows = self
                .borrowed_formal_actuals
                .get(original.call_site())
                .ok_or_else(|| freeze("static-tagged/actual-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(actual) = &rows.phase else {
                return Err(freeze("static-tagged/source-phase"));
            };
            let value = literal_identity_v1(source, original, actual, rows, formal)?
                .ok_or_else(|| freeze("static-tagged/literal-unavailable"))?;
            replacements.push((
                original.call_site().clone(),
                PreparedBorrowedCallActualsV1 {
                    phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(
                        StaticTaggedInputFinishV1 {
                            identity: actual.clone(),
                            formal,
                            completion: Rc::clone(completion),
                        },
                    ),
                    ordered_arguments: rows.ordered_arguments.clone(),
                    opaque_actuals: Box::new([PreparedBorrowedFormalActualV1 {
                        ordinal: 0,
                        site: original.argument_sites()[0].clone(),
                        formal,
                        source: BorrowedFormalActualSourceV1::Integer(value),
                    }]),
                },
            ));
        }
        // Every selected cohort passed before replacing any actual or route.
        for (site, rows) in replacements {
            self.borrowed_formal_actuals.insert(site.clone(), Ok(rows));
            if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
                selected.insert(site);
            }
        }
        Ok(())
    }
}
