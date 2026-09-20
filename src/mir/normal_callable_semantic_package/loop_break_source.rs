//! Complete LoopBreak source coverage for one semantic callable package.
//!
//! The package owner observes every resolver batch row exactly once. The
//! Builder-owned source/Facts issuer supplies the only candidate or typed
//! absence; this module only joins those rows to batch slots and rejects
//! incomplete/foreign observations.

use crate::mir::builder::{
    issue_callable_loop_break_source_facts_v1, CallableLoopBreakSourceFactsDispositionV1,
    CallableLoopBreakSourceFactsIssueV1, GenericLoopFactsPolicyFrameV1,
    VerifiedCallableLoopBreakSourceFactsV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableSemanticBatchLoanErrorV1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{FunctionOriginV1, FunctionOwnerIdV1};

#[derive(Debug)]
pub(super) enum LoopBreakSourcePackageIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    OwnerMismatch {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
    },
    FunctionOriginMismatch {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
    },
    Unresolved {
        batch_slot: u32,
        issue: CallableLoopBreakSourceFactsIssueV1,
    },
    Rejected {
        batch_slot: u32,
        issue: CallableLoopBreakSourceFactsIssueV1,
    },
}

#[derive(Debug)]
pub(super) enum OwnedLoopBreakSourcePackageRowV1 {
    Candidate(VerifiedCallableLoopBreakSourceFactsV1),
    SupportedNonCandidate { loop_count: usize },
}

#[derive(Debug)]
pub(super) struct OwnedLoopBreakSourcePackageRowEnvelopeV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    row: OwnedLoopBreakSourcePackageRowV1,
}

#[derive(Debug)]
pub(super) struct VerifiedLoopBreakSourcePackageV1 {
    rows: Box<[OwnedLoopBreakSourcePackageRowEnvelopeV1]>,
}

impl VerifiedLoopBreakSourcePackageV1 {
    pub(super) fn rows(&self) -> &[OwnedLoopBreakSourcePackageRowEnvelopeV1] {
        &self.rows
    }

    #[cfg(test)]
    pub(super) fn candidate_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| matches!(row.row, OwnedLoopBreakSourcePackageRowV1::Candidate(_)))
            .count()
    }
}

pub(super) fn issue_loop_break_source_package_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    policy: GenericLoopFactsPolicyFrameV1,
) -> Result<VerifiedLoopBreakSourcePackageV1, LoopBreakSourcePackageIssueV1> {
    let mut rows = Vec::with_capacity(batch.declarations().len());
    for declaration in batch.declarations() {
        let batch_slot = declaration.batch_slot();
        let owner = declaration.owner();
        let function_origin = declaration.function_origin();
        let disposition = batch
            .with_lowering_input(batch_slot, |input| {
                issue_callable_loop_break_source_facts_v1(input, policy)
            })
            .map_err(LoopBreakSourcePackageIssueV1::BatchLoan)?;
        let row = match disposition {
            CallableLoopBreakSourceFactsDispositionV1::Candidate(facts) => {
                if facts.owner() != owner {
                    return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot, owner });
                }
                if facts.function_origin() != function_origin {
                    return Err(LoopBreakSourcePackageIssueV1::FunctionOriginMismatch {
                        batch_slot,
                        owner,
                    });
                }
                OwnedLoopBreakSourcePackageRowV1::Candidate(facts)
            }
            CallableLoopBreakSourceFactsDispositionV1::SupportedNonCandidate {
                owner: observed_owner,
                loop_count,
            } => {
                if observed_owner != owner {
                    return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot, owner });
                }
                OwnedLoopBreakSourcePackageRowV1::SupportedNonCandidate { loop_count }
            }
            CallableLoopBreakSourceFactsDispositionV1::Unresolved {
                owner: observed_owner,
                issue,
            } => {
                if observed_owner != owner {
                    return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot, owner });
                }
                return Err(LoopBreakSourcePackageIssueV1::Unresolved { batch_slot, issue });
            }
            CallableLoopBreakSourceFactsDispositionV1::Rejected {
                owner: observed_owner,
                issue,
            } => {
                if observed_owner != owner {
                    return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot, owner });
                }
                return Err(LoopBreakSourcePackageIssueV1::Rejected { batch_slot, issue });
            }
        };
        rows.push(OwnedLoopBreakSourcePackageRowEnvelopeV1 {
            batch_slot,
            owner,
            function_origin,
            row,
        });
    }
    if rows.len() != batch.declarations().len() {
        return Err(LoopBreakSourcePackageIssueV1::BatchLoan(
            ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage,
        ));
    }
    Ok(VerifiedLoopBreakSourcePackageV1 {
        rows: rows.into_boxed_slice(),
    })
}
