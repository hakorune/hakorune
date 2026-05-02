use super::*;

#[test]
fn derive_string_kernel_plan_prefers_direct_entry_and_collects_barriers() {
    let function = make_loop_function();
    let publication_plan = StringCorridorCandidatePlan {
        corridor_root: ValueId::new(7),
        source_root: Some(ValueId::new(1)),
        borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: Some(StringPublishReason::StableObjectDemand),
        publish_repr_policy: Some(StringPublishReprPolicy::StableOwned),
        stable_view_provenance: None,
        start: Some(ValueId::new(2)),
        end: Some(ValueId::new(3)),
        known_length: Some(2),
        publication_contract: Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId::new(4)),
            left_source: ValueId::new(1),
            left_start: ValueId::new(4),
            left_end: ValueId::new(5),
            middle: ValueId::new(6),
            right_value: Some(ValueId::new(8)),
            right_source: ValueId::new(1),
            right_start: ValueId::new(5),
            right_end: ValueId::new(9),
            shared_source: true,
        },
    };
    let plan = StringCorridorCandidatePlan {
        corridor_root: ValueId::new(7),
        source_root: Some(ValueId::new(1)),
        borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        start: Some(ValueId::new(2)),
        end: Some(ValueId::new(3)),
        known_length: Some(2),
        publication_contract: Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId::new(4)),
            left_source: ValueId::new(1),
            left_start: ValueId::new(4),
            left_end: ValueId::new(5),
            middle: ValueId::new(6),
            right_value: Some(ValueId::new(8)),
            right_source: ValueId::new(1),
            right_start: ValueId::new(5),
            right_end: ValueId::new(9),
            shared_source: true,
        },
    };
    let candidates = vec![
        StringCorridorCandidate {
            kind: StringCorridorCandidateKind::PublicationSink,
            state: StringCorridorCandidateState::AlreadySatisfied,
            reason: "publish boundary is already sunk at the current corridor exit",
            plan: Some(publication_plan),
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
        },
        StringCorridorCandidate {
            kind: StringCorridorCandidateKind::MaterializationSink,
            state: StringCorridorCandidateState::Candidate,
            reason: "slice result may stay borrowed until a later boundary",
            plan: Some(plan),
            publication_boundary: None,
        },
        StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "borrowed slice corridor can target a direct kernel entry before publication",
            plan: Some(plan),
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
        },
    ];

    let kernel_plan =
        derive_string_kernel_plan(&function, ValueId::new(8), &candidates).expect("kernel plan");

    assert_eq!(kernel_plan.plan_value, ValueId::new(8));
    assert_eq!(kernel_plan.version, 1);
    assert_eq!(
        kernel_plan.family,
        StringKernelPlanFamily::ConcatTripletWindow
    );
    assert_eq!(kernel_plan.corridor_root, ValueId::new(7));
    assert_eq!(kernel_plan.source_root, Some(ValueId::new(1)));
    assert_eq!(
        kernel_plan.borrow_contract,
        Some(StringKernelPlanBorrowContract::BorrowTextFromObject)
    );
    assert_eq!(
        kernel_plan.publish_reason,
        Some(StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        kernel_plan.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
    assert_eq!(kernel_plan.known_length, Some(2));
    assert_eq!(
        kernel_plan.retained_form,
        StringKernelPlanRetainedForm::BorrowedText
    );
    assert_eq!(
        kernel_plan.publication_boundary,
        Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary)
    );
    assert_eq!(
        kernel_plan.publication_contract,
        Some(StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary)
    );
    assert_eq!(
        kernel_plan.publication,
        Some(StringCorridorCandidateState::AlreadySatisfied)
    );
    assert_eq!(
        kernel_plan.materialization,
        Some(StringCorridorCandidateState::Candidate)
    );
    assert_eq!(
        kernel_plan.direct_kernel_entry,
        Some(StringCorridorCandidateState::Candidate)
    );
    assert_eq!(
        kernel_plan.consumer,
        Some(StringKernelPlanConsumer::DirectKernelEntry)
    );
    assert_eq!(kernel_plan.text_consumer, None);
    assert_eq!(kernel_plan.carrier, None);
    assert_eq!(
        kernel_plan.verifier_owner,
        Some(StringKernelPlanVerifierOwner::LoweringDirectKernelEntry)
    );
    let parts = kernel_plan.parts();
    assert_eq!(parts.len(), 3);
    assert_eq!(
        kernel_plan.legality(),
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: true,
            requires_kernel_text_slot: false,
            rejects_early_stable_box_now: false,
            rejects_early_fresh_registry_handle: false,
            rejects_registry_backed_carrier: false,
        }
    );
    assert_eq!(
        kernel_plan.read_alias,
        StringKernelPlanReadAliasFacts {
            same_receiver: true,
            source_window: true,
            followup_substring: false,
            piecewise_subrange: false,
            direct_set_consumer: false,
            shared_receiver: false,
        }
    );
}
