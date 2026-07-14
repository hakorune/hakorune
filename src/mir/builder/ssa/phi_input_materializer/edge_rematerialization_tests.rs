use super::edge_rematerialization::for_pred;
use super::test_support::test_signature;
use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId,
};
use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};

#[test]
fn rematerializes_runtime_data_substring_for_phi_pred() {
    let mut func = MirFunction::new(test_signature("substring_phi"), BasicBlockId::new(0));
    func.add_block(BasicBlock::new(BasicBlockId::new(1)));
    func.add_block(BasicBlock::new(BasicBlockId::new(2)));

    let text = func.next_value_id();
    let start = func.next_value_id();
    let end = func.next_value_id();
    let substring = func.next_value_id();

    {
        let entry = func.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::Const {
            dst: text,
            value: ConstValue::String("abcdef".to_string()),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: start,
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: end,
            value: ConstValue::Integer(3),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: start,
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: Some(substring),
            func: ValueId::new(u32::MAX),
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(text),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![text, start, end],
            effects: EffectMask::READ,
        });

    let materialized = for_pred(&mut func, BasicBlockId::new(2), substring, "test", "phi").unwrap();
    assert_ne!(materialized, substring);

    let pred = func.get_block(BasicBlockId::new(2)).unwrap();
    assert!(matches!(
        pred.instructions.last(),
        Some(MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                method,
                receiver: Some(receiver),
                ..
            }),
            args,
            ..
        }) if *dst == materialized
            && method == "substring"
            && args.first() == Some(receiver)
    ));
}
