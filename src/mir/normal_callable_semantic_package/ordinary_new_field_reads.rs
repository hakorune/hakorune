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
pub(super) use owned_ready::{OwnedReadyFieldReadSourceV1, TerminalFieldValueSourceV1};

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

#[derive(Debug)]
pub(super) struct FieldRead {
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: BindingRefV1,
    pub(super) home: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) value_source: TerminalFieldValueSourceV1,
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

/// Immutable declaration descriptor, shared by the original source row and
/// its exact take/emission. Metadata is not a numeric producer/value proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalFieldReadSourceV1 {
    field: CanonicalFieldRefV1,
    result: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
    declared_type_name: Option<Box<str>>,
    expression_path: Option<FieldPathSourceV1>,
    received: Option<ReceivedFieldReadSourceV1>,
    returned: Option<returned::ReturnedFieldArgumentSourceV1>,
    owned_ready: Option<OwnedReadyFieldReadSourceV1>,
}
impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_declaration(
        field: CanonicalFieldRefV1,
        result: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
        declared_type_name: Option<Box<str>>,
    ) -> Rc<Self> {
        Rc::new(Self { field, result, declared_type_name, expression_path: None, received: None, returned: None, owned_ready: None })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn with_received(
        original: &Rc<Self>, received: ReceivedFieldReadSourceV1,
    ) -> Result<Rc<Self>, String> {
        if original.received.is_some() || original.expression_path.is_some() || original.returned.is_some() || original.owned_ready.is_some()
            || original.result != crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar
            || received.mixed().is_some_and(|loan| loan.object() != original.field.object()) {
            return Err(fault("received/field-declaration"));
        }
        let mut source = original.as_ref().clone();
        source.received = Some(received);
        Ok(Rc::new(source))
    }
    pub(in crate::mir) fn requires_checked_field_read_v1(&self) -> bool {
        (self.declared_type_name.is_none()
            && self.result == crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar)
            || self.owned_ready.is_some() || self.received.is_some() || self.statement_path().is_some_and(|path| path.requires_checked_scalar_v1())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn received_root_source_v1(&self) -> Option<&ReceivedFieldReadSourceV1> {
        self.received.as_ref().or_else(|| self.statement_path().and_then(|path| path.received_root_v1()))
    }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn result(&self) -> &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1 {
        &self.result
    }
    pub(crate) fn declared_type_name(&self) -> Option<&str> { self.declared_type_name.as_deref() }
    fn corroborates_instruction(&self, instruction: &MirInstruction) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        if self.requires_checked_field_read_v1() { return false; }
        match (self.result(), instruction) {
            (LocalFieldReadResultV1::BorrowedArray(source), MirInstruction::FieldGet { field, declared_type: Some(crate::mir::MirType::Box(class)), .. }) =>
                source.requirement.field() == self.field() && source.requirement.field_name() == field && class == "ArrayBox",
            (LocalFieldReadResultV1::BorrowedArray(_), _) => false,
            (LocalFieldReadResultV1::ArrayField(source), MirInstruction::FieldGet { field, declared_type: Some(crate::mir::MirType::Box(class)), .. }) =>
                source.field() == self.field() && source.name() == field && class == "ArrayBox",
            (LocalFieldReadResultV1::ArrayField(_), _) => false,
            (_, MirInstruction::ObjectFieldGet { field, .. }) => *field == self.field(),
            _ => false,
        }
    }
}

/// One original local/expression read. The source descriptor stays on the same
/// row; affine progress retains the exact descriptor received by each consumer.
#[derive(Debug)]
pub(super) struct LocalFieldRead {
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
    Emitted { block: BasicBlockId, instruction: MirInstruction, source: Rc<LocalFieldReadSourceV1> },
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
    ) -> Result<Option<(ValueId, Rc<LocalFieldReadSourceV1>)>, String> {
        let mut reads = self.local_field_reads.borrow_mut();
        let Some(row) = reads.get_mut(site) else {
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
        let base = resolve_receiver(row.receiver)?;
        let source = Rc::clone(&row.source);
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
        let mut reads = self.local_field_reads.borrow_mut();
        let row = reads.get_mut(site).ok_or_else(|| fault("missing-source-site"))?;
        if !matches!(&row.progress, LocalReadProgress::Taken { base: expected, source: taken }
            if *expected == base && Rc::ptr_eq(taken, source))
            || !Rc::ptr_eq(&row.source, source)
        {
            return Err(fault("emission-mismatch"));
        }
        self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
        row.progress = LocalReadProgress::Emitted {
            block, instruction, source: Rc::clone(source),
        };
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
            let LocalReadProgress::Emitted { block, instruction, source } = &row.progress else {
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
        let LocalReadProgress::Emitted { block, instruction, source } = &row.progress else {
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

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_local_field_source_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead, context: CallValidationContextV1<'_>) -> Result<(), String> {
        self.recheck_local_field_source_using_v1(site, row, context,
            || self.recheck_result_path_field_v1(site, row, context))
    }
    fn recheck_local_field_source_using_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead,
        context: CallValidationContextV1<'_>, recheck_path: impl FnOnce() -> Result<(), String>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
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
            context.with_taken_original_row_v1(received.target().call_site(), |original|
                received.recheck_issuing_target_v1(&self.callable_result_classes, ingress, original.source_target(), request))?;
        } else if requirement.is_some_and(|required| required.request().received.is_some()) {
            return Err(fault("received/source-missing"));
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
                source
            },
        };
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("integer-source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        source.recheck_read(&self.borrowed_i64_results, ingress, site,
            &row.receiver_site, row.receiver, row.home, &row.source)
    }

    fn recheck_integer_field_source_v1(&self, source: &super::lexical_instance_call::IntegerFieldSourceLoanV1,
        site: &OwnedExprSiteV1, row: &FieldRead) -> Result<(), String> {
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("integer-source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        source.recheck(&self.borrowed_i64_results, ingress, site, &row.receiver_site, row.receiver, row.home, row.field)
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
            self.root_exits.borrow().get(&(site.owner(), exit_site.into())),
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
        self.recheck_terminal_field_value_v1(site, row, CallValidationContextV1::Lowering(self))?;
        // `installs` — not `installs_ordinary`: a non-null-narrowed
        // received nullable is a `CallReceived` row, not an `Ordinary`
        // `new` Home, and it still names a physically materialized base.
        // A borrowed formal needs no local commit: the co-sealed object
        // view already proves its physical base is the incoming tagged
        // param, and the physical verifier checks the emitted read's base
        // is that exact formal/copy root in the non-null cone.
        if !matches!(row.value_source, TerminalFieldValueSourceV1::Numeric(_)) && (!self
            .local_commits
            .borrow()
            .values()
            .any(|local| local.installs(row.home))
            && self
                .borrowed_formal_source
                .as_ref()
                .and_then(|source| source.as_ref().ok())
                .and_then(|source| source.formal_object_view(row.home))
                .is_none())
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
        self.recheck_terminal_field_value_v1(site, row, CallValidationContextV1::Lowering(self))?;
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
                .all(|row| matches!(row.progress, LocalReadProgress::Emitted { .. } | LocalReadProgress::CheckedEmitted { .. }))
    }

    pub(super) fn validate_field_reads(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_field_reads_with_projection_v1(owner, function, CallValidationContextV1::Lowering(self), |binding| Ok(Some(binding.clone())))
    }

    pub(super) fn validate_field_reads_with_projection_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction, context: CallValidationContextV1<'_>,
        mut project: impl FnMut(&(BasicBlockId, MirInstruction)) -> Result<Option<(BasicBlockId, MirInstruction)>, String>,
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
            self.recheck_terminal_field_value_v1(site, row, context)?;
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
        // One checked source loan lives only for this immutable validation.
        // Each row still rechecks its own path/descriptor and physical relation.
        let result = self.validate_result_path_fields_present_v1(owner, &local_reads)?;
        if let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) {
            for field in flow.source_expression_fields() {
                let site = &field.request().site;
                if site.owner() != owner { return Err(fault("foreign-source-requirement")); }
                let row = local_reads.get(site).ok_or_else(|| fault("required-source-read-missing"))?;
                self.recheck_local_field_source_using_v1(site, row, context, ||
                    self.recheck_result_path_field_using_result_v1(site, row, context, result.as_ref()))?;
            }
        }
        for (site, row) in local_reads.iter() {
            if site.owner() != owner { continue; }
            self.recheck_local_field_source_using_v1(site, row, context, ||
                self.recheck_result_path_field_using_result_v1(site, row, context, result.as_ref()))?;
            if matches!(row.progress, LocalReadProgress::CheckedEmitted { .. }) { continue; }
            let LocalReadProgress::Emitted { block, instruction, source } = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            if !Rc::ptr_eq(&row.source, source)
                || !source.corroborates_instruction(instruction)
            {
                return Err(fault("source-descriptor-drift"));
            }
            if let Some(local) = &row.local { local.validate_function(function, row)?; }
            expected.push((*block, instruction));
        }
        let array_destinations: BTreeSet<_> = expected.iter().filter_map(|(_, instruction)| match instruction {
            MirInstruction::FieldGet { dst, .. } => Some(*dst), _ => None,
        }).collect();
        for block in function.blocks.values() {
            for actual in block
                .all_instructions()
                .filter(|i| matches!(i, MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. }))
            {
                // Finishing may contract the read's block into a surviving
                // neighbour; dst/base/field identity is the provenance, the
                // boundary's sequence check owns positional drift.
                if matches!(actual, MirInstruction::FieldGet { dst, .. } if !array_destinations.contains(dst)) { continue; }
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
        self.validate_owned_ready_reads_v1(owner, function, context, &mut project)?;
        self.validate_checked_field_census_v1(owner, function,
            context, project)
            .map(|_| ())
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
