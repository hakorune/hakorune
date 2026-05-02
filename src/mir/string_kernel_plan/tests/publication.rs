use super::*;

#[test]
fn infer_string_kernel_text_consumer_marks_return_boundary_as_explicit_cold_publish() {
    use crate::ast::Span;

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(3),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(8),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(2),
                ValueId::new(3),
            ],
            effects: EffectMask::PURE,
        },
    ]);
    block.instruction_spans.extend(vec![Span::unknown(); 4]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    assert_eq!(
        infer_string_kernel_text_consumer(&function, ValueId::new(10)),
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    );
}

#[test]
fn derive_string_kernel_plan_refines_explicit_cold_publish_reason() {
    use crate::ast::Span;

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(3),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(8),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(2),
                ValueId::new(3),
            ],
            effects: EffectMask::PURE,
        },
    ]);
    block.instruction_spans.extend(vec![Span::unknown(); 4]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });

    let plan = StringCorridorCandidatePlan {
        corridor_root: ValueId::new(10),
        source_root: Some(ValueId::new(0)),
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
            left_value: Some(ValueId::new(0)),
            left_source: ValueId::new(0),
            left_start: ValueId::new(2),
            left_end: ValueId::new(2),
            middle: ValueId::new(1),
            right_value: Some(ValueId::new(0)),
            right_source: ValueId::new(0),
            right_start: ValueId::new(2),
            right_end: ValueId::new(3),
            shared_source: true,
        },
    };
    let candidates = vec![StringCorridorCandidate {
        kind: StringCorridorCandidateKind::DirectKernelEntry,
        state: StringCorridorCandidateState::Candidate,
        reason: "direct kernel entry candidate",
        plan: Some(plan),
        publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
    }];

    let kernel_plan =
        derive_string_kernel_plan(&function, ValueId::new(10), &candidates).expect("kernel plan");

    assert_eq!(
        kernel_plan.publish_reason,
        Some(StringPublishReason::ExplicitApiReplay)
    );
    assert_eq!(
        kernel_plan.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
}
