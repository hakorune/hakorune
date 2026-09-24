//! Complete LoopBreak source coverage for one semantic callable package.
//!
//! The package owner observes every resolver batch row exactly once. The
//! Builder-owned source/Facts issuer supplies the only candidate or typed
//! absence; this module only joins those rows to batch slots and rejects
//! incomplete/foreign observations.

use crate::mir::builder::{
    issue_callable_loop_break_source_facts_v1, CallableLoopBreakSourceFactsDispositionV1,
    CallableLoopBreakSourceFactsIssueV1, LoopFactsPolicyFrameV1,
    VerifiedCallableLoopBreakCompositeSourceFactsV1, VerifiedCallableLoopBreakSourceFactsV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableSemanticBatchLoanErrorV1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{FunctionOriginV1, FunctionOwnerIdV1, SourceStmtSiteV1};
use std::collections::BTreeSet;

#[derive(Debug)]
pub(in crate::mir) enum LoopBreakSourcePackageIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    OwnerMismatch {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
    },
    FunctionOriginMismatch {
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
    },
    MissingRow {
        batch_slot: u32,
    },
    DuplicateRow {
        batch_slot: u32,
    },
    UnexpectedRow {
        batch_slot: u32,
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
    CompositeCandidate(VerifiedCallableLoopBreakCompositeSourceFactsV1),
    SupportedNonCandidate { loop_count: usize },
    Consumed,
}

/// One owner-scoped LoopBreak source product lent to the selected lowering
/// scope. The package keeps the row sealed until this one-shot move; the
/// lowering state then owns the product for the later physical consumer.
#[derive(Debug)]
pub(in crate::mir) enum LoopBreakSourcePackageLoanV1 {
    Candidate(VerifiedCallableLoopBreakSourceFactsV1),
    CompositeCandidate(VerifiedCallableLoopBreakCompositeSourceFactsV1),
    SupportedNonCandidate {
        owner: FunctionOwnerIdV1,
        loop_count: usize,
    },
}

impl LoopBreakSourcePackageLoanV1 {
    pub(in crate::mir) fn is_candidate(&self) -> bool {
        match self {
            Self::Candidate(facts) => {
                let _ = facts;
                true
            }
            Self::CompositeCandidate(facts) => {
                let _ = facts;
                true
            }
            Self::SupportedNonCandidate { owner, loop_count } => {
                let _ = (owner, loop_count);
                false
            }
        }
    }

    pub(in crate::mir) fn take_candidate_for_site(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Option<crate::mir::builder::VerifiedCallableLoopBreakSourceCandidateV1> {
        match self {
            Self::Candidate(facts) => facts.take_candidate_for_site(site),
            Self::CompositeCandidate(_) | Self::SupportedNonCandidate { .. } => None,
        }
    }

    pub(in crate::mir) fn has_candidate_for_site(&self, site: &SourceStmtSiteV1) -> bool {
        match self {
            Self::Candidate(facts) => facts
                .candidates()
                .iter()
                .any(|candidate| candidate.projection().loop_site() == site),
            Self::CompositeCandidate(_) | Self::SupportedNonCandidate { .. } => false,
        }
    }

    pub(in crate::mir) fn take_composite_candidate_for_site(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Option<crate::mir::builder::VerifiedCallableLoopBreakCompositeSourceCandidateV1> {
        match self {
            Self::CompositeCandidate(facts) => facts.take_candidate_for_site(site),
            Self::Candidate(_) | Self::SupportedNonCandidate { .. } => None,
        }
    }

    pub(in crate::mir) fn has_composite_candidate_for_site(&self, site: &SourceStmtSiteV1) -> bool {
        match self {
            Self::CompositeCandidate(facts) => facts
                .candidates()
                .iter()
                .any(|candidate| candidate.projection().loop_site() == site),
            Self::Candidate(_) | Self::SupportedNonCandidate { .. } => false,
        }
    }

    pub(in crate::mir) fn finish_empty(&self) -> Result<(), String> {
        match self {
            Self::Candidate(facts) if !facts.candidates().is_empty() => Err(format!(
                "[freeze:contract][callable-loop-break/source-package/residual-candidates] owner={:?} count={}",
                facts.owner(),
                facts.candidates().len(),
            )),
            Self::CompositeCandidate(facts) if !facts.candidates().is_empty() => Err(format!(
                "[freeze:contract][callable-loop-break/source-package/residual-composite-candidates] owner={:?} count={}",
                facts.owner(),
                facts.candidates().len(),
            )),
            Self::Candidate(_) | Self::CompositeCandidate(_) | Self::SupportedNonCandidate { .. } => Ok(()),
        }
    }
}

#[derive(Debug)]
pub(super) struct OwnedLoopBreakSourcePackageRowEnvelopeV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    row: OwnedLoopBreakSourcePackageRowV1,
}

#[derive(Debug, Clone, Copy)]
struct ExpectedLoopBreakSourcePackageRowV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
}

#[derive(Debug)]
pub(super) struct VerifiedLoopBreakSourcePackageV1 {
    rows: Box<[OwnedLoopBreakSourcePackageRowEnvelopeV1]>,
}

impl VerifiedLoopBreakSourcePackageV1 {
    pub(super) fn rows(&self) -> &[OwnedLoopBreakSourcePackageRowEnvelopeV1] {
        &self.rows
    }

    pub(in crate::mir) fn take_for_owner(
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Result<LoopBreakSourcePackageLoanV1, String> {
        let Some(row) = self.rows.iter_mut().find(|row| row.owner == owner) else {
            return Err(
                "[freeze:contract][callable-loop-break/source-package/owner-row-missing]"
                    .to_owned(),
            );
        };
        if let OwnedLoopBreakSourcePackageRowV1::CompositeCandidate(facts) = &row.row {
            facts.validate_package_candidate()?;
        }
        let previous = std::mem::replace(&mut row.row, OwnedLoopBreakSourcePackageRowV1::Consumed);
        match previous {
            OwnedLoopBreakSourcePackageRowV1::Candidate(facts) => {
                Ok(LoopBreakSourcePackageLoanV1::Candidate(facts))
            }
            OwnedLoopBreakSourcePackageRowV1::CompositeCandidate(facts) => {
                Ok(LoopBreakSourcePackageLoanV1::CompositeCandidate(facts))
            }
            OwnedLoopBreakSourcePackageRowV1::SupportedNonCandidate { loop_count } => {
                Ok(LoopBreakSourcePackageLoanV1::SupportedNonCandidate { owner, loop_count })
            }
            OwnedLoopBreakSourcePackageRowV1::Consumed => Err(
                "[freeze:contract][callable-loop-break/source-package/owner-row-duplicate-take]"
                    .to_owned(),
            ),
        }
    }

    #[cfg(test)]
    pub(super) fn candidate_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                matches!(
                    row.row,
                    OwnedLoopBreakSourcePackageRowV1::Candidate(_)
                        | OwnedLoopBreakSourcePackageRowV1::CompositeCandidate(_)
                )
            })
            .count()
    }

    #[cfg(test)]
    pub(super) fn candidate_owner(&self) -> Option<FunctionOwnerIdV1> {
        self.rows.iter().find_map(|row| {
            matches!(
                row.row,
                OwnedLoopBreakSourcePackageRowV1::Candidate(_)
                    | OwnedLoopBreakSourcePackageRowV1::CompositeCandidate(_)
            )
            .then_some(row.owner)
        })
    }

    #[cfg(test)]
    pub(super) fn first_owner(&self) -> Option<FunctionOwnerIdV1> {
        self.rows.first().map(|row| row.owner)
    }
}

pub(super) fn issue_loop_break_source_package_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    policy: LoopFactsPolicyFrameV1,
) -> Result<VerifiedLoopBreakSourcePackageV1, LoopBreakSourcePackageIssueV1> {
    let declarations = batch.declarations().collect::<Vec<_>>();
    let expected = declarations
        .iter()
        .map(|declaration| ExpectedLoopBreakSourcePackageRowV1 {
            batch_slot: declaration.batch_slot(),
            owner: declaration.owner(),
            function_origin: declaration.function_origin(),
        })
        .collect::<Vec<_>>();
    let mut rows = Vec::with_capacity(expected.len());
    for declaration in declarations {
        let batch_slot = declaration.batch_slot();
        let owner = declaration.owner();
        let function_origin = declaration.function_origin();
        let disposition = batch
            .with_lowering_input(batch_slot, |input| {
                issue_callable_loop_break_source_facts_v1(input, policy)
            })
            .map_err(LoopBreakSourcePackageIssueV1::BatchLoan)?;
        let (row, observed_owner, observed_function_origin) = match disposition {
            CallableLoopBreakSourceFactsDispositionV1::Candidate(facts) => {
                let observed_owner = facts.owner();
                let observed_function_origin = facts.function_origin();
                (
                    OwnedLoopBreakSourcePackageRowV1::Candidate(facts),
                    observed_owner,
                    observed_function_origin,
                )
            }
            CallableLoopBreakSourceFactsDispositionV1::CompositeCandidate(facts) => {
                let observed_owner = facts.owner();
                let observed_function_origin = facts.function_origin();
                (
                    OwnedLoopBreakSourcePackageRowV1::CompositeCandidate(facts),
                    observed_owner,
                    observed_function_origin,
                )
            }
            CallableLoopBreakSourceFactsDispositionV1::SupportedNonCandidate {
                owner: observed_owner,
                loop_count,
            } => {
                if observed_owner != owner {
                    return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot, owner });
                }
                (
                    OwnedLoopBreakSourcePackageRowV1::SupportedNonCandidate { loop_count },
                    observed_owner,
                    function_origin,
                )
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
            owner: observed_owner,
            function_origin: observed_function_origin,
            row,
        });
    }
    validate_loop_break_source_package_rows_v1(&expected, &rows)?;
    Ok(VerifiedLoopBreakSourcePackageV1 {
        rows: rows.into_boxed_slice(),
    })
}

fn validate_loop_break_source_package_rows_v1(
    expected: &[ExpectedLoopBreakSourcePackageRowV1],
    rows: &[OwnedLoopBreakSourcePackageRowEnvelopeV1],
) -> Result<(), LoopBreakSourcePackageIssueV1> {
    let expected_slots = expected
        .iter()
        .map(|row| row.batch_slot)
        .collect::<BTreeSet<_>>();
    let mut observed_slots = BTreeSet::new();
    for row in rows {
        if !observed_slots.insert(row.batch_slot) {
            return Err(LoopBreakSourcePackageIssueV1::DuplicateRow {
                batch_slot: row.batch_slot,
            });
        }
        let Some(expected_row) = expected
            .iter()
            .find(|expected_row| expected_row.batch_slot == row.batch_slot)
        else {
            return Err(LoopBreakSourcePackageIssueV1::UnexpectedRow {
                batch_slot: row.batch_slot,
            });
        };
        if row.owner != expected_row.owner {
            return Err(LoopBreakSourcePackageIssueV1::OwnerMismatch {
                batch_slot: row.batch_slot,
                owner: expected_row.owner,
            });
        }
        if row.function_origin != expected_row.function_origin {
            return Err(LoopBreakSourcePackageIssueV1::FunctionOriginMismatch {
                batch_slot: row.batch_slot,
                owner: expected_row.owner,
            });
        }
    }
    let Some(missing_slot) = expected_slots.difference(&observed_slots).next() else {
        return Ok(());
    };
    Err(LoopBreakSourcePackageIssueV1::MissingRow {
        batch_slot: *missing_slot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn owner_pair() -> (FunctionOwnerIdV1, FunctionOwnerIdV1) {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        (
            issuer.issue().expect("first owner"),
            issuer.issue().expect("second owner"),
        )
    }

    fn expected(batch_slot: u32, owner: FunctionOwnerIdV1) -> ExpectedLoopBreakSourcePackageRowV1 {
        ExpectedLoopBreakSourcePackageRowV1 {
            batch_slot,
            owner,
            function_origin: FunctionOriginV1::new(0, batch_slot),
        }
    }

    fn supported_row(
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
    ) -> OwnedLoopBreakSourcePackageRowEnvelopeV1 {
        OwnedLoopBreakSourcePackageRowEnvelopeV1 {
            batch_slot,
            owner,
            function_origin,
            row: OwnedLoopBreakSourcePackageRowV1::SupportedNonCandidate { loop_count: 0 },
        }
    }

    #[test]
    fn package_rows_reject_foreign_owner() {
        let (expected_owner, foreign_owner) = owner_pair();
        let expected = [expected(0, expected_owner)];
        let rows = [supported_row(0, foreign_owner, FunctionOriginV1::new(0, 0))];
        assert!(matches!(
            validate_loop_break_source_package_rows_v1(&expected, &rows),
            Err(LoopBreakSourcePackageIssueV1::OwnerMismatch { batch_slot: 0, .. })
        ));
    }

    #[test]
    fn package_rows_reject_duplicate_batch_slot() {
        let (owner, _) = owner_pair();
        let expected = [expected(0, owner)];
        let rows = [
            supported_row(0, owner, FunctionOriginV1::new(0, 0)),
            supported_row(0, owner, FunctionOriginV1::new(0, 0)),
        ];
        assert!(matches!(
            validate_loop_break_source_package_rows_v1(&expected, &rows),
            Err(LoopBreakSourcePackageIssueV1::DuplicateRow { batch_slot: 0 })
        ));
    }

    #[test]
    fn package_rows_reject_missing_batch_slot() {
        let (owner, _) = owner_pair();
        let expected = [expected(0, owner), expected(1, owner)];
        let rows = [supported_row(0, owner, FunctionOriginV1::new(0, 0))];
        assert!(matches!(
            validate_loop_break_source_package_rows_v1(&expected, &rows),
            Err(LoopBreakSourcePackageIssueV1::MissingRow { batch_slot: 1 })
        ));
    }

    #[test]
    fn package_rows_reject_unexpected_batch_slot() {
        let (owner, _) = owner_pair();
        let expected = [expected(0, owner)];
        let rows = [supported_row(3, owner, FunctionOriginV1::new(0, 3))];
        assert!(matches!(
            validate_loop_break_source_package_rows_v1(&expected, &rows),
            Err(LoopBreakSourcePackageIssueV1::UnexpectedRow { batch_slot: 3 })
        ));
    }
}
