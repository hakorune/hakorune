#[cfg(all(test, feature = "cranelift-jit", not(feature = "jit-direct-only")))]
mod tests {

    #[allow(unused_imports)]
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };
    use crate::mir::definitions::call_unified::TypeCertainty;

    // Build a MIR that exercises Array.get/set/len, Map.set/size/has/get, and String.len
    #[cfg(feature = "cranelift-jit")]
    fn make_module() -> MirModule {
        let mut module = MirModule::new("identical_collections".to_string());
        let sig = FunctionSignature {
            name: "main".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut f = MirFunction::new(sig, BasicBlockId::new(0));
        let bb = f.entry_block;

        // Build: arr = NewBox(ArrayBox); arr.set(0, "x"); len = arr.len();
        let arr = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::NewBox {
                dst: arr,
                box_type: "ArrayBox".into(),
                args: vec![],
            });
        let idx0 = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: idx0,
                value: ConstValue::Integer(0),
            });
        let s = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: s,
                value: ConstValue::String("x".into()),
            });
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                None,
                arr,
                "ArrayBox",
                "set",
                vec![idx0, s],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));
        let alen = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                Some(alen),
                arr,
                "ArrayBox",
                "len",
                vec![],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));

        // Map: m = NewBox(MapBox); m.set("k", 42); size = m.size(); has = m.has("k"); get = m.get("k")
        let m = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::NewBox {
                dst: m,
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
                value: ConstValue::Integer(42),
            });
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                None,
                m,
                "MapBox",
                "set",
                vec![k, v],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));
        let msize = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                Some(msize),
                m,
                "MapBox",
                "size",
                vec![],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));
        let mhas = f.next_value_id();
        let k2 = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: k2,
                value: ConstValue::String("k".into()),
            });
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                Some(mhas),
                m,
                "MapBox",
                "has",
                vec![k2],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));
        let mget = f.next_value_id();
        let k3 = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: k3,
                value: ConstValue::String("k".into()),
            });
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                Some(mget),
                m,
                "MapBox",
                "get",
                vec![k3],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));

        // String.len: sb = "hello"; slen = sb.len()
        let sb = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: sb,
                value: ConstValue::String("hello".into()),
            });
        let slen = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                Some(slen),
                sb,
                "StringBox",
                "len",
                vec![],
                EffectMask::PURE,
                TypeCertainty::Known,
            ));

        // Return: alen + msize + (mhas?1:0) + slen + (mget coerced to int or 0)
        // Simplify: just return alen
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return { value: Some(alen) });

        module.add_function(f);
        module
    }

    #[cfg(feature = "cranelift-jit")]
    #[test]
    fn identical_vm_and_jit_array_map_string() {
        let module = make_module();
        let mut vm = VM::new();
        // Prefer vtable path for VM
        std::env::set_var("NYASH_ABI_VTABLE", "1");
        let vm_out = vm.execute_module(&module).expect("VM exec");
        let vm_s = vm_out.to_string_box().value;

        // JIT with host bridge enabled for parity
        std::env::set_var("NYASH_JIT_HOST_BRIDGE", "1");
        let jit_out =
            crate::backend::cranelift_compile_and_execute(&module, "identical_collections")
                .expect("JIT exec");
        let jit_s = jit_out.to_string_box().value;

        assert_eq!(
            vm_s, jit_s,
            "VM and JIT results should match for collection ops"
        );
    }
}
