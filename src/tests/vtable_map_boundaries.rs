#[test]
fn vtable_map_boundary_cases() {
    use crate::backend::VM;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };
    use crate::mir::definitions::call_unified::TypeCertainty;
    std::env::set_var("NYASH_ABI_VTABLE", "1");

    // Case 1: empty-string key set/get/has
    let sig1 = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f1 = MirFunction::new(sig1, BasicBlockId::new(0));
    let bb1 = f1.entry_block;
    let m = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: m,
            box_type: "MapBox".into(),
            args: vec![],
        });
    // set("", 1)
    let k_empty = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k_empty,
            value: ConstValue::String("".into()),
        });
    let v1 = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(1),
        });
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m,
            "MapBox",
            "set",
            vec![k_empty, v1],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // has("") -> true
    let h = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(h),
            m,
            "MapBox",
            "has",
            vec![k_empty],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // get("") -> 1
    let g = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(g),
            m,
            "MapBox",
            "get",
            vec![k_empty],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // return has + get (true->1) + size == 1 + 1 + 1 = 3 (coerce Bool true to 1 via toString parse in BinOp fallback)
    let sz = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(sz),
            m,
            "MapBox",
            "size",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let tmp = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: tmp,
            op: crate::mir::BinaryOp::Add,
            lhs: h,
            rhs: g,
        });
    let sum = f1.next_value_id();
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: sum,
            op: crate::mir::BinaryOp::Add,
            lhs: tmp,
            rhs: sz,
        });
    f1.get_block_mut(bb1)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(sum) });
    let mut m1 = MirModule::new("map_boundary_empty_key".into());
    m1.add_function(f1);
    let mut vm1 = VM::new();
    let out1 = vm1.execute_module(&m1).expect("vm exec");
    // Expect 3 as described above
    assert_eq!(out1.to_string_box().value, "3");

    // Case 2: duplicate key overwrite, missing key get message shape, and delete using slot 205
    let sig2 = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f2 = MirFunction::new(sig2, BasicBlockId::new(0));
    let bb2 = f2.entry_block;
    let m2 = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: m2,
            box_type: "MapBox".into(),
            args: vec![],
        });
    // set("k", 1); set("k", 2)
    let k = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k,
            value: ConstValue::String("k".into()),
        });
    let one = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: one,
            value: ConstValue::Integer(1),
        });
    let two = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: two,
            value: ConstValue::Integer(2),
        });
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2,
            "MapBox",
            "set",
            vec![k, one],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2,
            "MapBox",
            "set",
            vec![k, two],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // get("k") should be 2
    let g2 = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(g2),
            m2,
            "MapBox",
            "get",
            vec![k],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // delete("missing") using method name; ensure no panic and still size==1
    let missing = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: missing,
            value: ConstValue::String("missing".into()),
        });
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2,
            "MapBox",
            "delete",
            vec![missing],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // size()
    let sz2 = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(sz2),
            m2,
            "MapBox",
            "size",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let sum2 = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: sum2,
            op: crate::mir::BinaryOp::Add,
            lhs: g2,
            rhs: sz2,
        });
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(sum2) });
    let mut m2m = MirModule::new("map_boundary_overwrite_delete".into());
    m2m.add_function(f2);
    let mut vm2 = VM::new();
    let out2 = vm2.execute_module(&m2m).expect("vm exec");
    // get("k") == 2 and size()==1 => 3
    assert_eq!(out2.to_string_box().value, "3");
}
