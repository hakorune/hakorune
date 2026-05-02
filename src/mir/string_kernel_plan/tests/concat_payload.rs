use super::*;

#[test]
fn derive_string_kernel_plan_collects_concat_loop_payload() {
    let function = make_loop_function();
    let plan = StringCorridorCandidatePlan {
        corridor_root: ValueId::new(21),
        source_root: Some(ValueId::new(21)),
        borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        start: Some(ValueId::new(71)),
        end: Some(ValueId::new(72)),
        known_length: Some(2),
        publication_contract: Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId::new(26)),
            left_source: ValueId::new(21),
            left_start: ValueId::new(46),
            left_end: ValueId::new(47),
            middle: ValueId::new(66),
            right_value: Some(ValueId::new(27)),
            right_source: ValueId::new(21),
            right_start: ValueId::new(47),
            right_end: ValueId::new(42),
            shared_source: true,
        },
    };
    let candidates = vec![StringCorridorCandidate {
        kind: StringCorridorCandidateKind::DirectKernelEntry,
        state: StringCorridorCandidateState::Candidate,
        reason: "direct kernel entry candidate",
        plan: Some(plan),
        publication_boundary: None,
    }];

    let kernel_plan =
        derive_string_kernel_plan(&function, ValueId::new(21), &candidates).expect("kernel plan");
    let payload = kernel_plan.loop_payload.expect("loop payload");

    assert_eq!(payload.seed_value, ValueId::new(3));
    assert_eq!(payload.seed_literal, "line-seed-abcdef");
    assert_eq!(payload.seed_length, 16);
    assert_eq!(payload.loop_bound, 300000);
    assert_eq!(payload.split_length, 8);
    assert_eq!(kernel_plan.middle_literal.as_deref(), Some("xx"));
}
