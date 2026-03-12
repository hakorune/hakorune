#[test]
fn core13_array_boxcall_push_len_get() {
    use crate::backend::vm::VM;
    use crate::mir::definitions::call_unified::TypeCertainty;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };

    // Build: a = new ArrayBox(); a.push(7); r = a.len() + a.get(0); return r
    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;
    let a = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: a,
            box_type: "ArrayBox".into(),
            args: vec![],
        });
    // push(7)
    let seven = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: seven,
            value: ConstValue::Integer(7),
        });
    f.get_block_mut(bb).unwrap().add_instruction(
        crate::mir::ssot::method_call::runtime_method_call(
            None,
            a,
            "ArrayBox",
            "push",
            vec![seven],
            EffectMask::PURE,
            TypeCertainty::Known,
        ),
    );
    // len()
    let ln = f.next_value_id();
    f.get_block_mut(bb).unwrap().add_instruction(
        crate::mir::ssot::method_call::runtime_method_call(
            Some(ln),
            a,
            "ArrayBox",
            "len",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ),
    );
    // get(0)
    let zero = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: zero,
            value: ConstValue::Integer(0),
        });
    let g0 = f.next_value_id();
    f.get_block_mut(bb).unwrap().add_instruction(
        crate::mir::ssot::method_call::runtime_method_call(
            Some(g0),
            a,
            "ArrayBox",
            "get",
            vec![zero],
            EffectMask::PURE,
            TypeCertainty::Known,
        ),
    );
    // sum
    let sum = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: sum,
            op: crate::mir::BinaryOp::Add,
            lhs: ln,
            rhs: g0,
        });
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(sum) });

    let mut m = MirModule::new("core13_array_push_len_get".into());
    m.add_function(f);
    let mut vm = VM::new();
    let out = vm.execute_module(&m).expect("vm exec");
    assert_eq!(out.to_string_box().value, "8");
}

#[test]
fn core13_array_boxcall_set_get() {
    use crate::backend::vm::VM;
    use crate::mir::definitions::call_unified::TypeCertainty;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };

    // Build: a = new ArrayBox(); a.set(0, 5); return a.get(0)
    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;
    let a = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: a,
            box_type: "ArrayBox".into(),
            args: vec![],
        });
    let zero = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: zero,
            value: ConstValue::Integer(0),
        });
    let five = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: five,
            value: ConstValue::Integer(5),
        });
    f.get_block_mut(bb).unwrap().add_instruction(
        crate::mir::ssot::method_call::runtime_method_call(
            None,
            a,
            "ArrayBox",
            "set",
            vec![zero, five],
            EffectMask::PURE,
            TypeCertainty::Known,
        ),
    );
    let outv = f.next_value_id();
    let zero2 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: zero2,
            value: ConstValue::Integer(0),
        });
    f.get_block_mut(bb).unwrap().add_instruction(
        crate::mir::ssot::method_call::runtime_method_call(
            Some(outv),
            a,
            "ArrayBox",
            "get",
            vec![zero2],
            EffectMask::PURE,
            TypeCertainty::Known,
        ),
    );
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(outv) });

    let mut m = MirModule::new("core13_array_set_get".into());
    m.add_function(f);
    let mut vm = VM::new();
    let out = vm.execute_module(&m).expect("vm exec");
    assert_eq!(out.to_string_box().value, "5");
}
