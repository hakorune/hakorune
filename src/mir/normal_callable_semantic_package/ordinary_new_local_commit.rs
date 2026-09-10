//! Physical completion of source-issued ordinary-New destinations.
//!
//! Target take, whole-expression completion and local installation are distinct.
//! This retains their exact relation; it does not issue Home availability or
//! claim that Fault cleanup is implemented.

use super::birth_abi_handoff::BirthAbiHandoffV1;
use super::OrdinaryNewClaimLedgerV1;
use super::{CallerNewHomePrefixV1, HomePrefixUnavailableV1};
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
use crate::mir::function::{RootOrdinaryNewObservation, RootOrdinaryNewUnavailable};
use crate::mir::instruction::InvokeOperation;
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalI64AddReturnV1, TerminalI64FieldReturnV1, TerminalIntegerLiteralReturnV1,
    TerminalRelationV1, TerminalUnitReturnV1,
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
    CallReturn { owner: FunctionOwnerIdV1 },
    I64AddReturn { owner: FunctionOwnerIdV1 },
    UnitReturn { owner: FunctionOwnerIdV1 },
    IntegerLiteralReturn { owner: FunctionOwnerIdV1 },
    I64FieldReturn { owner: FunctionOwnerIdV1 },
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
        let mut selected = rows
            .values()
            .filter(|row| row.binding().owner() == owner)
            .peekable();
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
            row.ordinary().is_some_and(|row| {
                matches!(
                    row.emission,
                    NewEmissionProgress::RetainedUnavailable { .. }
                )
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

    /// Select from retained source products before argument descent. No new
    /// source fact is issued here; prior Homes retain the issuer's order.
    pub(crate) fn prepare_new_emission(
        &self,
        claim: &super::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        if let super::OrdinaryNewConstructorDispositionV1::Birth(recipe) = &claim.constructor {
            let physical = claim
                .arity()
                .checked_add(1)
                .ok_or_else(|| freeze("arity-overflow"))?;
            recipe
                .abi()
                .validate(claim.arity(), physical)
                .map_err(|error| format!("[freeze:contract][ordinary-new/abi/{error:?}]"))?;
        }
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
        let reclaim = match (&claim.constructor, claim.construction()) {
            (super::OrdinaryNewConstructorDispositionV1::NoBirthZero, _) => None,
            (super::OrdinaryNewConstructorDispositionV1::Birth(_), Err(_)) => None,
            (super::OrdinaryNewConstructorDispositionV1::Birth(recipe), Ok(plan)) => {
                let (constructor_source, constructor_owner) = plan
                    .constructor()
                    .ok_or_else(|| freeze("reclaim-origin-constructor-missing"))?;
                if !plan.reclaims_unpublished_outer_storage()
                    || plan.object() != claim.object()
                    || !constructor_source.same_as(recipe.source_id())
                {
                    return Err(freeze("reclaim-origin-source-drift"));
                }
                Some(ReclaimUnpublishedOriginV1 {
                    site: claim.site().clone(),
                    constructor_source: constructor_source.clone(),
                    constructor_owner: *constructor_owner,
                    object: plan.object(),
                })
            }
        };
        let mut operands = Vec::new();
        let mut available = claim.construction().is_ok();
        match claim.home_prefix() {
            Err(_) => available = false,
            Ok(prefix) => {
                if prefix.required_unwind() != claim.site() {
                    return Err(freeze("prepare-outward-site"));
                }
                for binding in prefix.prior_homes() {
                    let prior = installed_home(&rows, *binding).map_err(|error| match error {
                        HomeLookupError::Missing => freeze("prior-home-not-installed"),
                        HomeLookupError::Duplicate => freeze("duplicate-prior-home"),
                    })?;
                    available &= prior.end_available();
                    operands.push(prior.end_operation());
                }
            }
        }
        rows.get_mut(claim.site())
            .and_then(LocalCommitV1::ordinary_mut)
            .expect("checked ordinary row")
            .emission = if available {
            NewEmissionProgress::Prepared { operands, reclaim }
        } else {
            NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::PendingExpression,
            }
        };
        Ok(available)
    }

    pub(crate) fn begin_new_emission(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<(Vec<InvokeOperation>, Option<ReclaimUnpublishedOriginV1>), String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .and_then(LocalCommitV1::ordinary_mut)
            .ok_or_else(|| freeze("emit-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Prepared { .. }) {
            return Err(freeze("emit-without-prepare-or-duplicate"));
        }
        let NewEmissionProgress::Prepared { operands, reclaim } =
            std::mem::replace(&mut row.emission, NewEmissionProgress::Emitting)
        else {
            unreachable!()
        };
        Ok((operands, reclaim))
    }

    /// These are validation snapshots of actual instructions, not metadata
    /// operands used for liveness. The physical instructions own every use.
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
            .and_then(LocalCommitV1::ordinary_mut)
            .ok_or_else(|| freeze("record-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("record-without-emission-or-duplicate"));
        }
        let source_arguments = row
            .argument_rows
            .as_ref()
            .map_err(|_| freeze("argument-source-unavailable"))?;
        if source_arguments.len() != arguments.len() {
            return Err(freeze("argument-count-drift"));
        }
        let arguments = source_arguments
            .iter()
            .cloned()
            .zip(arguments)
            .map(|(source, value)| EmittedNewArgumentV1 { source, value })
            .collect();
        row.emission = NewEmissionProgress::Emitted {
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
            .filter(|row| row.binding().owner() == owner)
        {
            match row {
                LocalCommitV1::Ordinary(row) => row.emission.mark_checked(),
                LocalCommitV1::Map(row) => row.mark_checked(),
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
        let boundary = physical_boundary::PhysicalBoundary::capture(function, &bindings)?;
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
            .and_then(LocalCommitV1::ordinary_mut)
            .ok_or_else(|| freeze("expression-without-target-take"))?;
        if row.box_source.name() != class {
            return Err(freeze("expression-parent-mismatch"));
        }
        row.emission.complete_expression(value)
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
            let SourceBindingSiteV1::Local { ordinal, .. } = row.declaration() else {
                unreachable!("at_statement requires Local");
            };
            let (_, _, initializer, local) = completed
                .iter()
                .find(|(binding, index, _, _)| *binding == row.binding() && index == ordinal)
                .ok_or_else(|| freeze("local-binding-or-ordinal-mismatch"))?;
            if row.local().is_some() {
                return Err(freeze("duplicate-local-installation"));
            }
            if row.initializer() != Some(*initializer)
                || (matches!(row, LocalCommitV1::Ordinary(_)) && initializer == local)
                || (matches!(row, LocalCommitV1::Map(_)) && initializer != local)
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
