#[cfg(test)]
mod tests {
    use crate::backend::VM;
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
            .add_instruction(MirInstruction::SumMake {
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
            .add_instruction(MirInstruction::SumProject {
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
    fn vm_sum_project_fail_fast_on_tag_mismatch() {
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
            .add_instruction(MirInstruction::SumMake {
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
            .add_instruction(MirInstruction::SumProject {
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
}
