//! One mixed result upgrades the existing lexical cell after full source sealing.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallObservationV1, LocalCallResultClassV1};
use std::rc::Rc;

impl LexicalInstanceCallDispositionRowV1 {
    /// Borrow the original co-seal; only its ledger may corroborate this loan.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn mixed_result_source_v1(&self)
        -> Option<&super::super::completion_index::VerifiedMixedCallResultV1> {
        self.mixed_result.as_deref()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn mixed_call_teardown_v1<'a>(
        &self, row: &'a LexicalInstanceCallDispositionRowV1,
    ) -> Result<&'a super::super::completion_index::ObjectReturnTeardownDescriptorV1, String> {
        self.mixed_call_teardown_using_v1(row, CallValidationContextV1::Lowering(self))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn mixed_call_teardown_using_v1<'a>(
        &self, row: &'a LexicalInstanceCallDispositionRowV1, context: CallValidationContextV1<'_>,
    ) -> Result<&'a super::super::completion_index::ObjectReturnTeardownDescriptorV1, String> {
        self.corroborate_mixed_call_using_v1(row, context)?;
        Ok(row.mixed_result.as_ref().ok_or_else(|| freeze("mixed-call/result-unsealed"))?.teardown())
    }

    pub(crate) fn mixed_call_source_v1(&self, site: &OwnedExprSiteV1) -> Option<&LocalCallObservationV1> {
        self.local_call_for_owner(site.owner(), site.site()).filter(|row|
            row.result() == LocalCallResultClassV1::MixedObject && row.site() == site && row.mixed().is_some())
    }

    pub(crate) fn corroborate_mixed_call_v1(&self, row: &LexicalInstanceCallDispositionRowV1) -> Result<InvokeCallResultKind, String> {
        self.corroborate_mixed_call_using_v1(row, CallValidationContextV1::Lowering(self))
    }

    pub(in crate::mir) fn corroborate_mixed_call_using_v1(&self,
        row: &LexicalInstanceCallDispositionRowV1, context: CallValidationContextV1<'_>,
    ) -> Result<InvokeCallResultKind, String> {
        context.require_ledger(self)?;
        let source = row.source_target();
        let loan = source.mixed_local_source().ok_or_else(|| freeze("mixed-call/source-missing"))?;
        self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
        let proof = row.mixed_result.as_ref().ok_or_else(|| freeze("mixed-call/result-unsealed"))?;
        if proof.source() != source || proof.teardown().object() != loan.object() {
            return Err(freeze("mixed-call/result-source-drift"));
        }
        self.recheck_mixed_call_result_using_v1(proof, context)?;
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("mixed-call/ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let mut originals = ingress.source_incoming.exact_rows().filter(|incoming| &incoming.call == source.call_site());
        let incoming = originals.next().ok_or_else(|| freeze("mixed-call/incoming-missing"))?;
        let observed = self.mixed_call_source_v1(source.call_site()).ok_or_else(|| freeze("mixed-call/observation-missing"))?;
        let (selected, signatures) = self.original_instance_input_issuance.as_ref().ok_or_else(|| freeze("mixed-call/input-issuance-missing"))?;
        let contracts = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("mixed-call/catalog-missing"))?;
        let arguments = self.checked_completed_original_instance_target_arguments_v1(source, selected, contracts, signatures)?
            .ok_or_else(|| freeze("mixed-call/input-incomplete"))?;
        let kind = InvokeCallResultKind::MixedHandle { formal_ordinal: loan.callee_ordinal() };
        if originals.next().is_some() || incoming.source.require_instance()? != source
            || incoming.callee != loan.callee_owner() || loan.callee_ordinal() >= source.target().arity()
            || observed.mixed() != Some(loan) || observed.arguments() != arguments
            || observed.local_binding() != Some((loan.declaration(), loan.destination()))
            || row.result() != Some(kind) || row.object_packet.is_some() || row.scalar_result.is_some()
            || row.unit_completion.is_some() || row.borrowed_completion.is_some()
        { return Err(freeze("mixed-call/source-input-drift")); }
        Ok(kind)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_mixed_call_slots_v1(
        &mut self, selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let pending: Vec<_> = self.lexical_instance_calls.borrow().iter().filter_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::SourcePending(source) if source.mixed_local_source().is_some() => Some((site.clone(), source.clone())),
            _ => None,
        }).collect();
        let mut upgrades = Vec::new();
        for (site, source) in &pending {
            let loan = source.mixed_local_source().ok_or_else(|| freeze("mixed-call/source-missing"))?;
            self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
            let Some(arguments) = self.checked_completed_original_instance_target_arguments_v1(source, selected, contracts, signatures)?
            else { continue; };
            let observed = self.mixed_call_source_v1(site).ok_or_else(|| freeze("mixed-call/observation-missing"))?;
            if observed.mixed() != Some(loan) || observed.arguments() != arguments { return Err(freeze("mixed-call/arguments-drift")); }
            let Some(proof) = self.checked_mixed_call_result_v1(source, results)? else { continue; };
            let signature = signatures.row(source.target_batch_slot()).ok_or_else(|| freeze("mixed-call/signature-missing"))?;
            if signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != source.target().arity() || signature.receiver_lane_count() != 1
            { return Err(freeze("mixed-call/signature-drift")); }
            let row = LexicalInstanceCallDispositionRowV1 {
                source: source.clone(), result: Some(InvokeCallResultKind::MixedHandle { formal_ordinal: loan.callee_ordinal() }),
                mixed_result: Some(Rc::new(proof)), scalar_result: None, unit_completion: None, borrowed_completion: None, object_packet: None,
            };
            self.corroborate_mixed_call_v1(&row)?;
            upgrades.push((site.clone(), row));
        }
        let mut slots = self.lexical_instance_calls.borrow_mut();
        for (site, source) in &pending {
            if !matches!(slots.get(site), Some(LexicalInstanceCallDispositionSlotV1::SourcePending(current)) if current == source) {
                return Err(freeze("mixed-call/slot-drift"));
            }
        }
        for (site, row) in upgrades { slots.insert(site, LexicalInstanceCallDispositionSlotV1::Ready(row)); }
        Ok(())
    }
}


impl PreparedBorrowedFormalIngressV1 {
    /// The original Mixed request lends input observation, never entry or Home.
    pub(in crate::mir::normal_callable_semantic_package) fn original_mixed_input_source_v1(
        &self, target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&super::super::result_class_claim::MixedLocalCallSourceLoanV1>, String> {
        let mut originals = self.source_incoming.exact_rows()
            .filter(|row| &row.call == target.call_site());
        let Some(original) = originals.next() else { return Ok(None); };
        if originals.next().is_some() { return Err(freeze("mixed-input/original-duplicate")); }
        let retained = original.source.require_instance()?;
        if retained != target || original.callee != target.callee_owner() {
            return Err(freeze("mixed-input/original-target"));
        }
        let Some(loan) = retained.mixed_local_source() else { return Ok(None); };
        if loan.call() != target.call_site() || loan.key() != target.target()
            || loan.callee_owner() != original.callee || loan.receiver() != target.receiver_site()
            || loan.destination().owner() != loan.call().owner()
            || loan.support().owner() != loan.call().owner()
            || loan.callee_ordinal() as usize >= target.argument_sites().len()
            || original.arguments.iter().filter(|(ordinal, site, _)|
                *ordinal == loan.callee_ordinal()
                    && target.argument_sites().get(*ordinal as usize) == Some(site)).count() != 1
        { return Err(freeze("mixed-input/original-loan")); }
        Ok(Some(loan))
    }

    /// Borrow the original source loan, independent of Completion/Take phase.
    pub(in crate::mir::normal_callable_semantic_package) fn mixed_received_actual_source_v1(
        &self, binding: BindingRefV1,
    ) -> Result<Option<&super::super::result_class_claim::MixedLocalCallSourceLoanV1>, String> {
        let mut rows = self.source_incoming.exact_rows().filter_map(|row|
            row.source.instance().and_then(|target| target.mixed_local_source())
                .filter(|loan| loan.destination() == binding));
        let Some(loan) = rows.next() else { return Ok(None); };
        if rows.next().is_some() || loan.call().owner() != binding.owner() {
            return Err(freeze("mixed-actual/source-acquisition"));
        }
        Ok(Some(loan))
    }
}
