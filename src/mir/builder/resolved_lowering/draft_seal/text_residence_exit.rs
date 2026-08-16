//! Proof-only exit obligation for the pinned-Text residence bridge.
//!
//! This module deliberately stops before runtime finish, MIR lifecycle
//! markers, and Return materialization.  It only co-seals the existing
//! Completion claims, the prepared exit set, and the already-issued backend
//! frame stamps.  DraftSeal remains the sole Return writer in a later slice.

use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameBorrowV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, ValueId};

use super::super::completion_consumption::{ExplicitReturnWitnessV1, ReadyFunctionCompletionV1};
use super::{PreparedFunctionExitSetV1, PreparedFunctionExitV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PinnedTextResidenceExitRowV1 {
    site: SourceStmtSiteV1,
    block: BasicBlockId,
    value: ValueId,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PinnedTextResidenceExitObligationV1 {
    owner: FunctionOwnerIdV1,
    plan_stamp: u64,
    frame_revision: u32,
    rows: Box<[PinnedTextResidenceExitRowV1]>,
}

impl PinnedTextResidenceExitObligationV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) const fn plan_stamp(&self) -> u64 {
        self.plan_stamp
    }

    pub(in crate::mir::builder::resolved_lowering) const fn frame_revision(&self) -> u32 {
        self.frame_revision
    }

    pub(in crate::mir::builder::resolved_lowering) fn count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum PinnedTextResidenceExitObligationRejectV1 {
    CompletionOwnerMismatch,
    PlanStampMismatch,
    UnsupportedExitKind,
    ExitCardinalityMismatch { expected: usize, actual: usize },
    ExitClaimMismatch,
    DuplicateExitSite,
}

pub(in crate::mir::builder::resolved_lowering) fn issue_pinned_text_residence_exit_obligation_v1(
    completion: &ReadyFunctionCompletionV1,
    plans: &PinnedTextAccessPlanTableV1,
    frame: PinnedTextBackendFrameBorrowV1<'_>,
    exits: &PreparedFunctionExitSetV1,
) -> Result<PinnedTextResidenceExitObligationV1, PinnedTextResidenceExitObligationRejectV1> {
    if completion.owner() != frame.owner() {
        return Err(PinnedTextResidenceExitObligationRejectV1::CompletionOwnerMismatch);
    }
    if plans.stamp() != frame.plan_stamp() {
        return Err(PinnedTextResidenceExitObligationRejectV1::PlanStampMismatch);
    }
    if !completion.returns_value() {
        return Err(PinnedTextResidenceExitObligationRejectV1::UnsupportedExitKind);
    }

    let claims = completion.explicit_claims();
    let rows = match exits {
        PreparedFunctionExitSetV1::Single(exit) => {
            if claims.len() != 1 {
                return Err(
                    PinnedTextResidenceExitObligationRejectV1::ExitCardinalityMismatch {
                        expected: 1,
                        actual: claims.len(),
                    },
                );
            }
            let PreparedFunctionExitV1::ExplicitValue { block, value } = *exit else {
                return Err(PinnedTextResidenceExitObligationRejectV1::UnsupportedExitKind);
            };
            let ExplicitReturnWitnessV1::Value(witness) = claims[0].witness() else {
                return Err(PinnedTextResidenceExitObligationRejectV1::ExitClaimMismatch);
            };
            if witness.block() != block || witness.value() != value {
                return Err(PinnedTextResidenceExitObligationRejectV1::ExitClaimMismatch);
            }
            vec![PinnedTextResidenceExitRowV1 {
                site: claims[0].site().clone(),
                block,
                value,
            }]
        }
        PreparedFunctionExitSetV1::ExactTwo(exit_claims) => {
            if claims.len() != 2 {
                return Err(
                    PinnedTextResidenceExitObligationRejectV1::ExitCardinalityMismatch {
                        expected: 2,
                        actual: claims.len(),
                    },
                );
            }
            let mut rows = Vec::with_capacity(2);
            for exit_claim in exit_claims {
                let site = exit_claim.site();
                if rows
                    .iter()
                    .any(|row: &PinnedTextResidenceExitRowV1| row.site == *site)
                {
                    return Err(PinnedTextResidenceExitObligationRejectV1::DuplicateExitSite);
                }
                let PreparedFunctionExitV1::ExplicitValue { block, value } = exit_claim.exit()
                else {
                    return Err(PinnedTextResidenceExitObligationRejectV1::UnsupportedExitKind);
                };
                let Some(completion_claim) = claims.iter().find(|claim| claim.site() == site)
                else {
                    return Err(PinnedTextResidenceExitObligationRejectV1::ExitClaimMismatch);
                };
                let ExplicitReturnWitnessV1::Value(witness) = completion_claim.witness() else {
                    return Err(PinnedTextResidenceExitObligationRejectV1::ExitClaimMismatch);
                };
                if witness.block() != block || witness.value() != value {
                    return Err(PinnedTextResidenceExitObligationRejectV1::ExitClaimMismatch);
                }
                rows.push(PinnedTextResidenceExitRowV1 {
                    site: site.clone(),
                    block,
                    value,
                });
            }
            rows
        }
    };

    Ok(PinnedTextResidenceExitObligationV1 {
        owner: frame.owner(),
        plan_stamp: frame.plan_stamp(),
        frame_revision: frame.frame_revision(),
        rows: rows.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::completion_consumption::{
        ExplicitReturnClaimV1, ReadyFunctionCompletionV1,
    };
    use crate::mir::builder::resolved_lowering::draft_seal::multi_site_exit::DetachedFunctionExitClaimV1;
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::pinned_text_access_plan::{
        PinnedTextAccessKindV1, PinnedTextAccessPlanTableV1, PinnedTextRootIdV1,
    };
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };

    fn site(index: u32) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
        ]))
    }

    fn frame_and_plans(
        owner: FunctionOwnerIdV1,
        stamp: u64,
    ) -> (
        PinnedTextBackendFrameContractV1,
        PinnedTextAccessPlanTableV1,
    ) {
        let mut plans = PinnedTextAccessPlanTableV1::new(stamp);
        plans.issue(PinnedTextAccessKindV1::ByteLen {
            root: PinnedTextRootIdV1::from_frame_row(0),
        });
        (
            PinnedTextBackendFrameContractV1::from_test(owner, stamp, 1),
            plans,
        )
    }

    #[test]
    fn issues_single_value_exit_obligation_from_existing_facts() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 19);
        let claim_site = site(0);
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                claim_site.clone(),
                BasicBlockId::new(4),
                ValueId::new(9),
            )]
            .into_boxed_slice(),
        );
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(4),
            value: ValueId::new(9),
        });

        let obligation = issue_pinned_text_residence_exit_obligation_v1(
            &completion,
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        assert_eq!(obligation.owner(), owner);
        assert_eq!(obligation.plan_stamp(), 19);
        assert_eq!(obligation.frame_revision(), 1);
        assert_eq!(obligation.count(), 1);
    }

    #[test]
    fn issues_exact_two_by_site_even_when_exit_order_differs() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 23);
        let first = site(1);
        let second = site(2);
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![
                ExplicitReturnClaimV1::from_test_value(
                    first.clone(),
                    BasicBlockId::new(10),
                    ValueId::new(20),
                ),
                ExplicitReturnClaimV1::from_test_value(
                    second.clone(),
                    BasicBlockId::new(11),
                    ValueId::new(21),
                ),
            ]
            .into_boxed_slice(),
        );
        let exits = PreparedFunctionExitSetV1::exact_two([
            DetachedFunctionExitClaimV1::from_test(
                second,
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(11),
                    value: ValueId::new(21),
                },
            ),
            DetachedFunctionExitClaimV1::from_test(
                first,
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(10),
                    value: ValueId::new(20),
                },
            ),
        ]);

        let obligation = issue_pinned_text_residence_exit_obligation_v1(
            &completion,
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        assert_eq!(obligation.count(), 2);
    }

    #[test]
    fn rejects_owner_stamp_and_unsupported_exit_drift() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let foreign = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 31);
        let claim_site = site(0);
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                claim_site,
                BasicBlockId::new(1),
                ValueId::new(2),
            )]
            .into_boxed_slice(),
        );
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(1),
            value: ValueId::new(2),
        });

        let foreign_frame = PinnedTextBackendFrameContractV1::from_test(foreign, 31, 1);
        assert_eq!(
            issue_pinned_text_residence_exit_obligation_v1(
                &completion,
                &plans,
                foreign_frame.borrow(),
                &exits,
            ),
            Err(PinnedTextResidenceExitObligationRejectV1::CompletionOwnerMismatch)
        );

        let (wrong_frame, _) = frame_and_plans(owner, 32);
        assert_eq!(
            issue_pinned_text_residence_exit_obligation_v1(
                &completion,
                &plans,
                wrong_frame.borrow(),
                &exits,
            ),
            Err(PinnedTextResidenceExitObligationRejectV1::PlanStampMismatch)
        );

        let unit = ReadyFunctionCompletionV1::from_test_explicit_unit(owner, Box::default());
        let unit_exit = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitUnit {
            block: BasicBlockId::new(1),
        });
        assert_eq!(
            issue_pinned_text_residence_exit_obligation_v1(
                &unit,
                &plans,
                frame.borrow(),
                &unit_exit,
            ),
            Err(PinnedTextResidenceExitObligationRejectV1::UnsupportedExitKind)
        );
    }

    #[test]
    fn rejects_duplicate_exact_two_site_and_implicit_unit() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 41);
        let duplicate = site(5);
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![
                ExplicitReturnClaimV1::from_test_value(
                    duplicate.clone(),
                    BasicBlockId::new(1),
                    ValueId::new(2),
                ),
                ExplicitReturnClaimV1::from_test_value(
                    site(6),
                    BasicBlockId::new(3),
                    ValueId::new(4),
                ),
            ]
            .into_boxed_slice(),
        );
        let exits = PreparedFunctionExitSetV1::exact_two([
            DetachedFunctionExitClaimV1::from_test(
                duplicate.clone(),
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(1),
                    value: ValueId::new(2),
                },
            ),
            DetachedFunctionExitClaimV1::from_test(
                duplicate,
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(1),
                    value: ValueId::new(2),
                },
            ),
        ]);
        assert_eq!(
            issue_pinned_text_residence_exit_obligation_v1(
                &completion,
                &plans,
                frame.borrow(),
                &exits,
            ),
            Err(PinnedTextResidenceExitObligationRejectV1::DuplicateExitSite)
        );

        let implicit = ReadyFunctionCompletionV1::from_test_implicit_void(owner);
        let implicit_exit =
            PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ImplicitUnit {
                block: BasicBlockId::new(0),
            });
        assert_eq!(
            issue_pinned_text_residence_exit_obligation_v1(
                &implicit,
                &plans,
                frame.borrow(),
                &implicit_exit,
            ),
            Err(PinnedTextResidenceExitObligationRejectV1::UnsupportedExitKind)
        );
    }
}
