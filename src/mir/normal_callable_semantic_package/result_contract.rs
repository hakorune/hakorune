//! Package-owned retention of one verified callable result/completion row.
//!
//! The completion seed is issued once by the resolver-owned verifier. This
//! cohort keeps the original non-`Clone` Completion alive for every selected
//! ordinary Cataloged callable after S6C takes its exclusive seed. Successful
//! Dynamic uses the same borrowed view over its canonical authority Completion.
//! Physical headers borrow this product and never become a second Completion
//! owner.

use crate::mir::builder::SelectedCallableConsumptionRoleV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
#[cfg(test)]
use crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::parser::CallableDeclarationIdentityV1;

use super::completion_seed::VerifiedCallableCompletionSeedV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CallableResultContractIssueV1 {
    DuplicateBatchSlot { _batch_slot: u32 },
    CompletionOwnerMismatch { _batch_slot: u32 },
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableResultContractCohortV1 {
    rows: Box<[VerifiedCallableResultContractRowV1]>,
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
    completion: VerifiedFunctionCompletionV1,
    terminal_relation: Option<TerminalRelationV1>,
}

#[derive(Clone, Copy)]
pub(crate) struct CallableResultContractRefV1<'a> {
    owner: FunctionOwnerIdV1,
    identity: &'a CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
    result: Option<ExactTrivialScalarAbiV1>,
    completion: &'a VerifiedFunctionCompletionV1,
    terminal_relation: Option<&'a TerminalRelationV1>,
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

    pub(super) fn borrow(&self) -> CallableResultContractRefV1<'_> {
        CallableResultContractRefV1::from_completion(
            self.owner,
            &self.identity,
            self.role,
            self.result,
            &self.completion,
            self.terminal_relation.as_ref(),
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
        terminal_relation: Option<&'a TerminalRelationV1>,
    ) -> Self {
        Self {
            owner,
            identity,
            role,
            result,
            completion,
            terminal_relation,
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

    #[cfg(test)]
    pub(crate) fn declared_result(&self) -> &DeclaredFunctionResultContractV1 {
        self.completion.function_exit_contract().declared_result()
    }

    /// Absence does not infer Unit, an ABI kind, or empty cleanup.
    pub(crate) const fn terminal_relation(&self) -> Option<&'a TerminalRelationV1> {
        self.terminal_relation
    }

    /// Borrow the issued product; callers must not infer missing obligations
    /// from a partial summary or absent Home analysis.
    pub(crate) const fn completion(&self) -> &'a VerifiedFunctionCompletionV1 {
        self.completion
    }
}

pub(super) fn issue_callable_result_contract_cohort_v1(
    seeds: Vec<VerifiedCallableCompletionSeedV1>,
) -> Result<VerifiedCallableResultContractCohortV1, CallableResultContractIssueV1> {
    let mut rows = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let (batch_slot, owner, identity, role, result, completion, terminal_relation) =
            seed.into_parts();
        if rows
            .iter()
            .any(|row: &VerifiedCallableResultContractRowV1| row.batch_slot == batch_slot)
        {
            return Err(CallableResultContractIssueV1::DuplicateBatchSlot {
                _batch_slot: batch_slot,
            });
        }
        if completion.owner() != owner {
            return Err(CallableResultContractIssueV1::CompletionOwnerMismatch {
                _batch_slot: batch_slot,
            });
        }
        rows.push(VerifiedCallableResultContractRowV1 {
            batch_slot,
            owner,
            identity,
            role,
            result,
            completion,
            terminal_relation,
        });
    }
    rows.sort_by_key(|row| row.batch_slot);
    Ok(VerifiedCallableResultContractCohortV1 {
        rows: rows.into_boxed_slice(),
        completed_context: None,
    })
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
}

#[cfg(test)]
#[path = "completed_result_context_tests.rs"]
mod completed_context_tests;
