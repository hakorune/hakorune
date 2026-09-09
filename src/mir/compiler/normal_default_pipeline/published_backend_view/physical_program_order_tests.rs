use super::*;

#[test]
fn ordinary_calls_follow_sorted_block_order() {
    let mut function = MirFunction::new(
        crate::mir::FunctionSignature {
            name: "Main.main".into(),
            params: vec![],
            return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    for (block_id, method) in [(BasicBlockId(2), "zeta"), (BasicBlockId(1), "alpha")] {
        let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
            "Worker".into(),
            method.into(),
            0,
        )
        .unwrap();
        let call = MirCall::new(None, Callee::Global(target), vec![]);
        let instruction = MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::I64,
            },
            fault_frame: ValueId(0),
            normal_landing: BasicBlockId(0),
            fault_landing: BasicBlockId(0),
        };
        let mut block = crate::mir::BasicBlock::new(block_id);
        block.add_instruction(instruction);
        function.add_block(block);
    }
    let keys = super::collect_ordinary_calls(&function)
        .unwrap()
        .into_iter()
        .map(|call| super::ordinary_callable_key(&call.callee).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec![
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
                "Worker", "alpha", 0
            ),
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
                "Worker", "zeta", 0
            ),
        ]
    );
}
