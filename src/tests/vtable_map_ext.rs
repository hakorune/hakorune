#[test]
fn vtable_map_keys_values_delete_clear() {
    use crate::backend::VM;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };
    use crate::mir::definitions::call_unified::TypeCertainty;
    std::env::set_var("NYASH_ABI_VTABLE", "1");

    // keys/values size check
    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;
    let m = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: m,
            box_type: "MapBox".into(),
            args: vec![],
        });
    // set two entries
    let k1 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k1,
            value: ConstValue::String("a".into()),
        });
    let v1 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v1,
            value: ConstValue::Integer(1),
        });
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m,
            "MapBox",
            "set",
            vec![k1, v1],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let k2 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k2,
            value: ConstValue::String("b".into()),
        });
    let v2 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v2,
            value: ConstValue::Integer(2),
        });
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m,
            "MapBox",
            "set",
            vec![k2, v2],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    // keys().len + values().len == 4
    let keys = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(keys),
            m,
            "MapBox",
            "keys",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let klen = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(klen),
            keys,
            "ArrayBox",
            "len",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let vals = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(vals),
            m,
            "MapBox",
            "values",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let vlen = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(vlen),
            vals,
            "ArrayBox",
            "len",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let sum = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::BinOp {
            dst: sum,
            op: crate::mir::BinaryOp::Add,
            lhs: klen,
            rhs: vlen,
        });
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(sum) });
    let mut m1 = MirModule::new("map_keys_values".into());
    m1.add_function(f);
    let mut vm1 = VM::new();
    let out1 = vm1.execute_module(&m1).expect("vm exec");
    assert_eq!(out1.to_string_box().value, "4");

    // delete + clear → size 0
    let sig2 = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f2 = MirFunction::new(sig2, BasicBlockId::new(0));
    let bb2 = f2.entry_block;
    let m2v = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: m2v,
            box_type: "MapBox".into(),
            args: vec![],
        });
    let k = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k,
            value: ConstValue::String("x".into()),
        });
    let v = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v,
            value: ConstValue::String("y".into()),
        });
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2v,
            "MapBox",
            "set",
            vec![k, v],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let dk = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: dk,
            value: ConstValue::String("x".into()),
        });
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2v,
            "MapBox",
            "delete",
            vec![dk],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            None,
            m2v,
            "MapBox",
            "clear",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    let sz = f2.next_value_id();
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(sz),
            m2v,
            "MapBox",
            "size",
            vec![],
            EffectMask::PURE,
            TypeCertainty::Known,
        ));
    f2.get_block_mut(bb2)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(sz) });
    let mut mm2 = MirModule::new("map_delete_clear".into());
    mm2.add_function(f2);
    let mut vm2 = VM::new();
    let out2 = vm2.execute_module(&mm2).expect("vm exec");
    assert_eq!(out2.to_string_box().value, "0");
}

#[test]
fn vtable_method_callee_map_set_get_roundtrip() {
    use crate::backend::VM;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    std::env::set_var("NYASH_ABI_VTABLE", "1");

    // Canonical callsite path: Call(callee=Method) should support MapBox set/get via slot dispatch.
    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::String,
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;

    let mapv = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: mapv,
            box_type: "MapBox".into(),
            args: vec![],
        });

    let k = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k,
            value: ConstValue::String("k".into()),
        });
    let v = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v,
            value: ConstValue::String("v".into()),
        });

    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".into(),
                method: "set".into(),
                receiver: Some(mapv),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![k, v],
            effects: EffectMask::PURE,
        });

    let k2 = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: k2,
            value: ConstValue::String("k".into()),
        });

    let got = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: Some(got),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".into(),
                method: "get".into(),
                receiver: Some(mapv),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![k2],
            effects: EffectMask::PURE,
        });

    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(got) });

    let mut m = MirModule::new("map_method_callee_roundtrip".into());
    m.add_function(f);

    let mut vm = VM::new();
    let out = vm.execute_module(&m).expect("vm exec");
    assert_eq!(out.to_string_box().value, "v");
}

#[test]
fn vtable_method_callee_map_setfield_getfield_roundtrip() {
    use crate::backend::VM;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    std::env::set_var("NYASH_ABI_VTABLE", "1");

    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));
    let bb = f.entry_block;

    let mapv = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::NewBox {
            dst: mapv,
            box_type: "MapBox".into(),
            args: vec![],
        });

    let key = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: key,
            value: ConstValue::String("name".into()),
        });
    let val = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: val,
            value: ConstValue::Integer(42),
        });

    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".into(),
                method: "setField".into(),
                receiver: Some(mapv),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![key, val],
            effects: EffectMask::PURE,
        });

    let got = f.next_value_id();
    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: Some(got),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "MapBox".into(),
                method: "getField".into(),
                receiver: Some(mapv),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![key],
            effects: EffectMask::PURE,
        });

    f.get_block_mut(bb)
        .unwrap()
        .add_instruction(MirInstruction::Return { value: Some(got) });

    let mut m = MirModule::new("map_method_callee_setfield_roundtrip".into());
    m.add_function(f);

    let mut vm = VM::new();
    let out = vm.execute_module(&m).expect("vm exec");
    assert_eq!(out.to_string_box().value, "42");
}
