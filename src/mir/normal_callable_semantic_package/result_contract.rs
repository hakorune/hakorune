//! Package-owned retention of one verified callable result/completion row.
//!
//! The completion seed is issued once by the resolver-owned verifier. This
//! cohort keeps the original non-`Clone` Completion alive for every selected
//! Cataloged callable after the S6C child has consumed its exclusive seed.
//! Physical headers borrow this product and never become a second Completion
//! owner.

use crate::mir::builder::SelectedCallableConsumptionRoleV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
#[cfg(test)]
use crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};
use crate::parser::CallableDeclarationIdentityV1;

use super::completion_seed::VerifiedCallableCompletionSeedV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CallableResultContractIssueV1 {
    DuplicateBatchSlot { _batch_slot: u32 },
    CompletionOwnerMismatch { _batch_slot: u32 },
}

#[derive(Debug)]
pub(super) struct VerifiedCallableResultContractCohortV1 {
    rows: Box<[VerifiedCallableResultContractRowV1]>,
}

#[derive(Debug)]
pub(super) struct VerifiedCallableResultContractRowV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    identity: CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
    result: Option<ExactTrivialScalarAbiV1>,
    completion: VerifiedFunctionCompletionV1,
}

#[derive(Clone, Copy)]
pub(crate) struct CallableResultContractRefV1<'a> {
    row: &'a VerifiedCallableResultContractRowV1,
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
        CallableResultContractRefV1 { row: self }
    }
}

impl CallableResultContractRefV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.row.owner
    }

    pub(crate) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.row.identity
    }

    pub(crate) const fn role(&self) -> SelectedCallableConsumptionRoleV1 {
        self.row.role
    }

    pub(crate) const fn result(&self) -> Option<ExactTrivialScalarAbiV1> {
        self.row.result
    }

    #[cfg(test)]
    pub(crate) fn declared_result(&self) -> &DeclaredFunctionResultContractV1 {
        self.row
            .completion
            .function_exit_contract()
            .declared_result()
    }

    pub(crate) const fn completion_owner(&self) -> FunctionOwnerIdV1 {
        self.row.completion.owner()
    }

    pub(crate) const fn completion_target_function(&self) -> RegionId {
        self.row.completion.target_function()
    }

    pub(crate) const fn completion_returns_value(&self) -> bool {
        self.row.completion.returns_value()
    }

    pub(crate) fn completion_explicit_site_count(&self) -> usize {
        self.row.completion.explicit_sites().len()
    }

    pub(crate) fn completion_cleanup_is_empty(&self) -> bool {
        self.row.completion.cleanup().crossed_scopes().is_empty()
    }
}

pub(super) fn issue_callable_result_contract_cohort_v1(
    seeds: Vec<VerifiedCallableCompletionSeedV1>,
) -> Result<VerifiedCallableResultContractCohortV1, CallableResultContractIssueV1> {
    let mut rows = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let (batch_slot, owner, identity, role, result, completion) = seed.into_parts();
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
        });
    }
    rows.sort_by_key(|row| row.batch_slot);
    Ok(VerifiedCallableResultContractCohortV1 {
        rows: rows.into_boxed_slice(),
    })
}
