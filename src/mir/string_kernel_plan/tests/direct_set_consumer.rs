use super::*;

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
            callee: Some(Callee::Method {
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
            callee: Some(Callee::Extern(
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

    let plan = StringCorridorCandidatePlan {
        corridor_root: ValueId::new(10),
        source_root: Some(ValueId::new(0)),
        borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        start: Some(ValueId::new(3)),
        end: Some(ValueId::new(4)),
        known_length: Some(2),
        publication_contract: Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
