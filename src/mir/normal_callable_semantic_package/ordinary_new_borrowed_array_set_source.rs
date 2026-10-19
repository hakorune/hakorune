//! Original field-index Set requirements lent by the already qualified entry.
//! Physical emission and both finished operand scans remain separate checks.
use super::*;
use super::super::borrowed_formal_uses::{BorrowedFormalUseDraftKindV1 as Use,
    GuardedFormalFieldIndexV1, CheckedIntegerOperandReachV1};
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathV1, SourcePathSegmentV1};
use crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1;

#[derive(Debug)]
pub(in crate::mir) struct BorrowedArraySetSourceLoanV1 {
    call: OwnedExprSiteV1,
    receiver: OwnedExprSiteV1,
    value: (BindingRefV1, BindingRefV1, OwnedExprSiteV1),
    index: Rc<GuardedFormalFieldIndexV1>,
    reach: CheckedIntegerOperandReachV1,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    receiver_source: Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
    receiver_read: Rc<LocalFieldReadSourceV1>,
    index_read: Rc<LocalFieldReadSourceV1>,
}
impl BorrowedArraySetSourceLoanV1 {
    pub(in crate::mir) fn call(&self) -> &OwnedExprSiteV1 { &self.call }
    pub(in crate::mir) fn receiver(&self) -> &OwnedExprSiteV1 { &self.receiver }
    pub(in crate::mir) fn argument_sites(&self) -> [&SourceExprSiteV1; 2] {
        [self.index.parts_v1().0.site(), self.value.2.site()]
    }
    pub(in crate::mir) fn value_formal(&self) -> BindingRefV1 { self.value.1 }
    pub(in crate::mir::normal_callable_semantic_package) fn value_binding(&self) -> BindingRefV1 { self.value.0 }
    pub(in crate::mir) fn index_formal(&self) -> BindingRefV1 { self.index.parts_v1().3 }
    pub(in crate::mir) fn integer_guard(&self) -> &OwnedExprSiteV1 { self.reach.guard().binary_site_v1() }
    pub(in crate::mir::normal_callable_semantic_package) fn null_guard(&self) -> &SourceNodeSiteV1 {
        self.index.parts_v1().4
    }
    pub(in crate::mir) fn index_field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 {
        self.index_read.field()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_reads(
        &self, ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<[(crate::mir::BasicBlockId, crate::mir::MirInstruction); 2], String> {
        Ok([ledger.array_set_read_emission_v1(&self.receiver, &self.receiver_read)?,
            ledger.array_set_read_emission_v1(self.index.parts_v1().0, &self.index_read)?])
    }
    pub(in crate::mir) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let owner = self.call.owner();
        if ledger.completion_source_for_owner_v1(owner)
            .is_none_or(|original| !Rc::ptr_eq(original, &self.completion)) {
            return Err(freeze("borrowed-set/completion-identity"));
        }
        let entry = ledger.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        let mut originals = entry.legacy_uses_v1().filter_map(|row| match &row.kind {
            Use::ArrayElementFieldIndexValue { call, index, reach, receiver } if call == &self.call =>
                Some((row, index, reach, receiver)),
            _ => None,
        });
        let (row, index, reach, original_receiver) = originals.next().ok_or_else(|| freeze("borrowed-set/source-missing"))?;
        let (access, receiver, binding, formal, _) = index.parts_v1();
        if originals.next().is_some() || !Rc::ptr_eq(index, &self.index)
            || !Rc::ptr_eq(original_receiver, &self.receiver_source) || original_receiver.site() != &self.receiver
            || (row.binding, row.formal, &row.site) != (self.value.0, self.value.1, &self.value.2)
            || reach != &self.reach || !Rc::ptr_eq(reach.guard(), self.reach.guard())
            || reach.operand() != (row.binding, row.formal, &row.site)
            || entry.origins().get(&row.binding) != Some(&row.formal)
            || entry.origins().get(&binding) != Some(&formal)
            || [access.owner(), receiver.owner(), row.site.owner()].iter().any(|actual| *actual != owner)
            || !entry.formals.iter().any(|(_, original)| *original == row.formal)
            || !entry.formals.iter().any(|(_, original)| *original == formal)
        { return Err(freeze("borrowed-set/original-requirements")); }
        let actual_receiver = ledger.array_set_receiver_source_v1(&self.receiver_source)?;
        let actual_index = ledger.array_set_index_source_v1(access, receiver, binding, formal)?;
        if !Rc::ptr_eq(&actual_receiver, &self.receiver_read) || !Rc::ptr_eq(&actual_index, &self.index_read) {
            return Err(freeze("borrowed-set/field-source-identity"));
        }
        if entry.formal_object_view(formal)?.is_none_or(|view| view.object() != self.index_field().object())
            || !matches!(self.index_read.result(),
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar)
            || !matches!(self.receiver_read.result(),
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::ArrayField(_))
        { return Err(freeze("borrowed-set/field-class")); }
        Ok(())
    }
}
impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn array_set_receivers_v1(&self)
        -> impl Iterator<Item = &Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>> {
        self.legacy_uses_v1().filter_map(|row| match &row.kind {
            Use::ArrayElementFieldIndexValue { receiver, .. } => Some(receiver), _ => None,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn borrows_array_set_receiver_v1(
        &self, source: &Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
    ) -> bool {
        source.site().owner() == self.owner && source.entry_call_v1().is_some_and(|call| call.owner() == self.owner)
            && self.legacy_uses_v1().filter(|row| matches!(&row.kind,
                Use::ArrayElementFieldIndexValue { call, receiver, .. }
                    if source.entry_call_v1() == Some(call) && Rc::ptr_eq(receiver, source))).count() == 1
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn borrowed_array_set_source_at_v1(
        &self, owner: FunctionOwnerIdV1, call: &OwnedExprSiteV1,
    ) -> Result<Option<BorrowedArraySetSourceLoanV1>, String> {
        if call.owner() != owner { return Err(freeze("borrowed-set/source-owner")); }
        let Some(source) = self.borrowed_formal_source.as_ref().and_then(|row| row.as_ref().ok())
            else { return Ok(None) };
        let Some(definition) = source.definitions.get(&owner).or_else(|| source.source_only_definitions.get(&owner))
            else { return Ok(None) };
        if !definition.uses.iter().any(|row| matches!(&row.kind,
            Use::ArrayElementFieldIndexValue { call: original, .. } if original == call)) { return Ok(None); }
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        let mut originals = entry.legacy_uses_v1().filter_map(|row| match &row.kind {
            Use::ArrayElementFieldIndexValue { call: original, index, reach, receiver } if original == call =>
                Some((row, index, reach, receiver)),
            _ => None,
        });
        let (row, index, reach, original_receiver) = originals.next().ok_or_else(|| freeze("borrowed-set/source-missing"))?;
        if originals.next().is_some() { return Err(freeze("borrowed-set/source-duplicate")); }
        let receiver = OwnedExprSiteV1::new(owner,
            SourcePathV1::from_node(call.site().node()).child(SourcePathSegmentV1::Receiver).expr());
        let loan = BorrowedArraySetSourceLoanV1 {
            call: call.clone(), receiver_read: self.array_set_receiver_source_v1(original_receiver)?, receiver,
            receiver_source: Rc::clone(original_receiver),
            value: (row.binding, row.formal, row.site.clone()), index: Rc::clone(index), reach: reach.clone(),
            completion: Rc::clone(self.completion_source_for_owner_v1(owner)
                .ok_or_else(|| freeze("borrowed-set/completion-missing"))?),
            index_read: self.array_set_index_source_v1(index.parts_v1().0, index.parts_v1().1,
                index.parts_v1().2, index.parts_v1().3)?,
        };
        loan.validate_source(self)?;
        Ok(Some(loan))
    }
}
