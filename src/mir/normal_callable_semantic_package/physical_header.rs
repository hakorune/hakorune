//! Source-backed callable header cohort for the pre-physical boundary.
//!
//! This product is deliberately not a runtime ABI. It co-seals the existing
//! formal parameter rows with an explicit source result and the sole
//! Completion verifier so later physical work cannot receive only half of a
//! callable signature.

use crate::mir::callable_semantic_batch::ResolvedCallableSemanticBatchLoanErrorV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_control_flow::{DeclaredFunctionResultContractV1, FunctionCompletionVerificationErrorV1, VerifiedFunctionCompletionV1};
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
    /// R2: a declared scalar result whose exit proves the other scalar class.
    DeclaredResultDisagrees {
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
    result: ExactTrivialScalarAbiV1,
    result_contract: CallableResultContractRefV1<'a>,
}

impl<'a> CallablePhysicalHeaderRefV1<'a> {
    pub(super) fn from_result_contract(
        result_contract: CallableResultContractRefV1<'a>,
    ) -> Option<Self> {
        Some(Self {
            result: result_contract.result()?,
            result_contract,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.result_contract.owner()
    }

    pub(crate) const fn result(&self) -> ExactTrivialScalarAbiV1 {
        self.result
    }

    pub(crate) fn completion(
        &self,
    ) -> &'a crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        self.result_contract.completion()
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
            result: row.result,
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

/// O1a: the declaration agrees with a proven scalar source of the kind `want`: it names that kind
/// in the one table of declared spellings, or it is absent. Which scalar the body returns is the
/// solver's body proof (R2).
pub(super) fn declared_scalar_agrees_v1(
    result: Option<ExactTrivialScalarAbiV1>,
    completion: &VerifiedFunctionCompletionV1,
    want: ExactTrivialScalarAbiV1,
) -> bool {
    use crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1 as Declared;
    match (result, completion.function_exit_contract().declared_result()) {
        (Some(abi), DeclaredFunctionResultContractV1::Annotated(name)) =>
            abi == want && Declared::classify(name) == Some(Declared::Scalar(want)),
        (None, DeclaredFunctionResultContractV1::Unannotated) => true,
        _ => false,
    }
}

/// R2: no exit may prove a scalar class other than the declared one. An exit that proves no scalar
/// (Unit, a Text or Map value, an opaque call) is left to the later stages that own it.
pub(super) fn require_declared_result_agrees_with_exits_v1(
    result: Option<ExactTrivialScalarAbiV1>,
    terminals: &std::collections::BTreeMap<crate::mir::resolved_semantics::SourceStmtSiteV1, crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1>,
    completion: &VerifiedFunctionCompletionV1,
    batch_slot: u32,
) -> Result<(), CallablePhysicalHeaderIssueV1> {
    use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind as Kind;
    // S3: an owned `String` declaration admits no exit that proves a scalar.
    if let DeclaredFunctionResultContractV1::Annotated(name) = completion.function_exit_contract().declared_result() {
        if crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1::classify(name)
            == Some(crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1::OwnedText)
            && terminals.values().any(|terminal| terminal.scalar_class().is_some())
        {
            return Err(CallablePhysicalHeaderIssueV1::DeclaredResultDisagrees { _batch_slot: batch_slot, _name: name.clone() });
        }
    }
    let Some(abi) = result else { return Ok(()) };
    let declared = match abi {
        ExactTrivialScalarAbiV1::I64 => Kind::Integer,
        ExactTrivialScalarAbiV1::Bool => Kind::Bool,
    };
    if terminals.values().any(|terminal| terminal.scalar_class().is_some_and(|class| class != declared)) {
        return Err(CallablePhysicalHeaderIssueV1::DeclaredResultDisagrees { _batch_slot: batch_slot, _name: abi.source_type_name().into() });
    }
    Ok(())
}

pub(super) fn validate_result(
    completion: &VerifiedFunctionCompletionV1,
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
    declared_box: Option<&str>,
) -> Result<Option<ExactTrivialScalarAbiV1>, CallablePhysicalHeaderIssueV1> {
    let result = match completion.function_exit_contract().declared_result() {
        DeclaredFunctionResultContractV1::Annotated(name) => {
            crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1::classify_resolved(name, declared_box)
                .ok_or_else(|| CallablePhysicalHeaderIssueV1::UnsupportedResultAnnotation {
                    _batch_slot: batch_slot,
                    _name: name.clone(),
                })?.scalar_abi()
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

pub(super) fn validate_declared_result(
    completion: &VerifiedFunctionCompletionV1, owner: FunctionOwnerIdV1, batch_slot: u32,
    borrowed: Option<&crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>,
    declared_box: Option<&str>,
) -> Result<Option<ExactTrivialScalarAbiV1>, CallablePhysicalHeaderIssueV1> {
    let Some(read) = borrowed else { return validate_result(completion, owner, batch_slot, declared_box); };
    if completion.owner() == owner && read.owner() == owner
        && matches!(completion.function_exit_contract().declared_result(),
            DeclaredFunctionResultContractV1::Annotated(name) if name.as_ref() == read.declared_type()) {
        Ok(None)
    } else { Err(CallablePhysicalHeaderIssueV1::UnsupportedResultAnnotation {
        _batch_slot: batch_slot, _name: read.declared_type().into(),
    }) }
}
