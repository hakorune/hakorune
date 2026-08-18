//! Stamp-only lifecycle admission for the pinned-Text Residence seam.
//!
//! This module replaces the former copied-row proof prototype.  It consumes
//! the existing Completion and `PreparedFunctionExitSetV1` authorities, keeps
//! the exit set private, and issues no runtime effect, Return, or MIR value.
//! The later materializer must consume this move-only admission exactly once.

use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameBorrowV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::ValueId;

use super::super::completion_consumption::{ExplicitReturnWitnessV1, ReadyFunctionCompletionV1};
use super::{PreparedFunctionExitSetV1, PreparedFunctionExitV1};

/// Opaque provenance for one function-local exit-set lifecycle admission.
/// It carries no exit site, block, value, count, or order.
#[derive(Debug, PartialEq, Eq)]
struct FunctionExitSetStampV1 {
    owner: FunctionOwnerIdV1,
    invocation_ordinal: u64,
    plan_stamp: u64,
    frame_revision: u32,
    target_profile_id: &'static str,
    target_triple: &'static str,
}

impl FunctionExitSetStampV1 {
    fn from_frame(owner: FunctionOwnerIdV1, frame: &PinnedTextBackendFrameBorrowV1<'_>) -> Self {
        Self {
            owner,
            invocation_ordinal: frame.invocation_ordinal(),
            plan_stamp: frame.plan_stamp(),
            frame_revision: frame.frame_revision(),
            target_profile_id: frame.target_profile_id(),
            target_triple: frame.target_triple(),
        }
    }

    fn matches_frame(
        &self,
        owner: FunctionOwnerIdV1,
        plans: &PinnedTextAccessPlanTableV1,
        frame: &PinnedTextBackendFrameBorrowV1<'_>,
    ) -> bool {
        self == &Self::from_frame(owner, frame)
            && frame.owner() == owner
            && plans.stamp() == frame.plan_stamp()
    }
}

/// Move-only admission issued after the canonical Completion and exit-set
/// checks. It retains only a lifetime-bound borrow of the exact set owned by
/// DraftSeal; no exit rows are copied or re-paired.
#[must_use = "a lifecycle admission must be consumed by the canonical materializer"]
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedTextFormalExitFinishSetV1<'exits> {
    stamp: FunctionExitSetStampV1,
    exits: &'exits PreparedFunctionExitSetV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum TextFormalExitFinishAdmissionRejectV1 {
    CompletionOwnerMismatch,
    PlanStampMismatch,
    UnsupportedExitKind,
    ExitCardinalityMismatch { expected: usize, actual: usize },
    ExitClaimMismatch,
    DuplicateExitSite,
    InvalidFrameProvenance,
    ResidenceCapabilityMismatch,
    ConsumerRejected,
}

/// Issue one provenance admission from the existing Completion, plan, frame,
/// and prepared exit-set authorities. The exact exit-set borrow remains tied
/// to its DraftSeal owner; no copied exit rows are produced or moved out.
pub(in crate::mir::builder::resolved_lowering) fn issue_pinned_text_residence_exit_finish_set_v1<
    'exits,
>(
    completion: &ReadyFunctionCompletionV1,
    plans: &PinnedTextAccessPlanTableV1,
    frame: PinnedTextBackendFrameBorrowV1<'_>,
    exits: &'exits PreparedFunctionExitSetV1,
) -> Result<PreparedTextFormalExitFinishSetV1<'exits>, TextFormalExitFinishAdmissionRejectV1> {
    let owner = completion.owner();
    if owner != frame.owner() {
        return Err(TextFormalExitFinishAdmissionRejectV1::CompletionOwnerMismatch);
    }
    if plans.stamp() != frame.plan_stamp() {
        return Err(TextFormalExitFinishAdmissionRejectV1::PlanStampMismatch);
    }
    if !completion.returns_value() {
        return Err(TextFormalExitFinishAdmissionRejectV1::UnsupportedExitKind);
    }
    validate_exit_set(completion, &exits)?;
    let stamp = FunctionExitSetStampV1::from_frame(owner, &frame);
    if !stamp.matches_frame(owner, plans, &frame) {
        return Err(TextFormalExitFinishAdmissionRejectV1::InvalidFrameProvenance);
    }
    Ok(PreparedTextFormalExitFinishSetV1 { stamp, exits })
}

impl<'exits> PreparedTextFormalExitFinishSetV1<'exits> {
    /// Consume the admission once.  `Result<(), E>` is deliberate: the exit
    /// set cannot escape as a returned aggregate or become a second authority.
    pub(in crate::mir::builder::resolved_lowering) fn consume_for_materializer(
        self,
        plans: &PinnedTextAccessPlanTableV1,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        callback: impl FnOnce(&PreparedFunctionExitSetV1) -> Result<(), String>,
    ) -> Result<(), TextFormalExitFinishAdmissionRejectV1> {
        let owner = self.stamp.owner;
        if !self.stamp.matches_frame(owner, plans, &frame) {
            return Err(TextFormalExitFinishAdmissionRejectV1::InvalidFrameProvenance);
        }
        callback(self.exits).map_err(|_| TextFormalExitFinishAdmissionRejectV1::ConsumerRejected)
    }
}

/// Private DraftSeal projector seam.  The canonical caller supplies the three
/// existing physical consumers; this helper owns only their ordering and the
/// one-shot exit iteration.  It emits no MIR or runtime effect by itself.
pub(in crate::mir::builder::resolved_lowering) fn project_pinned_text_residence_finish_before_return_v1<
    'exits,
>(
    admission: PreparedTextFormalExitFinishSetV1<'exits>,
    plans: &PinnedTextAccessPlanTableV1,
    frame: PinnedTextBackendFrameBorrowV1<'_>,
    mut materialize_return_operand: impl FnMut(ValueId) -> Result<(), String>,
    mut finish_residence: impl FnMut() -> Result<(), String>,
    mut emit_return: impl FnMut(PreparedFunctionExitV1) -> Result<(), String>,
) -> Result<(), TextFormalExitFinishAdmissionRejectV1> {
    admission.consume_for_materializer(plans, frame, |exits| {
        exits.try_for_each_exit(|exit| {
            let PreparedFunctionExitV1::ExplicitValue { value, .. } = exit else {
                return Err("lifecycle projector received a non-value exit".to_owned());
            };
            materialize_return_operand(value)?;
            finish_residence()?;
            emit_return(exit)
        })
    })
}

fn validate_exit_set(
    completion: &ReadyFunctionCompletionV1,
    exits: &PreparedFunctionExitSetV1,
) -> Result<(), TextFormalExitFinishAdmissionRejectV1> {
    let claims = completion.explicit_claims();
    match exits {
        PreparedFunctionExitSetV1::Single(exit) => {
            if claims.len() != 1 {
                return Err(
                    TextFormalExitFinishAdmissionRejectV1::ExitCardinalityMismatch {
                        expected: 1,
                        actual: claims.len(),
                    },
                );
            }
            validate_explicit_exit(*exit, &claims[0].witness())
        }
        PreparedFunctionExitSetV1::ExactTwo(exit_claims) => {
            if claims.len() != 2 {
                return Err(
                    TextFormalExitFinishAdmissionRejectV1::ExitCardinalityMismatch {
                        expected: 2,
                        actual: claims.len(),
                    },
                );
            }
            if exit_claims[0].site() == exit_claims[1].site() {
                return Err(TextFormalExitFinishAdmissionRejectV1::DuplicateExitSite);
            }
            for exit_claim in exit_claims {
                let Some(completion_claim) = claims
                    .iter()
                    .find(|claim| claim.site() == exit_claim.site())
                else {
                    return Err(TextFormalExitFinishAdmissionRejectV1::ExitClaimMismatch);
                };
                validate_explicit_exit(exit_claim.exit(), &completion_claim.witness())?;
            }
            Ok(())
        }
    }
}

fn validate_explicit_exit(
    exit: PreparedFunctionExitV1,
    witness: &ExplicitReturnWitnessV1,
) -> Result<(), TextFormalExitFinishAdmissionRejectV1> {
    let PreparedFunctionExitV1::ExplicitValue { block, value } = exit else {
        return Err(TextFormalExitFinishAdmissionRejectV1::UnsupportedExitKind);
    };
    let ExplicitReturnWitnessV1::Value(witness) = witness else {
        return Err(TextFormalExitFinishAdmissionRejectV1::ExitClaimMismatch);
    };
    if witness.block() != block || witness.value() != value {
        return Err(TextFormalExitFinishAdmissionRejectV1::ExitClaimMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::completion_consumption::ExplicitReturnClaimV1;
    use crate::mir::builder::resolved_lowering::draft_seal::multi_site_exit::DetachedFunctionExitClaimV1;
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::pinned_text_access_plan::{PinnedTextAccessKindV1, PinnedTextRootIdV1};
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    };
    use crate::mir::{BasicBlockId, ValueId};
    use std::cell::RefCell;

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

    fn completion_two(owner: FunctionOwnerIdV1) -> ReadyFunctionCompletionV1 {
        ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![
                ExplicitReturnClaimV1::from_test_value(
                    site(1),
                    BasicBlockId::new(10),
                    ValueId::new(20),
                ),
                ExplicitReturnClaimV1::from_test_value(
                    site(2),
                    BasicBlockId::new(11),
                    ValueId::new(21),
                ),
            ]
            .into_boxed_slice(),
        )
    }

    #[test]
    fn issues_lifetime_bound_admission_and_consumes_once() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 19);
        let exits = PreparedFunctionExitSetV1::exact_two([
            DetachedFunctionExitClaimV1::from_test(
                site(2),
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(11),
                    value: ValueId::new(21),
                },
            ),
            DetachedFunctionExitClaimV1::from_test(
                site(1),
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(10),
                    value: ValueId::new(20),
                },
            ),
        ]);
        let admission = issue_pinned_text_residence_exit_finish_set_v1(
            &completion_two(owner),
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        let mut visits = 0;
        admission
            .consume_for_materializer(&plans, frame.borrow(), |exits| {
                exits.try_for_each_exit(|_| {
                    visits += 1;
                    Ok::<(), String>(())
                })
            })
            .unwrap();
        assert_eq!(visits, 2);
    }

    #[test]
    fn canonical_projector_orders_operand_finish_return_per_exit() {
        #[derive(Debug, PartialEq, Eq)]
        enum Event {
            Operand(ValueId),
            Finish,
            Return(BasicBlockId),
        }

        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 23);
        let exits = PreparedFunctionExitSetV1::exact_two([
            DetachedFunctionExitClaimV1::from_test(
                site(2),
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(11),
                    value: ValueId::new(21),
                },
            ),
            DetachedFunctionExitClaimV1::from_test(
                site(1),
                PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(10),
                    value: ValueId::new(20),
                },
            ),
        ]);
        let admission = issue_pinned_text_residence_exit_finish_set_v1(
            &completion_two(owner),
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        let events = RefCell::new(Vec::new());
        project_pinned_text_residence_finish_before_return_v1(
            admission,
            &plans,
            frame.borrow(),
            |value| {
                events.borrow_mut().push(Event::Operand(value));
                Ok(())
            },
            || {
                events.borrow_mut().push(Event::Finish);
                Ok(())
            },
            |exit| {
                let PreparedFunctionExitV1::ExplicitValue { block, .. } = exit else {
                    unreachable!("validated lifecycle exit")
                };
                events.borrow_mut().push(Event::Return(block));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(
            events.into_inner(),
            vec![
                Event::Operand(ValueId::new(21)),
                Event::Finish,
                Event::Return(BasicBlockId::new(11)),
                Event::Operand(ValueId::new(20)),
                Event::Finish,
                Event::Return(BasicBlockId::new(10)),
            ]
        );
    }

    #[test]
    fn canonical_projector_suppresses_return_after_finish_failure() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 29);
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(1),
            value: ValueId::new(2),
        });
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                site(0),
                BasicBlockId::new(1),
                ValueId::new(2),
            )]
            .into_boxed_slice(),
        );
        let admission = issue_pinned_text_residence_exit_finish_set_v1(
            &completion,
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        let events = RefCell::new(Vec::new());
        assert_eq!(
            project_pinned_text_residence_finish_before_return_v1(
                admission,
                &plans,
                frame.borrow(),
                |_| {
                    events.borrow_mut().push("operand");
                    Ok(())
                },
                || {
                    events.borrow_mut().push("finish");
                    Err("finish failed".to_owned())
                },
                |_| {
                    events.borrow_mut().push("return");
                    Ok(())
                },
            ),
            Err(TextFormalExitFinishAdmissionRejectV1::ConsumerRejected)
        );
        assert_eq!(events.into_inner(), vec!["operand", "finish"]);
    }

    #[test]
    fn canonical_projector_suppresses_finish_after_operand_failure() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 37);
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(1),
            value: ValueId::new(2),
        });
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                site(0),
                BasicBlockId::new(1),
                ValueId::new(2),
            )]
            .into_boxed_slice(),
        );
        let admission = issue_pinned_text_residence_exit_finish_set_v1(
            &completion,
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        let events = RefCell::new(Vec::new());
        assert_eq!(
            project_pinned_text_residence_finish_before_return_v1(
                admission,
                &plans,
                frame.borrow(),
                |_| {
                    events.borrow_mut().push("operand");
                    Err("operand failed".to_owned())
                },
                || {
                    events.borrow_mut().push("finish");
                    Ok(())
                },
                |_| {
                    events.borrow_mut().push("return");
                    Ok(())
                },
            ),
            Err(TextFormalExitFinishAdmissionRejectV1::ConsumerRejected)
        );
        assert_eq!(events.into_inner(), vec!["operand"]);
    }

    #[test]
    fn rejects_foreign_plan_and_non_value_exit_before_effect() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let foreign = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 31);
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(1),
            value: ValueId::new(2),
        });
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                site(0),
                BasicBlockId::new(1),
                ValueId::new(2),
            )]
            .into_boxed_slice(),
        );
        let foreign_frame = PinnedTextBackendFrameContractV1::from_test(foreign, 31, 1);
        assert_eq!(
            issue_pinned_text_residence_exit_finish_set_v1(
                &completion,
                &plans,
                foreign_frame.borrow(),
                &exits,
            ),
            Err(TextFormalExitFinishAdmissionRejectV1::CompletionOwnerMismatch)
        );

        let unit = ReadyFunctionCompletionV1::from_test_explicit_unit(owner, Box::default());
        let unit_exit = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitUnit {
            block: BasicBlockId::new(1),
        });
        assert_eq!(
            issue_pinned_text_residence_exit_finish_set_v1(
                &unit,
                &plans,
                frame.borrow(),
                &unit_exit,
            ),
            Err(TextFormalExitFinishAdmissionRejectV1::UnsupportedExitKind)
        );
    }

    #[test]
    fn rejects_duplicate_or_mismatched_exact_two_claims() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 41);
        let duplicate = site(5);
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
            issue_pinned_text_residence_exit_finish_set_v1(
                &completion_two(owner),
                &plans,
                frame.borrow(),
                &exits,
            ),
            Err(TextFormalExitFinishAdmissionRejectV1::DuplicateExitSite)
        );
    }

    #[test]
    fn late_consumer_failure_consumes_admission_without_retry() {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let (frame, plans) = frame_and_plans(owner, 47);
        let exits = PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(1),
            value: ValueId::new(2),
        });
        let completion = ReadyFunctionCompletionV1::from_test_explicit_value(
            owner,
            vec![ExplicitReturnClaimV1::from_test_value(
                site(0),
                BasicBlockId::new(1),
                ValueId::new(2),
            )]
            .into_boxed_slice(),
        );
        let admission = issue_pinned_text_residence_exit_finish_set_v1(
            &completion,
            &plans,
            frame.borrow(),
            &exits,
        )
        .unwrap();
        assert_eq!(
            admission.consume_for_materializer(&plans, frame.borrow(), |_| {
                Err::<(), _>("late draft failure".to_owned())
            }),
            Err(TextFormalExitFinishAdmissionRejectV1::ConsumerRejected)
        );
    }
}
