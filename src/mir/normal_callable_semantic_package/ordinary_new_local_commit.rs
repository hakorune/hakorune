//! Physical completion of source-issued ordinary-New destinations.
//!
//! Target take, whole-expression completion and local installation are distinct.
//! This retains their exact relation; it does not issue Home availability or
//! claim that Fault cleanup is implemented.

use super::OrdinaryNewClaimLedgerV1;
use super::{CallerNewHomePrefixV1, HomePrefixUnavailableV1};
use crate::mir::function::{RootOrdinaryNewObservation, RootOrdinaryNewUnavailable};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceNodeSiteV1,
};
use crate::mir::ValueId;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
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
pub(super) enum RootHomeExitProgress {
    Unprepared,
    Unavailable,
    Prepared(Vec<(CanonicalObjectIdV1, ValueId)>),
    Emitting,
    Emitted(Vec<(BasicBlockId, MirInstruction)>),
}

#[derive(Debug)]
enum NewEmissionProgress {
    Unprepared,
    RetainedUnavailable,
    Prepared(Vec<(CanonicalObjectIdV1, ValueId)>),
    Emitting,
    Emitted {
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        checked: bool,
    },
}

#[derive(Debug)]
pub(super) struct NewLocalCommitV1 {
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    construction: super::ConstructionEligibilityV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    destruction: super::ObjectDestructionDispositionV1,
    birth_target: Option<CanonicalSameModuleCallableKeyV1>,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    initializer: Option<ValueId>,
    local: Option<ValueId>,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    emission: NewEmissionProgress,
}

/// Final artifact handoff for one exact root and its already-issued Birth keys.
/// It is an opaque retention of source products, never a source or ABI issuer.
#[derive(Debug)]
pub(crate) enum FinalizedRootBirthHandoffV1 {
    NoBirth {
        root_key: String,
    },
    Births {
        root_key: String,
        keys: Box<[CanonicalSameModuleCallableKeyV1]>,
    },
}

impl FinalizedRootBirthHandoffV1 {
    pub(crate) fn root_key(&self) -> &str {
        match self {
            Self::NoBirth { root_key } | Self::Births { root_key, .. } => root_key,
        }
    }

    pub(crate) fn birth_keys(&self) -> &[CanonicalSameModuleCallableKeyV1] {
        match self {
            Self::NoBirth { .. } => &[],
            Self::Births { keys, .. } => keys,
        }
    }

    pub(crate) fn into_parts(self) -> (String, Box<[CanonicalSameModuleCallableKeyV1]>) {
        match self {
            Self::NoBirth { root_key } => (root_key, Box::new([])),
            Self::Births { root_key, keys } => (root_key, keys),
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
    ) -> Self {
        Self {
            box_source,
            construction,
            object,
            destruction,
            birth_target,
            binding,
            declaration,
            initializer: None,
            local: None,
            home_prefix,
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
        let mut keys = BTreeSet::new();
        for row in self
            .local_commits
            .borrow()
            .values()
            .filter(|row| row.binding.owner() == owner)
        {
            if !row.is_complete() {
                return Err(freeze("artifact-local-commit-incomplete"));
            }
            let Some(key) = &row.birth_target else {
                continue;
            };
            let key = key.clone();
            if !construction_keys.contains(&key) {
                return Err(freeze("artifact-birth-construction-missing"));
            }
            if !keys.insert(key) {
                return Err(freeze("artifact-birth-recipe-duplicate"));
            }
        }
        Ok(if keys.is_empty() {
            FinalizedRootBirthHandoffV1::NoBirth { root_key }
        } else {
            FinalizedRootBirthHandoffV1::Births {
                root_key,
                keys: keys.into_iter().collect(),
            }
        })
    }

    pub(super) fn root_home_exit_is_complete(&self) -> bool {
        let Some(Ok(completion)) = &self.root_completion else {
            return true;
        };
        !matches!(completion.cleanup().terminal_homes(), Some(Ok(_)))
            || matches!(
                *self.root_exit.borrow(),
                RootHomeExitProgress::Unavailable | RootHomeExitProgress::Emitted(_)
            )
    }

    pub(crate) fn prepare_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        let Some(Ok(completion)) = &self.root_completion else {
            return Ok(false);
        };
        if completion.owner() != owner {
            return Ok(false);
        }
        let Some(Ok(homes)) = completion.cleanup().terminal_homes() else {
            return Ok(false);
        };
        if !completion
            .explicit_site()
            .is_some_and(|expected| expected.node() == site)
        {
            return Err(freeze("root-exit-site-mismatch"));
        }
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        let rows = self.local_commits.borrow();
        let mut operands = Vec::new();
        let mut available = true;
        for binding in homes {
            let mut candidates = rows.values().filter(|row| row.installs(*binding));
            let row = candidates
                .next()
                .ok_or_else(|| freeze("root-home-not-installed"))?;
            if candidates.next().is_some() {
                return Err(freeze("duplicate-root-home"));
            }
            available &= row.destruction == super::ObjectDestructionDispositionV1::PlainI64NoHook
                && matches!(row.emission, NewEmissionProgress::Emitted { .. });
            operands.push((row.object, row.local.expect("installed Home")));
        }
        *progress = if available {
            RootHomeExitProgress::Prepared(operands)
        } else {
            RootHomeExitProgress::Unavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_root_home_exit(
        &self,
    ) -> Result<Vec<(CanonicalObjectIdV1, ValueId)>, String> {
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Prepared(_)) {
            return Err(freeze("root-exit-not-prepared"));
        }
        let RootHomeExitProgress::Prepared(operands) =
            std::mem::replace(&mut *progress, RootHomeExitProgress::Emitting)
        else {
            unreachable!()
        };
        Ok(operands)
    }

    pub(crate) fn record_root_home_exit(
        &self,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        *progress = RootHomeExitProgress::Emitted(bindings);
        Ok(())
    }

    fn validate_root_home_exit(&self, function: &MirFunction) -> Result<(), String> {
        let Some(Ok(completion)) = &self.root_completion else {
            return Ok(());
        };
        if !matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
            return Ok(());
        }
        match &*self.root_exit.borrow() {
            RootHomeExitProgress::Unavailable => Ok(()),
            RootHomeExitProgress::Emitted(bindings) => {
                for (id, expected) in bindings {
                    if !function.blocks.get(id).is_some_and(|block| {
                        block.all_instructions().any(|actual| actual == expected)
                    }) {
                        return Err(freeze("root-exit-binding-drift"));
                    }
                }
                Ok(())
            }
            _ => Err(freeze("root-exit-unconsumed")),
        }
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

    /// Called on the exact physical root after all module finalization passes.
    /// Script-only packages never register a callable root; an empty New set
    /// does not erase a registered root's validation obligation.
    pub(crate) fn validate_finalized_new_root(
        &self,
        function: &MirFunction,
    ) -> Result<RootOrdinaryNewObservation, String> {
        let mut state = self.root_validation.borrow_mut();
        let owner = match *state {
            RootNewValidation::Unregistered => return Ok(RootOrdinaryNewObservation::NotIssued),
            RootNewValidation::Pending(owner) => owner,
            RootNewValidation::Checked(_) | RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-root-validation"));
            }
        };
        self.validate_new_emissions(owner, function)?;
        self.validate_root_home_exit(function)?;
        self.validate_field_reads(owner, function)?;
        let observation = self.finalized_root_observation(owner);
        *state = RootNewValidation::Checked(owner);
        Ok(observation)
    }

    /// Recheck the same retained source obligations after compiler finishing.
    /// The early observation cannot authorize a modified function. No source
    /// products are reconstructed from the final CFG or its metadata.
    pub(crate) fn validate_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let mut state = self.root_validation.borrow_mut();
        let owner = match *state {
            RootNewValidation::Unregistered => return Ok(()),
            RootNewValidation::Checked(owner) => owner,
            RootNewValidation::Pending(_) => return Err(freeze("root-before-draft-validation")),
            RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-finishing-validation"));
            }
        };
        self.validate_new_emissions(owner, function)?;
        self.validate_root_home_exit(function)?;
        self.validate_field_reads(owner, function)?;
        if self.finalized_root_observation(owner) != function.root_ordinary_new_observation() {
            return Err(freeze("root-observation-drift"));
        }
        *state = RootNewValidation::FinishingChecked;
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
        self.validate_root_home_exit(function)?;
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
            if let RootHomeExitProgress::Emitted(bindings) = &*exit {
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
            RootHomeExitProgress::Emitted(_) => SourceCompleteAtFinalization,
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
            NewEmissionProgress::Prepared(operands)
        } else {
            NewEmissionProgress::RetainedUnavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_new_emission(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Vec<(CanonicalObjectIdV1, ValueId)>, String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("emit-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Prepared(_)) {
            return Err(freeze("emit-without-prepare-or-duplicate"));
        }
        let NewEmissionProgress::Prepared(operands) =
            std::mem::replace(&mut row.emission, NewEmissionProgress::Emitting)
        else {
            unreachable!()
        };
        Ok(operands)
    }

    /// These are validation snapshots of actual instructions, not metadata
    /// operands used for liveness. The physical instructions own every use.
    pub(crate) fn record_new_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("record-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("record-without-emission-or-duplicate"));
        }
        row.emission = NewEmissionProgress::Emitted {
            result,
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

    pub(crate) fn validate_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        for row in self
            .local_commits
            .borrow()
            .values()
            .filter(|row| row.binding.owner() == owner)
        {
            match &row.emission {
                NewEmissionProgress::RetainedUnavailable => {}
                NewEmissionProgress::Emitted {
                    result, bindings, ..
                } => {
                    if row.initializer != Some(*result) || row.local.is_none() {
                        return Err(freeze("emission-local-result-drift"));
                    }
                    let local = row.local.expect("checked local installation");
                    let mut copies = function
                        .blocks
                        .values()
                        .flat_map(|block| block.all_instructions())
                        .filter(|instruction| {
                            matches!(instruction,
                            MirInstruction::Copy { dst, .. } if *dst == local)
                        });
                    if !matches!(copies.next(), Some(MirInstruction::Copy { src, .. }) if src == result)
                        || copies.next().is_some()
                    {
                        return Err(freeze("emission-local-copy-drift"));
                    }
                    for (block, expected) in bindings {
                        if !function.blocks.get(block).is_some_and(|block| {
                            block.all_instructions().any(|actual| actual == expected)
                        }) {
                            return Err(freeze("emission-binding-drift"));
                        }
                    }
                }
                _ => return Err(freeze("emission-residual")),
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
