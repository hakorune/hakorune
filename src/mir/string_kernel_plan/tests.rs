use crate::mir::*;

fn make_loop_function() -> MirFunction {
    let entry = BasicBlockId::new(0);
    let header = BasicBlockId::new(18);
    let body = BasicBlockId::new(19);
    let exit = BasicBlockId::new(21);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );

    function
        .blocks
        .get_mut(&entry)
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::String("line-seed-abcdef".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(4),
                src: ValueId::new(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(5),
                value: ConstValue::Integer(16),
            },
        ]);

    let mut header_block = BasicBlock::new(header);
    header_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(15),
            inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
            type_hint: Some(MirType::String),
        },
        MirInstruction::Const {
            dst: ValueId::new(41),
            value: ConstValue::Integer(300000),
        },
        MirInstruction::Compare {
            dst: ValueId::new(37),
            op: CompareOp::Lt,
            lhs: ValueId::new(15),
            rhs: ValueId::new(41),
        },
        MirInstruction::Branch {
            condition: ValueId::new(37),
            then_bb: body,
            else_bb: exit,
            then_edge_args: None,
            else_edge_args: None,
        },
    ]);
    function.blocks.insert(header, header_block);

    let mut body_block = BasicBlock::new(body);
    body_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(50),
            value: ConstValue::Integer(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(47),
            op: BinaryOp::Div,
            lhs: ValueId::new(5),
            rhs: ValueId::new(50),
        },
        MirInstruction::Const {
            dst: ValueId::new(66),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Copy {
            dst: ValueId::new(36),
            src: ValueId::new(21),
        },
    ]);
    function.blocks.insert(body, body_block);
    function.blocks.insert(exit, BasicBlock::new(exit));
    function
}

#[test]
fn derive_string_kernel_plan_prefers_direct_entry_and_collects_barriers() {
    let function = make_loop_function();
    let publication_plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: Some(crate::mir::StringPublishReason::StableObjectDemand),
            publish_repr_policy: Some(crate::mir::StringPublishReprPolicy::StableOwned),
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
        Some(crate::mir::StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        kernel_plan.publish_repr_policy,
        Some(crate::mir::StringPublishReprPolicy::StableOwned)
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

#[test]
fn derive_string_kernel_plan_collects_concat_loop_payload() {
    let function = make_loop_function();
    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(21),
            source_root: Some(ValueId::new(21)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(71)),
            end: Some(ValueId::new(72)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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

#[test]
fn derive_string_kernel_plan_marks_slot_text_consumer_for_same_corridor_substring() {
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
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
            value: ConstValue::Integer(6),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(3),
                ValueId::new(4),
            ],
            effects: EffectMask::PURE,
        },
        method_call(
            ValueId::new(11),
            ValueId::new(10),
            "substring",
            vec![ValueId::new(3), ValueId::new(4)],
        ),
    ]);
    block.instruction_spans.extend(vec![Span::unknown(); 6]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(3)),
            end: Some(ValueId::new(4)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(0)),
                left_source: ValueId::new(0),
                left_start: ValueId::new(3),
                left_end: ValueId::new(2),
                middle: ValueId::new(1),
                right_value: Some(ValueId::new(0)),
                right_source: ValueId::new(0),
                right_start: ValueId::new(2),
                right_end: ValueId::new(4),
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
        kernel_plan.text_consumer,
        Some(StringKernelPlanTextConsumer::SlotText)
    );
    assert_eq!(
        kernel_plan.carrier,
        Some(StringKernelPlanCarrier::KernelTextSlot)
    );
    assert_eq!(
        kernel_plan.legality(),
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: true,
            requires_kernel_text_slot: true,
            rejects_early_stable_box_now: true,
            rejects_early_fresh_registry_handle: true,
            rejects_registry_backed_carrier: true,
        }
    );
    assert_eq!(
        kernel_plan.read_alias,
        StringKernelPlanReadAliasFacts {
            same_receiver: true,
            source_window: true,
            followup_substring: true,
            piecewise_subrange: true,
            direct_set_consumer: false,
            shared_receiver: false,
        }
    );
}

#[test]
fn derive_string_kernel_plan_marks_shared_receiver_alias_fact() {
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

    fn method_call(
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
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
            value: ConstValue::Integer(6),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(3),
                ValueId::new(4),
            ],
            effects: EffectMask::PURE,
        },
        method_call(
            None,
            ValueId::new(20),
            "set",
            vec![ValueId::new(3), ValueId::new(10)],
        ),
        method_call(
            Some(ValueId::new(11)),
            ValueId::new(10),
            "substring",
            vec![ValueId::new(3), ValueId::new(4)],
        ),
    ]);
    block.instruction_spans.extend(vec![Span::unknown(); 7]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });

    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(3)),
            end: Some(ValueId::new(4)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(0)),
                left_source: ValueId::new(0),
                left_start: ValueId::new(3),
                left_end: ValueId::new(2),
                middle: ValueId::new(1),
                right_value: Some(ValueId::new(0)),
                right_source: ValueId::new(0),
                right_start: ValueId::new(2),
                right_end: ValueId::new(4),
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
        kernel_plan.text_consumer,
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    );
    assert_eq!(
        kernel_plan.read_alias,
        StringKernelPlanReadAliasFacts {
            same_receiver: true,
            source_window: true,
            followup_substring: false,
            piecewise_subrange: true,
            direct_set_consumer: false,
            shared_receiver: true,
        }
    );
}

#[test]
fn derive_string_kernel_plan_marks_direct_set_consumer_alias_fact() {
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

    fn method_call(
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
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
            value: ConstValue::Integer(6),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(3),
                ValueId::new(4),
            ],
            effects: EffectMask::PURE,
        },
        method_call(
            None,
            ValueId::new(20),
            "set",
            vec![ValueId::new(3), ValueId::new(10)],
        ),
    ]);
    block.instruction_spans.extend(vec![Span::unknown(); 6]);
    block.set_terminator(MirInstruction::Return { value: None });

    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
        corridor_root: ValueId::new(10),
        source_root: Some(ValueId::new(0)),
        borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        start: Some(ValueId::new(3)),
        end: Some(ValueId::new(4)),
        known_length: Some(2),
        publication_contract: Some(
            crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(ValueId::new(0)),
            left_source: ValueId::new(0),
            left_start: ValueId::new(3),
            left_end: ValueId::new(2),
            middle: ValueId::new(1),
            right_value: Some(ValueId::new(0)),
            right_source: ValueId::new(0),
            right_start: ValueId::new(2),
            right_end: ValueId::new(4),
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
        kernel_plan.read_alias,
        StringKernelPlanReadAliasFacts {
            same_receiver: true,
            source_window: true,
            followup_substring: false,
            piecewise_subrange: true,
            direct_set_consumer: true,
            shared_receiver: false,
        }
    );
}

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
            callee: Some(crate::mir::Callee::Extern(
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
            callee: Some(crate::mir::Callee::Extern(
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

    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
        Some(crate::mir::StringPublishReason::ExplicitApiReplay)
    );
    assert_eq!(
        kernel_plan.publish_repr_policy,
        Some(crate::mir::StringPublishReprPolicy::StableOwned)
    );
}

#[test]
fn refresh_function_collects_string_kernel_plans() {
    let mut function = make_loop_function();
    let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
