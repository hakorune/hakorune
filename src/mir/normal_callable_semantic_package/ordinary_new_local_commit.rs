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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalizedBirthActualsV1 {
    site: OwnedExprSiteV1,
    destination: BindingRefV1,
    target: CanonicalSameModuleCallableKeyV1,
    receiver: ValueId,
    arguments: Box<[EmittedNewArgumentV1]>,
}

impl FinalizedBirthActualsV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
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
    terminal: TerminalRelationV1,
    // Existing physical Call payload, moved from the owner-indexed ledger at
    // the finalization boundary. This is retention only; it does not select
    // an ABI or infer a target from emitted MIR.
    call_entry: Option<RootHomeExitEntry>,
    call_cleanup: Box<[(BasicBlockId, MirInstruction)]>,
}

impl FinalizedRootSourceHandoffV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        match &self.terminal {
            TerminalRelationV1::Call(row) => row.owner(),
            TerminalRelationV1::I64Add(row) => row.owner(),
            TerminalRelationV1::Unit(row) => row.owner(),
            TerminalRelationV1::IntegerLiteral(row) => row.owner(),
            TerminalRelationV1::I64Field(row) => row.owner(),
            TerminalRelationV1::Value(row) => row.owner(),
            TerminalRelationV1::OpaqueCall(row) => row.owner(),
            TerminalRelationV1::MapGet(row) => row.owner(),
        }
    }

    /// Derived at this result boundary; never retained as a second source tag.
    pub(crate) fn result_abi(&self) -> Option<FinalizedRootResultAbiV1> {
        Some(match &self.terminal {
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
            // A non-i64 value return derives no physical result ABI at this
            // boundary; the lifecycle capability lane supplies it.
            TerminalRelationV1::Value(_) => return None,
            // An opaque call return proves no result class at all.
            TerminalRelationV1::OpaqueCall(_) => return None,
            TerminalRelationV1::MapGet(row) => {
                FinalizedRootResultAbiV1::MapGetReturn { owner: row.owner() }
            }
        })
    }

    pub(crate) fn app_main_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.app_main_identity
    }

    pub(crate) fn call_entry(&self) -> Option<&RootHomeExitEntry> {
        self.call_entry.as_ref()
    }

    pub(crate) fn call_cleanup(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.call_cleanup
    }

    pub(crate) fn terminal_i64_add(&self) -> Option<&TerminalI64AddReturnV1> {
        match &self.terminal {
            TerminalRelationV1::I64Add(row) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        match &self.terminal {
            TerminalRelationV1::Unit(row) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_integer_literal(&self) -> Option<&TerminalIntegerLiteralReturnV1> {
        match &self.terminal {
            TerminalRelationV1::IntegerLiteral(row) => Some(row),
            _ => None,
        }
    }
    pub(crate) fn terminal_i64_field_return(&self) -> Option<&TerminalI64FieldReturnV1> {
        match &self.terminal {
            TerminalRelationV1::I64Field(row) => Some(row),
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

/// Caller-side commit row for `local h = <call>` where the callee's sealed
/// terminal is `return new <class>`: the callee's canonical object changed
/// ownership at the Return edge, and the caller installs the receiving
/// local as an owned Home owing exactly one `HomeRelease`. The row keeps
/// the callee's `object` identity — one object, one owner at a time — but
/// this is a call-site acquisition, never a local `new` home.
#[derive(Debug)]
pub(super) struct CallReceivedCommitV1 {
    owner: FunctionOwnerIdV1,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    progress: CallReceivedProgress,
}
#[derive(Debug)]
enum CallReceivedProgress {
    Emitting,
    Emitted {
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        phase: CallReceivedPhase,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallReceivedPhase {
    ExpressionCompleted,
    Installed,
    Checked,
}
impl CallReceivedCommitV1 {
    /// The call's normal-result value before the local install — the
    /// initializer the receiving statement binds.
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self.progress {
            CallReceivedProgress::Emitted {
                result,
                phase: CallReceivedPhase::ExpressionCompleted,
                ..
            } => Some(result),
            _ => None,
        }
    }
    /// The received handle value once the local is installed — the same
    /// value the sole `HomeRelease` reads at the caller's terminal exit.
    pub(super) fn local(&self) -> Option<ValueId> {
        match self.progress {
            CallReceivedProgress::Emitted {
                result,
                phase: CallReceivedPhase::Installed | CallReceivedPhase::Checked,
                ..
            } => Some(result),
            _ => None,
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match &mut self.progress {
            CallReceivedProgress::Emitted {
                result,
                phase,
                ..
            } if *result == local && *phase == CallReceivedPhase::ExpressionCompleted => {
                *phase = CallReceivedPhase::Installed
            }
            _ => unreachable!("call-received local batch preflight"),
        }
    }
    pub(super) fn mark_checked(&mut self) {
        match &mut self.progress {
            CallReceivedProgress::Emitted { phase, .. } => {
                *phase = CallReceivedPhase::Checked
            }
            _ => unreachable!("call-received emission batch validation"),
        }
    }
    pub(super) fn is_complete(&self) -> bool {
        matches!(
            self.progress,
            CallReceivedProgress::Emitted {
                phase: CallReceivedPhase::Checked,
                ..
            }
        )
    }
    pub(super) fn end_operation(&self) -> InvokeOperation {
        InvokeOperation::HomeRelease {
            object: self.object,
            value: self.local().expect("installed received handle"),
        }
    }
    pub(super) fn checked_bindings(&self) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        match &self.progress {
            CallReceivedProgress::Emitted {
                bindings,
                phase: CallReceivedPhase::Checked,
                ..
            } => Ok(bindings),
            _ => Err(freeze("artifact-handle-unchecked")),
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
    fn end_operation(&self) -> InvokeOperation {
        InvokeOperation::HomeRelease {
            object: self.object,
            value: self.emission.local().expect("installed Home"),
        }
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
        if !matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
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
        match self.root_exits.borrow().get(&owner) {
            Some(RootHomeExitProgress::Emitted { .. }) => SourceCompleteAtFinalization,
            Some(RootHomeExitProgress::Unavailable) => Unavailable(RootExitUnavailable),
            _ => unreachable!("final root validation rejects unconsumed exit"),
        }
    }

    /// Shared prepared-state computation: ABI check, reclaim origin and
    /// prior-Home unwind operands. `prior_homes` carries the caller's
    /// resolved `(prior_homes, required_unwind)`; `None` marks a retained
    /// unavailable prefix.
    fn compute_emission_prepare(
        rows: &std::collections::BTreeMap<OwnedExprSiteV1, LocalCommitV1>,
        site: &OwnedExprSiteV1,
        arity: usize,
        constructor: &super::OrdinaryNewConstructorDispositionV1,
        construction: &super::ConstructionEligibilityV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        prior_homes: Option<(&[BindingRefV1], &OwnedExprSiteV1)>,
    ) -> Result<NewEmissionProgress, String> {
        if let super::OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
            let physical = arity
                .checked_add(1)
                .ok_or_else(|| freeze("arity-overflow"))?;
            recipe
                .abi()
                .validate(arity, physical)
                .map_err(|error| format!("[freeze:contract][ordinary-new/abi/{error:?}]"))?;
        }
        let reclaim = match (constructor, construction) {
            (super::OrdinaryNewConstructorDispositionV1::NoBirthZero, _) => None,
            (super::OrdinaryNewConstructorDispositionV1::Birth(_), Err(_)) => None,
            (super::OrdinaryNewConstructorDispositionV1::Birth(recipe), Ok(plan)) => {
                let (constructor_source, constructor_owner) = plan
                    .constructor()
                    .ok_or_else(|| freeze("reclaim-origin-constructor-missing"))?;
                if !plan.reclaims_unpublished_outer_storage()
                    || plan.object() != object
                    || !constructor_source.same_as(recipe.source_id())
                {
                    return Err(freeze("reclaim-origin-source-drift"));
                }
                Some(ReclaimUnpublishedOriginV1 {
                    site: site.clone(),
                    constructor_source: constructor_source.clone(),
                    constructor_owner: *constructor_owner,
                    object: plan.object(),
                })
            }
        };
        let mut operands = Vec::new();
        let mut available = construction.is_ok();
        match prior_homes {
            None => available = false,
            Some((prior_homes, required_unwind)) => {
                if required_unwind != site {
                    return Err(freeze("prepare-outward-site"));
                }
                for binding in prior_homes {
                    let prior = installed_home(rows, *binding).map_err(|error| match error {
                        HomeLookupError::Missing => freeze("prior-home-not-installed"),
                        HomeLookupError::Duplicate => freeze("duplicate-prior-home"),
                    })?;
                    available &= prior.end_available();
                    operands.push(prior.end_operation());
                }
            }
        }
        Ok(if available {
            NewEmissionProgress::Prepared { operands, reclaim }
        } else {
            NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::PendingExpression,
            }
        })
    }

    /// Select from retained source products before argument descent. No new
    /// source fact is issued here; prior Homes retain the issuer's order.
    pub(crate) fn prepare_new_emission(
        &self,
        claim: &super::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get(claim.site())
            .and_then(LocalCommitV1::ordinary)
            .ok_or_else(|| freeze("prepare-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Unprepared)
            || row.object != claim.object()
            || row.binding != claim.destination
            || !row.box_source.same_source_as(claim.box_source())
        {
            return Err(freeze("prepare-state-or-source-mismatch"));
        }
        let prior_homes = claim
            .home_prefix()
            .ok()
            .map(|prefix| (prefix.prior_homes(), prefix.required_unwind()));
        let next = Self::compute_emission_prepare(
            &rows,
            claim.site(),
            claim.arity(),
            &claim.constructor,
            claim.construction(),
            claim.object(),
            prior_homes,
        )?;
        let available = matches!(next, NewEmissionProgress::Prepared { .. });
        rows.get_mut(claim.site())
            .and_then(LocalCommitV1::ordinary_mut)
            .expect("checked ordinary row")
            .emission = next;
        Ok(available)
    }

    /// Return-position claims prepare through the same selected products;
    /// the only absent facts are the destination binding and its statement.
    pub(crate) fn prepare_result_new_emission(
        &self,
        claim: &super::OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get(claim.site())
            .and_then(LocalCommitV1::result)
            .ok_or_else(|| freeze("prepare-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Unprepared)
            || row.object != claim.object()
            || !row.box_source.same_source_as(claim.box_source())
        {
            return Err(freeze("prepare-state-or-source-mismatch"));
        }
        let prior_homes = claim
            .home_prefix()
            .ok()
            .map(|prefix| (prefix.prior_homes(), prefix.required_unwind()));
        let mut next = Self::compute_emission_prepare(
            &rows,
            claim.site(),
            claim.arity(),
            &claim.constructor,
            claim.construction(),
            claim.object(),
            prior_homes,
        )?;
        // A non-trivial argument co-seal is a retained claim, not a take-time
        // freeze: the site keeps its raw-lane emission and the row records
        // the truthful unavailable terminal.
        if claim.argument_rows().is_err()
            && matches!(next, NewEmissionProgress::Prepared { .. })
        {
            next = NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::PendingExpression,
            };
        }
        let available = matches!(next, NewEmissionProgress::Prepared { .. });
        rows.get_mut(claim.site())
            .and_then(LocalCommitV1::result_mut)
            .expect("checked result row")
            .emission = next;
        Ok(available)
    }

    /// Shared begin for ordinary and result rows — the physical emission
    /// shape is identical; only the destination handling differs.
    pub(crate) fn begin_new_emission(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<(Vec<InvokeOperation>, Option<ReclaimUnpublishedOriginV1>), String> {
        let mut rows = self.local_commits.borrow_mut();
        let emission = rows
            .get_mut(site)
            .and_then(LocalCommitV1::new_emission_mut)
            .ok_or_else(|| freeze("emit-without-take"))?;
        if !matches!(emission, NewEmissionProgress::Prepared { .. }) {
            return Err(freeze("emit-without-prepare-or-duplicate"));
        }
        let NewEmissionProgress::Prepared { operands, reclaim } =
            std::mem::replace(emission, NewEmissionProgress::Emitting)
        else {
            unreachable!()
        };
        Ok((operands, reclaim))
    }

    /// These are validation snapshots of actual instructions, not metadata
    /// operands used for liveness. The physical instructions own every use.
    /// Shared by ordinary and result rows.
    pub(crate) fn record_new_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        arguments: Vec<ValueId>,
        reclaim: Option<(ReclaimUnpublishedOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("record-without-take"))?;
        let source_arguments: Vec<super::OrdinaryNewTrivialArgumentV1> = row
            .new_argument_rows()
            .ok_or_else(|| freeze("record-without-take"))?
            .as_ref()
            .map_err(|_| freeze("argument-source-unavailable"))?
            .to_vec();
        if source_arguments.len() != arguments.len() {
            return Err(freeze("argument-count-drift"));
        }
        let emission = row
            .new_emission_mut()
            .expect("new_argument_rows implies a new row");
        if !matches!(emission, NewEmissionProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("record-without-emission-or-duplicate"));
        }
        let arguments = source_arguments
            .iter()
            .cloned()
            .zip(arguments)
            .map(|(source, value)| EmittedNewArgumentV1 { source, value })
            .collect();
        *emission = NewEmissionProgress::Emitted {
            result,
            arguments,
            reclaim: reclaim.map(
                |(origin, block, instruction)| ReclaimUnpublishedEmissionV1 {
                    origin,
                    block,
                    instruction,
                },
            ),
            bindings,
            progress: EmittedLocalProgress::PendingExpression,
        };
        Ok(())
    }

    pub(crate) fn complete_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_new_emissions(owner, function)?;
        for row in self
            .local_commits
            .borrow_mut()
            .values_mut()
            .filter(|row| row.owner() == owner)
        {
            match row {
                LocalCommitV1::Ordinary(row) => row.emission.mark_checked(),
                LocalCommitV1::Result(row) => row.emission.mark_result_checked(),
                LocalCommitV1::Map(row) => row.mark_checked(),
                LocalCommitV1::CallReceived(row) => row.mark_checked(),
            }
        }
        Ok(())
    }

    pub(crate) fn validate_finalized_child_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        if self.child_physical_validation.borrow().contains_key(&owner) {
            return Err(freeze("duplicate-child-physical-validation"));
        }
        self.validate_new_emissions(owner, function)?;
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_integer_literal_return(owner, function)?;
        self.validate_terminal_i64_field_return(owner, function)?;
        self.validate_root_home_exit(owner, function, None)?;
        self.validate_root_cleanup_shape(owner, function)?;
        let bindings = self.lifecycle_bindings(owner)?;
        let copies = self.source_local_copies(owner)?;
        let boundary = physical_boundary::PhysicalBoundary::capture_with_source_copies(
            function, &bindings, &copies,
        )?;
        self.child_physical_validation.borrow_mut().insert(
            owner,
            ChildPhysicalValidation::Checked {
                symbol: function.signature.name.clone(),
                boundary,
            },
        );
        Ok(())
    }

    /// Called after all New overrides, never merely after Birth returns.
    pub(crate) fn complete_new_expression(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(());
        }
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("expression-without-target-take"))?;
        let Some(box_source) = row.new_box_source() else {
            return Err(freeze("expression-without-target-take"));
        };
        if box_source.name() != class {
            return Err(freeze("expression-parent-mismatch"));
        }
        row.new_emission_mut()
            .expect("new_box_source implies a new row")
            .complete_expression(value)
    }

    /// The sealed handle-result local call at this expression site: the
    /// caller receives an owned transferred object at the Return edge.
    pub(crate) fn handle_call_source(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<&LocalCallObservationV1> {
        self.completion_for_owner(site.owner())
            .and_then(|c| c.cleanup().root_flow())
            .and_then(|flow| {
                flow.local_calls().iter().find(|call| {
                    call.site() == site
                        && call.owner() == site.owner()
                        && call.result() == LocalCallResultClassV1::Handle
                })
            })
    }

    /// Prior-home unwind operands for a Handle local call's fault landing:
    /// every Home recorded live before the call must still be installed
    /// and releasable, released newest-first like every other selected
    /// unwind chain. A lexical receiver call can never strand a live
    /// prior Home.
    pub(crate) fn handle_call_prior_home_unwind(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        let call = self
            .handle_call_source(site)
            .ok_or_else(|| freeze("handle-call-source-missing"))?;
        let rows = self.local_commits.borrow();
        call.prior_homes()
            .iter()
            .rev()
            .map(|binding| match installed_home(&rows, *binding) {
                Ok(row) if row.end_available() => Ok(row.end_operation()),
                _ => Err(freeze("handle-call-prior-home-unavailable")),
            })
            .collect()
    }

    /// The canonical object the callee's result claim/commit minted for a
    /// `return new` site — the identity a caller-side received handle keeps.
    pub(crate) fn result_object(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<CanonicalObjectIdV1> {
        if let Some(LocalCommitV1::Result(row)) = self.local_commits.borrow().get(site) {
            return Some(row.object());
        }
        self.result_claims
            .borrow()
            .get(site)
            .map(|claim| claim.object())
    }

    /// Begin emission for a handle-result local `Call`: the sealed
    /// local-call relation is sole membership, the callee's retained
    /// `Value(Construction)` terminal is the sole result-kind authority,
    /// and its minted object is the identity the received handle keeps.
    pub(crate) fn begin_handle_call_emission(
        &self,
        site: &OwnedExprSiteV1,
        callee: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        let call = self
            .handle_call_source(site)
            .ok_or_else(|| freeze("handle-call-source-missing"))?;
        if !matches!(call.declaration(), SourceBindingSiteV1::Local { .. }) {
            return Err(freeze("handle-call-declaration-drift"));
        }
        let construction_site = match self.terminal_relation_for_owner(callee) {
            Some(TerminalRelationV1::Value(row)) if row.owner() == callee => {
                match row.returned() {
                    TerminalReturnedSourceV1::Construction(owned)
                        if owned.owner() == callee =>
                    {
                        owned.clone()
                    }
                    _ => return Err(freeze("handle-result-terminal-mismatch")),
                }
            }
            _ => return Err(freeze("handle-result-terminal-missing")),
        };
        let object = self
            .result_object(&construction_site)
            .ok_or_else(|| freeze("handle-result-object-missing"))?;
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(site) {
            return Err(freeze("handle-duplicate-emission"));
        }
        rows.insert(
            site.clone(),
            LocalCommitV1::CallReceived(CallReceivedCommitV1 {
                owner: site.owner(),
                binding: call.destination(),
                declaration: call.declaration().clone(),
                object,
                progress: CallReceivedProgress::Emitting,
            }),
        );
        Ok(())
    }

    pub(crate) fn record_handle_call_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get_mut(site) else {
            return Err(freeze("handle-record-without-begin"));
        };
        if !matches!(row.progress, CallReceivedProgress::Emitting)
            || bindings.is_empty()
        {
            return Err(freeze("handle-emission-state-drift"));
        }
        row.progress = CallReceivedProgress::Emitted {
            result,
            bindings,
            phase: CallReceivedPhase::ExpressionCompleted,
        };
        Ok(())
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
                || (matches!(
                    row,
                    LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_)
                ) && initializer != local)
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

#[path = "ordinary_new_local_commit/emission_validation.rs"]
mod emission_validation;
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
