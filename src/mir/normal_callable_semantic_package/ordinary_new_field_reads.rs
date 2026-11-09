//! Realization of exact terminal field reads in the existing New ledger.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::{BTreeMap, BTreeSet};

#[path = "ordinary_new_field_reads_shared.rs"]
mod shared;
#[path = "ordinary_new_result_field_path_progress.rs"]
mod result_path;
#[path = "ordinary_new_expression_field_path_source.rs"]
mod path_source;
use path_source::FieldPathSourceV1;
pub(in crate::mir::normal_callable_semantic_package) use path_source::StatementFieldPathSourceV1;
#[path = "ordinary_new_field_alias_lifetime.rs"]
mod field_alias;
#[path = "ordinary_new_borrowed_entry_field_reads.rs"]
mod borrowed_entry;

#[path = "ordinary_new_owned_ready_field_source.rs"]
mod owned_ready;
pub(super) use owned_ready::OwnedReadyFieldReadSourceV1;

#[path = "ordinary_new_received_field_source.rs"]
mod received;
#[path = "ordinary_new_checked_field_read.rs"]
mod checked;
#[path = "ordinary_new_checked_field_cleanup.rs"]
mod checked_cleanup;
use checked_cleanup::CheckedFieldReadCleanupV1;
#[path = "ordinary_new_returned_field_read.rs"]
mod returned;
pub(in crate::mir::normal_callable_semantic_package) use received::{ReceivedFieldReadSourceV1, ReceivedFieldContextV1};


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

#[path = "ordinary_new_field_read_source.rs"]
mod descriptor;
pub(crate) use descriptor::LocalFieldReadSourceV1;

/// One original local/expression read. The source descriptor stays on the same
/// row; affine progress retains the exact descriptor received by each consumer.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct LocalFieldRead {
    pub(super) local: Option<field_alias::FieldAliasLocalV1>,
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: BindingRefV1,
    pub(super) home: BindingRefV1,
    pub(super) source: Rc<LocalFieldReadSourceV1>,
    pub(super) numeric_source: Option<super::lexical_instance_call::ExpressionFieldSourceLoanV1>,
    pub(super) progress: LocalReadProgress,
}

#[derive(Debug)]
pub(super) enum LocalReadProgress {
    Pending,
    SourceReceiverPending { root: ValueId },
    Taken { base: ValueId, source: Rc<LocalFieldReadSourceV1> },
    Emitted { block: BasicBlockId, instruction: MirInstruction, source: Rc<LocalFieldReadSourceV1>, read: Option<Rc<crate::mir::builder::ExactLexicalReadV1>> },
    CheckedEmitted {
        source: Rc<LocalFieldReadSourceV1>,
        invoke: (BasicBlockId, MirInstruction), normal: (BasicBlockId, MirInstruction),
        read: Option<Rc<crate::mir::builder::ExactLexicalReadV1>>,
        cleanup: CheckedFieldReadCleanupV1,
    },
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
    /// The same Return field cannot be taken for a sibling exit or after its
    /// prepare window. Non-Return expression roles retain their own owner.
    pub(super) fn require_field_return_phase_v1(&self, site: &OwnedExprSiteV1) -> Result<(), String> {
        use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
            expression_root::StatementExpressionRoleV1};
        let Some(required) = self.source_field_requirement_v1(site)? else { return Ok(()); };
        let ExpressionPlanRootV1::Statement { statement, role: StatementExpressionRoleV1::ReturnValue, .. }
            = required.plan().root() else { return Ok(()); };
        let ready = self.root_exits.borrow().get(&(site.owner(), statement.clone().into()))
            .map(|state| matches!(state, local_commit::RootHomeExitProgress::Prepared(_)
                | local_commit::RootHomeExitProgress::Unavailable));
        if !match ready {
            Some(ready) => ready,
            None => self.root_home_exit_not_selected_v1(site.owner(), statement.node())?,
        } { return Err(fault("root-exit-phase")); }
        Ok(())
    }
    /// Take the staged local-initializer read for the exact `FieldAccess`
    /// initializer site once. The caller resolves the staged receiver
    /// binding to the live base value; the sealed result class returns
    /// with the field so the emitter can type the destination from the
    /// claim rather than a layout guess.
    pub(crate) fn take_local_field_read(
        &self,
        site: &OwnedExprSiteV1,
        resolve_receiver: impl FnOnce(BindingRefV1) -> Result<ValueId, String>,
    ) -> Result<Option<(ValueId, Rc<LocalFieldReadSourceV1>)>, String> {
        self.require_field_return_phase_v1(site)?;
        let reads = self.local_field_reads.borrow();
        let Some(row) = reads.get(site) else {
            if self.source_field_requirement_v1(site)?.is_some() { return Err(fault("required-source-read-missing")); }
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
        if !matches!(row.progress, LocalReadProgress::Pending) {
            return Err(fault("already-taken"));
        }
        self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
        if row.source.expression_path.is_some() { return Err(fault("result-path/direct-take")); }
        if self.source_field_requirement_v1(site)?.is_some_and(|required| matches!(required.plan().root(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
                role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue, .. }))
            && (row.source.owned_ready.is_some() || row.numeric_source.as_ref().is_some_and(|loan| loan.requires_installed_home_v1()))
            && !self.local_commits.borrow().values().any(|local| local.installs(row.home))
        {
            return Err(fault("home-not-installed"));
        }
        let receiver = row.receiver;
        let home = row.home;
        let receiver_site = row.receiver_site.clone();
        let source = Rc::clone(&row.source);
        drop(reads);
        let base = resolve_receiver(receiver)?;
        let mut reads = self.local_field_reads.borrow_mut();
        let row = reads.get_mut(site).ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(row.progress, LocalReadProgress::Pending) || !Rc::ptr_eq(&row.source, &source)
            || row.receiver != receiver || row.home != home || row.receiver_site != receiver_site {
            return Err(fault("emission-mismatch"));
        }
        row.progress = LocalReadProgress::Taken { base, source: Rc::clone(&source) };
        Ok(Some((base, source)))
    }

    #[cfg(test)]
    pub(crate) fn staged_local_field_source(&self, site: &OwnedExprSiteV1) -> Option<Rc<LocalFieldReadSourceV1>> {
        self.local_field_reads.borrow().get(site).map(|row| Rc::clone(&row.source))
    }

    /// Existing diagnostic projection, without erasing the retained descriptor.
    #[cfg(test)]
    pub(crate) fn staged_local_field_read(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<(CanonicalFieldRefV1, crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1)> {
        self.staged_local_field_source(site).map(|source| (source.field(), source.result().clone()))
    }

    pub(crate) fn record_local_field_read(
        &self,
        site: &OwnedExprSiteV1,
        block: BasicBlockId,
        dst: ValueId,
        base: ValueId,
        field: CanonicalFieldRefV1,
        source: &Rc<LocalFieldReadSourceV1>,
    ) -> Result<(), String> {
        if source.field() != field { return Err(fault("emission-mismatch")); }
        self.record_local_field_read_binding(site, block,
            MirInstruction::ObjectFieldGet { dst, base, field }, source)
    }

    pub(crate) fn record_local_field_read_binding(
        &self, site: &OwnedExprSiteV1, block: BasicBlockId, instruction: MirInstruction,
        source: &Rc<LocalFieldReadSourceV1>,
    ) -> Result<(), String> {
        if !source.corroborates_instruction(&instruction) { return Err(fault("source-emission-kind")); }
        let base = match &instruction {
            MirInstruction::ObjectFieldGet { base, .. } | MirInstruction::FieldGet { base, .. } => *base,
            _ => return Err(fault("source-emission-kind")),
        };
        let reads = self.local_field_reads.borrow();
        let row = reads.get(site).ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(&row.progress, LocalReadProgress::Taken { base: expected, source: taken }
            if *expected == base && Rc::ptr_eq(taken, source))
            || !Rc::ptr_eq(&row.source, source)
        {
            return Err(fault("emission-mismatch"));
        }
        self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
        drop(reads);
        let mut reads = self.local_field_reads.borrow_mut();
        let row = reads.get_mut(site).ok_or_else(|| fault("missing-source-site"))?;
        row.progress = LocalReadProgress::Emitted {
            block, instruction, source: Rc::clone(source), read: None,
        };
        Ok(())
    }

    pub(in crate::mir) fn retain_raw_field_receiver_v1(
        &self, site: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        exact: Rc<crate::mir::builder::ExactLexicalReadV1>,
    ) -> Result<(), String> {
        let mut rows = self.local_field_reads.borrow_mut();
        let row = rows.get_mut(site).ok_or_else(|| fault("missing-source-site"))?;
        let value = exact.value_for(site.owner(), row.receiver_site.node(), row.receiver)
            .map_err(|issue| issue.to_string())?;
        let LocalReadProgress::Emitted { instruction, source: emitted, read, .. } = &mut row.progress
            else { return Err(fault("source-emission-kind")); };
        if !Rc::ptr_eq(source, &row.source) || !Rc::ptr_eq(source, emitted) || read.is_some()
            || !source.corroborates_instruction(instruction)
            || !matches!(instruction, MirInstruction::ObjectFieldGet { base, .. } | MirInstruction::FieldGet { base, .. } if *base == value) {
            return Err(fault("source-descriptor-drift"));
        }
        *read = Some(exact);
        Ok(())
    }

    /// Lend only source-qualified field-resident Array reads. Emission and
    /// declaration must retain the same immutable source descriptor.
    pub(in crate::mir::normal_callable_semantic_package) fn visit_array_field_read_originals_v1(
        &self, owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(&OwnedExprSiteV1, &Rc<LocalFieldReadSourceV1>, &(BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        let rows = self.local_field_reads.borrow();
        for (site, row) in rows.iter() {
            if site.owner() != owner || !matches!(row.source.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(_)
                | crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::ArrayField(_)) {
                continue;
            }
            self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
            let LocalReadProgress::Emitted { block, instruction, source, .. } = &row.progress else {
                return Err(fault("array-read-unconsumed"));
            };
            if !Rc::ptr_eq(source, &row.source) || !source.corroborates_instruction(instruction) {
                return Err(fault("source-descriptor-drift"));
            }
            visit(site, source, &(*block, instruction.clone()))?;
        }
        Ok(())
    }

    /// Lend the exact source node's emitted read, preserving its qualified
    /// whole-expression loan. A same-typed sibling field is never a substitute.
    pub(in crate::mir::normal_callable_semantic_package) fn expression_field_read_emission_v1(
        &self, site: &OwnedExprSiteV1,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        let required = self.source_field_requirement_v1(site)?
            .ok_or_else(|| fault("expression-source-requirement-missing"))?;
        if !Rc::ptr_eq(required.plan(), plan) { return Err(fault("expression-source-plan")); }
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault("required-source-read-missing"))?;
        self.recheck_local_field_source_v1(site, row, context)?;
        let LocalReadProgress::Emitted { block, instruction, source, .. } = &row.progress else {
            return Err(fault("unconsumed-read"));
        };
        if !Rc::ptr_eq(source, &row.source) { return Err(fault("source-descriptor-drift")); }
        Ok((*block, instruction.clone()))
    }

    pub(in crate::mir) fn source_field_requirement_v1(&self, site: &OwnedExprSiteV1) -> Result<Option<&crate::mir::resolved_semantics::home_new_prefix::ExpressionFieldRequirementV1>, String> {
        let Some(flow) = self.completion_for_owner(site.owner()).and_then(|completion| completion.cleanup().root_flow())
            else { return Ok(None) };
        let mut fields = flow.source_expression_fields().filter(|field| field.request().site == *site);
        let field = fields.next();
        if fields.next().is_some() { return Err(fault("duplicate-source-requirement")); }
        Ok(field)
    }

    /// Source-use closure borrows the same complete declaration/path loan.
    /// No emitted field value or physical permission is issued here.
    pub(in crate::mir::normal_callable_semantic_package) fn check_original_expression_field_source_v1(
        &self, required: &crate::mir::resolved_semantics::home_new_prefix::ExpressionFieldRequirementV1,
    ) -> Result<(), String> {
        let request = required.request();
        let original = self.source_field_requirement_v1(&request.site)?
            .ok_or_else(|| fault("expression-source-requirement-missing"))?;
        if !std::ptr::eq(original, required) || request.node_in_plan(required.plan()).is_none() {
            return Err(fault("expression-source-plan"));
        }
        let rows = self.local_field_reads.borrow();
        let row = rows.get(&request.site).ok_or_else(|| fault("required-source-read-missing"))?;
        self.recheck_local_field_source_v1(&request.site, row, CallValidationContextV1::Lowering(self))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_local_field_source_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead, context: CallValidationContextV1<'_>) -> Result<(), String> {
        self.recheck_local_field_source_using_v1(site, row, context,
            || self.recheck_result_path_field_v1(site, row, context))
    }
    fn recheck_local_field_source_using_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead,
        context: CallValidationContextV1<'_>, recheck_path: impl FnOnce() -> Result<(), String>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        if let Some(returned) = &row.source.receiver_return {
            let required = self.source_field_requirement_v1(site)?.ok_or_else(|| fault("expression-source-requirement-missing"))?;
            if self.callable_result_classes.receiver_field_result_v1(required.request()).as_ref() != Some(returned) {
                return Err(fault("received/source-drift"));
            }
        }
        if row.source.shared_support.is_some() { return self.recheck_shared_field_read_v1(site, row); }
        if row.source.owned_ready.is_some() { return self.recheck_owned_local_read_v1(site, row, context); }
        if row.source.returned.is_some() { return self.recheck_returned_field_read_v1(site, row); }
        if row.source.expression_path.is_some() { return recheck_path(); }
        if let crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::ArrayField(source) = row.source.result() {
            return self.recheck_declared_array_read_v1(site, row, source);
        }
        let requirement = self.source_field_requirement_v1(site)?;
        if let Some(received) = &row.source.received {
            let request = requirement.ok_or_else(|| fault("received/expression-source-missing"))?.request();
            let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("received/ingress-missing"))?
                .as_ref().map_err(Clone::clone)?;
            for target in received.targets_v1() {
                context.with_taken_original_row_v1(target.call_site(), |original|
                    received.recheck_issuing_target_v1(&self.callable_result_classes, ingress, original.source_target(), request))?;
            }
        } else if requirement.is_some_and(|required| required.request().received.is_some()) {
            return Err(fault("received/source-missing"));
        }
        if let crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedObject(loan) = row.source.result() {
            let required = requirement.ok_or_else(|| fault("required-source-loan-missing"))?;
            let request = required.request();
            if row.numeric_source.is_some()
                || !crate::mir::declared_type_storage::declared_object_field_matches_v1(
                    row.source.declared_type_name(), loan.declared_v1().then_some(loan.child_class()))
                || loan.field() != row.source.field() || !loan.recheck_v1(&self.callable_result_classes, &request.field)
                || !(row.source.receiver_return.is_some() || request.is_nullable_object_comparison_v1(required.plan()))
                || row.receiver_site != request.receiver_site || row.receiver != request.receiver || row.home != request.home {
                return Err(fault("required-source-read-drift"));
            }
            return Ok(());
        }
        if let crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Text(loan) = row.source.result() {
            let required = requirement.ok_or_else(|| fault("required-source-loan-missing"))?;
            let request = required.request();
            let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("required-source-loan-missing"))?
                .as_ref().map_err(Clone::clone)?;
            if row.numeric_source.is_some() || !super::TextFieldValueSourceV1::declaration_admits_v1(row.source.declared_type_name())
                || loan.field() != row.source.field() || !loan.recheck(ingress, &self.field_write_claims, &request.field)
                || request.path.is_some() || request.received.is_some() || request.owned_ready.is_some()
                || !super::TextFieldValueSourceV1::role_admits_v1(required.plan(), site.site())
                || row.receiver_site != request.receiver_site || row.receiver != request.receiver || row.home != request.home {
                return Err(fault("required-source-read-drift"));
            }
            return Ok(());
        }
        let source = match (requirement, &row.numeric_source) {
            (None, None) => return Ok(()),
            (None, Some(_)) => return Err(fault("orphan-source-read")),
            (Some(_), None) => return Err(fault("required-source-loan-missing")),
            (Some(required), Some(source)) => {
                let request = required.request();
                if !source.matches_plan(required.plan()) || row.receiver_site != request.receiver_site
                    || row.receiver != request.receiver || row.home != request.home
                { return Err(fault("required-source-read-drift")); }
                source.recheck_declared_root_v1(self, request, context)?;
                source
            },
        };
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("integer-source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        source.recheck_read(&self.borrowed_i64_results, ingress, site,
            &row.receiver_site, row.receiver, row.home, &row.source)
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



    pub(super) fn field_reads_complete(&self) -> bool {
        self.argument_field_reads
                .borrow()
                .values()
                .all(|row| matches!(row.progress, Progress::Emitted(..) | Progress::Released))
            && self
                .local_field_reads
                .borrow()
                .values()
                .all(|row| matches!(row.progress, LocalReadProgress::Emitted { .. } | LocalReadProgress::CheckedEmitted { .. }))
    }


}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-field-read/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_field_reads_tests.rs"]
mod tests;

#[path = "ordinary_new_field_array_read_recipes.rs"]
mod field_array_recipes;
