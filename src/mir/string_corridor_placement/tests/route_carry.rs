use super::common::*;
use crate::ast::Span;

#[test]
fn borrowed_slice_plan_keeps_publication_contract_for_insert_mid_substring_route() {
    let mut function = make_function(MirType::Box("RuntimeDataBox".to_string()));
    let block = entry_block(&mut function);

    push_const(block, 1, ConstValue::String("xx".to_string()));
    push_const(block, 2, ConstValue::Integer(8));
    push_unknown_span(
        block,
        MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("nyash.string.insert_hsi".to_string())),
            args: vec![ValueId(0), ValueId(1), ValueId(2)],
            effects: EffectMask::PURE,
        },
    );
    push_const(block, 4, ConstValue::Integer(1));
    push_const(block, 5, ConstValue::Integer(17));
    push_unknown_span(
        block,
        method_call(
            ValueId(6),
            ValueId(3),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(5)],
        ),
    );
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(6)),
    });

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    crate::mir::refresh_function_string_corridor_relations(&mut function);
    refresh_function_string_corridor_candidates(&mut function);

    let candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(6))
        .expect("substring candidates");
    let publication = candidates
        .iter()
        .find(|candidate| candidate.kind == StringCorridorCandidateKind::PublicationSink)
        .expect("publication sink candidate");
    let plan = publication.plan.expect("plan metadata on substring result");
    assert_eq!(plan.corridor_root, ValueId(6));
    assert_eq!(plan.source_root, Some(ValueId(3)));
    assert_eq!(plan.start, Some(ValueId(4)));
    assert_eq!(plan.end, Some(ValueId(5)));
    assert_eq!(
        plan.publish_reason,
        Some(StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        plan.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
    assert_eq!(
        plan.publication_contract,
        Some(StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary)
    );
    assert!(matches!(
        plan.proof,
        StringCorridorCandidateProof::BorrowedSlice {
            source: ValueId(3),
            start: ValueId(4),
            end: ValueId(5),
        }
    ));
}

#[test]
fn refresh_function_carries_corridor_candidates_across_narrow_phi_route() {
    let mut function = make_function(MirType::Void);
    function.add_block(BasicBlock::new(BasicBlockId(1)));
    function.add_block(BasicBlock::new(BasicBlockId(2)));
    function.add_block(BasicBlock::new(BasicBlockId(3)));
    function.add_block(BasicBlock::new(BasicBlockId(4)));

    let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });

    let header = function.blocks.get_mut(&BasicBlockId(1)).expect("header");
    header.instructions.push(MirInstruction::Phi {
        dst: ValueId(21),
        inputs: vec![
            (BasicBlockId(0), ValueId(0)),
            (BasicBlockId(3), ValueId(22)),
        ],
        type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
    });
    header.instruction_spans.push(Span::unknown());
    header.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(2),
        edge_args: None,
    });

    let body = function.blocks.get_mut(&BasicBlockId(2)).expect("body");
    push_const(body, 46, ConstValue::Integer(0));
    push_const(body, 47, ConstValue::Integer(1));
    push_const(body, 48, ConstValue::Integer(2));
    push_unknown_span(
        body,
        method_call(
            ValueId(26),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(46), ValueId(47)],
        ),
    );
    push_unknown_span(
        body,
        method_call(
            ValueId(27),
            ValueId(21),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(47), ValueId(48)],
        ),
    );
    push_const(body, 66, ConstValue::String("xx".to_string()));
    push_const(body, 71, ConstValue::Integer(1));
    push_const(body, 72, ConstValue::Integer(3));
    push_unknown_span(
        body,
        MirInstruction::Call {
            dst: Some(ValueId(36)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId(26),
                ValueId(66),
                ValueId(27),
                ValueId(71),
                ValueId(72),
            ],
            effects: EffectMask::PURE,
        },
    );
    body.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(3),
        edge_args: None,
    });

    let latch = function.blocks.get_mut(&BasicBlockId(3)).expect("latch");
    latch.instructions.push(MirInstruction::Phi {
        dst: ValueId(22),
        inputs: vec![(BasicBlockId(2), ValueId(36))],
        type_hint: Some(MirType::Box("RuntimeDataBox".to_string())),
    });
    latch.instruction_spans.push(Span::unknown());
    latch.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });

    let exit = function.blocks.get_mut(&BasicBlockId(4)).expect("exit");
    exit.set_terminator(MirInstruction::Return { value: None });

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    crate::mir::refresh_function_string_corridor_relations(&mut function);
    refresh_function_string_corridor_candidates(&mut function);

    let helper = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(36))
        .expect("helper candidates");
    assert!(helper.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::DirectKernelEntry && candidate.plan.is_some()
    }));

    let latch_candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(22))
        .expect("phi %22 candidates");
    assert!(latch_candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::DirectKernelEntry && candidate.plan.is_some()
    }));
    assert!(latch_candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::PublicationSink && candidate.plan.is_some()
    }));
    assert!(latch_candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::MaterializationSink
            && candidate.plan.is_some()
    }));
    assert!(!latch_candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion }));

    let header_candidates = function
        .metadata
        .string_corridor_candidates
        .get(&ValueId(21))
        .expect("phi %21 candidates");
    assert!(header_candidates
        .iter()
        .all(|candidate| candidate.plan.is_none()));
    assert!(header_candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::PublicationSink }));
    assert!(header_candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::MaterializationSink }));
    assert!(header_candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::DirectKernelEntry }));
    assert!(!header_candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion }));
}
