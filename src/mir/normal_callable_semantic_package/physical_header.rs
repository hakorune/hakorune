//! Source-backed callable header cohort for the pre-physical boundary.
//!
//! This product is deliberately not a runtime ABI. It co-seals the existing
//! formal parameter rows with an explicit source result and the sole
//! Completion verifier so later physical work cannot receive only half of a
//! callable signature.

use crate::mir::callable_semantic_batch::ResolvedCallableSemanticBatchLoanErrorV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{
    FunctionCompletionVerificationErrorV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};

#[derive(Debug)]
pub(super) enum CallablePhysicalHeaderIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    SelectedBatchSlotUnavailable,
    DuplicateParameterContract {
        batch_slot: u32,
    },
    ParameterCoverage {
        batch_slot: u32,
    },
    ParameterOwnerMismatch {
        batch_slot: u32,
    },
    UnsupportedResultAnnotation {
        batch_slot: u32,
        name: Box<str>,
    },
    Completion {
        batch_slot: u32,
        issue: FunctionCompletionVerificationErrorV1,
    },
    CompletionOwnerMismatch {
        batch_slot: u32,
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
    completion: VerifiedFunctionCompletionV1,
}

#[derive(Clone, Copy)]
pub(crate) struct CallablePhysicalHeaderRefV1<'a> {
    row: &'a VerifiedCallablePhysicalHeaderRowV1,
}

impl VerifiedCallablePhysicalHeaderCohortV1 {
    pub(super) fn row(&self, batch_slot: u32) -> Option<&VerifiedCallablePhysicalHeaderRowV1> {
        self.rows
            .binary_search_by_key(&batch_slot, |row| row.batch_slot)
            .ok()
            .map(|index| &self.rows[index])
    }
}

impl VerifiedCallablePhysicalHeaderRowV1 {
    pub(super) fn borrow(&self) -> CallablePhysicalHeaderRefV1<'_> {
        CallablePhysicalHeaderRefV1 { row: self }
    }
}

impl CallablePhysicalHeaderRefV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.row.owner
    }

    pub(crate) const fn result(&self) -> ExactTrivialScalarAbiV1 {
        self.row.result
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

pub(super) fn issue_callable_physical_header_from_seeds_v1(
    seeds: Vec<super::completion_seed::VerifiedCallableCompletionSeedV1>,
) -> VerifiedCallablePhysicalHeaderCohortV1 {
    let mut rows = Vec::new();
    for seed in seeds {
        let Some(result) = seed.result() else {
            continue;
        };
        let batch_slot = seed.batch_slot();
        rows.push(VerifiedCallablePhysicalHeaderRowV1 {
            batch_slot,
            owner: seed.owner(),
            result,
            completion: seed.take_completion(),
        });
    }
    rows.sort_by_key(|row| row.batch_slot);
    VerifiedCallablePhysicalHeaderCohortV1 {
        rows: rows.into_boxed_slice(),
    }
}
