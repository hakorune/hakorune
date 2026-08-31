//! Source-backed callable header cohort for the pre-physical boundary.
//!
//! This product is deliberately not a runtime ABI. It co-seals the existing
//! formal parameter rows with an explicit source result and the sole
//! Completion verifier so later physical work cannot receive only half of a
//! callable signature.

use crate::mir::callable_semantic_batch::ResolvedCallableSemanticBatchLoanErrorV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::result_contract::{CallableResultContractRefV1, VerifiedCallableResultContractCohortV1};

#[derive(Debug)]
pub(in crate::mir) enum CallablePhysicalHeaderIssueV1 {
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
    },
    SelectedBatchSlotUnavailable,
    DuplicateParameterContract {
        _batch_slot: u32,
    },
    ParameterCoverage {
        _batch_slot: u32,
    },
    ParameterOwnerMismatch {
        _batch_slot: u32,
    },
    UnsupportedResultAnnotation {
        _batch_slot: u32,
        _name: Box<str>,
    },
    Completion {
        _batch_slot: u32,
        _issue: FunctionCompletionVerificationErrorV1,
    },
    CompletionOwnerMismatch {
        _batch_slot: u32,
    },
}

#[derive(Debug)]
pub(super) struct VerifiedCallablePhysicalHeaderCohortV1 {
    rows: Box<[VerifiedCallablePhysicalHeaderRowV1]>,
}

#[derive(Debug)]
pub(super) struct VerifiedCallablePhysicalHeaderRowV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    result: ExactTrivialScalarAbiV1,
}

#[derive(Clone, Copy)]
pub(crate) struct CallablePhysicalHeaderRefV1<'a> {
    row: &'a VerifiedCallablePhysicalHeaderRowV1,
    result_contract: CallableResultContractRefV1<'a>,
}

impl CallablePhysicalHeaderRefV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.row.owner
    }

    pub(crate) const fn result(&self) -> ExactTrivialScalarAbiV1 {
        self.row.result
    }

    pub(crate) const fn completion_owner(&self) -> FunctionOwnerIdV1 {
        self.result_contract.completion_owner()
    }

    pub(crate) const fn completion_target_function(
        &self,
    ) -> crate::mir::resolved_semantics::RegionId {
        self.result_contract.completion_target_function()
    }

    pub(crate) const fn completion_returns_value(&self) -> bool {
        self.result_contract.completion_returns_value()
    }

    pub(crate) fn completion_explicit_site_count(&self) -> usize {
        self.result_contract.completion_explicit_site_count()
    }

    pub(crate) fn completion_cleanup_is_empty(&self) -> bool {
        self.result_contract.completion_cleanup_is_empty()
    }
}

impl VerifiedCallablePhysicalHeaderCohortV1 {
    pub(super) fn row<'a>(
        &'a self,
        batch_slot: u32,
        result_contracts: &'a VerifiedCallableResultContractCohortV1,
    ) -> Option<CallablePhysicalHeaderRefV1<'a>> {
        let row = self
            .rows
            .binary_search_by_key(&batch_slot, |row| row.batch_slot)
            .ok()
            .map(|index| &self.rows[index])?;
        let result_contract = result_contracts.row(batch_slot)?;
        if result_contract.owner() != row.owner || result_contract.result() != Some(row.result) {
            return None;
        }
        Some(CallablePhysicalHeaderRefV1 {
            row,
            result_contract: result_contract.borrow(),
        })
    }
}

pub(super) fn issue_callable_physical_header_from_result_contract_v1(
    result_contracts: &VerifiedCallableResultContractCohortV1,
) -> VerifiedCallablePhysicalHeaderCohortV1 {
    let mut rows = Vec::new();
    for contract in result_contracts.rows() {
        let Some(result) = contract.result() else {
            continue;
        };
        rows.push(VerifiedCallablePhysicalHeaderRowV1 {
            batch_slot: contract.batch_slot(),
            owner: contract.owner(),
            result,
        });
    }
    rows.sort_by_key(|row| row.batch_slot);
    VerifiedCallablePhysicalHeaderCohortV1 {
        rows: rows.into_boxed_slice(),
    }
}
