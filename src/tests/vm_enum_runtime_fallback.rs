#[cfg(test)]
mod tests {
    use crate::backend::VM;
    use crate::mir::definitions::call_unified::TypeCertainty;
    use crate::mir::{
        BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType,
    };

    fn ensure_ring0_initialized() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
    }

    #[test]
    fn vm_exec_enum_match_through_sum_runtime_fallback() {
        ensure_ring0_initialized();
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId::new(0));
        let bb = func.entry_block;

        let payload = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: payload,
                value: ConstValue::Integer(41),
            });

        let sum_value = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantMake {
                dst: sum_value,
                enum_name: "Option".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload: Some(payload),
                payload_type: Some(MirType::Integer),
            });

        let projected = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantProject {
                dst: projected,
                value: sum_value,
                enum_name: "Option".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload_type: Some(MirType::Integer),
            });

        let plus_one = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: plus_one,
                value: ConstValue::Integer(1),
            });

        let result = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::BinOp {
                dst: result,
                op: BinaryOp::Add,
                lhs: projected,
                rhs: plus_one,
            });
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return {
                value: Some(result),
            });

        let mut module = MirModule::new("vm_sum_runtime_fallback_ok".to_string());
        module.metadata.enum_decls.insert(
            "Option".to_string(),
            crate::mir::MirEnumDecl {
                type_parameters: vec!["T".to_string()],
                variants: vec![
                    crate::mir::MirEnumVariantDecl {
                        name: "None".to_string(),
                        payload_type_name: None,
                    },
                    crate::mir::MirEnumVariantDecl {
                        name: "Some".to_string(),
                        payload_type_name: Some("T".to_string()),
                    },
                ],
            },
        );
        module.add_function(func);

        let mut vm = VM::new();
        let out = vm.execute_module(&module).expect("vm exec");
        assert_eq!(out.to_string_box().value, "42");
    }

    #[test]
    fn vm_variant_project_fail_fast_on_tag_mismatch() {
        ensure_ring0_initialized();
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId::new(0));
        let bb = func.entry_block;

        let payload = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: payload,
                value: ConstValue::Integer(7),
            });

        let sum_value = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantMake {
                dst: sum_value,
                enum_name: "Option".to_string(),
                variant: "None".to_string(),
                tag: 0,
                payload: None,
                payload_type: None,
            });

        let projected = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantProject {
                dst: projected,
                value: sum_value,
                enum_name: "Option".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload_type: Some(MirType::Integer),
            });
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return {
                value: Some(projected),
            });

        let mut module = MirModule::new("vm_sum_runtime_fallback".to_string());
        module.metadata.enum_decls.insert(
            "Option".to_string(),
            crate::mir::MirEnumDecl {
                type_parameters: vec!["T".to_string()],
                variants: vec![
                    crate::mir::MirEnumVariantDecl {
                        name: "None".to_string(),
                        payload_type_name: None,
                    },
                    crate::mir::MirEnumVariantDecl {
                        name: "Some".to_string(),
                        payload_type_name: Some("T".to_string()),
                    },
                ],
            },
        );
        module.add_function(func);

        let mut vm = VM::new();
        let error = vm
            .execute_module(&module)
            .expect_err("tag mismatch should fail fast");
        let message = error.to_string();
        assert!(message.contains("[vm/sum:project]"), "{}", message);
        assert!(message.contains("tag mismatch"), "{}", message);
    }

    #[test]
    fn vm_exec_variant_project_returns_box_payload_via_object_storage() {
        ensure_ring0_initialized();
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId::new(0));
        let bb = func.entry_block;

        let person = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::NewBox {
                dst: person,
                box_type: "Person".to_string(),
                args: vec![],
            });

        let key_name = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: key_name,
                value: ConstValue::String("name".to_string()),
            });

        let value_name = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value_name,
                value: ConstValue::String("Alice".to_string()),
            });

        func.get_block_mut(bb).unwrap().add_instruction(
            crate::mir::ssot::method_call::runtime_method_call(
                None,
                person,
                "InstanceBox",
                "setField",
                vec![key_name, value_name],
                EffectMask::PURE,
                TypeCertainty::Known,
            ),
        );

        let sum_value = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantMake {
                dst: sum_value,
                enum_name: "OptionPerson".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload: Some(person),
                payload_type: Some(MirType::Box("Person".to_string())),
            });

        let projected = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::VariantProject {
                dst: projected,
                value: sum_value,
                enum_name: "OptionPerson".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload_type: Some(MirType::Box("Person".to_string())),
            });

        let result = func.next_value_id();
        func.get_block_mut(bb).unwrap().add_instruction(
            crate::mir::ssot::method_call::runtime_method_call(
                Some(result),
                projected,
                "InstanceBox",
                "getField",
                vec![key_name],
                EffectMask::PURE,
                TypeCertainty::Known,
            ),
        );
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return {
                value: Some(result),
            });

        let mut module = MirModule::new("vm_sum_runtime_box_payload".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Person".to_string(), vec!["name".to_string()]);
        module.metadata.enum_decls.insert(
            "OptionPerson".to_string(),
            crate::mir::MirEnumDecl {
                type_parameters: vec![],
                variants: vec![
                    crate::mir::MirEnumVariantDecl {
                        name: "None".to_string(),
                        payload_type_name: None,
                    },
                    crate::mir::MirEnumVariantDecl {
                        name: "Some".to_string(),
                        payload_type_name: Some("Person".to_string()),
                    },
                ],
            },
        );
        module.add_function(func);

        let mut vm = VM::new();
        let out = vm.execute_module(&module).expect("vm exec");
        assert_eq!(out.to_string_box().value, "Alice");
    }
}
