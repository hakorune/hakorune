#[test]
fn mir_numeric_shift_right_negative_is_arithmetic_i64() {
    use crate::backend::VM;
    use crate::mir::{
        BasicBlockId, BinaryOp, ConstValue, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };

    let sig = FunctionSignature {
        name: "Main.main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: Default::default(),
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;

    let lhs = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: lhs,
            value: ConstValue::Integer(-8),
        });
    let rhs = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: rhs,
            value: ConstValue::Integer(1),
        });
    let shifted = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: shifted,
            op: BinaryOp::Shr,
            lhs,
            rhs,
        });
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return {
            value: Some(shifted),
        });

    let mut m = MirModule::new("numeric_shift_semantics".into());
    m.add_function(f);

    let mut vm = VM::new();
    let out = vm.execute_module(&m).expect("vm exec");
    assert_eq!(out.to_string_box().value, "-4");
}
