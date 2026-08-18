//! Caller-zero DraftSeal consumer for the pinned-Text lifecycle obligation.
//!
//! This child owns only the physical ordering seam.  It consumes one
//! move-only finish capability and the existing borrowed exit admission, then
//! lends each already-validated value exit to the canonical operand/finish/
//! Return consumers.  It does not create a second exit inventory or emit MIR.

use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameBorrowV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::pinned_text_residence_lifecycle::PinnedTextResidenceFinishCapabilityV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::text_residence_exit::{
    issue_pinned_text_residence_exit_finish_set_v1, PreparedTextFormalExitFinishSetV1,
    TextFormalExitFinishAdmissionRejectV1,
};
use super::{FunctionDraftSealProjectionV1, PreparedFunctionExitSetV1, PreparedFunctionExitV1};
use crate::mir::builder::resolved_lowering::completion_consumption::ReadyFunctionCompletionV1;

/// One affine DraftSeal physical consumer. The validated exit set and the
/// lifecycle obligation are moved into the same aggregate, so no later caller
/// can re-pair borrowed rows with a different Finish capability.
#[must_use = "the DraftSeal lifecycle consumer must be consumed exactly once"]
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedPinnedTextResidenceDraftSealConsumerV1
{
    admission: PreparedTextFormalExitFinishSetV1,
    finish: PinnedTextResidenceFinishCapabilityV1,
}

/// Co-seal the existing exit admission with the lifecycle capability issued
/// by the canonical Enter writer.  This is a physical BoxShape aggregate; it
/// issues no source meaning or runtime owner.
pub(in crate::mir::builder::resolved_lowering) fn issue_pinned_text_residence_draftseal_consumer_v1(
    completion: &ReadyFunctionCompletionV1,
    plans: &PinnedTextAccessPlanTableV1,
    frame: PinnedTextBackendFrameBorrowV1<'_>,
    exits: PreparedFunctionExitSetV1,
    finish: PinnedTextResidenceFinishCapabilityV1,
) -> Result<PreparedPinnedTextResidenceDraftSealConsumerV1, TextFormalExitFinishAdmissionRejectV1> {
    let residence = finish.residence();
    if residence.owner() != frame.owner() || residence.plan_stamp() != frame.plan_stamp() {
        return Err(TextFormalExitFinishAdmissionRejectV1::ResidenceCapabilityMismatch);
    }
    let admission =
        issue_pinned_text_residence_exit_finish_set_v1(completion, plans, frame, exits)?;
    Ok(PreparedPinnedTextResidenceDraftSealConsumerV1 { admission, finish })
}

impl PreparedPinnedTextResidenceDraftSealConsumerV1 {
    #[cfg(test)]
    pub(super) fn into_projection_parts(
        self,
        plans: &PinnedTextAccessPlanTableV1,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
    ) -> Result<
        (
            PreparedFunctionExitSetV1,
            PinnedTextResidenceFinishCapabilityV1,
        ),
        String,
    > {
        let Self { admission, finish } = self;
        let exits = admission
            .into_validated_exit_set(plans, frame)
            .map_err(|error| format!("{error:?}"))?;
        Ok((exits, finish))
    }

    /// Consume the aggregate once. The single projection callback receives
    /// each already-validated explicit exit in operand -> Finish -> Return
    /// order; the former three independently supplied callbacks are gone.
    pub(in crate::mir::builder::resolved_lowering) fn consume_for_draft_seal(
        self,
        plans: &PinnedTextAccessPlanTableV1,
        frame: PinnedTextBackendFrameBorrowV1<'_>,
        mut project_exit: impl FnMut(
            PreparedFunctionExitV1,
            crate::mir::pinned_text_residence_lifecycle::TextFormalResidenceIdV1,
        ) -> Result<(), String>,
    ) -> Result<(), TextFormalExitFinishAdmissionRejectV1> {
        let Self { admission, finish } = self;
        let residence = finish.into_residence();
        admission.consume_for_materializer(plans, frame, |exits| {
            exits.try_for_each_exit(|exit| {
                let PreparedFunctionExitV1::ExplicitValue { value, .. } = exit else {
                    return Err("lifecycle consumer received a non-value exit".to_owned());
                };
                let _ = value;
                project_exit(exit, residence)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::completion_consumption::ExplicitReturnClaimV1;
    use crate::mir::builder::resolved_lowering::draft_seal::multi_site_exit::DetachedFunctionExitClaimV1;
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::pinned_text_access_plan::{PinnedTextAccessKindV1, PinnedTextRootIdV1};
    use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
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

    fn fixture(
        count: usize,
        stamp: u64,
    ) -> (
        FunctionOwnerIdV1,
        PinnedTextAccessPlanTableV1,
        PinnedTextBackendFrameContractV1,
        ReadyFunctionCompletionV1,
        PreparedFunctionExitSetV1,
        PinnedTextResidenceFinishCapabilityV1,
    ) {
        let mut issuers = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuers.issue().unwrap();
        let mut plans = PinnedTextAccessPlanTableV1::new(stamp);
        plans.issue(PinnedTextAccessKindV1::ByteLen {
            root: PinnedTextRootIdV1::from_frame_row(0),
        });
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner, stamp, 1);
        let frame = frame_contract.borrow();
        let carrier = PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
            owner,
            &plans,
            frame,
            BasicBlockId::new(1),
            BasicBlockId::new(2),
        )
        .unwrap();
        let finish = PinnedTextResidenceFinishCapabilityV1::from_parts(carrier.residence());
        let (completion, exits) = if count == 1 {
            (
                ReadyFunctionCompletionV1::from_test_explicit_value(
                    owner,
                    vec![ExplicitReturnClaimV1::from_test_value(
                        site(1),
                        BasicBlockId::new(10),
                        ValueId::new(20),
                    )]
                    .into_boxed_slice(),
                ),
                PreparedFunctionExitSetV1::single(PreparedFunctionExitV1::ExplicitValue {
                    block: BasicBlockId::new(10),
                    value: ValueId::new(20),
                }),
            )
        } else {
            (
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
                ),
                PreparedFunctionExitSetV1::exact_two([
                    DetachedFunctionExitClaimV1::from_test(
                        site(1),
                        PreparedFunctionExitV1::ExplicitValue {
                            block: BasicBlockId::new(10),
                            value: ValueId::new(20),
                        },
                    ),
                    DetachedFunctionExitClaimV1::from_test(
                        site(2),
                        PreparedFunctionExitV1::ExplicitValue {
                            block: BasicBlockId::new(11),
                            value: ValueId::new(21),
                        },
                    ),
                ]),
            )
        };
        (owner, plans, frame_contract, completion, exits, finish)
    }

    #[test]
    fn consumes_single_exit_in_operand_finish_return_order() {
        let (_owner, plans, frame, completion, exits, finish) = fixture(1, 71);
        let admission = issue_pinned_text_residence_draftseal_consumer_v1(
            &completion,
            &plans,
            frame.borrow(),
            exits,
            finish,
        )
        .unwrap();
        let events = RefCell::new(Vec::new());
        admission
            .consume_for_draft_seal(&plans, frame.borrow(), |exit, _residence| {
                let PreparedFunctionExitV1::ExplicitValue { block, value } = exit else {
                    unreachable!("validated lifecycle exit")
                };
                events.borrow_mut().push(("operand", value));
                events.borrow_mut().push(("finish", ValueId::new(0)));
                events
                    .borrow_mut()
                    .push(("return", ValueId::new(block.as_u32())));
                Ok(())
            })
            .unwrap();
        assert_eq!(
            events.into_inner(),
            vec![
                ("operand", ValueId::new(20)),
                ("finish", ValueId::new(0)),
                ("return", ValueId::new(10))
            ]
        );
    }

    #[test]
    fn consumes_exact_two_and_discards_on_late_error() {
        let (_owner, plans, frame, completion, exits, finish) = fixture(2, 73);
        let admission = issue_pinned_text_residence_draftseal_consumer_v1(
            &completion,
            &plans,
            frame.borrow(),
            exits,
            finish,
        )
        .unwrap();
        assert_eq!(
            admission.consume_for_draft_seal(&plans, frame.borrow(), |_exit, _residence| Err(
                "late draft failure".to_owned()
            ),),
            Err(TextFormalExitFinishAdmissionRejectV1::ConsumerRejected)
        );
    }

    #[test]
    fn detached_projection_places_finish_before_each_existing_return() {
        let (_owner, plans, frame, completion, exits, finish) = fixture(2, 79);
        let consumer = issue_pinned_text_residence_draftseal_consumer_v1(
            &completion,
            &plans,
            frame.borrow(),
            exits,
            finish,
        )
        .unwrap();
        let mut builder = crate::mir::MirBuilder::new();
        builder.enter_function_for_test("pinned_text_detached_projection/0".to_owned());
        builder.ensure_block_exists(BasicBlockId::new(10)).unwrap();
        builder.ensure_block_exists(BasicBlockId::new(11)).unwrap();
        for block in [BasicBlockId::new(10), BasicBlockId::new(11)] {
            builder
                .function_state
                .current_function
                .as_mut()
                .expect("function")
                .get_block_mut(block)
                .expect("exit")
                .seal();
        }

        let projection = FunctionDraftSealProjectionV1::project_from_builder_pinned_text(
            &builder,
            consumer,
            &plans,
            frame.borrow(),
        )
        .expect("detached pinned-text projection");
        for (block, value) in [
            (BasicBlockId::new(10), ValueId::new(20)),
            (BasicBlockId::new(11), ValueId::new(21)),
        ] {
            let exit = projection
                .function()
                .get_block(block)
                .expect("projected exit");
            assert!(matches!(
                exit.instructions.as_slice(),
                [crate::mir::MirInstruction::PinnedTextResidenceFinish { .. }]
            ));
            assert!(matches!(
                exit.terminator,
                Some(crate::mir::MirInstruction::Return { value: Some(actual) })
                    if actual == value
            ));
            assert!(builder
                .function_state
                .current_function
                .as_ref()
                .expect("live function")
                .get_block(block)
                .expect("live exit")
                .terminator
                .is_none());
        }
    }
}
