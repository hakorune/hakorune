//! Source-backed callable header cohort for the pre-physical boundary.
//!
//! This product is deliberately not a runtime ABI. It co-seals the existing
//! formal parameter rows with an explicit source result and the sole
//! Completion verifier so later physical work cannot receive only half of a
//! callable signature.

use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::{
    ResolvedCallableSemanticBatchLoanErrorV1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
    VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};

use super::model::OwnedCallableParameterContractDeclarationV1;
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;

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
    CompletionNotValue {
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

pub(super) fn issue_callable_physical_header_cohort_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<Option<VerifiedCallablePhysicalHeaderCohortV1>, CallablePhysicalHeaderIssueV1> {
    let mut rows = Vec::new();
    let mut missing_result_annotation = false;
    for selected_key in selected.keys() {
        if !matches!(selected_key, SelectedNormalCallableKeyV1::Cataloged(_)) {
            continue;
        }
        let batch_slot = selected
            .batch_slot(selected_key)
            .ok_or(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable)?;
        let declaration = batch
            .declarations()
            .find(|row| row.batch_slot() == batch_slot)
            .ok_or(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable)?;
        let mut contracts = parameter_contracts
            .iter()
            .filter(|row| row.batch_slot == batch_slot);
        let Some(contract) = contracts.next() else {
            return Ok(None);
        };
        if contracts.next().is_some() {
            return Err(CallablePhysicalHeaderIssueV1::DuplicateParameterContract { batch_slot });
        }
        if contract.owner != declaration.owner() {
            return Err(CallablePhysicalHeaderIssueV1::ParameterOwnerMismatch { batch_slot });
        }
        if contract.parameters.len() != declaration.parameter_count() as usize {
            return Err(CallablePhysicalHeaderIssueV1::ParameterCoverage { batch_slot });
        }

        let maybe_result = batch
            .with_lowering_input(batch_slot, |input| {
                let completion = verify_function_completion_v1(input).map_err(|issue| {
                    CallablePhysicalHeaderIssueV1::Completion { batch_slot, issue }
                })?;
                let declared = completion.function_exit_contract().declared_result();
                let name = match declared {
                    crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Annotated(
                        name,
                    ) => name.as_ref(),
                    _ => {
                        missing_result_annotation = true;
                        return Ok(None);
                    }
                };
                let result = ExactTrivialScalarAbiV1::classify(name).ok_or_else(|| {
                    CallablePhysicalHeaderIssueV1::UnsupportedResultAnnotation {
                        batch_slot,
                        name: name.into(),
                    }
                })?;
                if completion.owner() != input.owner() {
                    return Err(CallablePhysicalHeaderIssueV1::CompletionOwnerMismatch {
                        batch_slot,
                    });
                }
                if !completion.returns_value() {
                    return Err(CallablePhysicalHeaderIssueV1::CompletionNotValue { batch_slot });
                }
                Ok(Some((result, completion)))
            })
            .map_err(CallablePhysicalHeaderIssueV1::BatchLoan)??;
        let Some((result, completion)) = maybe_result else {
            continue;
        };
        rows.push(VerifiedCallablePhysicalHeaderRowV1 {
            batch_slot,
            owner: declaration.owner(),
            result,
            completion,
        });
    }
    if missing_result_annotation {
        return Ok(None);
    }
    rows.sort_by_key(|row| row.batch_slot);
    Ok(Some(VerifiedCallablePhysicalHeaderCohortV1 {
        rows: rows.into_boxed_slice(),
    }))
}
