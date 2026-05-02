use super::*;

#[test]
fn refresh_function_collects_string_kernel_plans() {
    let mut function = make_loop_function();
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
    function.metadata.string_corridor_candidates.insert(
        ValueId::new(8),
        vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "borrowed slice corridor can target a direct kernel entry before publication",
            plan: Some(plan),
            publication_boundary: None,
        }],
    );

    refresh_function_string_kernel_plans(&mut function);

    let kernel_plans = &function.metadata.string_kernel_plans;
    let kernel_plan = kernel_plans.get(&ValueId::new(8)).expect("kernel plan");
    assert_eq!(kernel_plan.version, 1);
    assert_eq!(
        kernel_plan.family,
        StringKernelPlanFamily::ConcatTripletWindow
    );
    assert_eq!(
        kernel_plan.consumer,
        Some(StringKernelPlanConsumer::DirectKernelEntry)
    );
}
