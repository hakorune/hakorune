//! Mixed Call result seal in the existing completed-index owner.
//! Every original Return retains its own witness, acquisition or live support.
use super::super::{lexical_instance_call::LexicalInstanceCallSourceTargetV1,
    result_class_claim::{MixedLocalCallSourceLoanV1, ResultOriginWitnessV1, ResultWitnessStepV1},
    OrdinaryNewClaimLedgerV1};
use super::{normal_return::NormalReturnDispositionV1,
    return_leaf::{ObjectReturnTeardownDescriptorV1, VerifiedObjectReturnLeafV1},
    return_handoff::VerifiedObjectReturnHandoffV1};
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_control_flow::{DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1};
use crate::mir::resolved_semantics::home_new_prefix::{TerminalReturnedSourceV1, TerminalValueReturnV1};
use std::rc::Rc;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;

#[derive(Debug)]
enum MixedReturnEvidenceV1 {
    Leaf(VerifiedObjectReturnLeafV1),
    OwnedCall(Rc<VerifiedObjectReturnHandoffV1>),
    BorrowedLocal {
        loan: crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        completion: Rc<VerifiedFunctionCompletionV1>,
    },
}
/// Borrowed inspection of the original evidence, never a new result seal.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) enum MixedReturnSourceV1<'a> {
    Leaf(&'a VerifiedObjectReturnLeafV1),
    OwnedCall,
    BorrowedLocal(&'a crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1),
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct VerifiedMixedCallResultV1 {
    source: LexicalInstanceCallSourceTargetV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
    rows: Box<[(Rc<ResultOriginWitnessV1>, TerminalValueReturnV1, MixedReturnEvidenceV1)]>,
    teardown: ObjectReturnTeardownDescriptorV1,
}
impl VerifiedMixedCallResultV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn returns(&self)
        -> impl Iterator<Item = (&Rc<ResultOriginWitnessV1>, &TerminalValueReturnV1, MixedReturnSourceV1<'_>)> {
        self.rows.iter().map(|(root, terminal, evidence)| (root, terminal, match evidence {
            MixedReturnEvidenceV1::Leaf(leaf) => MixedReturnSourceV1::Leaf(leaf),
            MixedReturnEvidenceV1::OwnedCall(_) => MixedReturnSourceV1::OwnedCall,
            MixedReturnEvidenceV1::BorrowedLocal { loan, .. } => MixedReturnSourceV1::BorrowedLocal(loan),
        }))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn source(&self) -> &LexicalInstanceCallSourceTargetV1 { &self.source }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn teardown(&self) -> &ObjectReturnTeardownDescriptorV1 { &self.teardown }
}

fn same_borrowed_edge(
    witness: &ResultOriginWitnessV1,
    loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
) -> bool {
    let ResultWitnessStepV1::Call { site, key, callee, substitution } = witness.step()
    else { return false; };
    site == loan.call() && key == loan.key() && loan.witnesses().iter().any(|candidate| {
        if candidate.origin() != witness.origin() { return false; }
        matches!(candidate.step(), ResultWitnessStepV1::Call { site: s, key: k, callee: c, substitution: a }
            if s == site && k == key && Rc::ptr_eq(c, callee) && a == substitution)
    })
}

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the SAME inner source row before Take or from its retained
    /// physical packet afterwards. Affine consumption never remints its seal.
    fn mixed_borrowed_completion_v1(
        &self, loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<Option<Rc<VerifiedFunctionCompletionV1>>, String> {
        context.require_ledger(self)?;
        use super::super::lexical_instance_call::LexicalInstanceCallDispositionSlotV1;
        let check = |row: &super::super::lexical_instance_call::LexicalInstanceCallDispositionRowV1| {
            if row.source_target().borrowed_nullable_local_source() != Some(loan) { return Err(freeze("borrowed-call-source")); }
            self.corroborate_borrowed_nullable_call_v1(row)?;
            self.completion_source_for_owner_v1(loan.callee_owner()).map(Rc::clone)
                .ok_or_else(|| freeze("borrowed-completion-missing"))
        };
        let slots = self.lexical_instance_calls.borrow();
        match slots.get(loan.call()) {
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) => return check(row).map(Some),
            Some(LexicalInstanceCallDispositionSlotV1::Taken) => {},
            _ => return Ok(None),
        }
        drop(slots);
        context.with_taken_original_row_v1(loan.call(), check).map(Some)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn checked_mixed_call_result_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<VerifiedMixedCallResultV1>, String> {
        let loan = source.mixed_local_source().ok_or_else(|| freeze("source-missing"))?;
        self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
        if loan.key() != source.target() || loan.callee_owner() != source.callee_owner()
            || loan.call() != source.call_site() || loan.receiver() != source.receiver_site()
        { return Err(freeze("source-identity")); }
        let Some(completion) = self.completion_source_for_owner_v1(source.callee_owner()) else { return Ok(None); };
        let Some(contract) = results.row(source.target_batch_slot()) else { return Ok(None); };
        let Some(relations) = self.terminal_relation_index.get(&source.callee_owner()) else { return Ok(None); };
        if contract.owner() != source.callee_owner() || contract.result().is_some()
            || !std::ptr::eq(contract.borrow().completion(), completion.as_ref())
            || !std::ptr::eq(contract.borrow().terminal_relations(), relations.as_ref())
            || completion.function_exit_contract().declared_result() != &DeclaredFunctionResultContractV1::Unannotated
        { return Err(freeze("contract-completion")); }
        let Some(exits) = self.checked_completed_object_callee_target_exits_v1(source)? else { return Ok(None); };
        if exits.len() != loan.roots().len()
            || exits.iter().zip(loan.roots()).any(|((root, _), original)| !Rc::ptr_eq(root, original))
        { return Err(freeze("whole-root-coverage")); }
        let mut rows = Vec::new();
        let mut teardown = None;
        let mut missing = false;
        for (root, terminal) in exits.iter() {
            let evidence = match root.step() {
                ResultWitnessStepV1::NullLiteral | ResultWitnessStepV1::FreshConstruction => {
                    self.checked_mixed_return_leaf_v1(loan, root)?.map(MixedReturnEvidenceV1::Leaf)
                }
                ResultWitnessStepV1::Call { .. } => match terminal.returned() {
                    TerminalReturnedSourceV1::OwnedCall(original) => {
                        if !original.qualification().witnesses().iter().any(|witness| Rc::ptr_eq(witness, root)) {
                            return Err(freeze("owned-call-root"));
                        }
                        match self.normal_return_dispositions.as_ref().and_then(|map|
                            map.get(&(terminal.owner(), terminal.return_site().clone()))) {
                            Some(NormalReturnDispositionV1::Verified { proof }) if proof.acquisition().original() == original.as_ref() =>
                                Some(MixedReturnEvidenceV1::OwnedCall(Rc::clone(proof))),
                            Some(NormalReturnDispositionV1::Verified { .. }) => return Err(freeze("owned-call-disposition")),
                            _ => None,
                        }
                    }
                    TerminalReturnedSourceV1::BorrowedNullableLocal(original) => {
                        let current = self.checked_borrowed_nullable_return_source_v1(terminal.owner(), terminal.return_site())?
                            .ok_or_else(|| freeze("borrowed-return-source"))?;
                        if current != original.as_ref() || !same_borrowed_edge(root, current) {
                            return Err(freeze("borrowed-return-edge"));
                        }
                        self.mixed_borrowed_completion_v1(current, CallValidationContextV1::Lowering(self))?.map(|completion|
                            MixedReturnEvidenceV1::BorrowedLocal { loan: current.clone(), completion })
                    }
                    _ => return Err(freeze("call-terminal-role")),
                },
                // These source forms still need their own exact acquired-Home
                // or original-formal Return seal; kind alone grants nothing.
                ResultWitnessStepV1::Formal { .. } | ResultWitnessStepV1::MapStored { .. } => None,
            };
            missing |= self.normal_exit_projection_v1(terminal.owner(), terminal.return_site())?.is_none();
            let Some(evidence) = evidence else { missing = true; continue; };
            missing |= matches!(&evidence, MixedReturnEvidenceV1::Leaf(leaf)
                if leaf.requires_anchored_field_end_v1());
            let descriptor = match &evidence {
                MixedReturnEvidenceV1::Leaf(VerifiedObjectReturnLeafV1::Fresh { teardown, .. }) => Some(teardown),
                MixedReturnEvidenceV1::OwnedCall(proof) => proof.teardown().descriptor().map(|(descriptor, _)| descriptor),
                _ => None,
            };
            if let Some(descriptor) = descriptor {
                if descriptor.object() != loan.object() || teardown.as_ref().is_some_and(|prior| prior != descriptor) {
                    return Err(freeze("fresh-teardown-disagreement"));
                }
                teardown = Some(descriptor.clone());
            } else if matches!(evidence, MixedReturnEvidenceV1::OwnedCall(_)) { missing = true; }
            rows.push((Rc::clone(root), terminal.clone(), evidence));
        }
        let Some(teardown) = teardown else { return Ok(None); };
        if missing || rows.len() != exits.len() || !teardown.supports_direct_teardown(false) { return Ok(None); }
        let checked = VerifiedMixedCallResultV1 { source: source.clone(), completion: Rc::clone(completion), rows: rows.into_boxed_slice(), teardown };
        self.recheck_mixed_call_result_v1(&checked)?;
        Ok(Some(checked))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn recheck_mixed_call_result_v1(
        &self, proof: &VerifiedMixedCallResultV1,
    ) -> Result<(), String> {
        self.recheck_mixed_call_result_using_v1(proof, CallValidationContextV1::Lowering(self))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn recheck_mixed_call_result_using_v1(
        &self, proof: &VerifiedMixedCallResultV1, context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let source = &proof.source;
        let loan: &MixedLocalCallSourceLoanV1 = source.mixed_local_source().ok_or_else(|| freeze("source-missing"))?;
        self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
        if self.completion_source_for_owner_v1(source.callee_owner()).is_none_or(|current| !Rc::ptr_eq(current, &proof.completion)) {
            return Err(freeze("completion-drift"));
        }
        let exits = self.checked_completed_object_callee_target_exits_v1(source)?.ok_or_else(|| freeze("exits-unavailable"))?;
        if exits.len() != proof.rows.len() || proof.teardown.object() != loan.object()
            || !proof.teardown.supports_direct_teardown(false) { return Err(freeze("retained-coverage")); }
        for ((root, terminal), (held, original, evidence)) in exits.iter().zip(&proof.rows) {
            if !Rc::ptr_eq(root, held) || terminal != original { return Err(freeze("retained-return-drift")); }
            match evidence {
                MixedReturnEvidenceV1::Leaf(original) => {
                    if self.checked_mixed_return_leaf_v1(loan, root)?.as_ref() != Some(original) { return Err(freeze("leaf-drift")); }
                }
                MixedReturnEvidenceV1::OwnedCall(original) => {
                    if !matches!(self.normal_return_dispositions.as_ref().and_then(|map| map.get(&(terminal.owner(), terminal.return_site().clone()))),
                        Some(NormalReturnDispositionV1::Verified { proof }) if Rc::ptr_eq(proof, original)) {
                        return Err(freeze("owned-proof-drift"));
                    }
                }
                MixedReturnEvidenceV1::BorrowedLocal { loan: held, completion } => {
                    if self.checked_borrowed_nullable_return_source_v1(terminal.owner(), terminal.return_site())? != Some(held)
                        || !same_borrowed_edge(root, held)
                        || self.completion_source_for_owner_v1(held.callee_owner()).is_none_or(|current| !Rc::ptr_eq(current, completion))
                    { return Err(freeze("borrowed-proof-drift")); }
                    if self.mixed_borrowed_completion_v1(held, context)?.as_ref().is_none_or(|current| !Rc::ptr_eq(current, completion)) {
                        return Err(freeze("borrowed-call-unavailable"));
                    }
                }
            }
            if self.normal_exit_projection_v1(terminal.owner(), terminal.return_site())?.is_none() { return Err(freeze("normal-projection-drift")); }
            if matches!(evidence, MixedReturnEvidenceV1::Leaf(leaf) if leaf.requires_anchored_field_end_v1()) {
                return Err(freeze("anchored-field-end-unconnected"));
            }
            let descriptor = match evidence {
                MixedReturnEvidenceV1::Leaf(VerifiedObjectReturnLeafV1::Fresh { teardown, .. }) => Some(teardown),
                MixedReturnEvidenceV1::OwnedCall(child) => Some(child.teardown().descriptor()
                    .ok_or_else(|| freeze("retained-end-unavailable"))?.0),
                _ => None,
            };
            if descriptor.is_some_and(|current| current != &proof.teardown) { return Err(freeze("retained-end-drift")); }
        }
        Ok(())
    }
}
fn freeze(reason: &str) -> String { format!("[freeze:contract][ordinary-new/mixed-result/{reason}]") }
