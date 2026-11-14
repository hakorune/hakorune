//! Original input selection, passive source staging and closed domain queries.
use super::static_input_finish::{StaticInputEvidenceV1, StaticInputFinishV1};
use super::*;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;

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
                original.parameters().iter().any(|formal| formal.takes_declared_object_v1())
                    || prepared.original_finite_forward_input_request_v1(original.callee_owner())
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
                let actuals = prepared.refine_fresh_take_actuals_v1(input, site, contracts, &actuals)?;
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

    /// The original formal-use proof of a static input cohort: `Some(None)` only when
    /// every formal is an i64 actual (K21); any other formal needs the same proof as before;
    /// `None` while the proof is pending.
    pub(super) fn checked_static_input_uses_v1(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Result<Option<Option<Rc<super::borrowed_formal_uses::OriginalFormalUseSourceV1>>>, String>
    {
        if contract
            .parameters
            .iter()
            .all(|formal| formal.kind.is_i64_actual_v1())
        {
            return Ok(Some(None));
        }
        Ok(self.checked_input_use_source_v1(contract)?.map(Some))
    }

    /// Borrow an existing Return frame; no statement or entry is synthesized.
    pub(in crate::mir::normal_callable_semantic_package) fn has_static_zero_return_source_v1(
        &self,
        original: &Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
    ) -> bool {
        original.is_zeroarg_i64_v1() && self.terminal_relations_for_owner(original.call_site().owner())
            .iter().any(|relation| matches!(relation,
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Scalar(terminal)
                    if terminal.evaluation().is_some_and(|frame|
                        original.is_scalar_return_source_v1(frame.source()))))
    }

    pub(super) fn static_zero_input_uses_source_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        source
            .source_incoming
            .source_rows_v1()
            .filter(|row| row.callee == owner)
            .any(|row| {
                matches!(&row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if self.has_static_zero_return_source_v1(original))
            })
    }

    /// The finisher and its lender recheck the same original caller cohort.
    /// Only a retained qualified Return demands source-only zero callers.
    pub(super) fn static_zero_input_calls_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>, String>
    {
        let owners = [owner].into_iter().collect();
        let source_return = self.static_zero_input_uses_source_v1(source, owner);
        let calls = if source_return {
            source.source_incoming.project_source_v1(&owners)
        } else {
            source.source_incoming.project(&owners)
        };
        let calls = calls.map_err(|issue| {
            format!("{}: {issue:?}", freeze("borrowed-formal/incoming-coverage"))
        })?;
        if !source_return {
            return Ok(calls);
        }
        let (selected, _) = self
            .original_instance_input_issuance
            .as_ref()
            .ok_or_else(|| freeze("static-zero/selected-key"))?;
        // Keep every source target and validate its census above, but only
        // selected callers carry executable obligations. An omitted caller's
        // source-only row cannot lend (or demand) a finished execution input.
        Ok(calls
            .into_vec()
            .into_iter()
            .filter(|call| match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) => {
                    selected
                        .batch_slot(
                            &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                                original.caller().clone(),
                            ),
                        )
                        .is_some()
                }
                _ => true,
            })
            .collect())
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
                    finite_forward || text_source || scalar_expression_source || original.is_i64_input_text_v1()
                        || original.parameters().iter().any(|formal| formal.takes_declared_object_v1())
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

    pub(super) fn static_input_domains_v1(
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
                    .filter(|sum| !sum.alternatives().is_empty() && (!formal.takes_declared_object_v1()
                        || matches!(&formal.kind, CallableParameterContractKindV1::DeclaredObject(expected)
                            if sum.nullable_object_class_v1() == Some((expected.as_ref(), false)))))
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
}
