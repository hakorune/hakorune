#[cfg(test)]
mod tests {
    use crate::runtime::type_registry::{known_methods_for, resolve_slot_by_name};

    #[test]
    fn type_registry_resolves_core_slots() {
        // MapBox
        assert_eq!(resolve_slot_by_name("MapBox", "size", 0), Some(200));
        assert_eq!(resolve_slot_by_name("MapBox", "len", 0), Some(201));
        assert_eq!(resolve_slot_by_name("MapBox", "has", 1), Some(202));
        assert_eq!(resolve_slot_by_name("MapBox", "get", 1), Some(203));
        assert_eq!(resolve_slot_by_name("MapBox", "set", 2), Some(204));
        assert_eq!(resolve_slot_by_name("MapBox", "delete", 1), Some(205));
        assert_eq!(resolve_slot_by_name("MapBox", "remove", 1), Some(205));
        assert_eq!(resolve_slot_by_name("MapBox", "keys", 0), Some(206));
        assert_eq!(resolve_slot_by_name("MapBox", "values", 0), Some(207));
        assert_eq!(resolve_slot_by_name("MapBox", "clear", 0), Some(208));
        assert_eq!(resolve_slot_by_name("MapBox", "length", 0), Some(200));
        // ArrayBox
        assert_eq!(resolve_slot_by_name("ArrayBox", "get", 1), Some(100));
        assert_eq!(resolve_slot_by_name("ArrayBox", "set", 2), Some(101));
        assert_eq!(resolve_slot_by_name("ArrayBox", "length", 0), Some(102));
        assert_eq!(resolve_slot_by_name("ArrayBox", "len", 0), Some(102));
        assert_eq!(resolve_slot_by_name("ArrayBox", "size", 0), Some(102));
        assert_eq!(resolve_slot_by_name("ArrayBox", "clear", 0), Some(105));
        assert_eq!(resolve_slot_by_name("ArrayBox", "remove", 1), Some(112));
        assert_eq!(resolve_slot_by_name("ArrayBox", "insert", 2), Some(113));
        // StringBox
        assert_eq!(resolve_slot_by_name("StringBox", "len", 0), Some(300));
        assert_eq!(resolve_slot_by_name("StringBox", "size", 0), Some(300));
        assert_eq!(resolve_slot_by_name("StringBox", "substr", 2), Some(301));
        assert_eq!(resolve_slot_by_name("StringBox", "find", 1), Some(303));
        assert_eq!(resolve_slot_by_name("StringBox", "find", 2), Some(303));
        assert_eq!(resolve_slot_by_name("String", "size", 0), Some(300));
        assert_eq!(resolve_slot_by_name("String", "find", 2), Some(303));

        // Known methods listing should include representative entries
        let mm = known_methods_for("MapBox").expect("map methods");
        assert!(mm.contains(&"size"));
        assert!(mm.contains(&"length"));
        assert!(mm.contains(&"get"));
        assert!(mm.contains(&"set"));
    }

    #[test]
    #[ignore]
    fn vm_vtable_map_set_get_has() {
        use crate::backend::VM;
        use crate::mir::definitions::call_unified::TypeCertainty;
        use crate::mir::{
            BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
            MirModule, MirType,
        };

        // Enable vtable-preferred path
        std::env::set_var("NYASH_ABI_VTABLE", "1");

        // Program: m = new MapBox(); m.set("k","v"); h = m.has("k"); g = m.get("k"); return g
        let mut m = MirModule::new("nyash_abi_map_get".into());
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
        f.get_block_mut(bb).unwrap().add_instruction(
            crate::mir::ssot::method_call::runtime_method_call(
                None,
                mapv,
                "MapBox",
                "set",
                vec![k, v],
                EffectMask::PURE,
                TypeCertainty::Known,
            ),
        );

        let k2 = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: k2,
                value: ConstValue::String("k".into()),
            });
        let hasv = f.next_value_id();
        f.get_block_mut(bb).unwrap().add_instruction(
            crate::mir::ssot::method_call::runtime_method_call(
                Some(hasv),
                mapv,
                "MapBox",
                "has",
                vec![k2],
                EffectMask::PURE,
                TypeCertainty::Known,
            ),
        );

        let k3 = f.next_value_id();
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: k3,
                value: ConstValue::String("k".into()),
            });
        let got = f.next_value_id();
        f.get_block_mut(bb).unwrap().add_instruction(
            crate::mir::ssot::method_call::runtime_method_call(
                Some(got),
                mapv,
                "MapBox",
                "get",
                vec![k3],
                EffectMask::PURE,
                TypeCertainty::Known,
            ),
        );
        f.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return { value: Some(got) });

        m.add_function(f);

        let mut vm = VM::new();
        let out = vm.execute_module(&m).expect("vm exec");
        assert_eq!(out.to_string_box().value, "v");
    }

    #[test]
    fn mapbox_keys_values_return_arrays() {
        // Direct Box-level test (not via VM): keys()/values() should return ArrayBox
        use crate::box_trait::{IntegerBox, StringBox};
        use crate::boxes::map_box::MapBox;

        let map = MapBox::new();
        map.set(Box::new(StringBox::new("a")), Box::new(IntegerBox::new(1)));
        map.set(Box::new(StringBox::new("b")), Box::new(IntegerBox::new(2)));

        let keys = map.keys();
        let values = map.values();
        assert_eq!(keys.type_name(), "ArrayBox");
        assert_eq!(values.type_name(), "ArrayBox");
    }
}
