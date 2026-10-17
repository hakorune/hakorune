//! Borrow the current sole owner of original Call packets after affine moves.
//! This context owns no source facts, packet registry or completion seal.
use super::*;
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;

#[derive(Clone, Copy)]
pub(in crate::mir) enum CallValidationContextV1<'a> {
    Lowering(&'a OrdinaryNewClaimLedgerV1),
    Finalized(&'a FinalizedRootSourceHandoffV1),
}
impl<'a> CallValidationContextV1<'a> {
    /// Borrow the same source-owned write after the packet pool's affine move.
    /// Membership is safe during read take; full RHS validation is a later phase.
    pub(in crate::mir::normal_callable_semantic_package) fn with_original_field_write_v1<R>(
        self, frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldWriteFrameV1>,
        check: impl FnOnce(&EmittedBorrowedFieldWriteV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_groups_v1(frame.source().owner(), |groups| {
            let mut found = None;
            for group in groups {
                let Some(packet) = group.borrowed_field_write() else { continue };
                if !std::rc::Rc::ptr_eq(packet.source().frame(), frame) { continue; }
                if group.site() != &packet.source().value_site() || found.replace(packet).is_some() {
                    return Err(freeze("field-write/packet-membership"));
                }
                if matches!(self, Self::Finalized(_)) { group.require_sealed_condition_v1()?; }
            }
            check(found.ok_or_else(|| freeze("field-write/original-packet-missing"))?)
        })
    }
    pub(in crate::mir) fn ledger(self) -> &'a OrdinaryNewClaimLedgerV1 {
        match self { Self::Lowering(ledger) => ledger, Self::Finalized(source) => &source.ledger }
    }
    pub(in crate::mir) fn require_ledger(self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        if !std::ptr::eq(self.ledger(), ledger) { return Err(freeze("call-validation/foreign-ledger")); }
        Ok(())
    }
    /// Source membership follows the exact affine slot state, without retrying pools.
    pub(in crate::mir::normal_callable_semantic_package) fn with_current_instance_source_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallSourceTargetV1) -> Result<R, String>,
    ) -> Result<R, String> {
        use super::super::lexical_instance_call::LexicalInstanceCallDispositionSlotV1 as Slot;
        let rows = self.ledger().lexical_instance_calls.borrow();
        match rows.get(site) {
            Some(Slot::SourcePending(source)) => check(source),
            Some(Slot::Ready(row)) => check(row.source_target()),
            Some(Slot::Taken) => {
                drop(rows);
                self.with_taken_original_row_v1(site, |row| check(row.source_target()))
            },
            None => Err(freeze("call-validation/original-instance-source-missing")),
        }
    }
    /// Borrow groups from their current affine owner; never retry another pool.
    pub(super) fn with_local_call_groups_v1<R>(
        self, owner: FunctionOwnerIdV1,
        read: impl FnOnce(&[RootLifecyclePrefixBindingGroupV1]) -> Result<R, String>,
    ) -> Result<R, String> {
        match self {
            Self::Lowering(ledger) => read(ledger.root_local_call_bindings.borrow()
                .get(&owner).map(Vec::as_slice).unwrap_or(&[])),
            Self::Finalized(source) => read(source.local_calls.get(&owner)
                .map(Vec::as_slice).unwrap_or(&[])),
        }
    }

    /// Exit bindings move to the finalized root handoff exactly once.
    pub(super) fn visit_exit_bindings_v1(
        self, owner: FunctionOwnerIdV1, mut visit: impl FnMut(&[(BasicBlockId, MirInstruction)]),
    ) -> Result<(), String> {
        for ((exit_owner, site), progress) in self.ledger().root_exits.borrow().iter() {
            if *exit_owner != owner { continue; }
            match progress {
                RootHomeExitProgress::Emitted { bindings, .. } => visit(bindings),
                RootHomeExitProgress::Finalized(_) => {
                    if let Self::Finalized(source) = self {
                        let exit = site.explicit_site().ok_or_else(|| freeze("call-validation/finalized-exit-site"))?;
                        if source.owner != owner { return Err(freeze("call-validation/finalized-exit-owner")); }
                        let (_, bindings) = source.call_entries.get(exit)
                            .ok_or_else(|| freeze("call-validation/finalized-exit-missing"))?;
                        visit(bindings);
                    }
                }
                _ => {},
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn with_taken_original_row_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&LexicalInstanceCallDispositionRowV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_taken_packet_v1(site, |packet| check(packet.original_row()?))
    }

    pub(in crate::mir) fn with_taken_packet_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&EmittedLexicalCallProjectionV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_groups_v1(site.owner(), |groups| {
            let mut found = None;
            for group in groups {
                for packet in group.lexical_packets() {
                    if let Some(packet) = packet.retained_node_at_v1(site)? {
                        if found.replace(packet).is_some() { return Err(freeze("mixed-result/borrowed-consumed-packet-duplicate")); }
                    }
                }
            }
            check(found.ok_or_else(|| freeze("mixed-result/borrowed-consumed-packet-missing"))?)
        })
    }
}
