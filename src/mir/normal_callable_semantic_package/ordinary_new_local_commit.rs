//! Physical completion of source-issued ordinary-New destinations.
//!
//! Target take, whole-expression completion and local installation are distinct.
//! This retains their exact relation; it does not issue Home availability or
//! claim that Fault cleanup is implemented.

use super::birth_abi_handoff::BirthAbiHandoffV1;
use super::OrdinaryNewClaimLedgerV1;
use super::{CallerNewHomePrefixV1, HomePrefixUnavailableV1};
use crate::mir::function::{RootOrdinaryNewObservation, RootOrdinaryNewUnavailable};
use crate::mir::resolved_semantics::home_new_prefix::{TerminalI64AddReturnV1, TerminalUnitReturnV1};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceNodeSiteV1,
};
use crate::mir::ValueId;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use crate::parser::CallableDeclarationIdentityV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::BTreeSet;

#[derive(Debug)]
pub(super) enum RootNewValidation {
    Unregistered,
    Pending(FunctionOwnerIdV1),
    Checked(FunctionOwnerIdV1),
    FinishingChecked,
}

#[derive(Debug)]
enum NewEmissionProgress {
    Unprepared,
    RetainedUnavailable,
    Prepared {
        operands: Vec<(CanonicalObjectIdV1, ValueId)>,
        reclaim: Option<ReclaimUnpublishedOriginV1>,
    },
    Emitting,
    Emitted {
        result: ValueId,
        arguments: Box<[EmittedNewArgumentV1]>,
        reclaim: Option<ReclaimUnpublishedEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        checked: bool,
    },
}

/// Physical consumption of one already-issued selected-New argument row.
///
/// This is a finalizer-owned snapshot of an emitted MIR value.  It neither
/// issues source meaning nor selects an ABI lane.
#[derive(Debug)]
struct EmittedNewArgumentV1 {
    source: super::OrdinaryNewTrivialArgumentV1,
    value: ValueId,
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
    initializer: Option<ValueId>,
    local: Option<ValueId>,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    argument_rows: Result<
        Box<[super::OrdinaryNewTrivialArgumentV1]>,
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    >,
    emission: NewEmissionProgress,
}

/// Final artifact handoff for one exact root and its already-issued Birth keys.
/// It is an opaque retention of source products, never a source or ABI issuer.
#[derive(Debug)]
pub(crate) enum FinalizedRootBirthHandoffV1 {
    NoBirth {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
        root_result: Option<FinalizedRootResultAbiV1>,
    },
    Births {
        root_key: String,
        root_source: Option<FinalizedRootSourceHandoffV1>,
        root_result: Option<FinalizedRootResultAbiV1>,
        keys: Box<[CanonicalSameModuleCallableKeyV1]>,
        births: Box<[BirthAbiHandoffV1]>,
    },
}

/// Exact source relation retained after its matching physical root passed
/// final validation. This is transport only: it cannot select an entry ABI or
/// recreate source membership from a physical key.
#[derive(Debug, Clone)]
pub(crate) struct FinalizedRootSourceHandoffV1 {
    app_main_identity: CallableDeclarationIdentityV1,
    terminal_i64_add: Option<TerminalI64AddReturnV1>,
    terminal_unit_return: Option<TerminalUnitReturnV1>,
}

impl FinalizedRootSourceHandoffV1 {
    pub(crate) fn app_main_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.app_main_identity
    }

    pub(crate) fn terminal_i64_add(&self) -> Option<&TerminalI64AddReturnV1> {
        self.terminal_i64_add.as_ref()
    }
    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        self.terminal_unit_return.as_ref()
    }
}

/// Final-handoff projection of the already-issued terminal source relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalizedRootResultAbiV1 {
    I64AddReturn { owner: FunctionOwnerIdV1 },
    UnitReturn { owner: FunctionOwnerIdV1 },
}

impl FinalizedRootBirthHandoffV1 {
    pub(crate) fn root_key(&self) -> &str {
        match self {
            Self::NoBirth { root_key, .. } | Self::Births { root_key, .. } => root_key,
        }
    }

    pub(crate) fn root_result(&self) -> Option<FinalizedRootResultAbiV1> {
        match self {
            Self::NoBirth { root_result, .. } | Self::Births { root_result, .. } => *root_result,
        }
    }

    pub(crate) fn root_source(&self) -> Option<&FinalizedRootSourceHandoffV1> {
        match self {
            Self::NoBirth { root_source, .. } | Self::Births { root_source, .. } => {
                root_source.as_ref()
            }
        }
    }

    pub(crate) fn births(&self) -> &[BirthAbiHandoffV1] {
        match self {
            Self::NoBirth { .. } => &[],
            Self::Births { births, .. } => births,
        }
    }

    pub(crate) fn birth_keys(&self) -> &[CanonicalSameModuleCallableKeyV1] {
        match self {
            Self::NoBirth { .. } => &[],
            Self::Births { keys, .. } => keys,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        String,
        Option<FinalizedRootSourceHandoffV1>,
        Option<FinalizedRootResultAbiV1>,
        Box<[BirthAbiHandoffV1]>,
    ) {
        match self {
            Self::NoBirth {
                root_key,
                root_source,
                root_result,
            } => (root_key, root_source, root_result, Box::new([])),
            Self::Births {
                root_key,
                root_source,
                root_result,
                keys: _,
                births,
            } => (root_key, root_source, root_result, births),
        }
    }
}

impl NewLocalCommitV1 {
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
            initializer: None,
            local: None,
            home_prefix,
            argument_rows,
            emission: NewEmissionProgress::Unprepared,
        }
    }

    pub(super) fn is_complete(&self) -> bool {
        self.initializer.is_some()
            && self.local.is_some()
            && matches!(
                self.emission,
                NewEmissionProgress::RetainedUnavailable
                    | NewEmissionProgress::Emitted { checked: true, .. }
            )
    }

    pub(super) fn installs(&self, binding: BindingRefV1) -> bool {
        self.binding == binding
            && self.initializer.is_some()
            && self.local.is_some()
            && matches!(&self.home_prefix, Ok(prefix) if prefix.destination() == binding)
    }

    fn at_statement(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> bool {
        self.binding.owner() == owner
            && matches!(&self.declaration,
            SourceBindingSiteV1::Local { statement, .. } if statement.node() == site)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn seal_finalized_root_birth_handoff(
        &self,
        root_key: String,
        construction_keys: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
    ) -> Result<FinalizedRootBirthHandoffV1, String> {
        match *self.root_validation.borrow() {
            RootNewValidation::FinishingChecked => {}
            _ => return Err(freeze("artifact-root-not-finished")),
        }
        let owner = match self.root_completion.as_ref() {
            Some(Ok(completion)) => completion.owner(),
            _ => return Err(freeze("artifact-root-completion-unavailable")),
        };
        let root_source = match (&self.terminal_result, &self.terminal_unit_return) {
            (Some(_), Some(_)) => return Err(freeze("artifact-root-result-conflict")),
            (Some(relation), None) => Some({
                if relation.owner() != owner || !self.terminal_result_complete() {
                    return Err(freeze("artifact-root-result-unavailable"));
                }
                let app_main_identity = self
                    .app_main_identity
                    .as_ref()
                    .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                    .clone();
                FinalizedRootSourceHandoffV1 {
                    app_main_identity,
                    terminal_i64_add: Some(relation.clone()),
                    terminal_unit_return: None,
                }
            }),
            (None, Some(relation)) => Some({
                if relation.owner() != owner { return Err(freeze("artifact-root-unit-owner-drift")); }
                let completion = self.root_completion.as_ref().and_then(|row| row.as_ref().ok())
                    .ok_or_else(|| freeze("artifact-root-completion-unavailable"))?;
                if completion.explicit_site() != Some(relation.return_site()) {
                    return Err(freeze("artifact-root-unit-site-drift"));
                }
                let app_main_identity = self.app_main_identity.as_ref()
                    .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?.clone();
                FinalizedRootSourceHandoffV1 {
                    app_main_identity,
                    terminal_i64_add: None,
                    terminal_unit_return: Some(relation.clone()),
                }
            }),
            (None, None) => None,
        };
        let root_result = root_source.as_ref().map(|source| {
            if source.terminal_i64_add().is_some() {
                FinalizedRootResultAbiV1::I64AddReturn { owner }
            } else {
                FinalizedRootResultAbiV1::UnitReturn { owner }
            }
        });
        let mut keys = BTreeSet::new();
        let mut births = Vec::new();
        for (_, row) in self
            .local_commits
            .borrow()
            .iter()
            .filter(|(_, row)| row.binding.owner() == owner)
        {
            if !row.is_complete() {
                return Err(freeze("artifact-local-commit-incomplete"));
            }
            let Some(key) = &row.birth_target else {
                if row.birth_abi.is_some() {
                    return Err(freeze("artifact-birth-abi-without-target"));
                }
                continue;
            };
            let key = key.clone();
            let relation = row
                .birth_abi
                .as_ref()
                .ok_or_else(|| freeze("artifact-birth-abi-missing"))?;
            if relation.target() != &key || relation.owner() == owner {
                return Err(freeze("artifact-birth-abi-drift"));
            }
            if relation.object() != row.object {
                return Err(freeze("artifact-birth-object-drift"));
            }
            if !construction_keys.contains(&key) {
                return Err(freeze("artifact-birth-construction-missing"));
            }
            // Multiple exact New sites may invoke one canonical Birth
            // definition. Local emission validation above remains per site;
            // the final handoff retains each definition relation once.
            if keys.insert(key.clone()) {
                births.push(relation.clone());
            } else if !births.iter().any(|existing| existing == relation) {
                return Err(freeze("artifact-birth-abi-duplicate-drift"));
            }
        }
        Ok(if births.is_empty() {
            FinalizedRootBirthHandoffV1::NoBirth {
                root_key,
                root_source,
                root_result,
            }
        } else {
            FinalizedRootBirthHandoffV1::Births {
                root_key,
                root_source,
                root_result,
                keys: keys.into_iter().collect(),
                births: births.into_boxed_slice(),
            }
        })
    }

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

    /// Artifact validation consumes the retained source state, not the public
    /// observation. Shared frame bindings are deduplicated; actual duplicates
    /// and lifecycle instructions without an emitted source obligation reject.
    pub(crate) fn validate_artifact_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let owner = match *self.root_validation.borrow() {
            RootNewValidation::Checked(owner) => owner,
            _ => return Err(freeze("artifact-root-not-checked")),
        };
        self.validate_new_emissions(owner, function)?;
        self.validate_terminal_unit_return(owner, function)?;
        self.validate_root_home_exit(function)?;
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_i64_add_return(owner, function)?;
        if self.finalized_root_observation(owner)
            != RootOrdinaryNewObservation::SourceCompleteAtFinalization
        {
            return Err(freeze("artifact-source-unavailable"));
        }
        {
            let rows = self.local_commits.borrow();
            let exit = self.root_exit.borrow();
            let mut expected = Vec::new();
            for row in rows.values().filter(|row| row.binding.owner() == owner) {
                let NewEmissionProgress::Emitted {
                    bindings,
                    checked: true,
                    ..
                } = &row.emission
                else {
                    return Err(freeze("artifact-emission-unchecked"));
                };
                for (block, instruction) in bindings {
                    if instruction.requires_lifecycle_validation()
                        && !expected.contains(&(*block, instruction))
                    {
                        expected.push((*block, instruction));
                    }
                }
            }
            if let RootHomeExitProgress::Emitted { bindings, .. } = &*exit {
                for (block, instruction) in bindings {
                    if instruction.requires_lifecycle_validation()
                        && !expected.contains(&(*block, instruction))
                    {
                        expected.push((*block, instruction));
                    }
                }
            }
            for block in function.blocks.values() {
                for actual in block
                    .all_instructions()
                    .filter(|i| i.requires_lifecycle_validation())
                {
                    let index = expected
                        .iter()
                        .position(|(id, instruction)| *id == block.id && *instruction == actual)
                        .ok_or_else(|| freeze("artifact-unowned-lifecycle-site"))?;
                    expected.swap_remove(index);
                }
            }
            if !expected.is_empty() {
                return Err(freeze("artifact-lifecycle-residual"));
            }
        }
        self.validate_after_compiler_finishing(function)
    }

    fn finalized_root_observation(&self, owner: FunctionOwnerIdV1) -> RootOrdinaryNewObservation {
        use RootOrdinaryNewObservation::{
            NoSelectedLocalNew, SourceCompleteAtFinalization, Unavailable,
        };
        use RootOrdinaryNewUnavailable::*;
        let rows = self.local_commits.borrow();
        let mut selected = rows
            .values()
            .filter(|row| row.binding.owner() == owner)
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
        if selected.any(|row| matches!(row.emission, NewEmissionProgress::RetainedUnavailable)) {
            return Unavailable(NewEmissionUnavailable);
        }
        match &*self.root_exit.borrow() {
            RootHomeExitProgress::Emitted { .. } => SourceCompleteAtFinalization,
            RootHomeExitProgress::Unavailable => Unavailable(RootExitUnavailable),
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
                    let mut matches = rows.values().filter(|row| row.installs(*binding));
                    let prior = matches
                        .next()
                        .ok_or_else(|| freeze("prior-home-not-installed"))?;
                    if matches.next().is_some() {
                        return Err(freeze("duplicate-prior-home"));
                    }
                    available &=
                        prior.destruction == super::ObjectDestructionDispositionV1::PlainI64NoHook;
                    operands.push((prior.object, prior.local.expect("installed Home")));
                }
            }
        }
        rows.get_mut(claim.site()).expect("checked row").emission = if available {
            NewEmissionProgress::Prepared { operands, reclaim }
        } else {
            NewEmissionProgress::RetainedUnavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_new_emission(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<
        (
            Vec<(CanonicalObjectIdV1, ValueId)>,
            Option<ReclaimUnpublishedOriginV1>,
        ),
        String,
    > {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
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
            checked: false,
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
            .filter(|row| row.binding.owner() == owner)
        {
            if let NewEmissionProgress::Emitted { checked, .. } = &mut row.emission {
                *checked = true;
            }
        }
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
        if row.box_source.name() != class {
            return Err(freeze("expression-parent-mismatch"));
        }
        if row.initializer.is_some() || row.local.is_some() {
            return Err(freeze("duplicate-expression-completion"));
        }
        match &row.emission {
            NewEmissionProgress::RetainedUnavailable => {}
            NewEmissionProgress::Emitted { result, .. } if *result == value => {}
            _ => return Err(freeze("expression-before-emission-or-result-drift")),
        }
        row.initializer = Some(value);
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
            let SourceBindingSiteV1::Local { ordinal, .. } = &row.declaration else {
                unreachable!("at_statement requires Local");
            };
            let (_, _, initializer, local) = completed
                .iter()
                .find(|(binding, index, _, _)| *binding == row.binding && index == ordinal)
                .ok_or_else(|| freeze("local-binding-or-ordinal-mismatch"))?;
            if row.local.is_some() {
                return Err(freeze("duplicate-local-installation"));
            }
            if row.initializer != Some(*initializer) || initializer == local {
                return Err(freeze("local-initializer-mismatch"));
            }
            commits.push((site.clone(), *local));
        }
        for (site, local) in commits {
            rows.get_mut(&site)
                .expect("validated row remains present")
                .local = Some(local);
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
#[path = "ordinary_new_local_commit/root_home.rs"]
mod root_home;
#[path = "ordinary_new_local_commit/root_validation.rs"]
mod root_validation;

use reclaim::ReclaimUnpublishedEmissionV1;
pub(crate) use reclaim::ReclaimUnpublishedOriginV1;
pub(super) use root_home::RootHomeExitProgress;
