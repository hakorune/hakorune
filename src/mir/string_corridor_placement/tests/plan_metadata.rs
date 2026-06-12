use super::common::*;

#[test]
fn refresh_function_collects_candidates_from_existing_facts() {
    let mut function = make_function(MirType::Void);
    function.metadata.string_corridor_facts.insert(
        ValueId::new(1),
        StringCorridorFact::str_len(StringCorridorCarrier::MethodCall),
    );

    crate::mir::refresh_function_string_corridor_relations(&mut function);
    refresh_function_string_corridor_candidates(&mut function);

    let candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId::new(1))
        .expect("candidates");
    assert!(candidates
        .iter()
        .any(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry));
}

#[test]
fn refresh_function_attaches_plan_metadata_for_concat_corridor_candidates() {
    let mut function = make_function(MirType::Integer);
    let block = entry_block(&mut function);

    push_unknown_span(
        block,
        method_call(ValueId(1), ValueId(0), "StringBox", "length", vec![]),
    );
    push_const(block, 2, ConstValue::Integer(2));
    push_binop(block, 3, BinaryOp::Div, 1, 2);
    push_const(block, 4, ConstValue::Integer(0));
    push_unknown_span(
        block,
        method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
        ),
    );
    push_unknown_span(
        block,
        method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
        ),
    );
    push_const(block, 7, ConstValue::String("xx".to_string()));
    push_binop(block, 8, BinaryOp::Add, 5, 7);
    push_binop(block, 9, BinaryOp::Add, 8, 6);
    push_unknown_span(
        block,
        method_call(ValueId(10), ValueId(9), "RuntimeDataBox", "length", vec![]),
    );
    push_const(block, 11, ConstValue::Integer(1));
    push_binop(block, 12, BinaryOp::Add, 1, 11);
    push_unknown_span(
        block,
        method_call(
            ValueId(13),
            ValueId(9),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(11), ValueId(12)],
        ),
    );
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(10)),
    });

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    crate::mir::refresh_function_string_corridor_relations(&mut function);
    refresh_function_string_corridor_candidates(&mut function);

    let len_candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(10))
        .expect("len candidates");
    let len_direct = len_candidates
        .iter()
        .find(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry)
        .expect("direct kernel candidate");
    let len_plan = len_direct.plan.expect("plan metadata on len candidate");
    assert_eq!(len_plan.corridor_root, ValueId(9));
    assert_eq!(len_plan.source_root, Some(ValueId(0)));
    assert_eq!(len_plan.known_length, Some(2));
    assert_eq!(len_plan.start, None);
    assert_eq!(len_plan.end, None);
    assert!(matches!(
        len_plan.proof,
        StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId(5)),
            left_source: ValueId(0),
            left_start: ValueId(4),
            left_end: ValueId(3),
            middle: ValueId(7),
            right_value: Some(ValueId(6)),
            right_source: ValueId(0),
            right_start: ValueId(3),
            right_end: ValueId(1),
            shared_source: true,
        }
    ));

    let substring_candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(13))
        .expect("substring candidates");
    let publication = substring_candidates
        .iter()
        .find(|candidate| candidate.kind == StringCorridorCandidateKind::PublicationSink)
        .expect("publication candidate");
    let substring_plan = publication
        .plan
        .expect("plan metadata on substring candidate");
    assert_eq!(substring_plan.corridor_root, ValueId(9));
    assert_eq!(substring_plan.source_root, Some(ValueId(0)));
    assert_eq!(substring_plan.start, Some(ValueId(11)));
    assert_eq!(substring_plan.end, Some(ValueId(12)));
    assert_eq!(substring_plan.known_length, Some(2));
    assert_eq!(
        substring_plan.publish_reason,
        Some(StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        substring_plan.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
    assert_eq!(
        substring_plan.publication_contract,
        Some(StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary)
    );
}

#[test]
fn runtime_export_substring_concat_keeps_publication_sink_candidate() {
    let mut function = make_function(MirType::Box("RuntimeDataBox".to_string()));
    let block = entry_block(&mut function);

    push_unknown_span(
        block,
        method_call(ValueId(1), ValueId(0), "RuntimeDataBox", "length", vec![]),
    );
    push_const(block, 2, ConstValue::Integer(2));
    push_binop(block, 3, BinaryOp::Div, 1, 2);
    push_const(block, 4, ConstValue::Integer(0));
    push_unknown_span(
        block,
        method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
        ),
    );
    push_unknown_span(
        block,
        method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
        ),
    );
    push_const(block, 7, ConstValue::String("xx".to_string()));
    push_const(block, 8, ConstValue::Integer(1));
    push_binop(block, 9, BinaryOp::Add, 1, 8);
    push_unknown_span(
        block,
        MirInstruction::Call {
            dst: Some(ValueId(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
            effects: EffectMask::PURE,
        },
    );
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(10)),
    });

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    crate::mir::refresh_function_string_corridor_relations(&mut function);
    refresh_function_string_corridor_candidates(&mut function);

    let candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(10))
        .expect("substring concat result candidates");
    let publication = candidates
        .iter()
        .find(|candidate| candidate.kind == StringCorridorCandidateKind::PublicationSink)
        .expect("publication sink candidate");
    let plan = publication.plan.expect("plan metadata on helper result");
    assert_eq!(plan.corridor_root, ValueId(10));
    assert_eq!(plan.source_root, Some(ValueId(0)));
    assert_eq!(plan.start, Some(ValueId(8)));
    assert_eq!(plan.end, Some(ValueId(9)));
    assert_eq!(plan.known_length, Some(2));
    assert_eq!(
        plan.publish_reason,
        Some(StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        plan.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
    assert!(matches!(
        plan.proof,
        StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId(5)),
            left_source: ValueId(0),
            left_start: ValueId(4),
            left_end: ValueId(3),
            middle: ValueId(7),
            right_value: Some(ValueId(6)),
            right_source: ValueId(0),
            right_start: ValueId(3),
            right_end: ValueId(1),
            shared_source: true,
        }
    ));
}
