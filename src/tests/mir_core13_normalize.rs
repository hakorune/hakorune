mod tests {
    use crate::mir::contracts::backend_core_ops::lowered_away_tag;
    use crate::mir::optimizer::MirOptimizer;
    use crate::mir::{
        BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction as I, MirModule, MirType, ValueId,
    };
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

    fn mk_func(name: &str) -> (MirFunction, BasicBlockId) {
        let sig = FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        (MirFunction::new(sig, entry), entry)
    }

    #[test]
    fn core13_normalize_keeps_canonical_method_call_forms() {
        let (mut f, bb0) = mk_func("core13_method_call_stable");
        let mut b0 = BasicBlock::new(bb0);
        let arr = ValueId::new(0);
        let idx = ValueId::new(1);
        let val = ValueId::new(2);
        let get_dst = ValueId::new(3);

        b0.add_instruction(I::NewBox {
            dst: arr,
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        b0.add_instruction(I::Const {
            dst: idx,
            value: ConstValue::Integer(0),
        });
        b0.add_instruction(I::Const {
            dst: val,
            value: ConstValue::Integer(7),
        });
        // Use canonical Call with Callee::Method (replaces BoxCall)
        b0.add_instruction(I::Call {
            dst: Some(get_dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "get".to_string(),
                receiver: Some(arr),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![idx],
            effects: EffectMask::READ,
        });
        b0.add_instruction(I::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "set".to_string(),
                receiver: Some(arr),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![idx, val],
            effects: EffectMask::WRITE,
        });
        b0.add_instruction(I::Return { value: Some(get_dst) });
        f.add_block(b0);

        let mut m = MirModule::new("test_core13_method_call".into());
        m.add_function(f);

        let mut opt = MirOptimizer::new();
        let _ = opt.optimize_module(&mut m);

        let func = m.get_function("core13_method_call_stable").unwrap();
        let block = func.get_block(bb0).unwrap();
        let mut saw_get = false;
        let mut saw_set = false;
        for inst in block.all_instructions() {
            match inst {
                I::Call {
                    callee: Some(Callee::Method { method, .. }),
                    ..
                } if method == "get" => saw_get = true,
                I::Call {
                    callee: Some(Callee::Method { method, .. }),
                    ..
                } if method == "set" => saw_set = true,
                _ => {}
            }
        }
        assert!(saw_get && saw_set);
    }

    #[test]
    fn core13_normalize_produces_no_lowered_away_tags() {
        let (mut f, bb0) = mk_func("core13_no_lowered_away");
        let mut b0 = BasicBlock::new(bb0);
        let box_val = ValueId::new(0);
        let ref_val = ValueId::new(1);
        let field_name = ValueId::new(2);
        let set_val = ValueId::new(3);

        b0.add_instruction(I::NewBox {
            dst: box_val,
            box_type: "RecordBox".to_string(),
            args: vec![],
        });
        b0.add_instruction(I::RefNew {
            dst: ref_val,
            box_val,
        });
        b0.add_instruction(I::Const {
            dst: field_name,
            value: ConstValue::String("x".to_string()),
        });
        b0.add_instruction(I::Const {
            dst: set_val,
            value: ConstValue::Integer(9),
        });
        b0.add_instruction(I::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: ref_val,
        });
        // Use canonical Call with Callee::Method (replaces BoxCall)
        b0.add_instruction(I::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "InstanceBox".to_string(),
                method: "setField".to_string(),
                receiver: Some(ref_val),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![field_name, set_val],
            effects: EffectMask::WRITE,
        });
        b0.add_instruction(I::Return { value: None });
        f.add_block(b0);

        let mut m = MirModule::new("test_core13_lowered_away".into());
        m.add_function(f);

        let mut opt = MirOptimizer::new();
        let _ = opt.optimize_module(&mut m);

        let func = m.get_function("core13_no_lowered_away").unwrap();
        let block = func.get_block(bb0).unwrap();
        for inst in block.all_instructions() {
            assert!(
                lowered_away_tag(inst).is_none(),
                "lowered-away instruction survived normalize: {:?}",
                inst
            );
        }
    }
}
