//! Realization of exact terminal field reads in the existing New ledger.
use super::*;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use hakorune_mir_defs::CanonicalFieldRefV1;

#[derive(Debug)]
pub(super) struct FieldRead {
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: BindingRefV1,
    pub(super) home: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) progress: Progress,
}

#[derive(Debug)]
pub(super) enum Progress {
    Pending,
    Taken(ValueId),
    Emitted(BasicBlockId, MirInstruction),
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn take_terminal_field_read(
        &self, site: &OwnedExprSiteV1,
        resolve_receiver: impl FnOnce(BindingRefV1) -> Result<ValueId, String>,
    ) -> Result<Option<(ValueId, CanonicalFieldRefV1)>, String> {
        let mut reads = self.field_reads.borrow_mut();
        let Some(row) = reads.get_mut(site) else { return Ok(None); };
        if !matches!(*self.root_exit.borrow(),
            local_commit::RootHomeExitProgress::Prepared(_)
                | local_commit::RootHomeExitProgress::Unavailable) {
            return Err(fault("root-exit-phase"));
        }
        if row.receiver.owner() != site.owner() || row.home.owner() != site.owner() {
            return Err(fault("foreign-binding"));
        }
        let Some((last, parent)) = row.receiver_site.node().segments().split_last()
            else { return Err(fault("receiver-source-site")); };
        if *last != SourcePathSegmentV1::Receiver || parent != site.site().node().segments() {
            return Err(fault("receiver-source-site"));
        }
        if !self.local_commits.borrow().values().any(|local| local.installs(row.home)) {
            return Err(fault("home-not-installed"));
        }
        if !matches!(row.progress, Progress::Pending) { return Err(fault("already-taken")); }
        let base = resolve_receiver(row.receiver)?;
        row.progress = Progress::Taken(base);
        Ok(Some((base, row.field)))
    }

    pub(crate) fn record_terminal_field_read(
        &self, site: &OwnedExprSiteV1, block: BasicBlockId,
        dst: ValueId, base: ValueId, field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let mut reads = self.field_reads.borrow_mut();
        let row = reads.get_mut(site).ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(row.progress, Progress::Taken(expected) if expected == base) || row.field != field {
            return Err(fault("emission-mismatch"));
        }
        row.progress = Progress::Emitted(block, MirInstruction::ObjectFieldGet { dst, base, field });
        Ok(())
    }

    pub(super) fn field_reads_complete(&self) -> bool {
        self.field_reads.borrow().values().all(|row| matches!(row.progress, Progress::Emitted(..)))
    }

    pub(super) fn validate_field_reads(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
    ) -> Result<(), String> {
        let reads = self.field_reads.borrow();
        let mut expected = Vec::new();
        for (site, row) in reads.iter() {
            if site.owner() != owner { return Err(fault("foreign-owner")); }
            let Progress::Emitted(block, instruction) = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            expected.push((*block, instruction));
        }
        for block in function.blocks.values() {
            for actual in block.all_instructions().filter(|i| matches!(i, MirInstruction::ObjectFieldGet { .. })) {
                let index = expected.iter().position(|(id, inst)| *id == block.id && *inst == actual)
                    .ok_or_else(|| fault("unowned-or-drifted-read"))?;
                expected.swap_remove(index);
            }
        }
        if !expected.is_empty() { return Err(fault("missing-emission")); }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-field-read/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_field_reads_tests.rs"]
mod tests;
