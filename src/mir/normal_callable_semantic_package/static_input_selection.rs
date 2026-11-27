//! Original input selection and closed domain queries.
use super::static_input_finish::{StaticInputEvidenceV1, StaticInputFinishV1};
use super::*;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
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

    fn observes_static_result_v1(
        &self,
        original: &Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
        result: crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1;

        let site = original.call_site();
        self.local_call_for_owner(site.owner(), site.site())
            .is_some_and(|call| {
                call.arguments().len() == original.argument_sites().len()
                    && call.result() == result
                    && matches!(call.result_source(), Some(LocalCallResultSourceV1::Ordinary(loan))
                        if loan.static_source().is_some_and(|source| Rc::ptr_eq(source, original)))
            })
    }

    fn observes_borrowed_static_return_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        original: &Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1;
        let site = original.call_site();
        source.can_extend_static_return_v1(original)
            && self.observes_static_result_v1(original, LocalCallResultClassV1::I64)
            && self.local_call_for_owner(site.owner(), site.site())
                .and_then(|call| self.borrow_return_expression_evaluation_v1(site.owner(), call.statement()).ok().flatten())
                .is_some_and(|evaluation| original.is_scalar_return_source_v1(evaluation.source()))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
    fn has_borrowed_static_return_inputs_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        source.source_incoming.source_rows_v1().any(|row| {
            row.callee == owner && matches!(&row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if self.observes_borrowed_static_return_v1(source, original))
        })
    }

    /// The one rule for owing a Static actual row, asked after the caller
    /// census is validated. Every caller keeps its source identity and vetoes;
    /// only a caller that the co-seal recorded as omitted from this closed-app
    /// artifact, under the same caller owner and declaration key, may lack its
    /// actual row. Root, Birth, selected and unclassified callers owe one.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn static_input_caller_owes_actuals_v1(
        &self,
        call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> bool {
        use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;

        let BorrowedIncomingSourceV1::Static(original) = &call.source else {
            return true;
        };
        !self.omitted_static_callers.as_deref().is_some_and(|omitted| {
            omitted.get(&call.call.owner()) == Some(original.caller())
        })
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn allows_general_tagged_static_input_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        // A downstream forwarding read cannot establish this entry's input
        // protocol. Every caller that owes an actual row must have its source row.
        let mut callers = source.source_incoming.source_rows_v1().filter(|row| {
            row.callee == owner && self.static_input_caller_owes_actuals_v1(row)
        }).peekable();
        !self.static_expression_input_owner_v1(source, owner, false)
            && callers.peek().is_some()
            && callers.all(|row| {
                self.borrowed_formal_actuals
                    .get(&row.call)
                    .and_then(|row| row.as_ref().ok())
                    .and_then(PreparedBorrowedCallActualsV1::source_static_snapshot_v1)
                    .is_some()
            })
    }

    pub(super) fn uses_tagged_static_inputs_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        source.has_tagged_static_input_v1(contract.owner, &contract.parameters)
            && self.allows_general_tagged_static_input_v1(source, contract.owner)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn is_static_expression_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.static_expression_input_owner_v1(source, owner, true)
    }

    fn static_expression_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        allow_tagged: bool,
    ) -> bool {
        let signed_expression = source.has_signed_static_expression_input_v1(owner);
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
                    let exact_input_text = original.is_i64_input_text_v1()
                        || original.has_exact_i64_inputs_v1()
                            && self.observes_static_result_v1(original,
                                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Text);
                    // A call with no input and an I64 result also closes here once its
                    // original row exists; no separate zero-input protocol remains.
                    (allow_tagged && source.has_tagged_static_input_v1(owner, original.parameters()))
                        || finite_forward || text_source || signed_expression || exact_input_text
                        || self.observes_borrowed_static_return_v1(source, original)
                        || original.is_zeroarg_i64_v1()
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
        if self.uses_tagged_static_inputs_v1(source, contract) {
            return Some(Box::new([]));
        }
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
