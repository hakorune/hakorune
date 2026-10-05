//! Realization of exact terminal field reads in the existing New ledger.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(super) struct FieldRead {
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: BindingRefV1,
    pub(super) home: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) progress: Progress,
}

/// One argument-position `receiver.field` read staged by the issuer's
/// proof. The site key is the exact `FieldAccess` argument expression site
/// — the read belongs to that argument, never to a terminal exit.
#[derive(Debug)]
pub(super) struct ArgumentFieldRead {
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) object: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) progress: Progress,
}

/// One `receiver.field` read staged by the issuer's `local_read_field`
/// proof. The exact FieldAccess site belongs to a direct initializer or
/// a completely proved pure scalar expression/condition root, never to
/// a terminal exit or a selected `new` argument.
/// `receiver` is the local binding whose materialized value is the read
/// base; `result` is the sealed declared-type classification.
#[derive(Debug)]
pub(super) struct LocalFieldRead {
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: BindingRefV1,
    pub(super) home: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) result: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
    pub(super) progress: Progress,
}

#[derive(Debug)]
pub(super) enum Progress {
    Pending,
    Taken(ValueId),
    Emitted(BasicBlockId, MirInstruction),
    /// Retired by the enclosing claim's `RetainedUnavailable` decline:
    /// no selected emitter exists, so the raw lane owns the physical
    /// read at that site. Distinct from `Taken`/`Emitted` — released
    /// rows never produce an `ObjectFieldGet` expectation.
    Released,
}

pub(super) fn merge_staged_field_reads(
    destination: &mut BTreeMap<OwnedExprSiteV1, FieldRead>,
    owner: FunctionOwnerIdV1,
    staged: BTreeMap<OwnedExprSiteV1, FieldRead>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for (site, row) in &staged {
        if site.owner() != owner || row.receiver.owner() != owner || row.home.owner() != owner {
            return Err(OrdinaryNewCoSealIssueV1::FieldReadOwnerMismatch { site: site.clone() });
        }
        if destination.contains_key(site) {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
    }
    destination.extend(staged);
    Ok(())
}

pub(super) fn merge_terminal_relation_field_reads(
    destination: &mut BTreeMap<OwnedExprSiteV1, FieldRead>,
    owner: FunctionOwnerIdV1,
    relations: &BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    staged: BTreeMap<OwnedExprSiteV1, FieldRead>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let mut consumed = false;
    for relation in relations.values() {
        match relation {
            TerminalRelationV1::I64Add(result) => {
                if result.owner() != owner
                    || result
                        .field_reads()
                        .iter()
                        .any(|site| !staged.contains_key(site))
                {
                    return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                        site: result.add_site().clone(),
                    });
                }
                consumed = true;
            }
            TerminalRelationV1::I64Field(result) => {
                if result.owner() != owner || !staged.contains_key(result.field_read_site()) {
                    return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                        site: result.field_read_site().clone(),
                    });
                }
                consumed = true;
            }
            _ => {}
        }
    }
    if !consumed {
        return Ok(());
    }
    merge_staged_field_reads(destination, owner, staged)
}

/// Merge staged argument-position reads into the ledger. Every sealed
/// `I64Field` argument row must carry a staged read at its exact site
/// (referenced ⊆ staged), and only referenced rows enter the ledger —
/// a staged-but-unreferenced read would gate emission on evidence no
/// claim can consume.
pub(super) fn merge_staged_argument_field_reads(
    destination: &mut BTreeMap<OwnedExprSiteV1, ArgumentFieldRead>,
    owner: FunctionOwnerIdV1,
    observations: &BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    staged: BTreeMap<OwnedExprSiteV1, ArgumentFieldRead>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let mut referenced = BTreeSet::new();
    for observation in observations.values() {
        let Ok(rows) = observation.arguments() else {
            continue;
        };
        for row in rows.iter() {
            if matches!(row.kind(), SelectedNewArgumentKindV1::I64Field { .. }) {
                referenced.insert(OwnedExprSiteV1::new(owner, row.site().clone()));
            }
        }
    }
    for site in &referenced {
        if !staged.contains_key(site) {
            return Err(OrdinaryNewCoSealIssueV1::ArgumentFieldReadMissing { site: site.clone() });
        }
    }
    for (site, row) in &staged {
        if site.owner() != owner || row.object.owner() != owner {
            return Err(OrdinaryNewCoSealIssueV1::FieldReadOwnerMismatch { site: site.clone() });
        }
        if destination.contains_key(site) {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
    }
    destination.extend(
        staged
            .into_iter()
            .filter(|(site, _)| referenced.contains(site)),
    );
    Ok(())
}

/// Merge staged local/expression reads into the ledger. Every staged row
/// corresponds to a complete root the source walk admitted atomically, so
/// all staged rows enter; ownership and uniqueness are
/// still checked against the claim's owner.
pub(super) fn merge_staged_local_field_reads(
    destination: &mut BTreeMap<OwnedExprSiteV1, LocalFieldRead>,
    owner: FunctionOwnerIdV1,
    staged: BTreeMap<OwnedExprSiteV1, LocalFieldRead>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for (site, row) in &staged {
        if site.owner() != owner || row.receiver.owner() != owner || row.home.owner() != owner {
            return Err(OrdinaryNewCoSealIssueV1::FieldReadOwnerMismatch { site: site.clone() });
        }
        if destination.contains_key(site) {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
    }
    destination.extend(staged);
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    /// Take the staged local-initializer read for the exact `FieldAccess`
    /// initializer site once. The caller resolves the staged receiver
    /// binding to the live base value; the sealed result class returns
    /// with the field so the emitter can type the destination from the
    /// claim rather than a layout guess.
    pub(crate) fn take_local_field_read(
        &self,
        site: &OwnedExprSiteV1,
        resolve_receiver: impl FnOnce(BindingRefV1) -> Result<ValueId, String>,
    ) -> Result<
        Option<(
            ValueId,
            CanonicalFieldRefV1,
            crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
        )>,
        String,
    > {
        let mut reads = self.local_field_reads.borrow_mut();
        let Some(row) = reads.get_mut(site) else {
            return Ok(None);
        };
        if row.receiver.owner() != site.owner() || row.home.owner() != site.owner() {
            return Err(fault("foreign-binding"));
        }
        let Some((last, parent)) = row.receiver_site.node().segments().split_last() else {
            return Err(fault("receiver-source-site"));
        };
        if *last != SourcePathSegmentV1::Receiver || parent != site.site().node().segments() {
            return Err(fault("receiver-source-site"));
        }
        if !matches!(row.progress, Progress::Pending) {
            return Err(fault("already-taken"));
        }
        let base = resolve_receiver(row.receiver)?;
        row.progress = Progress::Taken(base);
        Ok(Some((base, row.field, row.result.clone())))
    }

    /// Test/diagnostic view of one staged local-initializer read — the
    /// sealed canonical field reference and declared result class at the
    /// exact `FieldAccess` initializer site.
    #[cfg(test)]
    pub(crate) fn staged_local_field_read(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<(
        CanonicalFieldRefV1,
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
    )> {
        self.local_field_reads
            .borrow()
            .get(site)
            .map(|row| (row.field, row.result.clone()))
    }

    pub(crate) fn record_local_field_read(
        &self,
        site: &OwnedExprSiteV1,
        block: BasicBlockId,
        dst: ValueId,
        base: ValueId,
        field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let mut reads = self.local_field_reads.borrow_mut();
        let row = reads
            .get_mut(site)
            .ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(row.progress, Progress::Taken(expected) if expected == base)
            || row.field != field
        {
            return Err(fault("emission-mismatch"));
        }
        row.progress =
            Progress::Emitted(block, MirInstruction::ObjectFieldGet { dst, base, field });
        Ok(())
    }

    pub(crate) fn take_terminal_field_read(
        &self,
        site: &OwnedExprSiteV1,
        resolve_receiver: impl FnOnce(BindingRefV1) -> Result<ValueId, String>,
    ) -> Result<Option<(ValueId, CanonicalFieldRefV1)>, String> {
        let mut reads = self.field_reads.borrow_mut();
        let Some(row) = reads.get_mut(site) else {
            return Ok(None);
        };
        // The read belongs to the relation of its own exact exit site; the
        // exit row for that site — never a sibling exit's — must be in the
        // prepare window.
        let exit_site = self
            .terminal_relations_for_owner(site.owner())
            .into_iter()
            .find_map(|relation| match relation {
                TerminalRelationV1::I64Add(relation) if relation.field_reads().contains(site) => {
                    Some(relation.return_site().clone())
                }
                TerminalRelationV1::I64Field(relation) if relation.field_read_site() == site => {
                    Some(relation.return_site().clone())
                }
                _ => None,
            });
        let Some(exit_site) = exit_site else {
            return Err(fault("root-exit-phase"));
        };
        if !matches!(
            self.root_exits.borrow().get(&(site.owner(), exit_site)),
            Some(
                local_commit::RootHomeExitProgress::Prepared(_)
                    | local_commit::RootHomeExitProgress::Unavailable,
            )
        ) {
            return Err(fault("root-exit-phase"));
        }
        if row.receiver.owner() != site.owner() || row.home.owner() != site.owner() {
            return Err(fault("foreign-binding"));
        }
        let Some((last, parent)) = row.receiver_site.node().segments().split_last() else {
            return Err(fault("receiver-source-site"));
        };
        if *last != SourcePathSegmentV1::Receiver || parent != site.site().node().segments() {
            return Err(fault("receiver-source-site"));
        }
        // `installs` — not `installs_ordinary`: a non-null-narrowed
        // received nullable is a `CallReceived` row, not an `Ordinary`
        // `new` Home, and it still names a physically materialized base.
        // A borrowed formal needs no local commit: the co-sealed object
        // view already proves its physical base is the incoming tagged
        // param, and the physical verifier checks the emitted read's base
        // is that exact formal/copy root in the non-null cone.
        if !self
            .local_commits
            .borrow()
            .values()
            .any(|local| local.installs(row.home))
            && self
                .borrowed_formal_source
                .as_ref()
                .and_then(|source| source.as_ref().ok())
                .and_then(|source| source.formal_object_view(row.home))
                .is_none()
        {
            return Err(fault("home-not-installed"));
        }
        if !matches!(row.progress, Progress::Pending) {
            return Err(fault("already-taken"));
        }
        let base = resolve_receiver(row.receiver)?;
        row.progress = Progress::Taken(base);
        Ok(Some((base, row.field)))
    }

    /// Take the staged argument-position read for the exact `FieldAccess`
    /// argument site once. The argument row's receiver binding must match
    /// the staged object — a foreign or mismatched binding faults, never
    /// silently substitutes.
    pub(crate) fn take_argument_field_read(
        &self,
        site: &OwnedExprSiteV1,
        object: BindingRefV1,
        resolve_receiver: impl FnOnce(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<(ValueId, CanonicalFieldRefV1), String> {
        let mut reads = self.argument_field_reads.borrow_mut();
        let row = reads
            .get_mut(site)
            .ok_or_else(|| fault("missing-source-site"))?;
        if row.object != object || row.object.owner() != site.owner() {
            return Err(fault("foreign-binding"));
        }
        let Some((last, parent)) = row.receiver_site.node().segments().split_last() else {
            return Err(fault("receiver-source-site"));
        };
        if *last != SourcePathSegmentV1::Receiver || parent != site.site().node().segments() {
            return Err(fault("receiver-source-site"));
        }
        if !matches!(row.progress, Progress::Pending) {
            return Err(fault("already-taken"));
        }
        let base = resolve_receiver(row.object, row.receiver_site.node())?;
        row.progress = Progress::Taken(base);
        Ok((base, row.field))
    }

    /// Retire the staged argument reads owned by one `new` site whose
    /// claim just declined to `RetainedUnavailable`. Staged read keys are
    /// the `FieldAccess` argument sites — exactly the claim site's direct
    /// `Argument(n)` children in the same owner. Only `Pending` rows may
    /// release; `Taken`/`Emitted` state here is a contract violation.
    pub(crate) fn release_staged_argument_reads(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<(), String> {
        let parent = site.site().node().segments();
        let mut reads = self.argument_field_reads.borrow_mut();
        for (read_site, row) in reads.iter_mut() {
            if read_site.owner() != site.owner() {
                continue;
            }
            let Some((last, path)) = read_site.site().node().segments().split_last() else {
                continue;
            };
            if !matches!(last, SourcePathSegmentV1::Argument(_)) || path != parent {
                continue;
            }
            if !matches!(row.progress, Progress::Pending) {
                return Err(fault("release-non-pending"));
            }
            row.progress = Progress::Released;
        }
        Ok(())
    }

    pub(crate) fn record_argument_field_read(
        &self,
        site: &OwnedExprSiteV1,
        block: BasicBlockId,
        dst: ValueId,
        base: ValueId,
        field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let mut reads = self.argument_field_reads.borrow_mut();
        let row = reads
            .get_mut(site)
            .ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(row.progress, Progress::Taken(expected) if expected == base)
            || row.field != field
        {
            return Err(fault("emission-mismatch"));
        }
        row.progress =
            Progress::Emitted(block, MirInstruction::ObjectFieldGet { dst, base, field });
        Ok(())
    }

    pub(crate) fn record_terminal_field_read(
        &self,
        site: &OwnedExprSiteV1,
        block: BasicBlockId,
        dst: ValueId,
        base: ValueId,
        field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let mut reads = self.field_reads.borrow_mut();
        let row = reads
            .get_mut(site)
            .ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(row.progress, Progress::Taken(expected) if expected == base)
            || row.field != field
        {
            return Err(fault("emission-mismatch"));
        }
        row.progress =
            Progress::Emitted(block, MirInstruction::ObjectFieldGet { dst, base, field });
        Ok(())
    }

    pub(super) fn field_reads_complete(&self) -> bool {
        self.field_reads
            .borrow()
            .values()
            .all(|row| matches!(row.progress, Progress::Emitted(..)))
            && self
                .argument_field_reads
                .borrow()
                .values()
                .all(|row| matches!(row.progress, Progress::Emitted(..) | Progress::Released))
            && self
                .local_field_reads
                .borrow()
                .values()
                .all(|row| matches!(row.progress, Progress::Emitted(..)))
    }

    pub(super) fn validate_field_reads(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let reads = self.field_reads.borrow();
        let mut expected = Vec::new();
        let stored_receivers = self.stored_terminal_receiver_reads_v1(owner)?;
        expected.extend(
            stored_receivers
                .iter()
                .map(|(block, instruction)| (*block, instruction)),
        );
        for (site, row) in reads.iter() {
            if site.owner() != owner {
                continue;
            }
            let Progress::Emitted(block, instruction) = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            expected.push((*block, instruction));
        }
        let argument_reads = self.argument_field_reads.borrow();
        for (site, row) in argument_reads.iter() {
            if site.owner() != owner || matches!(row.progress, Progress::Released) {
                continue;
            }
            let Progress::Emitted(block, instruction) = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            expected.push((*block, instruction));
        }
        let local_reads = self.local_field_reads.borrow();
        for (site, row) in local_reads.iter() {
            if site.owner() != owner {
                continue;
            }
            let Progress::Emitted(block, instruction) = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            expected.push((*block, instruction));
        }
        for block in function.blocks.values() {
            for actual in block
                .all_instructions()
                .filter(|i| matches!(i, MirInstruction::ObjectFieldGet { .. }))
            {
                // Finishing may contract the read's block into a surviving
                // neighbour; dst/base/field identity is the provenance, the
                // boundary's sequence check owns positional drift.
                let index = expected
                    .iter()
                    .position(|(_, inst)| *inst == actual)
                    .ok_or_else(|| fault("unowned-or-drifted-read"))?;
                expected.swap_remove(index);
            }
        }
        if !expected.is_empty() {
            return Err(fault("missing-emission"));
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-field-read/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_field_reads_tests.rs"]
mod tests;
