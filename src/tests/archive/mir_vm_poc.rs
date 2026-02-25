//! PoC tests for MIR unified ops and VM execution

#[cfg(test)]
mod tests {
    use crate::backend::VM;
    use crate::mir::{BasicBlockId, ConstValue, Effect, EffectMask, MirInstruction, MirType};
    use crate::mir::{FunctionSignature, MirFunction, MirModule};

    fn make_main() -> MirFunction {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        MirFunction::new(sig, BasicBlockId::new(0))
    }

    // Legacy VM / typeop PoC（現行の VM 実装とは前提がズレるためアーカイブ扱い）.
    #[test]
    #[ignore]
    fn vm_exec_typeop_check_and_cast() {
        let mut func = make_main();
        let bb = func.entry_block;

        let v0 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: v0,
                value: ConstValue::Integer(42),
            });

        let v1 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::TypeOp {
                dst: v1,
                op: crate::mir::TypeOpKind::Check,
                value: v0,
                ty: MirType::Integer,
            });

        // console.log(result) via ExternCall
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(crate::mir::ssot::extern_call::extern_call(
                None,
                "env.console".to_string(),
                "log".to_string(),
                vec![v1],
                EffectMask::IO,
            ));

        // Cast (no-op for PoC semantics)
        let v2 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::TypeOp {
                dst: v2,
                op: crate::mir::TypeOpKind::Cast,
                value: v0,
                ty: MirType::Integer,
            });

        // Return void
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return { value: None });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let mut vm = VM::new();
        let _ = vm
            .execute_module(&module)
            .expect("VM should execute module");
    }

    #[test]
    #[ignore]
    fn vm_exec_typeop_cast_int_float() {
        let mut func = make_main();
        let bb = func.entry_block;

        let v0 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: v0,
                value: ConstValue::Integer(3),
            });

        let v1 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::TypeOp {
                dst: v1,
                op: crate::mir::TypeOpKind::Cast,
                value: v0,
                ty: MirType::Float,
            });
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return { value: None });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);
        let mut vm = VM::new();
        let _ = vm
            .execute_module(&module)
            .expect("int->float cast should succeed");
    }

    #[test]
    #[ignore]
    fn vm_exec_typeop_cast_float_int() {
        let mut func = make_main();
        let bb = func.entry_block;

        let v0 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: v0,
                value: ConstValue::Float(3.7),
            });

        let v1 = func.next_value_id();
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::TypeOp {
                dst: v1,
                op: crate::mir::TypeOpKind::Cast,
                value: v0,
                ty: MirType::Integer,
            });
        func.get_block_mut(bb)
            .unwrap()
            .add_instruction(MirInstruction::Return { value: None });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);
        let mut vm = VM::new();
        let _ = vm
            .execute_module(&module)
            .expect("float->int cast should succeed");
    }
}
