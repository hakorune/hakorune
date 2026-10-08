//! Package-owned retention of one verified callable result/completion row.
//!
//! The completion is issued once by the resolver-owned verifier. One builder
//! collects contract rows inside the source loan; the S6C child then takes its
//! exclusive row, `seal` validates and retains the remaining rows, and the
//! physical header borrows the cohort. No stage verifies, copies, or reowns the
//! same `VerifiedFunctionCompletionV1`.

use crate::mir::builder::SelectedCallableConsumptionRoleV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{
    DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::parser::CallableDeclarationIdentityV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::model::OwnedCallableParameterContractDeclarationV1;
use super::physical_header::CallablePhysicalHeaderIssueV1;
use super::selected_mapping::{
    SelectedCallableBatchMapRowRefV1, VerifiedSelectedCallableBatchMapV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CallableResultContractIssueV1 {
    DuplicateBatchSlot { _batch_slot: u32 },
    CompletionOwnerMismatch { _batch_slot: u32 },
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultContractCohortV1 {
    rows: Box<[VerifiedCallableResultContractRowV1]>,
    named_array_emissions: Box<[super::EmittedNamedArrayRequirementV1]>,
    completed_context: Option<(
        super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        Box<[super::model::OwnedCallableParameterContractDeclarationV1]>,
    )>,
}

#[derive(Debug)]
pub(super) struct VerifiedCallableResultContractRowV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    identity: CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
    result: Option<ExactTrivialScalarAbiV1>,
    completion: Rc<VerifiedFunctionCompletionV1>,
    top_level_input: Option<top_level::VerifiedTopLevelScalarInputV1>,
    // Exit-site keyed terminal relations; consumers needing a single row
    // must name the site or prove a uniform projection over the whole map.
    terminal_relations: Rc<BTreeMap<SourceStmtSiteV1, TerminalRelationV1>>,
}

#[derive(Clone, Copy)]
pub(crate) struct CallableResultContractRefV1<'a> {
    owner: FunctionOwnerIdV1,
    identity: &'a CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
    result: Option<ExactTrivialScalarAbiV1>,
    completion: &'a VerifiedFunctionCompletionV1,
    terminal_relations: &'a BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
}

impl VerifiedCallableResultContractCohortV1 {
    pub(super) fn row(&self, batch_slot: u32) -> Option<&VerifiedCallableResultContractRowV1> {
        self.rows
            .binary_search_by_key(&batch_slot, |row| row.batch_slot)
            .ok()
            .map(|index| &self.rows[index])
    }

    pub(super) fn rows(&self) -> impl Iterator<Item = &VerifiedCallableResultContractRowV1> {
        self.rows.iter()
    }
}

impl VerifiedCallableResultContractRowV1 {
    pub(super) const fn batch_slot(&self) -> u32 {
        self.batch_slot
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(super) const fn role(&self) -> SelectedCallableConsumptionRoleV1 {
        self.role
    }

    pub(super) const fn result(&self) -> Option<ExactTrivialScalarAbiV1> {
        self.result
    }

    /// Exclusive handoff for the S6C child: the row is consumed, its
    /// `Rc<Completion>` unwrapped by the receiver. Ordinary consumers borrow
    /// instead.
    pub(super) fn into_parts(
        self,
    ) -> (
        u32,
        FunctionOwnerIdV1,
        CallableDeclarationIdentityV1,
        SelectedCallableConsumptionRoleV1,
        Option<ExactTrivialScalarAbiV1>,
        Rc<VerifiedFunctionCompletionV1>,
        Rc<BTreeMap<SourceStmtSiteV1, TerminalRelationV1>>,
        Option<top_level::VerifiedTopLevelScalarInputV1>,
    ) {
        (
            self.batch_slot,
            self.owner,
            self.identity,
            self.role,
            self.result,
            self.completion,
            self.terminal_relations,
            self.top_level_input,
        )
    }

    pub(super) fn borrow(&self) -> CallableResultContractRefV1<'_> {
        CallableResultContractRefV1::from_completion(
            self.owner,
            &self.identity,
            self.role,
            self.result,
            self.completion.as_ref(),
            self.terminal_relations.as_ref(),
        )
    }
}

impl<'a> CallableResultContractRefV1<'a> {
    pub(super) fn from_completion(
        owner: FunctionOwnerIdV1,
        identity: &'a CallableDeclarationIdentityV1,
        role: SelectedCallableConsumptionRoleV1,
        result: Option<ExactTrivialScalarAbiV1>,
        completion: &'a VerifiedFunctionCompletionV1,
        terminal_relations: &'a BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    ) -> Self {
        Self {
            owner,
            identity,
            role,
            result,
            completion,
            terminal_relations,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        self.identity
    }

    pub(crate) const fn role(&self) -> SelectedCallableConsumptionRoleV1 {
        self.role
    }

    pub(crate) const fn result(&self) -> Option<ExactTrivialScalarAbiV1> {
        self.result
    }

    /// Corroborate a source-proven I64 class without manufacturing an annotation.
    /// This is declaration agreement only; the caller must retain its original
    /// complete source/result proof. None alone never proves I64 or excludes Unit.
    pub(super) fn declared_result_agrees_with_i64_source(&self) -> bool {
        match (
            self.result,
            self.completion.function_exit_contract().declared_result(),
        ) {
            (
                Some(ExactTrivialScalarAbiV1::I64),
                DeclaredFunctionResultContractV1::Annotated(name),
            ) => ExactTrivialScalarAbiV1::classify(name) == Some(ExactTrivialScalarAbiV1::I64),
            (None, DeclaredFunctionResultContractV1::Unannotated) => true,
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn declared_result(&self) -> &DeclaredFunctionResultContractV1 {
        self.completion.function_exit_contract().declared_result()
    }

    /// Every retained terminal relation keyed by its exact exit site.
    /// An empty map does not infer Unit, an ABI kind, or empty cleanup.
    pub(crate) const fn terminal_relations(
        &self,
    ) -> &'a BTreeMap<SourceStmtSiteV1, TerminalRelationV1> {
        self.terminal_relations
    }

    /// Test-only: the sole retained relation, `None` when the row keeps
    /// zero or several exit relations — never an arbitrary pick.
    #[cfg(test)]
    pub(crate) fn sole_terminal_relation(&self) -> Option<&'a TerminalRelationV1> {
        (self.terminal_relations.len() == 1)
            .then(|| self.terminal_relations.values().next())
            .flatten()
    }

    /// Borrow the issued product; callers must not infer missing obligations
    /// from a partial summary or absent Home analysis.
    pub(crate) const fn completion(&self) -> &'a VerifiedFunctionCompletionV1 {
        self.completion
    }
}

/// Accumulates contract rows inside the ordinary source loan. The rows are
/// already the retained contract shape — no destructure/rebuild hop — so the
/// S6C child takes one row and `seal` keeps the rest under the same ownership.
#[derive(Debug)]
pub(super) struct VerifiedCallableResultContractBuilderV1 {
    rows: Vec<VerifiedCallableResultContractRowV1>,
}

impl VerifiedCallableResultContractBuilderV1 {
    pub(super) fn new() -> Self {
        Self { rows: Vec::new() }
    }

    // Called inside the same source loan as ordinary-New candidate issuance.
    pub(super) fn push_completion(
        &mut self,
        declaration: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
        selected: &VerifiedSelectedCallableBatchMapV1,
        completion: Rc<VerifiedFunctionCompletionV1>,
        terminal_relations: BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    ) -> Result<(), CallablePhysicalHeaderIssueV1> {
        self.push_completion_with_top_level_input(
            declaration,
            selected,
            completion,
            terminal_relations,
            None,
        )
    }

    pub(super) fn push_completion_with_top_level_input(
        &mut self,
        declaration: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
        selected: &VerifiedSelectedCallableBatchMapV1,
        completion: Rc<VerifiedFunctionCompletionV1>,
        terminal_relations: BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
        top_level_input: Option<top_level::VerifiedTopLevelScalarInputV1>,
    ) -> Result<(), CallablePhysicalHeaderIssueV1> {
        let batch_slot = declaration.batch_slot();
        let result = validate_result(completion.as_ref(), declaration.owner(), batch_slot)?;
        let role = selected
            .role_for_batch_slot(batch_slot)
            .ok_or(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable)?;
        match (
            selected.key_for_batch_slot(batch_slot),
            top_level_input.as_ref(),
        ) {
            (Some(key @ SelectedNormalCallableKeyV1::TopLevel(_)), Some(input)) => {
                input.validate_attachment(
                    batch_slot,
                    declaration.owner(),
                    declaration.identity(),
                    key,
                )?;
                input.validate_declaration_origin(declaration.function_origin())?;
            }
            (Some(SelectedNormalCallableKeyV1::Cataloged(_)), None) => {}
            _ => {
                return Err(CallablePhysicalHeaderIssueV1::ParameterCoverage {
                    _batch_slot: batch_slot,
                })
            }
        }
        self.rows.push(VerifiedCallableResultContractRowV1 {
            batch_slot,
            owner: declaration.owner(),
            identity: declaration.identity().clone(),
            role,
            result,
            completion,
            top_level_input,
            terminal_relations: Rc::new(terminal_relations),
        });
        Ok(())
    }

    pub(super) fn take_main_child_row(
        &mut self,
        row: SelectedCallableBatchMapRowRefV1<'_>,
    ) -> Option<VerifiedCallableResultContractRowV1> {
        let index = self.rows.iter().position(|candidate| {
            candidate.batch_slot == row.batch_slot()
                && candidate.identity().same_as(row.identity())
                && candidate.role == row.role()
        })?;
        Some(self.rows.remove(index))
    }

    pub(super) fn peek_main_child_result(
        &self,
        row: SelectedCallableBatchMapRowRefV1<'_>,
    ) -> Option<Option<ExactTrivialScalarAbiV1>> {
        self.rows.iter().find_map(|candidate| {
            (candidate.batch_slot() == row.batch_slot()
                && candidate.identity().same_as(row.identity())
                && candidate.role() == row.role())
            .then_some(candidate.result())
        })
    }

    pub(super) fn finish(mut self) -> Self {
        self.rows.sort_by_key(|row| row.batch_slot);
        self
    }

    pub(super) fn completion_index(
        &self,
    ) -> std::collections::BTreeMap<FunctionOwnerIdV1, Rc<VerifiedFunctionCompletionV1>> {
        self.rows
            .iter()
            .map(|row| (row.owner, Rc::clone(&row.completion)))
            .collect()
    }

    pub(super) fn terminal_relation_index(
        &self,
    ) -> std::collections::BTreeMap<
        FunctionOwnerIdV1,
        Rc<BTreeMap<SourceStmtSiteV1, TerminalRelationV1>>,
    > {
        self.rows
            .iter()
            .filter(|row| !row.terminal_relations.is_empty())
            .map(|row| (row.owner, Rc::clone(&row.terminal_relations)))
            .collect()
    }

    /// Retain the collected rows as the sealed cohort. This checks
    /// correspondence, never reissues source meaning or infers absent rows.
    pub(super) fn seal(
        mut self,
    ) -> Result<VerifiedCallableResultContractCohortV1, CallableResultContractIssueV1> {
        for (index, row) in self.rows.iter().enumerate() {
            if self.rows[..index]
                .iter()
                .any(|earlier| earlier.batch_slot == row.batch_slot)
            {
                return Err(CallableResultContractIssueV1::DuplicateBatchSlot {
                    _batch_slot: row.batch_slot,
                });
            }
            if row.completion.owner() != row.owner {
                return Err(CallableResultContractIssueV1::CompletionOwnerMismatch {
                    _batch_slot: row.batch_slot,
                });
            }
        }
        self.rows.sort_by_key(|row| row.batch_slot);
        Ok(VerifiedCallableResultContractCohortV1 {
            rows: self.rows.into_boxed_slice(),
            completed_context: None,
            named_array_emissions: Box::new([]),
        })
    }
}

pub(super) fn preflight_declaration(
    declaration: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
    selected: &VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<bool, CallablePhysicalHeaderIssueV1> {
    let batch_slot = declaration.batch_slot();
    if !matches!(
        selected.key_for_batch_slot(batch_slot),
        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
            _
        ))
    ) {
        return Ok(false);
    }
    let mut contracts = parameter_contracts
        .iter()
        .filter(|row| row.batch_slot == batch_slot);
    let Some(contract) = contracts.next() else {
        return Ok(false);
    };
    if contracts.next().is_some() {
        return Err(CallablePhysicalHeaderIssueV1::DuplicateParameterContract {
            _batch_slot: batch_slot,
        });
    }
    if contract.owner != declaration.owner() {
        return Err(CallablePhysicalHeaderIssueV1::ParameterOwnerMismatch {
            _batch_slot: batch_slot,
        });
    }
    if contract.parameters.len() != declaration.parameter_count() as usize {
        return Err(CallablePhysicalHeaderIssueV1::ParameterCoverage {
            _batch_slot: batch_slot,
        });
    }
    if selected.role_for_batch_slot(batch_slot).is_none() {
        return Err(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable);
    }
    Ok(true)
}

pub(super) fn validate_result(
    completion: &VerifiedFunctionCompletionV1,
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
) -> Result<Option<ExactTrivialScalarAbiV1>, CallablePhysicalHeaderIssueV1> {
    let result = match completion.function_exit_contract().declared_result() {
        DeclaredFunctionResultContractV1::Annotated(name) => {
            Some(ExactTrivialScalarAbiV1::classify(name).ok_or_else(|| {
                CallablePhysicalHeaderIssueV1::UnsupportedResultAnnotation {
                    _batch_slot: batch_slot,
                    _name: name.clone(),
                }
            })?)
        }
        DeclaredFunctionResultContractV1::Unannotated | DeclaredFunctionResultContractV1::Void => {
            None
        }
    };
    if completion.owner() != owner {
        return Err(CallablePhysicalHeaderIssueV1::CompletionOwnerMismatch {
            _batch_slot: batch_slot,
        });
    }
    Ok(result)
}

impl VerifiedCallableResultContractCohortV1 {
    /// Move the same package context after successful consumption. This checks
    /// correspondence, never reissues source meaning or infers absent rows.
    pub(super) fn retain_completed_context(
        mut self,
        selected: super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        parameters: Box<[super::model::OwnedCallableParameterContractDeclarationV1]>,
    ) -> Result<Self, super::NormalCallableSemanticPackageInstallIssueV1> {
        use super::NormalCallableSemanticPackageInstallIssueV1 as Issue;
        if self.completed_context.is_some() {
            return Err(Issue::ResultContractMismatch);
        }
        for row in &self.rows {
            if selected.key_for_batch_slot(row.batch_slot).is_none()
                || !selected
                    .identity_for_batch_slot(row.batch_slot)
                    .is_some_and(|identity| identity.same_as(&row.identity))
                || selected.role_for_batch_slot(row.batch_slot) != Some(row.role)
            {
                return Err(Issue::ResultContractMismatch);
            }
            if let Some(key @ SelectedNormalCallableKeyV1::TopLevel(_)) =
                selected.key_for_batch_slot(row.batch_slot)
            {
                row.top_level_input
                    .as_ref()
                    .ok_or(Issue::MissingParameterContract)?
                    .validate_attachment(row.batch_slot, row.owner, &row.identity, key)
                    .map_err(|_| Issue::ResultContractMismatch)?;
                if parameters
                    .iter()
                    .any(|parameter| parameter.batch_slot == row.batch_slot)
                {
                    return Err(Issue::DuplicateParameterContract);
                }
                continue;
            }
            if row.top_level_input.is_some() {
                return Err(Issue::ResultContractMismatch);
            }
            let mut matches = parameters.iter().filter(|p| p.batch_slot == row.batch_slot);
            let parameter = matches.next().ok_or(Issue::MissingParameterContract)?;
            if matches.next().is_some() {
                return Err(Issue::DuplicateParameterContract);
            }
            if parameter.owner != row.owner {
                return Err(Issue::ParameterContractOwnerMismatch);
            }
        }
        self.completed_context = Some((selected, parameters));
        Ok(self)
    }

    pub(crate) fn completed_result(
        &self,
        key: &crate::mir::builder::SelectedNormalCallableKeyV1,
    ) -> Option<CallableResultContractRefV1<'_>> {
        let (selected, _) = self.completed_context.as_ref()?;
        self.row(selected.batch_slot(key)?).map(|row| row.borrow())
    }

    /// Resolve an already-selected catalog key to its invocation-local owner.
    /// This is a correspondence lookup for physical caller attribution; it
    /// does not issue or reconstruct a source key.
    pub(crate) fn owner_for_canonical_key(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<FunctionOwnerIdV1> {
        let (selected, _) = self.completed_context.as_ref()?;
        match key.namespace() {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::FreeFunction => {
                let mut matches = self.rows.iter().filter(|row| {
                    row.top_level_input
                        .as_ref()
                        .is_some_and(|input| input.matches_canonical_key(key))
                });
                let row = matches.next()?;
                (matches.next().is_none()).then_some(row.owner())
            }
            _ => {
                let selected_key = SelectedNormalCallableKeyV1::Cataloged(key.clone());
                let batch_slot = selected.batch_slot(&selected_key)?;
                self.row(batch_slot).map(|row| row.owner())
            }
        }
    }
}

#[cfg(test)]
#[path = "completed_result_context_tests.rs"]
mod completed_context_tests;

impl VerifiedCallableResultContractCohortV1 {
    pub(super) fn retain_named_array_emissions(
        mut self,
        rows: Box<[super::EmittedNamedArrayRequirementV1]>,
    ) -> Result<Self, super::NormalCallableSemanticPackageInstallIssueV1> {
        for row in &rows {
            let key = SelectedNormalCallableKeyV1::Cataloged(row.caller().clone());
            if self.completed_result(&key).map(|contract| contract.owner()) != Some(row.owner()) {
                return Err(
                    super::NormalCallableSemanticPackageInstallIssueV1::CoreMethodSource(
                        "named-array-completed-owner-mismatch".into(),
                    ),
                );
            }
        }
        self.named_array_emissions = rows;
        Ok(self)
    }

    pub(crate) fn named_array_emissions(&self) -> &[super::EmittedNamedArrayRequirementV1] {
        &self.named_array_emissions
    }

    pub(crate) fn take_named_array_emissions(
        &mut self,
    ) -> Box<[super::EmittedNamedArrayRequirementV1]> {
        std::mem::take(&mut self.named_array_emissions)
    }
}

#[path = "result_contract_top_level.rs"]
mod top_level;
pub(super) use top_level::{issue_top_level_scalar_input_v1, VerifiedTopLevelScalarInputV1};
