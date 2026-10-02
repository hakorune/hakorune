//! Physical completion of source-issued ordinary-New destinations.
//!
//! Target take, whole-expression completion and local installation are distinct.
//! This retains their exact relation; it does not issue Home availability or
//! claim that Fault cleanup is implemented.

use super::birth_abi_handoff::BirthAbiHandoffV1;
use super::OrdinaryNewClaimLedgerV1;
use super::{CallerNewHomePrefixV1, HomePrefixUnavailableV1, ResultNewHomePrefixV1};
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
use crate::mir::function::{RootOrdinaryNewObservation, RootOrdinaryNewUnavailable};
use crate::mir::instruction::InvokeOperation;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallObservationV1, LocalCallResultClassV1, TerminalI64AddReturnV1,
    TerminalI64FieldReturnV1, TerminalIntegerLiteralReturnV1, TerminalRelationV1,
    TerminalReturnedSourceV1, TerminalUnitReturnV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceNodeSiteV1,
    SourceStmtSiteV1,
};
use crate::mir::ValueId;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use crate::parser::CallableDeclarationIdentityV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

use self::root_home::RootHomeExitEntry;

#[derive(Debug)]
pub(super) enum RootNewValidation {
    Unregistered,
    Pending(FunctionOwnerIdV1),
    Checked(FunctionOwnerIdV1, physical_boundary::PhysicalBoundary),
    FinishingChecked,
    ArtifactFinalized,
}

/// Physical validation retained for one selected ordinary child.  This is
/// request-local finishing state, not a source receipt or a second owner.
#[derive(Debug)]
pub(super) enum ChildPhysicalValidation {
    Checked {
        symbol: String,
        boundary: physical_boundary::PhysicalBoundary,
    },
    FinishingChecked,
}

#[path = "ordinary_new_local_commit/progress.rs"]
mod progress;
use progress::{EmittedLocalProgress, NewEmissionProgress, UnavailableLocalProgress};

/// Physical consumption of one already-issued selected-New argument row.
///
/// This is a finalizer-owned snapshot of an emitted MIR value.  It neither
/// issues source meaning nor selects an ABI lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EmittedNewArgumentV1 {
    source: super::OrdinaryNewTrivialArgumentV1,
    value: ValueId,
}

impl EmittedNewArgumentV1 {
    pub(crate) fn source(&self) -> &super::OrdinaryNewTrivialArgumentV1 {
        &self.source
    }
    pub(crate) fn value(&self) -> ValueId {
        self.value
    }
}

/// Existing checked emission retained per New, not deduplicated per definition.
/// A return-position `new` installs no binding, so `destination` is `None`
/// there; the site owner remains the only owner authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalizedBirthActualsV1 {
    site: OwnedExprSiteV1,
    destination: Option<BindingRefV1>,
    target: CanonicalSameModuleCallableKeyV1,
    receiver: ValueId,
    arguments: Box<[EmittedNewArgumentV1]>,
}

impl FinalizedBirthActualsV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn destination(&self) -> Option<BindingRefV1> {
        self.destination
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.destination.map_or_else(|| self.site.owner(), |binding| binding.owner())
    }
    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
    pub(crate) fn receiver(&self) -> ValueId {
        self.receiver
    }
    pub(crate) fn arguments(&self) -> &[EmittedNewArgumentV1] {
        &self.arguments
    }
}

/// A provider `new Child(args)` inside a Birth unit: the emitted checked
/// `new_box` + `birth_call` + `object_field_set` chain is recorded at
/// emission time because the store itself is the commit — there is no
/// claim row to finalize separately.
#[derive(Debug)]
pub(super) struct ProviderBirthRecordV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) object: hakorune_mir_defs::CanonicalObjectIdV1,
    pub(super) handoff: BirthAbiHandoffV1,
    pub(super) receiver: ValueId,
    pub(super) arguments: Box<[EmittedNewArgumentV1]>,
}

#[derive(Debug)]
pub(super) struct NewLocalCommitV1 {
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    construction: super::ConstructionEligibilityV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    destruction: super::ObjectDestructionDispositionV1,
    birth_target: Option<CanonicalSameModuleCallableKeyV1>,
    birth_abi: Option<BirthAbiHandoffV1>,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    argument_rows: Result<
        Box<[super::OrdinaryNewTrivialArgumentV1]>,
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    >,
    /// Sealed owned field children in declaration order for
    /// owned-field-disposition objects; `None` means the disposition does
    /// not apply or the proof failed — `end_available` rejects the latter.
    children: Option<Box<[super::OwnedFieldChildV1]>>,
    emission: NewEmissionProgress,
}

/// Destination-less commit row for a return-position `new` claim. The
/// emitted object transfers to the caller through `Return { value }` —
/// there is no binding, declaration, or local installation. The shared
/// `NewEmissionProgress` state machine applies; `Checked` marks the emitted
/// result value itself.
#[derive(Debug)]
pub(super) struct NewResultCommitV1 {
    site: OwnedExprSiteV1,
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    construction: super::ConstructionEligibilityV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    destruction: super::ObjectDestructionDispositionV1,
    birth_target: Option<CanonicalSameModuleCallableKeyV1>,
    birth_abi: Option<BirthAbiHandoffV1>,
    home_prefix: Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    argument_rows: Result<
        Box<[super::OrdinaryNewTrivialArgumentV1]>,
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    >,
    /// Same sealed residence plan as `NewLocalCommitV1` — the
    /// construction-fault reclaim releases these children before storage.
    children: Option<Box<[super::OwnedFieldChildV1]>>,
    emission: NewEmissionProgress,
}

#[path = "ordinary_new_local_commit/map.rs"]
mod map;
use map::MapLocalProgress;

#[path = "ordinary_new_local_commit/local_entry.rs"]
mod local_entry;
pub(super) use local_entry::LocalCommitV1;

/// Exact source relation retained after its matching physical root passed
/// final validation. This is transport only: it cannot select an entry ABI or
/// recreate source membership from a physical key.
#[derive(Debug)]
pub(crate) struct FinalizedRootSourceHandoffV1 {
    app_main_identity: CallableDeclarationIdentityV1,
    /// The root owner's retained terminal relations keyed by their exact
    /// source exit site. Multi-exit roots keep every row; no single relation
    /// is privileged and none may stand in for a sibling exit.
    terminals: std::collections::BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    // Existing physical Call payloads, moved from the owner/site-indexed
    // ledger at the finalization boundary. This is retention only; it does
    // not select an ABI or infer a target from emitted MIR.
    call_entries: std::collections::BTreeMap<
        SourceStmtSiteV1,
        (RootHomeExitEntry, Vec<(BasicBlockId, MirInstruction)>),
    >,
    // Original local prefix groups from every selected owner. The same
    // packet can be shared with sibling exit entries; its Taken rows are not copied.
    local_calls: std::collections::BTreeMap<FunctionOwnerIdV1, Vec<RootLocalCallBindingGroupV1>>,
}

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn local_call_binding_groups(
        &self,
    ) -> impl Iterator<Item = (FunctionOwnerIdV1, &RootLocalCallBindingGroupV1)> {
        self.local_calls.iter().flat_map(|(owner, groups)| {
            groups.iter().map(move |group| (*owner, group))
        })
    }

    fn sole_terminal(&self) -> Option<&TerminalRelationV1> {
        (self.terminals.len() == 1)
            .then(|| self.terminals.values().next())
            .flatten()
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.terminals
            .values()
            .next()
            .map(TerminalRelationV1::owner)
            .expect("root handoff retains at least one exit relation")
    }

    /// Derived at this result boundary; never retained as a second source
    /// tag. Every exit must project to the same physical result class —
    /// divergent exit kinds name no single ABI and yield `None`.
    pub(crate) fn result_abi(&self) -> Option<FinalizedRootResultAbiV1> {
        let mut abi = None;
        for terminal in self.terminals.values() {
            let projected = match terminal {
                TerminalRelationV1::Call(row) => {
                    FinalizedRootResultAbiV1::CallReturn { owner: row.owner() }
                }
                TerminalRelationV1::I64Add(row) => {
                    FinalizedRootResultAbiV1::I64AddReturn { owner: row.owner() }
                }
                TerminalRelationV1::Unit(row) => {
                    FinalizedRootResultAbiV1::UnitReturn { owner: row.owner() }
                }
                TerminalRelationV1::IntegerLiteral(row) => {
                    FinalizedRootResultAbiV1::IntegerLiteralReturn { owner: row.owner() }
                }
                TerminalRelationV1::I64Field(row) => {
                    FinalizedRootResultAbiV1::I64FieldReturn { owner: row.owner() }
                }
                // A non-i64 value return derives no physical result ABI at
                // this boundary; the lifecycle capability lane supplies it.
                TerminalRelationV1::Value(_) => return None,
                // An opaque call return proves no result class at all.
                TerminalRelationV1::OpaqueCall(_) => return None,
                TerminalRelationV1::MapGet(row) => {
                    FinalizedRootResultAbiV1::MapGetReturn { owner: row.owner() }
                }
            };
            match abi {
                None => abi = Some(projected),
                Some(existing) if existing == projected => {}
                Some(_) => return None,
            }
        }
        abi
    }

    pub(crate) fn app_main_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.app_main_identity
    }

    /// The sole Call payload — `None` when zero or several exits carry Call
    /// entries. Callers that need a specific exit must index
    /// `call_entries()` by its site instead.
    pub(crate) fn call_entry(&self) -> Option<&RootHomeExitEntry> {
        (self.call_entries.len() == 1)
            .then(|| self.call_entries.values().next().map(|(entry, _)| entry))
            .flatten()
    }

    pub(crate) fn call_entries(
        &self,
    ) -> impl Iterator<
        Item = (
            &SourceStmtSiteV1,
            &RootHomeExitEntry,
            &[(BasicBlockId, MirInstruction)],
        ),
    > {
        self.call_entries
            .iter()
            .map(|(site, (entry, cleanup))| (site, entry, cleanup.as_slice()))
    }

    /// Cleanup bindings of the sole Call payload. `None` for zero or
    /// several Call exits — never a silently picked row.
    pub(crate) fn call_cleanup(&self) -> Option<&[(BasicBlockId, MirInstruction)]> {
        (self.call_entries.len() == 1)
            .then(|| {
                self.call_entries
                    .values()
                    .next()
                    .map(|(_, cleanup)| cleanup.as_slice())
            })
            .flatten()
    }

    pub(crate) fn terminal_i64_add(&self) -> Option<&TerminalI64AddReturnV1> {
        match self.sole_terminal() {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        match self.sole_terminal() {
            Some(TerminalRelationV1::Unit(row)) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_integer_literal(&self) -> Option<&TerminalIntegerLiteralReturnV1> {
        match self.sole_terminal() {
            Some(TerminalRelationV1::IntegerLiteral(row)) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_i64_field_return(&self) -> Option<&TerminalI64FieldReturnV1> {
        match self.sole_terminal() {
            Some(TerminalRelationV1::I64Field(row)) => Some(row),
            _ => None,
        }
    }
}

/// Final-handoff projection of the already-issued terminal source relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalizedRootResultAbiV1 {
    CallReturn {
        owner: FunctionOwnerIdV1,
    },
    I64AddReturn {
        owner: FunctionOwnerIdV1,
    },
    UnitReturn {
        owner: FunctionOwnerIdV1,
    },
    IntegerLiteralReturn {
        owner: FunctionOwnerIdV1,
    },
    I64FieldReturn {
        owner: FunctionOwnerIdV1,
    },
    /// `return <map>.get("<literal>")` — the readable-Map terminal. The
    /// checked read produces the exact i64 payload; the map itself is
    /// never the returned value.
    MapGetReturn {
        owner: FunctionOwnerIdV1,
    },
}

impl NewLocalCommitV1 {
    pub(super) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(super) fn construction(&self) -> &super::ConstructionEligibilityV1 {
        &self.construction
    }

    pub(super) fn pending(
        binding: BindingRefV1,
        declaration: SourceBindingSiteV1,
        home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
        box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
        construction: super::ConstructionEligibilityV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        destruction: super::ObjectDestructionDispositionV1,
        birth_target: Option<CanonicalSameModuleCallableKeyV1>,
        birth_abi: Option<BirthAbiHandoffV1>,
        argument_rows: Result<
            Box<[super::OrdinaryNewTrivialArgumentV1]>,
            crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
        >,
        children: Option<Box<[super::OwnedFieldChildV1]>>,
    ) -> Self {
        Self {
            box_source,
            construction,
            object,
            destruction,
            birth_target,
            birth_abi,
            binding,
            declaration,
            home_prefix,
            argument_rows,
            children,
            emission: NewEmissionProgress::Unprepared,
        }
    }

    pub(super) fn is_complete(&self) -> bool {
        self.emission.is_complete()
    }

    pub(super) fn installs(&self, binding: BindingRefV1) -> bool {
        self.binding == binding
            && self.emission.local().is_some()
            && matches!(&self.home_prefix, Ok(prefix) if prefix.destination() == binding)
    }

    fn at_statement(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> bool {
        self.binding.owner() == owner
            && matches!(&self.declaration,
            SourceBindingSiteV1::Local { statement, .. } if statement.node() == site)
    }
}

impl NewResultCommitV1 {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn pending(
        site: OwnedExprSiteV1,
        home_prefix: Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
        box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
        construction: super::ConstructionEligibilityV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        destruction: super::ObjectDestructionDispositionV1,
        birth_target: Option<CanonicalSameModuleCallableKeyV1>,
        birth_abi: Option<BirthAbiHandoffV1>,
        argument_rows: Result<
            Box<[super::OrdinaryNewTrivialArgumentV1]>,
            crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
        >,
        children: Option<Box<[super::OwnedFieldChildV1]>>,
    ) -> Self {
        Self {
            site,
            box_source,
            construction,
            object,
            destruction,
            birth_target,
            birth_abi,
            home_prefix,
            argument_rows,
            children,
            emission: NewEmissionProgress::Unprepared,
        }
    }

    pub(super) const fn object(&self) -> hakorune_mir_defs::CanonicalObjectIdV1 {
        self.object
    }

    #[cfg(test)]
    pub(super) fn construction(&self) -> &super::ConstructionEligibilityV1 {
        &self.construction
    }

    pub(super) fn is_complete(&self) -> bool {
        match &self.emission {
            // A retained result-position row installs no local: once the raw
            // lane records the emitted expression value, the row has reached
            // its terminal state and downstream coverage still owns the
            // unowned-lifecycle rejection.
            NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::ExpressionCompleted { .. },
            } => true,
            _ => self.emission.is_complete(),
        }
    }
}

// One lookup over installed physical bindings; source order remains caller-owned.
#[derive(Debug)]
pub(super) enum HomeLookupError {
    Missing,
    Duplicate,
}

pub(super) fn installed_home(
    rows: &std::collections::BTreeMap<OwnedExprSiteV1, LocalCommitV1>,
    binding: BindingRefV1,
) -> Result<&LocalCommitV1, HomeLookupError> {
    let mut candidates = rows.values().filter(|row| row.installs(binding));
    let row = candidates.next().ok_or(HomeLookupError::Missing)?;
    if candidates.next().is_some() {
        return Err(HomeLookupError::Duplicate);
    }
    Ok(row)
}

impl NewLocalCommitV1 {
    /// Teardown plan for this home in execution order: owned field
    /// children release in reverse declaration order — zero-initialized
    /// slots make the `if-live` check discharge only born children — then
    /// the object's own Home. `end_available` already gated the sealed
    /// residence proof, so `children` is always `Some` on an owned-field
    /// disposition.
    fn end_plan(
        &self,
    ) -> Box<[(root_home::RootHomeReleaseSubjectV1, InvokeOperation)]> {
        let base = self.emission.local().expect("installed Home");
        let children = match (&self.children, self.destruction) {
            (Some(children), _) => children.as_ref(),
            // An owned teardown without the sealed residence proof can never
            // plan a plain release — `end_available` rejects it earlier.
            (
                None,
                super::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                | super::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook,
            ) => unreachable!("owned teardown without sealed residences"),
            (None, _) => &[],
        };
        let children = children.iter().rev().map(|child| {
            (
                root_home::RootHomeReleaseSubjectV1::FieldResidence {
                    binding: self.binding,
                    field: child.field,
                },
                match child.kind {
                    super::OwnedFieldChildKindV1::Array => {
                        InvokeOperation::OwnedFieldResidenceRelease {
                            field: child.field,
                            base,
                        }
                    }
                    super::OwnedFieldChildKindV1::Object(child_object) => {
                        InvokeOperation::OwnedObjectFieldRelease {
                            field: child.field,
                            base,
                            child: child_object,
                        }
                    }
                },
            )
        });
        children
            .chain(std::iter::once((
                root_home::RootHomeReleaseSubjectV1::Binding(self.binding),
                InvokeOperation::HomeRelease {
                    object: self.object,
                    value: base,
                },
            )))
            .collect()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn register_new_root(&self, owner: FunctionOwnerIdV1) -> Result<(), String> {
        let mut state = self.root_validation.borrow_mut();
        if !matches!(*state, RootNewValidation::Unregistered) {
            return Err(freeze("duplicate-root-registration"));
        }
        if self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .is_some_and(|completion| completion.owner() != owner)
        {
            return Err(freeze("root-completion-owner-mismatch"));
        }
        *state = RootNewValidation::Pending(owner);
        Ok(())
    }

    fn finalized_root_observation(&self, owner: FunctionOwnerIdV1) -> RootOrdinaryNewObservation {
        use RootOrdinaryNewObservation::{
            NoSelectedLocalNew, SourceCompleteAtFinalization, Unavailable,
        };
        use RootOrdinaryNewUnavailable::*;
        let rows = self.local_commits.borrow();
        let mut selected = rows.values().filter(|row| row.owner() == owner).peekable();
        if selected.peek().is_none() {
            return NoSelectedLocalNew;
        }
        let completion = match &self.root_completion {
            None => return Unavailable(CompletionMissing),
            Some(Err(_)) => return Unavailable(CompletionRejected),
            Some(Ok(completion)) => completion,
        };
        if !completion
            .cleanup()
            .root_flow()
            .is_some_and(|flow| flow.all_exits_ready())
        {
            return Unavailable(TerminalHomesUnavailable);
        }
        if selected.any(|row| {
            let emission = match row {
                LocalCommitV1::Ordinary(row) => Some(&row.emission),
                LocalCommitV1::Result(row) => Some(&row.emission),
                LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_) => None,
            };
            emission.is_some_and(|emission| {
                matches!(emission, NewEmissionProgress::RetainedUnavailable { .. })
            })
        }) {
            return Unavailable(NewEmissionUnavailable);
        }
        // Every explicit exit keeps its own progress row; a single
        // `Unavailable` or missing row means the physical root is not
        // lifecycle-complete no matter how cleanly sibling exits emitted.
        let mut complete = false;
        for ((row_owner, _), progress) in self.root_exits.borrow().iter() {
            if *row_owner != owner {
                continue;
            }
            match progress {
                RootHomeExitProgress::Emitted { .. } => complete = true,
                RootHomeExitProgress::Unavailable => {
                    return Unavailable(RootExitUnavailable);
                }
                _ => unreachable!("final root validation rejects unconsumed exit"),
            }
        }
        match complete {
            true => SourceCompleteAtFinalization,
            false => unreachable!("final root validation rejects unconsumed exit"),
        }
    }

    /// The caller supplies exact BindingRefs from the existing callable state
    /// and values from the sole completed-local terminal. Validate the entire
    /// statement before committing any row, including ordinal and RHS identity.
    pub(crate) fn complete_local_installation(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &SourceNodeSiteV1,
        completed: &[(BindingRefV1, u32, ValueId, ValueId)],
    ) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        for (binding, ordinal, _, _) in completed {
            if binding.owner() != owner || !seen.insert(*ordinal) {
                return Err(freeze("foreign-or-duplicate-local"));
            }
        }
        if self.claims.borrow().values().any(|claim| {
            claim.destination.owner() == owner
                && matches!(&claim.declaration,
                SourceBindingSiteV1::Local { statement: expected, .. }
                    if expected.node() == statement)
        }) {
            return Err(freeze("local-before-target-take"));
        }
        let mut rows = self.local_commits.borrow_mut();
        let mut commits = Vec::new();
        for (site, row) in rows
            .iter()
            .filter(|(_, row)| row.at_statement(owner, statement))
        {
            let Some(SourceBindingSiteV1::Local { ordinal, .. }) = row.declaration() else {
                unreachable!("at_statement requires Local");
            };
            let (_, _, initializer, local) = completed
                .iter()
                .find(|(binding, index, _, _)| Some(*binding) == row.binding() && index == ordinal)
                .ok_or_else(|| freeze("local-binding-or-ordinal-mismatch"))?;
            if row.local().is_some() {
                return Err(freeze("duplicate-local-installation"));
            }
            if row.initializer() != Some(*initializer)
                || (matches!(row, LocalCommitV1::Ordinary(_)) && initializer == local)
                || (matches!(row, LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_))
                    && initializer != local)
            {
                return Err(freeze("local-initializer-mismatch"));
            }
            commits.push((site.clone(), *local));
        }
        for (site, local) in commits {
            rows.get_mut(&site)
                .expect("validated row remains present")
                .install(local);
        }
        Ok(())
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/local-commit/{reason}]")
}

#[path = "ordinary_new_local_commit/call_received.rs"]
mod call_received;
use call_received::{
    CallReceivedCommitV1, CallReceivedPhase, CallReceivedProgress, CallReceivedReleaseV1,
};
#[path = "ordinary_new_local_commit/emission_prepare.rs"]
mod emission_prepare;
#[path = "ordinary_new_local_commit/emission_validation.rs"]
mod emission_validation;
#[path = "ordinary_new_local_commit/handle_call.rs"]
mod handle_call;
#[path = "ordinary_new_local_commit/reclaim.rs"]
mod reclaim;
#[path = "ordinary_new_local_commit/root_cleanup_graph.rs"]
mod root_cleanup_graph;
#[path = "ordinary_new_local_commit/root_home.rs"]
mod root_home;
#[path = "ordinary_new_local_commit/root_validation.rs"]
mod root_validation;

use reclaim::ReclaimUnpublishedEmissionV1;
pub(crate) use reclaim::ReclaimUnpublishedOriginV1;
pub(super) use root_home::RootHomeExitProgress;

#[path = "ordinary_new_local_commit/finalized_root_handoff.rs"]
mod finalized_root_handoff;

#[path = "ordinary_new_local_commit/physical_boundary.rs"]
mod physical_boundary;

pub(in crate::mir) use root_home::{
    EmittedLexicalCallProjectionV1, LexicalCallArgumentProjectionV1,
    PreparedLexicalCallProjectionV1,
};

pub(crate) use root_home::RootLocalCallBindingGroupV1;
