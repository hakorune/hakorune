use super::*;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature,
    MirInstruction, MirType, ValueId,
};

#[test]
fn slice_fact_emits_borrowed_corridor_and_sink_candidates() {
    let fact = StringCorridorFact::str_slice(StringCorridorCarrier::MethodCall);
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let function = MirFunction::new(signature, BasicBlockId::new(0));
    let def_map = build_value_def_map(&function);
    let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

    assert!(candidates
        .iter()
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::BorrowCorridorFusion }));
    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::PublicationSink
            && candidate.state == StringCorridorCandidateState::Candidate
    }));
    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::MaterializationSink
            && candidate.state == StringCorridorCandidateState::Candidate
    }));
}

#[test]
fn freeze_fact_marks_materialization_sink_as_already_satisfied() {
    let fact = StringCorridorFact::freeze_str(StringCorridorCarrier::CanonicalIntrinsic);
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let function = MirFunction::new(signature, BasicBlockId::new(0));
    let def_map = build_value_def_map(&function);
    let candidates = infer_candidates(&function, ValueId::new(1), &fact, &def_map);

    assert!(candidates.iter().any(|candidate| {
        candidate.kind == StringCorridorCandidateKind::MaterializationSink
            && candidate.state == StringCorridorCandidateState::AlreadySatisfied
    }));
}

#[test]
fn refresh_function_collects_candidates_from_existing_facts() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
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
        .any(|candidate| { candidate.kind == StringCorridorCandidateKind::DirectKernelEntry }));
}

#[test]
fn refresh_function_attaches_plan_metadata_for_concat_corridor_candidates() {
    use crate::ast::Span;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "StringBox",
        "length",
        vec![],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(3),
        op: BinaryOp::Div,
        lhs: ValueId(1),
        rhs: ValueId(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(8),
        op: BinaryOp::Add,
        lhs: ValueId(5),
        rhs: ValueId(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: BinaryOp::Add,
        lhs: ValueId(8),
        rhs: ValueId(6),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(10),
        ValueId(9),
        "RuntimeDataBox",
        "length",
        vec![],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(11),
        value: ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(12),
        op: BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(11),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(13),
        ValueId(9),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(11), ValueId(12)],
    ));
    block.instruction_spans.push(Span::unknown());
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
    use crate::ast::Span;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(1),
        ValueId(0),
        "RuntimeDataBox",
        "length",
        vec![],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(3),
        op: BinaryOp::Div,
        lhs: ValueId(1),
        rhs: ValueId(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(3)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(3), ValueId(1)],
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(7),
        value: ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(8),
        value: ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(9),
        op: BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(8),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(10)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(
            "nyash.string.substring_concat3_hhhii".to_string(),
        )),
        args: vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
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

#[test]
fn borrowed_slice_plan_keeps_publication_contract_for_insert_mid_substring_route() {
    use crate::ast::Span;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Box("RuntimeDataBox".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::String("xx".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(8),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("nyash.string.insert_hsi".to_string())),
        args: vec![ValueId(0), ValueId(1), ValueId(2)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(5),
        value: ConstValue::Integer(17),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(3),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(4), ValueId(5)],
    ));
    block.instruction_spans.push(Span::unknown());
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
    use crate::ast::Span;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
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
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(46),
        value: ConstValue::Integer(0),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(47),
        value: ConstValue::Integer(1),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(48),
        value: ConstValue::Integer(2),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(method_call(
        ValueId(26),
        ValueId(21),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(46), ValueId(47)],
    ));
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(method_call(
        ValueId(27),
        ValueId(21),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(47), ValueId(48)],
    ));
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(66),
        value: ConstValue::String("xx".to_string()),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(71),
        value: ConstValue::Integer(1),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Const {
        dst: ValueId(72),
        value: ConstValue::Integer(3),
    });
    body.instruction_spans.push(Span::unknown());
    body.instructions.push(MirInstruction::Call {
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
    });
    body.instruction_spans.push(Span::unknown());
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
