//! Whole Static input closure uses the original inventory, signature and Completion.
//! Finished evidence stays in the existing actual row; no borrowed entry is issued.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewClaimLedgerV1,
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    evidence: StaticInputEvidenceV1,
    text_result: Option<
        crate::mir::normal_callable_semantic_package::result_contract::CompletedTextResultSourceV1,
    >,
}
#[derive(Debug, Clone)]
enum StaticInputEvidenceV1 {
    Zero,
    Qualified {
        snapshot: Box<PreparedBorrowedCallActualsV1>,
        uses: Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>,
        domains: Box<[super::super::borrowed_formal_entry::BorrowedFormalDomainV1]>,
    },
}
impl PartialEq for StaticInputEvidenceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Zero, Self::Zero) => true,
            (
                Self::Qualified {
                    snapshot: a,
                    uses: au,
                    domains: ad,
                },
                Self::Qualified {
                    snapshot: b,
                    uses: bu,
                    domains: bd,
                },
            ) => a == b && Rc::ptr_eq(au, bu) && ad == bd,
            _ => false,
        }
    }
}
impl Eq for StaticInputEvidenceV1 {}
impl PartialEq for StaticInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && self.evidence == other.evidence
            && self.text_result == other.text_result
    }
}
impl Eq for StaticInputFinishV1 {}
impl StaticInputFinishV1 {
    pub(super) fn corroborates_source_v1(&self, source: &Rc<StaticIncomingSourceV1>) -> bool {
        Rc::ptr_eq(&self.source, source)
            && matches!(self.evidence, StaticInputEvidenceV1::Zero)
            && source.is_zero_input_source_v1()
            && self.completion.owner() == source.callee_owner()
    }
    pub(super) fn source_snapshot_v1(&self) -> Option<&PreparedBorrowedCallActualsV1> {
        match &self.evidence {
            StaticInputEvidenceV1::Qualified { snapshot, .. } => Some(snapshot),
            _ => None,
        }
    }
}

/// Text borrows the full original census; the existing I64 lane still requires
/// its original executable observation. Neither branch reconstructs a source.
pub(super) fn original_zero_input_source_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
) -> Result<Option<&'a Rc<StaticIncomingSourceV1>>, String> {
    if original.is_zeroarg_text_v1() {
        super::static_source::original_static_source_v1(prepared, original.call_site())
    } else {
        prepared
            .source_incoming
            .static_observations()
            .get(original.call_site())
            .map(|row| row.as_ref().map_err(Clone::clone))
            .transpose()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Complete passive source rows for selected Static input cohorts before
    /// any callee entry asks to borrow the whole cohort. The normal caller
    /// walk may not have visited a Loop body's expression yet; this source-only
    /// pass retains the exact argument plans from the same resolved batch and
    /// grants no execution or physical-call permission.
    pub(in crate::mir::normal_callable_semantic_package) fn stage_missing_static_input_sources_v1(
        &mut self,
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), String> {
        let Some(prepared) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let prepared = prepared.as_ref().map_err(Clone::clone)?;
        let sources: Vec<_> = prepared.original_static_input_sources_v1()
            .filter(|original| {
                prepared.original_finite_forward_input_request_v1(original.callee_owner())
                    || original.parameters().iter().any(|formal|
                        matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                            && prepared.source_value_sum_v1(formal.binding).is_some_and(|sum|
                                sum.alternatives().contains(&super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)))
            })
            .cloned()
            .collect();

        for original in sources {
            let site = original.call_site();
            if self.borrowed_formal_actuals.contains_key(site) {
                continue;
            }
            let Some(caller) = batch.declarations().find(|row| row.owner() == site.owner()) else {
                return Err(freeze("static-input/caller-source-missing"));
            };
            let source_result = self
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("static-input/source-missing"))?;
            let staged = batch.with_lowering_input(caller.batch_slot(), |input| {
                if input.owner() != site.owner() { return Err(freeze("static-input/caller-owner")); }
                let actuals: Vec<_> = original.argument_sites().iter().enumerate().map(|(ordinal, argument)| {
                    let value = crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, argument)
                        .unwrap_or(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::Unknown);
                    crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualCandidateV1 {
                        ordinal: ordinal as u32,
                        site: argument.clone(),
                        value,
                    }
                }).collect();
                // Only source atoms whose identity is stable without caller
                // flow may be staged here. Bindings/fields remain owned by
                // the ordinary prefix walk and cannot be guessed early.
                if actuals.iter().any(|actual| matches!(actual.value,
                    crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::Binding(_)
                    | crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::FieldAccess
                    | crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::ReceivedField { .. }
                    | crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::Unknown)) {
                    return Ok(None);
                }
                let Some(mut rows) = super::prepare_borrowed_call_actuals_v1(
                    source_result, contracts, site, &actuals, &[], None, &mut |_| None,
                )? else { return Ok(None) };
                if !matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(_)) {
                    return Ok(None);
                }
                super::bind_argument_sources_v1(input, prepared, Some(contracts), site, None, Some(&mut rows))?;
                Ok(Some(rows))
            }).map_err(|_| freeze("static-input/caller-source-loan"))??;
            if let Some(rows) = staged {
                super::stage_observed_call_actuals_v1(
                    &mut self.borrowed_formal_actuals,
                    site,
                    Ok(Some(rows)),
                );
            }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        // Preserve the original I64 executable census; Text uses the complete
        // original source census, including a still-passive sibling.
        let zero_i64 = source
            .source_incoming
            .exact_rows()
            .filter_map(|call| match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.current_owner_source().is_some()
                        && original.is_zeroarg_i64_v1() =>
                {
                    Some(call.callee)
                }
                _ => None,
            });
        let zero = zero_i64.chain(source.source_incoming.source_rows_v1().filter_map(|call| {
            match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.is_zeroarg_text_v1()
                        && selected
                            .key_for_batch_slot(original.target_batch_slot())
                            .is_some() =>
                {
                    Some(call.callee)
                }
                _ => None,
            }
        }));
        let qualified = source
            .source_incoming
            .source_rows_v1()
            .filter(|call| self.is_static_expression_input_owner_v1(source, call.callee))
            .map(|call| call.callee);
        let owners: std::collections::BTreeSet<_> = zero.chain(qualified).collect();
        for owner in owners {
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-zero/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("static-zero/contract-duplicate"));
            }
            let prepared = if self.is_static_expression_input_owner_v1(source, owner) {
                // Preserve original route membership even while Completion is
                // pending. Demand must not retry a generic Static writer.
                let incoming = source
                    .source_incoming
                    .project_source_v1(&[owner].into_iter().collect())
                    .map_err(|issue| {
                        format!("{}: {issue:?}", freeze("static-input/source-cohort-veto"))
                    })?;
                let sites: Vec<_> = incoming
                    .iter()
                    .filter(|call| {
                        self.local_call_for_owner(call.call.owner(), call.call.site())
                            .is_some()
                    })
                    .map(|call| call.call.clone())
                    .collect();
                if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
                    selected.extend(sites);
                }
                self.prepare_static_expression_input_owner_v1(
                    source, selected, contract, signatures, results,
                )?
            } else {
                self.prepare_static_zero_input_owner_v1(
                    source, selected, contract, signatures, results,
                )?
            };
            match prepared {
                TypedInputClosureV1::Pending => {}
                TypedInputClosureV1::Refused(issue) => {
                    for call in source
                        .source_incoming
                        .source_rows_v1()
                        .filter(|row| row.callee == owner)
                    {
                        if let Some(row) = self.borrowed_formal_actuals.get_mut(&call.call) {
                            if row.is_ok() {
                                *row = Err(issue.clone());
                            }
                        }
                    }
                }
                TypedInputClosureV1::Ready(rows) => {
                    for (site, actuals) in rows {
                        self.borrowed_formal_actuals.insert(site, Ok(actuals));
                    }
                }
            }
        }
        Ok(())
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

    pub(in crate::mir::normal_callable_semantic_package) fn is_static_expression_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        source.source_incoming.source_rows_v1().filter(|row| row.callee == owner).any(|row| {
            let source_input = match &row.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) => {
                    let finite_forward =
                        source.original_finite_forward_input_request_v1(owner);
                    let text_source = original.parameters().iter().any(|formal| {
                        matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                            && source
                                .source_value_sum_v1(formal.binding)
                                .is_some_and(|sum| {
                                    sum.alternatives().contains(
                                        &super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                                    )
                                })
                    });
                    let scalar_expression_source = self
                        .borrowed_formal_actuals
                        .get(&row.call)
                        .and_then(|rows| rows.as_ref().ok())
                        .and_then(PreparedBorrowedCallActualsV1::source_static_snapshot_v1)
                        .is_some_and(|snapshot| {
                            let opaque = original.parameters().iter().filter(|formal| {
                                matches!(
                                    formal.kind,
                                    CallableParameterContractKindV1::OpaqueHandle
                                )
                            });
                            let all_signed = opaque.clone().all(|formal| {
                                source
                                    .source_value_sum_v1(formal.binding)
                                    .is_some_and(|sum| sum.signed_only())
                            });
                            let has_expression = opaque.into_iter().any(|formal| {
                                matches!(
                                    snapshot
                                        .ordered_arguments
                                        .get(formal.ordinal as usize),
                                    Some(LocalCallArgumentV1::Expression { ordinal, site })
                                        if *ordinal == formal.ordinal
                                            && original
                                                .argument_sites()
                                                .get(formal.ordinal as usize)
                                                == Some(site)
                                )
                            });
                            all_signed && has_expression
                        });
                    finite_forward || text_source || scalar_expression_source
                }
                _ => false,
            };
            let snapshot = self.borrowed_formal_actuals.get(&row.call).is_some_and(|rows|
                rows.as_ref().is_ok_and(|rows| rows.source_static_snapshot_v1().is_some()));
            if crate::config::env::builder_debug_enabled() {
                let local_call = self.local_call_for_owner(row.call.owner(), row.call.site()).is_some();
                eprintln!("[static-input/owner] owner={owner:?} call={:?} source-input={source_input} snapshot={snapshot} local-call={local_call}", row.call);
            }
            source_input && snapshot
        })
    }

    /// One signature/result/Completion correspondence for both zero and
    /// qualified source inputs. Specialized scalar/mixed owners remain separate.
    fn verify_static_input_header_v1(
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
                        || matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))) {
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
            || !result.declared_result_agrees_with_i64_source()
            || !completion.returns_value()
            || completion.owner() != contract.owner
            || !std::ptr::eq(result.completion(), completion.as_ref())
        {
            return Err(fail("result-completion-identity"));
        }
        Ok(true)
    }

    fn static_input_domains_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Option<Box<[super::super::borrowed_formal_entry::BorrowedFormalDomainV1]>> {
        contract
            .parameters
            .iter()
            .filter(|formal| formal.kind.is_ordinary_borrowed_handle())
            .map(|formal| {
                source
                    .lend_source_value_sum_v1(formal.binding)
                    .filter(|sum| !sum.alternatives().is_empty())
                    .map(|sum| {
                        super::super::borrowed_formal_entry::BorrowedFormalDomainV1::from_source_v1(
                            formal.ordinal,
                            formal.binding,
                            sum,
                        )
                    })
            })
            .collect::<Option<Vec<_>>>()
            .map(Vec::into_boxed_slice)
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

    /// A final entry borrows completed domains; passive staging has none. Any
    /// completed sibling demands the entire original cohort before this loan.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_static_input_domains_for_owner_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Result<Box<[super::super::borrowed_formal_entry::BorrowedFormalDomainV1]>, String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(Box::new([]));
        };
        for call in source
            .source_incoming
            .source_rows_v1()
            .filter(|call| call.callee == owner)
        {
            if !self
                .borrowed_formal_actuals
                .get(&call.call)
                .is_some_and(|row| {
                    row.as_ref().is_ok_and(|row| {
                        matches!(
                            &row.phase,
                            BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(
                                StaticInputFinishV1 {
                                    evidence: StaticInputEvidenceV1::Qualified { .. },
                                    ..
                                }
                            )
                        )
                    })
                })
            {
                continue;
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &call.source
            else {
                return Err(freeze("static-input/domain-source"));
            };
            let completed = self
                .checked_completed_static_expression_input_v1(original)?
                .ok_or_else(|| freeze("static-input/domain-unfinished"))?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(proof) = &completed.phase
            else {
                unreachable!()
            };
            let StaticInputEvidenceV1::Qualified { domains, .. } = &proof.evidence else {
                unreachable!()
            };
            return Ok(domains.clone());
        }
        Ok(Box::new([]))
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
            .checked_input_use_source_v1(contract)?
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
                || !Rc::ptr_eq(retained, &uses)
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

    fn checked_static_expression_actuals_v1(
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

    fn prepare_static_expression_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let debug_pending = |reason: &str| {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[static-input/pending] owner={owner:?} reason={reason}");
            }
        };
        if !source
            .source_incoming
            .source_registered_owners_v1()
            .contains(&owner)
            || source
                .source_incoming
                .sum_vetoed_owners_v1()
                .contains(&owner)
        {
            debug_pending("source-owner");
            return Ok(TypedInputClosureV1::Pending);
        }
        let Some(completion) = self.completion_index.get(&owner) else {
            debug_pending("completion-missing");
            return Ok(TypedInputClosureV1::Pending);
        };
        let completion = match completion {
            Ok(completion) => completion,
            Err(error) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {error:?}",
                    freeze("static-input/callee-completion")
                )))
            }
        };
        if !self.verify_static_input_header_v1(
            selected,
            contract,
            signatures,
            results,
            completion,
            "static-input",
        )? {
            debug_pending("header");
            return Ok(TypedInputClosureV1::Pending);
        }
        let Some(uses) = self.checked_input_use_source_v1(contract)? else {
            debug_pending("uses");
            return Ok(TypedInputClosureV1::Pending);
        };
        let Some(domains) = self.static_input_domains_v1(source, contract) else {
            debug_pending("domains");
            return Ok(TypedInputClosureV1::Pending);
        };
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("static-input/selected-key"));
        };
        let mut seen = std::collections::BTreeSet::new();
        let mut missing = false;
        let mut replacements = Vec::new();
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
            if original.call_site() != &call.call
                || original.callee_owner() != owner
                || original.target_batch_slot() != contract.batch_slot
                || original.target() != key
                || original.parameters().len() != contract.parameters.len()
                || original
                    .parameters()
                    .iter()
                    .zip(&contract.parameters)
                    .any(|(a, b)| {
                        a.ordinal != b.ordinal || a.binding != b.binding || a.kind != b.kind
                    })
                || original.argument_sites().len() != contract.parameters.len()
            {
                return Err(freeze("static-input/original-target"));
            }
            let text_result = results
                .row(contract.batch_slot)
                .and_then(|row| row.completed_text_source_v1(selected, original));
            if matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
                && text_result.is_none() { debug_pending("text-result"); return Ok(TypedInputClosureV1::Pending); }
            let Some(rows) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let rows = match rows {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            let Some(snapshot) = rows.source_static_snapshot_v1() else {
                missing = true;
                continue;
            };
            let actuals = self.checked_static_expression_actuals_v1(source, call, snapshot)?;
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceStatic(_) => {
                    replacements.push((
                        call.call.clone(),
                        PreparedBorrowedCallActualsV1 {
                            phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(
                                StaticInputFinishV1 {
                                    source: Rc::clone(original),
                                    completion: Rc::clone(completion),
                                    text_result,
                                    evidence: StaticInputEvidenceV1::Qualified {
                                        snapshot: Box::new(snapshot.clone()),
                                        uses: Rc::clone(&uses),
                                        domains: domains.clone(),
                                    },
                                },
                            ),
                            opaque_actuals: actuals.into_boxed_slice(),
                            ordered_arguments: snapshot.ordered_arguments.clone(),
                            dynamic_invocation_arguments: None,
                        },
                    ));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(proof) => {
                    let StaticInputEvidenceV1::Qualified {
                        uses: retained,
                        domains: retained_domains,
                        ..
                    } = &proof.evidence
                    else {
                        return Err(freeze("static-input/evidence-kind"));
                    };
                    if !Rc::ptr_eq(&proof.source, original)
                        || !Rc::ptr_eq(&proof.completion, completion)
                        || !Rc::ptr_eq(retained, &uses)
                        || retained_domains != &domains
                        || rows.opaque_actuals.as_ref() != actuals.as_slice()
                        || rows.ordered_arguments != snapshot.ordered_arguments
                    {
                        return Err(freeze("static-input/completed-source-drift"));
                    }
                    rows.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("static-input/actual-phase")),
            }
        }
        if missing || seen.is_empty() {
            debug_pending(if missing {
                "incoming-actuals"
            } else {
                "empty-incoming"
            });
        }
        Ok(if missing || seen.is_empty() {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
    }

    fn prepare_static_zero_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("static-zero/selected-key"));
        };
        if contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
            || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            || key.arity() != 0
            || !contract.parameters.is_empty()
        {
            return Err(freeze("static-zero/contract-identity"));
        }
        let owners = [owner].into_iter().collect();
        let text = source.source_incoming.source_rows_v1().any(|row| row.callee == owner
            && matches!(&row.source, super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                if original.is_zeroarg_text_v1()));
        let calls = match if text {
            source.source_incoming.project_source_v1(&owners)
        } else {
            source.source_incoming.project(&owners)
        } {
            Ok(rows) => rows,
            Err(issue) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {issue:?}",
                    freeze("borrowed-formal/incoming-coverage")
                )))
            }
        };
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(TypedInputClosureV1::Pending);
        };
        let completion = match completion {
            Ok(row) => row,
            Err(issue) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {issue:?}",
                    freeze("static-zero/callee-completion")
                )))
            }
        };
        if !self.verify_static_input_header_v1(
            selected,
            contract,
            signatures,
            results,
            completion,
            "static-zero",
        )? {
            return Ok(TypedInputClosureV1::Pending);
        }
        let mut missing = false;
        let mut replacements = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for call in &calls {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-zero/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &call.source
            else {
                return Err(freeze("static-zero/original-source-kind"));
            };
            let retained = original_zero_input_source_v1(source, original)?
                .ok_or_else(|| freeze("static-zero/original-source-missing"))?;
            if call.callee != owner
                || !Rc::ptr_eq(original, retained)
                || !original.is_zero_input_source_v1()
                || original.callee_owner() != owner
                || original.target() != key
                || original.target_batch_slot() != contract.batch_slot
                || original.call_site() != &call.call
                || !call.arguments.is_empty()
            {
                return Err(freeze("static-zero/original-target"));
            }
            let text_result = results
                .row(contract.batch_slot)
                .and_then(|row| row.completed_text_source_v1(selected, original));
            if matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
                && text_result.is_none() { return Ok(TypedInputClosureV1::Pending); }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let actuals = match actuals {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            if !actuals.opaque_actuals.is_empty() || !actuals.ordered_arguments.is_empty() {
                return Err(freeze("static-zero/original-actuals"));
            }
            match &actuals.phase {
                BorrowedCallActualEvidencePhaseV1::SourceStatic(staged) => {
                    if !Rc::ptr_eq(&staged.source, original)
                        || !staged.candidates.is_empty()
                        || !staged.integer_evidence.is_empty()
                    {
                        return Err(freeze("static-zero/original-actuals"));
                    }
                    let finish = StaticInputFinishV1 {
                        source: Rc::clone(original),
                        completion: Rc::clone(completion),
                        evidence: StaticInputEvidenceV1::Zero,
                        text_result,
                    };
                    let regenerated = construct_borrowed_call_actuals_v1(
                        source,
                        call,
                        std::slice::from_ref(contract),
                        &call.call,
                        &staged.candidates,
                        object_source::ObjectActualHomeSourceV1::Candidates(&[]),
                        None,
                        &mut |_| None,
                        Some(finish),
                        None,
                        &mut |_, _, _| Ok(None),
                    )?
                    .ok_or_else(|| freeze("static-zero/constructor-unavailable"))?;
                    if regenerated.ordered_arguments != actuals.ordered_arguments {
                        return Err(freeze("static-zero/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) => {
                    if !finish.corroborates_source_v1(original)
                        || !Rc::ptr_eq(&finish.completion, completion)
                    {
                        return Err(freeze("static-zero/completion-drift"));
                    }
                    actuals.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("static-zero/actual-phase")),
            }
        }
        Ok(if missing {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_input_finish_tests.rs"]
mod tests;
