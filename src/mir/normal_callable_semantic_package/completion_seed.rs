//! One package-owned Completion verification pass for ordinary and S6C rows.
//!
//! The seed cohort is private to the package issuer.  It prevents the generic
//! header issuer and the S6C child issuer from independently verifying and
//! owning the same `VerifiedFunctionCompletionV1`.

use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::parser::CallableDeclarationIdentityV1;

use super::model::OwnedCallableParameterContractDeclarationV1;
use super::physical_header::CallablePhysicalHeaderIssueV1;
use super::selected_mapping::{
    SelectedCallableBatchMapRowRefV1, VerifiedSelectedCallableBatchMapV1,
};

#[derive(Debug)]
pub(super) struct VerifiedCallableCompletionSeedV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    identity: CallableDeclarationIdentityV1,
    role: crate::mir::builder::SelectedCallableConsumptionRoleV1,
    result: Option<ExactTrivialScalarAbiV1>,
    completion: VerifiedFunctionCompletionV1,
}

impl VerifiedCallableCompletionSeedV1 {
    pub(super) const fn batch_slot(&self) -> u32 {
        self.batch_slot
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(super) const fn role(&self) -> crate::mir::builder::SelectedCallableConsumptionRoleV1 {
        self.role
    }

    pub(super) const fn result(&self) -> Option<ExactTrivialScalarAbiV1> {
        self.result
    }

    pub(super) fn take_completion(self) -> VerifiedFunctionCompletionV1 {
        self.completion
    }
}

#[derive(Debug)]
pub(super) struct VerifiedCallableCompletionSeedCohortV1 {
    rows: Vec<VerifiedCallableCompletionSeedV1>,
    missing_parameter_contract: bool,
}

impl VerifiedCallableCompletionSeedCohortV1 {
    pub(super) fn take_main_child_seed(
        &mut self,
        row: SelectedCallableBatchMapRowRefV1<'_>,
    ) -> Option<VerifiedCallableCompletionSeedV1> {
        let index = self.rows.iter().position(|seed| {
            seed.batch_slot == row.batch_slot()
                && seed.identity().same_as(row.identity())
                && seed.role == row.role()
        })?;
        Some(self.rows.remove(index))
    }

    pub(super) fn into_rows(self) -> Vec<VerifiedCallableCompletionSeedV1> {
        self.rows
    }

    pub(super) const fn missing_parameter_contract(&self) -> bool {
        self.missing_parameter_contract
    }

    pub(super) fn peek_main_child_result(
        &self,
        row: SelectedCallableBatchMapRowRefV1<'_>,
    ) -> Option<Option<ExactTrivialScalarAbiV1>> {
        self.rows.iter().find_map(|seed| {
            (seed.batch_slot() == row.batch_slot()
                && seed.identity().same_as(row.identity())
                && seed.role() == row.role())
            .then_some(seed.result())
        })
    }
}

pub(super) fn issue_callable_completion_seed_cohort_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<VerifiedCallableCompletionSeedCohortV1, CallablePhysicalHeaderIssueV1> {
    let mut rows = Vec::new();
    let mut missing_parameter_contract = false;
    for selected_key in selected.keys() {
        if !matches!(
            selected_key,
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(_)
        ) {
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
            missing_parameter_contract = true;
            continue;
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
        let role = selected
            .role_for_batch_slot(batch_slot)
            .ok_or(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable)?;
        let identity = declaration.identity().clone();
        let result = batch
            .with_lowering_input(batch_slot, |input| {
                let completion = verify_function_completion_v1(input).map_err(|issue| {
                    CallablePhysicalHeaderIssueV1::Completion { batch_slot, issue }
                })?;
                let result = match completion.function_exit_contract().declared_result() {
                    DeclaredFunctionResultContractV1::Annotated(name) => {
                        Some(ExactTrivialScalarAbiV1::classify(name).ok_or_else(|| {
                            CallablePhysicalHeaderIssueV1::UnsupportedResultAnnotation {
                                batch_slot,
                                name: name.clone(),
                            }
                        })?)
                    }
                    DeclaredFunctionResultContractV1::Unannotated
                    | DeclaredFunctionResultContractV1::Void => None,
                };
                if completion.owner() != input.owner() {
                    return Err(CallablePhysicalHeaderIssueV1::CompletionOwnerMismatch {
                        batch_slot,
                    });
                }
                if !completion.returns_value() {
                    return Err(CallablePhysicalHeaderIssueV1::CompletionNotValue { batch_slot });
                }
                Ok((result, completion))
            })
            .map_err(CallablePhysicalHeaderIssueV1::BatchLoan)??;
        rows.push(VerifiedCallableCompletionSeedV1 {
            batch_slot,
            owner: declaration.owner(),
            identity,
            role,
            result: result.0,
            completion: result.1,
        });
    }
    rows.sort_by_key(|row| row.batch_slot);
    Ok(VerifiedCallableCompletionSeedCohortV1 {
        rows,
        missing_parameter_contract,
    })
}
