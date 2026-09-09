//! One package-owned Completion verification pass for ordinary and S6C rows.
//!
//! The seed cohort is private to the package issuer.  It prevents the generic
//! header issuer and the S6C child issuer from independently verifying and
//! owning the same `VerifiedFunctionCompletionV1`. A successful Dynamic slot
//! validates its already-retained Completion and emits no ordinary seed.

use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{
    DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::parser::CallableDeclarationIdentityV1;
use std::rc::Rc;

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
    completion: Rc<VerifiedFunctionCompletionV1>,
    terminal_relation: Option<Rc<TerminalRelationV1>>,
}

impl VerifiedCallableCompletionSeedV1 {
    pub(super) const fn batch_slot(&self) -> u32 {
        self.batch_slot
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

    pub(super) fn into_parts(
        self,
    ) -> (
        u32,
        FunctionOwnerIdV1,
        CallableDeclarationIdentityV1,
        crate::mir::builder::SelectedCallableConsumptionRoleV1,
        Option<ExactTrivialScalarAbiV1>,
        Rc<VerifiedFunctionCompletionV1>,
        Option<Rc<TerminalRelationV1>>,
    ) {
        (
            self.batch_slot,
            self.owner,
            self.identity,
            self.role,
            self.result,
            self.completion,
            self.terminal_relation,
        )
    }
}

#[derive(Debug)]
pub(super) struct VerifiedCallableCompletionSeedCohortV1 {
    rows: Vec<VerifiedCallableCompletionSeedV1>,
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

impl VerifiedCallableCompletionSeedCohortV1 {
    pub(super) fn new() -> Self {
        Self { rows: Vec::new() }
    }

    // Called inside the same source loan as ordinary-New candidate issuance.
    pub(super) fn push_completion(
        &mut self,
        declaration: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
        selected: &VerifiedSelectedCallableBatchMapV1,
        completion: VerifiedFunctionCompletionV1,
        terminal_relation: Option<TerminalRelationV1>,
    ) -> Result<(), CallablePhysicalHeaderIssueV1> {
        let batch_slot = declaration.batch_slot();
        let result = validate_result(&completion, declaration.owner(), batch_slot)?;
        let role = selected
            .role_for_batch_slot(batch_slot)
            .ok_or(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable)?;
        self.rows.push(VerifiedCallableCompletionSeedV1 {
            batch_slot,
            owner: declaration.owner(),
            identity: declaration.identity().clone(),
            role,
            result,
            completion: Rc::new(completion),
            terminal_relation: terminal_relation.map(Rc::new),
        });
        Ok(())
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
    ) -> std::collections::BTreeMap<FunctionOwnerIdV1, Rc<TerminalRelationV1>> {
        self.rows
            .iter()
            .filter_map(|row| {
                row.terminal_relation
                    .as_ref()
                    .map(|relation| (row.owner, Rc::clone(relation)))
            })
            .collect()
    }
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
