//! Recheck retained whole-input evidence against the same source and completion.
use super::*;
use super::static_input_finish::{StaticInputFinishV1, StaticInputEvidenceV1, original_zero_input_source_v1};
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewClaimLedgerV1,
};
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    /// One signature/result/Completion correspondence for both zero and
    /// qualified source inputs. Specialized scalar/mixed owners remain separate.
    pub(super) fn verify_static_input_header_v1(
        &self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        tag: &str,
    ) -> Result<bool, String> {
        use crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1 as Role;
        let fail = |suffix: &str| freeze(&format!("{tag}/{suffix}"));
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(fail("selected-key"));
        };
        if contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
            || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || contract.parameters.iter().enumerate().any(|(ordinal, formal)|
                formal.ordinal as usize != ordinal || formal.binding.owner() != contract.owner
                    || !(matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                        || matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64())
                        || formal.takes_declared_object_v1())) {
            return Err(fail("contract-identity"));
        }
        let Some(signature) = signatures.row(contract.batch_slot) else {
            return Ok(false);
        };
        let identity = selected
            .identity_for_batch_slot(contract.batch_slot)
            .ok_or_else(|| fail("signature-identity"))?;
        let arity = contract.parameters.len() as u32;
        if !signature.identity().same_as(identity)
            || signature.owner() != contract.owner
            || signature.mode() != contract.mode
            || signature.source_logical_arity() != arity
            || signature.receiver_lane_count() != 0
            || signature.physical_formal_lane_count() != arity
            || signature.physical_callable_lane_count() != arity
            || signature.lanes().len() != arity as usize
            || signature
                .lanes()
                .iter()
                .zip(&contract.parameters)
                .any(|(lane, formal)| {
                    lane.index() != formal.ordinal
                        || lane.logical_ordinal() != Some(formal.ordinal)
                        || lane.binding() != formal.binding
                        || lane.role() != Role::OrdinaryScalar
                })
        {
            return Err(fail("signature-identity"));
        }
        let Some(result) = results.row(contract.batch_slot) else {
            return Ok(false);
        };
        let result = result.borrow();
        if result.owner() != contract.owner
            || !result.identity().same_as(identity)
            || !result.declared_result_agrees_with_source()
            || !completion.returns_value()
            || completion.owner() != contract.owner
            || !std::ptr::eq(result.completion(), completion.as_ref())
        {
            return Err(fail("result-completion-identity"));
        }
        Ok(true)
    }

    /// Borrow only the SAME completed whole input cohort. No caller result,
    /// transport or affine publication permission is issued by this lender.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_zero_arguments_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        if !original.is_zero_input_source_v1() {
            return Err(freeze("static-zero/source-contract"));
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("static-zero/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let retained = original_zero_input_source_v1(source, original)?
            .ok_or_else(|| freeze("static-zero/original-source-missing"))?;
        if !Rc::ptr_eq(retained, original) {
            return Err(freeze("static-zero/original-source-drift"));
        }
        let owner = original.callee_owner();
        let owners = [owner].into_iter().collect();
        let calls = if original.is_zeroarg_text_v1() {
            source.source_incoming.project_source_v1(&owners)
        } else {
            source.source_incoming.project(&owners)
        }
        .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-formal/incoming-coverage")))?;
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("static-zero/callee-completion")))?;
        if completion.owner() != owner {
            return Err(freeze("static-zero/completion-owner"));
        }
        let mut missing = false;
        let mut requested = None;
        let mut seen = std::collections::BTreeSet::new();
        for call in &calls {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-zero/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source_call) =
                &call.source
            else {
                return Err(freeze("static-zero/original-source-kind"));
            };
            let retained = original_zero_input_source_v1(source, source_call)?
                .ok_or_else(|| freeze("static-zero/original-source-missing"))?;
            if !Rc::ptr_eq(source_call, retained)
                || !source_call.is_zero_input_source_v1()
                || source_call.target() != original.target()
                || source_call.callee_owner() != owner
                || source_call.target_batch_slot() != original.target_batch_slot()
                || source_call.call_site() != &call.call
                || !call.arguments.is_empty()
            {
                return Err(freeze("static-zero/original-target"));
            }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let actuals = actuals.as_ref().map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) = &actuals.phase
            else {
                missing = true;
                continue;
            };
            if !finish.corroborates_source_v1(source_call)
                || !Rc::ptr_eq(&finish.completion, completion)
            {
                return Err(freeze("static-zero/completion-drift"));
            }
            let arguments = actuals.ordered_arguments_for_v1(call)?;
            if !arguments.is_empty() || !actuals.opaque_actuals.is_empty() {
                return Err(freeze("static-zero/argument-snapshot-drift"));
            }
            if &call.call == original.call_site() {
                if requested.is_some() {
                    return Err(freeze("static-zero/original-row-duplicate"));
                }
                requested = Some(arguments);
            }
        }
        for retained in source
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
        {
            if retained.callee_owner() == owner && !seen.contains(retained.call_site()) {
                return Err(freeze("static-zero/original-row-missing"));
            }
        }
        Ok(if missing { None } else { requested })
    }

    /// Reborrow the same completed Text source retained by whole-input finishing.
    pub(in crate::mir) fn checked_completed_static_text_result_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<bool, String> {
        let Some(rows) = self.borrowed_formal_actuals.get(original.call_site()) else {
            return Ok(false);
        };
        let rows = rows.as_ref().map_err(Clone::clone)?;
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) = &rows.phase else {
            return Ok(false);
        };
        let Some(text) = &finish.text_result else {
            return Ok(false);
        };
        let (selected, _) = self
            .original_instance_input_issuance
            .as_ref()
            .ok_or_else(|| freeze("static-input/completed-source-drift"))?;
        let completion = self
            .completion_source_for_owner_v1(original.callee_owner())
            .ok_or_else(|| freeze("static-input/completed-source-drift"))?;
        let terminals = self
            .terminal_relation_index
            .get(&original.callee_owner())
            .ok_or_else(|| freeze("static-input/completed-source-drift"))?;
        if !Rc::ptr_eq(&finish.source, original)
            || !Rc::ptr_eq(&finish.completion, completion)
            || !text.matches_retained(completion, terminals)
            || !text.corroborates(selected, original)
        {
            return Err(freeze("static-input/completed-source-drift"));
        }
        Ok(true)
    }

    pub(in crate::mir) fn corroborates_completed_static_text_publication_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        publication: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Result<bool, String> {
        if !self.checked_completed_static_text_result_v1(original)? {
            return Ok(false);
        }
        let rows = self.borrowed_formal_actuals[original.call_site()]
            .as_ref()
            .map_err(Clone::clone)?;
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) = &rows.phase else {
            return Ok(false);
        };
        let Some(text) = &finish.text_result else {
            return Ok(false);
        };
        let (selected, _) = self
            .original_instance_input_issuance
            .as_ref()
            .ok_or_else(|| freeze("static-input/completed-source-drift"))?;
        Ok(original.corroborates_completed_text_publication_v1(publication, selected, text))
    }

    /// Publication borrows the completed input proof, without exposing or
    /// issuing another argument cohort to the packet consumer.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_completed_static_input_publication_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        publication: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Result<bool, String> {
        Ok(self.checked_completed_static_expression_input_v1(original)?.is_some()
            && original.matches_publication_handoff_identity(publication))
    }

    /// Demand the already completed whole cohort; unlike preparation this
    /// never accepts a sibling that could merely be completed in a future pass.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_completed_static_expression_input_v1(
        &self,
        requested: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&PreparedBorrowedCallActualsV1>, String> {
        let Some(rows) = self.borrowed_formal_actuals.get(requested.call_site()) else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[static-input/readiness] call={:?} actual-row=missing",
                    requested.call_site()
                );
            }
            return Ok(None);
        };
        let rows = rows.as_ref().map_err(Clone::clone)?;
        if !matches!(
            &rows.phase,
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(StaticInputFinishV1 {
                evidence: StaticInputEvidenceV1::Qualified { .. },
                ..
            })
        ) {
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[static-input/readiness] call={:?} phase={:?} opaque={} args={}",
                    requested.call_site(),
                    rows.phase,
                    rows.opaque_actuals.len(),
                    rows.ordered_arguments.len()
                );
            }
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("static-input/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let owner = requested.callee_owner();
        if !source
            .source_incoming
            .source_registered_owners_v1()
            .contains(&owner)
            || source
                .source_incoming
                .sum_vetoed_owners_v1()
                .contains(&owner)
        {
            return Err(freeze("static-input/source-cohort-veto"));
        }
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("static-input/catalog-missing"))?;
        let mut matching = catalog.iter().filter(|row| row.owner == owner);
        let contract = matching
            .next()
            .ok_or_else(|| freeze("static-input/contract-missing"))?;
        if matching.next().is_some() {
            return Err(freeze("static-input/contract-duplicate"));
        }
        let completion = self
            .completion_index
            .get(&owner)
            .ok_or_else(|| freeze("static-input/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-input/callee-completion")))?;
        let uses = self
            .checked_static_input_uses_v1(contract)?
            .ok_or_else(|| freeze("static-input/source-uses-unfinished"))?;
        let domains = self
            .static_input_domains_v1(source, contract)
            .ok_or_else(|| freeze("static-input/domain-unfinished"))?;
        let mut seen = std::collections::BTreeSet::new();
        let mut found = None;
        let incoming = source
            .source_incoming
            .project_source_v1(&[owner].into_iter().collect())
            .map_err(|issue| format!("{}: {issue:?}", freeze("static-input/source-cohort-veto")))?;
        for call in &incoming {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-input/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &call.source
            else {
                return Err(freeze("static-input/original-source-kind"));
            };
            if original.target() != requested.target()
                || original.target_batch_slot() != contract.batch_slot
                || original.callee_owner() != owner
                || original.call_site() != &call.call
            {
                return Err(freeze("static-input/original-target"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("static-input/cohort-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(proof) = &actuals.phase
            else {
                return Err(freeze("static-input/cohort-unfinished"));
            };
            let StaticInputEvidenceV1::Qualified {
                snapshot,
                uses: retained,
                domains: retained_domains,
            } = &proof.evidence
            else {
                return Err(freeze("static-input/evidence-kind"));
            };
            if !Rc::ptr_eq(&proof.source, original)
                || !Rc::ptr_eq(&proof.completion, completion)
                || retained.as_ref().map(Rc::as_ptr) != uses.as_ref().map(Rc::as_ptr)
                || retained_domains != &domains
                || !matches!(
                    snapshot.phase,
                    BorrowedCallActualEvidencePhaseV1::SourceStatic(_)
                )
                || actuals.ordered_arguments != snapshot.ordered_arguments
                || actuals.opaque_actuals.as_ref()
                    != self
                        .checked_static_expression_actuals_v1(source, call, snapshot)?
                        .as_slice()
            {
                return Err(freeze("static-input/completed-source-drift"));
            }
            if proof.text_result.is_some()
                && !self.checked_completed_static_text_result_v1(original)?
            {
                return Err(freeze("static-input/completed-source-drift"));
            }
            actuals.ordered_arguments_for_v1(call)?;
            if &call.call == requested.call_site() {
                if !Rc::ptr_eq(original, requested) {
                    return Err(freeze("static-input/requested-source-drift"));
                }
                found = Some(actuals);
            }
        }
        found
            .map(Some)
            .ok_or_else(|| freeze("static-input/requested-incoming-missing"))
    }

    pub(super) fn checked_static_expression_actuals_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
        snapshot: &PreparedBorrowedCallActualsV1,
    ) -> Result<Vec<PreparedBorrowedFormalActualV1>, String> {
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &call.source
        else {
            return Err(freeze("static-input/original-source-kind"));
        };
        let identity = super::static_source::validate_static_source_actuals_v1(
            source,
            snapshot,
            &call.call,
            original.required_i64_arguments(),
        )?;
        if !Rc::ptr_eq(&identity.source, original) {
            return Err(freeze("static-input/snapshot-source-drift"));
        }
        super::static_source::validate_qualified_argument_sources_v1(
            source,
            &self.borrowed_formal_actuals,
            &call.call,
            identity,
        )?;
        let mut actuals = Vec::new();
        for (formal, candidate) in original.parameters().iter().zip(identity.candidates.iter()) {
            if !formal.kind.is_ordinary_borrowed_handle() {
                if matches!(
                    snapshot.ordered_arguments.get(formal.ordinal as usize),
                    Some(LocalCallArgumentV1::Expression { .. })
                ) {
                    self.argument_expression_frame_v1(&call.call, formal.ordinal, &candidate.site)?;
                }
                continue;
            }
            let actual = match &candidate.value {
                BorrowedCallActualValueV1::Home { .. } => PreparedBorrowedFormalActualV1 {
                    ordinal: formal.ordinal, site: candidate.site.clone(), formal: formal.binding,
                    source: object_source::ObjectActualHomeSourceV1::Ledger(self)
                        .prepare_home_actual_v1(&call.call, formal, candidate)?
                        .ok_or_else(|| freeze("static-input/argument-frame-missing"))?,
                },
                BorrowedCallActualValueV1::Integer(value) => PreparedBorrowedFormalActualV1 {
                    ordinal: formal.ordinal,
                    site: candidate.site.clone(),
                    formal: formal.binding,
                    source: BorrowedFormalActualSourceV1::Integer(*value),
                },
                BorrowedCallActualValueV1::Bool(value) => PreparedBorrowedFormalActualV1 {
                    ordinal: formal.ordinal,
                    site: candidate.site.clone(),
                    formal: formal.binding,
                    source: BorrowedFormalActualSourceV1::Bool(*value),
                },
                BorrowedCallActualValueV1::Null => PreparedBorrowedFormalActualV1 {
                    ordinal: formal.ordinal,
                    site: candidate.site.clone(),
                    formal: formal.binding,
                    source: BorrowedFormalActualSourceV1::Null,
                },
                BorrowedCallActualValueV1::Scalar(binding, kind) => {
                    PreparedBorrowedFormalActualV1 {
                        ordinal: formal.ordinal,
                        site: candidate.site.clone(),
                        formal: formal.binding,
                        source: BorrowedFormalActualSourceV1::Scalar {
                            binding: *binding,
                            kind: *kind,
                        },
                    }
                }
                BorrowedCallActualValueV1::SelfRooted { .. }
                    if source
                        .static_arguments
                        .contains_key(&(call.call.clone(), formal.ordinal)) =>
                {
                    super::static_selected_actual::original_static_forwarded_source_actual_v1(
                        source,
                        snapshot,
                        original,
                        formal.ordinal,
                    )?
                }
                _ => {
                    let Some(super::ArgumentExpressionV1::Qualified(frame)) =
                        identity.argument_sources.get(&formal.ordinal)
                    else {
                        return Err(freeze("static-input/argument-frame-missing"));
                    };
                    PreparedBorrowedFormalActualV1 {
                        ordinal: formal.ordinal,
                        site: candidate.site.clone(),
                        formal: formal.binding,
                        source: BorrowedFormalActualSourceV1::retain_expression_frame_v1(
                            Rc::clone(frame),
                        )?,
                    }
                }
            };
            actuals.push(actual);
        }
        if actuals.len() != call.arguments.len()
            || actuals
                .iter()
                .zip(call.arguments.iter())
                .any(|(actual, (ordinal, site, formal))| {
                    actual.ordinal != *ordinal || actual.site != *site || actual.formal != *formal
                })
        {
            return Err(freeze("static-input/original-actuals"));
        }
        Ok(actuals)
    }
}
