//! Original nullable LOCAL borrower through the existing affine lexical slot.
//! This source seal creates neither an owning packet nor a result Home.
use super::*;
use std::rc::Rc;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallObservationV1, LocalCallResultClassV1};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn borrowed_nullable_call_source_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<&LocalCallObservationV1> {
        self.local_call_for_owner(site.owner(), site.site()).filter(|row|
            row.result() == LocalCallResultClassV1::BorrowedNullable
                && row.site() == site && row.borrowed_nullable().is_some())
    }

    /// Re-demand the original relation without consulting affine slot state.
    pub(crate) fn corroborate_borrowed_nullable_call_v1(
        &self, row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<InvokeCallResultKind, String> {
        let (kind, completion) = self.checked_borrowed_nullable_call_source_v1(row.source_target())?;
        if row.result() != Some(kind) || row.scalar_result.is_some() || row.unit_completion.is_some()
            || row.object_packet.is_some()
            || row.borrowed_completion.as_ref().is_none_or(|held| !Rc::ptr_eq(held, completion)) {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        }
        Ok(kind)
    }

    fn checked_borrowed_nullable_call_source_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<(InvokeCallResultKind, &Rc<VerifiedFunctionCompletionV1>), String> {
        let loan = source.borrowed_nullable_local_source()
            .ok_or_else(|| freeze("borrowed-nullable-call/source-missing"))?;
        let support = loan.result_support_v1();
        let kind = InvokeCallResultKind::BorrowedNullableHandle { support };
        let roots = self.callable_result_classes.checked_original_source_roots_v1(source.target(), source.callee_owner())
            .map_err(|issue| freeze(issue))?;
        self.callable_result_classes.check_borrowed_nullable_roots_v1(loan, &roots)
            .map_err(|issue| freeze(issue))?;
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-nullable-call/ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-nullable-call/catalog-missing"))?;
        let incoming = Self::original_instance_input_call_v1(ingress, catalog, source.call_site())?
            .ok_or_else(|| freeze("borrowed-nullable-call/incoming-missing"))?;
        let completion = self.completion_source_for_owner_v1(source.callee_owner())
            .ok_or_else(|| freeze("borrowed-nullable-call/completion-missing"))?;
        let observed = self.borrowed_nullable_call_source_v1(source.call_site())
            .ok_or_else(|| freeze("borrowed-nullable-call/observation-missing"))?;
        let (selected, signatures) = self.original_instance_input_issuance.as_ref()
            .ok_or_else(|| freeze("borrowed-nullable-call/input-issuance-missing"))?;

        let arguments = self.checked_completed_original_instance_target_arguments_v1(source, selected, catalog, signatures)?
            .ok_or_else(|| freeze("borrowed-nullable-call/input-incomplete"))?;
        if incoming.callee != source.callee_owner()
            || incoming.source.require_instance()? != source || loan.call() != source.call_site()
            || loan.key() != source.target() || loan.callee_owner() != source.callee_owner()
            || support.formal_ordinal().is_some_and(|ordinal| ordinal >= source.target().arity())
            || observed.borrowed_nullable() != Some(loan) || observed.arguments() != arguments
            || observed.local_binding().map(|(_, binding)| binding) != Some(loan.destination())
            || !completion.returns_value()
            || !self.root_home_flow_ready_v1(completion)?
        { return Err(freeze("borrowed-nullable-call/source-completion-drift")); }
        if loan.is_map_slot() {
            let result = self.callable_result_source_v1(source.callee_owner())?
                .ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
            let borrowed = result.borrowed_nullable()
                .ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
            borrowed.require_local_source_v1(loan, completion, &roots)?;
            if self.checked_completed_object_callee_target_exits_v1(source)?.is_none() {
                return Err(freeze("borrowed-nullable-call/source-completion-drift"));
            }
        }
        Ok((kind, completion))
    }

    /// Structural receiver source was checked at original target issuance.
    /// Complete numeric write proofs now exclude slot replacement and old-End
    /// callbacks throughout this Unit callee; the holder remains caller-owned.
    pub(crate) fn checked_borrowed_map_receiver_source_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let Some(loan) = source.borrowed_call_receiver_source_v1() else { return Ok(false); };
        let Some((class, object)) = loan.map_payload_v1() else {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        };
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-nullable-call/catalog-missing"))?;
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-nullable-call/ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let producer = Self::original_instance_input_call_v1(ingress, catalog, loan.call())?
            .ok_or_else(|| freeze("borrowed-nullable-call/incoming-missing"))?;
        let producer = producer.source.require_instance()?;
        if producer.borrowed_nullable_local_source() != Some(loan)
            || loan.destination() != source.receiver_binding()? || source.call_site().owner() != loan.call().owner()
            || source.target().owner() != class || !source.is_unit_discard_source_v1() {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        }
        self.checked_borrowed_nullable_call_source_v1(producer)?;
        let completion = self.completion_source_for_owner_v1(source.callee_owner())
            .ok_or_else(|| freeze("borrowed-nullable-call/completion-missing"))?;
        let flow = completion.cleanup().root_flow()
            .ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
        self.check_borrowed_receiver_writes_v1(source, object, Some(loan.result_support_v1()), false)?;
        if !flow.maps().is_empty() || !flow.named_arrays().is_empty()
            || completion.returns_value() || !completion.explicit_sites().is_empty()
            || !flow.implicit_exit().is_some_and(|exit| exit.is_ok_and(|exit| exit.homes().is_empty()))
            || !self.root_home_flow_ready_v1(completion)? {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        }
        Ok(true)
    }

    /// Source effects only: neither Map slot liveness nor physical Call is
    /// granted by this check. Every original field writer must match once.
    fn check_borrowed_receiver_writes_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        protected: Option<crate::mir::instruction::BorrowedResultSupportV1>, numeric_only: bool,
    ) -> Result<(), String> {
        let completion = self.completion_source_for_owner_v1(source.callee_owner())
            .ok_or_else(|| freeze("borrowed-nullable-call/completion-missing"))?;
        let flow = completion.cleanup().root_flow()
            .ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
        let originals: Vec<_> = self.field_write_claims.writes().iter()
            .filter(|write| write.owner() == source.callee_owner()).collect();
        let mut writes = std::collections::BTreeSet::new();
        for plan in flow.expression_plans() {
            let Some(frame) = plan.field_write_frame() else { continue; };
            let Some(entry) = frame.entry() else { return Err(freeze("borrowed-nullable-call/source-completion-drift")); };
            let rhs = plan.field_write_rhs().ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
            entry.recheck(self)?;
            if !rhs.is_consistent(frame.source()) || !Rc::ptr_eq(entry.source(), frame.source())
                || entry.field().object() != object || (numeric_only && !entry.is_numeric_v1())
                || match protected {
                    Some(crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { field, .. }) => entry.field() == field,
                    Some(_) => return Err(freeze("borrowed-nullable-call/source-completion-drift")),
                    None => false,
                }
                || !originals.iter().any(|write| std::ptr::eq(*write, entry.original().write()))
                || !writes.insert(entry.original().write().assignment().statement_site().clone()) {
                return Err(freeze("borrowed-nullable-call/source-completion-drift"));
            }
        }
        if writes.len() != originals.len() || !flow.maps().is_empty() || !flow.named_arrays().is_empty() {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        }
        Ok(())
    }

    pub(in crate::mir) fn check_direct_map_numeric_effects_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<(), String> {
        let loan = source.direct_map_receiver_source_v1()
            .ok_or_else(|| freeze("borrowed-nullable-call/source-completion-drift"))?;
        let get = loan.source();
        get.recheck_v1(self)?;
        if source.target().owner() != get.payload_source_v1().payload_class_v1() {
            return Err(freeze("borrowed-nullable-call/source-completion-drift"));
        }
        self.check_borrowed_receiver_writes_v1(source, get.payload_source_v1().payload_object_v1(), None, true)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_borrowed_nullable_slots_v1(
        &mut self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let pending: Vec<_> = self.lexical_instance_calls.borrow().iter().filter_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::SourcePending(source)
                if source.borrowed_nullable_local_source().is_some() && !source.has_borrowed_receiver_source_v1() => Some((site.clone(), source.clone())),
            _ => None,
        }).collect();
        let mut upgrades = Vec::new();
        for (site, source) in &pending {
            let loan = source.borrowed_nullable_local_source().ok_or_else(|| freeze("borrowed-nullable-call/source-missing"))?;
            // A storage result keeps its own support. Its SAME whole-result
            // source/Completion must close before this logical Call is ready;
            // final physical Return/ABI consumers still verify their packets.
            let support = loan.result_support_v1();
            if loan.is_map_slot() && self.checked_completed_object_callee_target_exits_v1(source)?.is_none() {
                continue;
            }
            self.callable_result_classes.recheck_borrowed_nullable_local_source_v1(batch, selected, &self.field_write_claims, contracts, loan, self.app_main_caller_source_ref_v1())
                .map_err(|issue| freeze(issue))?;
            let Some(arguments) = self.checked_completed_original_instance_target_arguments_v1(source, selected, contracts, signatures)?
                else { continue; };
            let observed = self.borrowed_nullable_call_source_v1(site)
                .ok_or_else(|| freeze("borrowed-nullable-call/observation-missing"))?;
            if observed.arguments() != arguments { return Err(freeze("borrowed-nullable-call/arguments-drift")); }
            let Some(completion) = self.completion_source_for_owner_v1(source.callee_owner()) else { continue; };
            let contract = results.row(source.target_batch_slot()).ok_or_else(|| freeze("borrowed-nullable-call/result-contract-missing"))?;
            let signature = signatures.row(source.target_batch_slot()).ok_or_else(|| freeze("borrowed-nullable-call/signature-missing"))?;
            if contract.owner() != source.callee_owner() || contract.result().is_some()
                || !std::ptr::eq(contract.borrow().completion(), completion.as_ref())
                || signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != source.target().arity() || signature.receiver_lane_count() != 1
            { return Err(freeze("borrowed-nullable-call/contract-signature-drift")); }
            let row = LexicalInstanceCallDispositionRowV1 {
                source: source.clone(), result: Some(InvokeCallResultKind::BorrowedNullableHandle { support }),
                scalar_result: None, unit_completion: None, borrowed_completion: Some(Rc::<VerifiedFunctionCompletionV1>::clone(completion)), object_packet: None, mixed_result: None,
            };
            self.corroborate_borrowed_nullable_call_v1(&row)?;
            upgrades.push((site.clone(), row));
        }
        let mut slots = self.lexical_instance_calls.borrow_mut();
        for (site, source) in &pending {
            if !matches!(slots.get(site), Some(LexicalInstanceCallDispositionSlotV1::SourcePending(held)) if held == source) {
                return Err(freeze("borrowed-nullable-call/slot-drift"));
            }
        }
        for (site, row) in upgrades { slots.insert(site, LexicalInstanceCallDispositionSlotV1::Ready(row)); }
        drop(slots);
        self.finish_borrowed_map_receiver_slots_v1(selected, contracts, signatures, results)
    }
    fn finish_borrowed_map_receiver_slots_v1(
        &mut self, selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let pending: Vec<_> = self.lexical_instance_calls.borrow().iter().filter_map(|(site, slot)| match slot {
            LexicalInstanceCallDispositionSlotV1::SourcePending(source)
                if source.borrowed_call_receiver_source_v1().is_some() && source.is_unit_discard_source_v1() => Some((site.clone(), source.clone())),
            _ => None,
        }).collect();
        let mut upgrades = Vec::new();
        for (site, source) in &pending {
            let loan = source.borrowed_call_receiver_source_v1().expect("selected borrowed receiver");
            {
                let slots = self.lexical_instance_calls.borrow();
                let Some(LexicalInstanceCallDispositionSlotV1::Ready(producer)) = slots.get(loan.call()) else { continue; };
                self.corroborate_borrowed_nullable_call_v1(producer)?;
                if producer.source_target().borrowed_nullable_local_source() != Some(loan) {
                    return Err(freeze("borrowed-nullable-call/source-completion-drift"));
                }
            }
            let Some(arguments) = self.checked_completed_original_instance_target_arguments_v1(source, selected, contracts, signatures)? else { continue; };
            let observed = self.unit_discard_source_v1(site)
                .ok_or_else(|| freeze("unit-call/discard-observation-missing"))?;
            if observed.unit_source() != Some(source) || observed.arguments() != arguments {
                return Err(freeze("unit-call/original-discard-drift"));
            }
            let completion = self.original_unit_completion_v1(source)?;
            let contract = results.row(source.target_batch_slot()).ok_or_else(|| freeze("unit-call/result-contract-missing"))?;
            let signature = signatures.row(source.target_batch_slot()).ok_or_else(|| freeze("borrowed-nullable-call/signature-missing"))?;
            if contract.owner() != source.callee_owner() || contract.result().is_some()
                || !std::ptr::eq(contract.borrow().completion(), completion.as_ref())
                || signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != source.target().arity() || signature.receiver_lane_count() != 1 {
                return Err(freeze("unit-call/result-completion-drift"));
            }
            let row = LexicalInstanceCallDispositionRowV1 { source: source.clone(), result: Some(InvokeCallResultKind::Unit),
                scalar_result: None, unit_completion: Some(completion), borrowed_completion: None, object_packet: None, mixed_result: None };
            self.corroborate_unit_discard_v1(&row)?;
            upgrades.push((site.clone(), row));
        }
        let mut slots = self.lexical_instance_calls.borrow_mut();
        for (site, source) in &pending {
            if !matches!(slots.get(site), Some(LexicalInstanceCallDispositionSlotV1::SourcePending(held)) if held == source) {
                return Err(freeze("borrowed-nullable-call/slot-drift"));
            }
        }
        for (site, row) in upgrades { slots.insert(site, LexicalInstanceCallDispositionSlotV1::Ready(row)); }
        Ok(())
    }

}
