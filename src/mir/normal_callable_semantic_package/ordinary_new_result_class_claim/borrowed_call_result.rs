//! Destination-neutral Call identity, support and immutable result witnesses.
//! Real local binding/declaration obligations stay in the local wrapper.
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct BorrowedCallResultSourceLoanV1 {
    pub(super) declared_borrow: Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
    pub(super) call: OwnedExprSiteV1,
    pub(super) receiver: SourceExprSiteV1,
    pub(super) key: CanonicalSameModuleCallableKeyV1,
    pub(super) callee_owner: FunctionOwnerIdV1,
    pub(super) support: BorrowedCallSupportV1,
    pub(super) witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl BorrowedCallResultSourceLoanV1 {
    pub(crate) fn access_v1(&self) -> crate::mir::instruction::BorrowedResultAccessV1 {
        use crate::mir::instruction::BorrowedResultAccessV1 as Access;
        if self.declared_borrow.is_some() { Access::ReadOnly } else { Access::LegacyUnqualified }
    }
    pub(crate) fn result_kind_v1(&self) -> crate::mir::instruction::InvokeCallResultKind {
        crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
            support: self.result_support_v1(), access: self.access_v1(),
        }
    }
    pub(crate) fn is_declared_read_v1(&self) -> bool {
        self.access_v1() == crate::mir::instruction::BorrowedResultAccessV1::ReadOnly
    }
    pub(crate) fn matches_read_home_actual_v1(&self, call: &OwnedExprSiteV1, ordinal: u32,
        binding: BindingRefV1, root: BindingRefV1, acquisition: &OwnedExprSiteV1,
        read: &crate::mir::resolved_semantics::SourceExprSiteV1, moved: bool) -> bool {
        self.is_declared_read_v1() && self.reborrowed_read_v1().is_none() && !moved && self.call() == call
            && self.callee_ordinal() == Some(ordinal)
            && self.owned_local_support_v1().is_some_and(|(original, acquired, at)|
                binding == original && root == original && acquired == acquisition
                    && at.owner() == call.owner() && at.site() == read)
    }
    pub(crate) fn declared_read_result_v1(&self) -> Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>> {
        self.declared_borrow.as_ref()
    }

    /// Qualifiers belong to the original declaration, not a copied bool/ordinal.
    pub(crate) fn declared_result_matches_v1(&self,
        declaration: Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
    ) -> bool {
        crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1::same_source(
            self.declared_borrow.as_ref(), declaration,
        )
    }

    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 {
        &self.receiver
    }
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }
    pub(crate) fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.callee_owner
    }
    /// Project the SAME original support without completing its physical ABI.
    pub(crate) fn result_support_v1(&self) -> crate::mir::instruction::BorrowedResultSupportV1 {
        use crate::mir::instruction::BorrowedResultSupportV1 as Support;
        match &self.support {
            BorrowedCallSupportV1::EntryForward { callee, .. }
                | BorrowedCallSupportV1::Reborrow { callee, .. } => *callee,
            BorrowedCallSupportV1::OwnedLocal { callee_ordinal, .. } => Support::Formal(*callee_ordinal),
            BorrowedCallSupportV1::MapSlot(map) => map.source.result_support_v1(),
            BorrowedCallSupportV1::OwnedReceiver { .. } => Support::Receiver,
        }
    }
    pub(crate) fn callee_ordinal(&self) -> Option<u32> {
        self.result_support_v1().formal_ordinal()
    }
    pub(crate) fn lifetime_anchor_v1(&self) -> Option<BorrowedLifetimeAnchorV1> {
        if let BorrowedCallSupportV1::Reborrow { actual, .. } = &self.support { return Some(actual.anchor().clone()); }
        if let Some((root, caller_support, _)) = self.entry_support_v1() {
            Some(BorrowedLifetimeAnchorV1::Entry { root, caller_support })
        } else {
            let (root, acquisition, _) = self.owned_local_support_v1()?;
            Some(BorrowedLifetimeAnchorV1::Owned { root, acquisition: acquisition.clone() })
        }
    }
    /// The old payload-formal root exists only on the formal-forward lane.
    pub(crate) fn root(&self) -> Option<BindingRefV1> {
        match self.lifetime_anchor_v1()? {
            BorrowedLifetimeAnchorV1::Entry { root, caller_support: crate::mir::instruction::BorrowedResultSupportV1::Formal(_) } => Some(root), _ => None,
        }
    }
    /// Original entry support and read; a receiver is never a payload formal.
    pub(crate) fn entry_support_v1(&self) -> Option<(BindingRefV1, crate::mir::instruction::BorrowedResultSupportV1, &OwnedExprSiteV1)> {
        match &self.support { BorrowedCallSupportV1::EntryForward { root, caller, actual, .. } => Some((*root, *caller, actual)),
            BorrowedCallSupportV1::Reborrow { actual, .. } => actual.single_loan()?.entry_support_v1(), _ => None }
    }
    /// Lifetime anchor only. A Map holder is not the returned payload receiver.
    pub(crate) fn support_root(&self) -> BindingRefV1 {
        match &self.support { BorrowedCallSupportV1::Reborrow { actual, .. } => match actual.anchor() { BorrowedLifetimeAnchorV1::Entry { root, .. } | BorrowedLifetimeAnchorV1::Owned { root, .. } => *root },
            BorrowedCallSupportV1::EntryForward { root, .. } => *root,
            BorrowedCallSupportV1::MapSlot(map) => map.holder,
            BorrowedCallSupportV1::OwnedLocal { source, .. } | BorrowedCallSupportV1::OwnedReceiver { source, .. } => source.binding }
    }
    /// Read source and original acquisition, for the caller's live-Home check.
    pub(crate) fn owned_local_support_v1(&self) -> Option<(BindingRefV1, &OwnedExprSiteV1, &OwnedExprSiteV1)> {
        match &self.support {
            BorrowedCallSupportV1::Reborrow { actual, .. } => actual.single_loan()?.owned_local_support_v1(),
            BorrowedCallSupportV1::OwnedLocal { actual, source, .. } => Some((source.binding, &source.acquisition, &actual.site)),
            BorrowedCallSupportV1::OwnedReceiver { read, source } => Some((source.binding, &source.acquisition, read)),
            _ => None,
        }
    }
    pub(crate) fn reborrowed_read_v1(&self) -> Option<&BorrowedSupportReadV1> {
        match &self.support { BorrowedCallSupportV1::Reborrow { actual, .. } => Some(actual), _ => None }
    }
    pub(crate) fn is_map_slot(&self) -> bool { matches!(self.support, BorrowedCallSupportV1::MapSlot(_)) }
    /// Nominal payload only; the holder is not this object and owns no result.
    pub(crate) fn map_payload_v1(&self) -> Option<(&str, hakorune_mir_defs::CanonicalObjectIdV1)> {
        let BorrowedCallSupportV1::MapSlot(map) = &self.support else { return None; };
        Some((map.source.payload_class_v1(), map.source.payload_object_v1()))
    }
    /// Original Map key correspondence only, never a Text or executable grant.
    pub(crate) fn map_key_actual_v1(&self) -> Option<(u32, &OwnedExprSiteV1, BindingRefV1)> {
        let BorrowedCallSupportV1::MapSlot(map) = &self.support else { return None; };
        Some((map.source.ordinal, &map.key.site, map.key.binding?))
    }
    /// Payload nonnull is independent of keeping the original borrowed value.
    /// A local New narrows formal-derived Null, never a literal Null return.
    pub(crate) fn payload_nonnull_v1(&self) -> bool {
        !self.witnesses.is_empty() && match &self.support {
            BorrowedCallSupportV1::Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal), actual } =>
                actual.payload_nonnull() && self.witnesses.iter().all(|root| root.formal_ordinal() == Some(*ordinal)),
            BorrowedCallSupportV1::Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1::Receiver, actual } =>
                actual.payload_nonnull() && self.witnesses.iter().all(|root| root.receiver_payload_nonnull_v1()),
            BorrowedCallSupportV1::OwnedLocal { callee_ordinal, .. } => self.witnesses.iter()
                .all(|root| root.formal_ordinal() == Some(*callee_ordinal)),
            BorrowedCallSupportV1::OwnedReceiver { .. } => self.witnesses.iter().all(|root| root.receiver_payload_nonnull_v1()),
            _ => false,
        }
    }
    pub(crate) fn witnesses(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.witnesses
    }
    /// A Return may have another source site, but must retain the SAME inner
    /// callee witness and formal substitution. This grants no access downgrade.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_call_witness_v1(
        &self, witness: &ResultOriginWitnessV1,
    ) -> bool {
        let ResultWitnessStepV1::Call { site, key, callee, substitution } = witness.step()
            else { return false; };
        if site != self.call() || key != self.key() { return false; }
        if let BorrowedCallSupportV1::Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal), actual } = &self.support {
            if !self.witnesses().iter().any(|root| Rc::ptr_eq(root, callee)) { return false; }
            if substitution.is_none() { return callee.is_literal_null_v1() && witness.origin() == &ResultValueOriginV1::Null; }
            let Some(ResultSupportSubstitutionV1::Read { callee_ordinal, source }) = substitution else { return false; };
            let Some((_, caller_ordinal)) = substitution.as_ref().and_then(|row| row.caller_formal_v1(site, callee)) else { return false; };
            let origin = match callee.origin() { ResultValueOriginV1::ForwardFormal { .. } => ResultValueOriginV1::ForwardFormal { ordinal: caller_ordinal }, other => other.clone() };
            return callee_ordinal == ordinal && source.matches_support_v1(actual) && witness.origin() == &origin;
        }
        self.witnesses().iter().any(|candidate| candidate.same_call_source_v1(witness))
    }
}
impl BorrowedCallResultSourceLoanV1 {
    fn same_header_v1(&self, other: &Self) -> bool {
        self.declared_result_matches_v1(other.declared_read_result_v1())
            && self.call == other.call && self.receiver == other.receiver
            && self.key == other.key && self.callee_owner == other.callee_owner
    }
    /// Regenerating a Call may allocate a new wrapper, but never a new callee
    /// witness, substitution, source declaration or permission.
    pub(super) fn same_original_source_v1(&self, other: &Self) -> bool {
        let same_witness = |a: &Rc<ResultOriginWitnessV1>, b: &Rc<ResultOriginWitnessV1>|
            Rc::ptr_eq(a, b) || (a.site() == b.site() && a.same_call_source_v1(b));
        self.same_header_v1(other)
            && self.support.same_original_source_v1(&other.support)
            && self.witnesses.len() == other.witnesses.len()
            && self.witnesses.iter().all(|root|
                self.witnesses.iter().filter(|a| same_witness(a, root)).count()
                    == other.witnesses.iter().filter(|b| same_witness(b, root)).count())
    }
}
impl PartialEq for BorrowedCallResultSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.same_header_v1(other)
            && self.support == other.support
            && self.witnesses.len() == other.witnesses.len()
            && self
                .witnesses
                .iter()
                .zip(&other.witnesses)
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for BorrowedCallResultSourceLoanV1 {}
