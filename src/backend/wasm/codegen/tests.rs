#[cfg(test)]
mod tests {
    use super::super::{wasm_result_type, WasmCodegen, WasmModule};
    use crate::backend::wasm::WasmError;
    use crate::mir::{Callee, ConstValue, EffectMask, MirInstruction, MirType, ValueId};

    #[test]
    fn test_wasm_module_wat_generation() {
        let mut module = WasmModule::new();
        module.memory = "(memory (export \"memory\") 1)".to_string();
        module
            .imports
            .push("(import \"env\" \"print\" (func $print (param i32)))".to_string());

        let wat = module.to_wat();
        assert!(wat.contains("(module"));
        assert!(wat.contains("memory"));
        assert!(wat.contains("import"));
    }

    #[test]
    fn test_constant_generation() {
        let mut codegen = WasmCodegen::new();
        let dst = ValueId::new(0);

        // This requires current_locals to be populated, which normally happens in generate_function.
        // We can't easily unit test generate_const in isolation without mocking the internal state.
        // For now, we expect it to fail or we need to set up the state manually if we expose it.
        // Since get_local_index checks current_locals, it will return Err.

        let result = codegen.generate_instruction(&crate::mir::MirInstruction::Const {
            dst,
            value: ConstValue::Integer(42),
        });

        assert!(result.is_err()); // Should fail without local mapping
    }

    #[test]
    fn test_null_constant_generation_uses_zero_handle() {
        let mut codegen = WasmCodegen::new();
        let dst = ValueId::new(0);
        codegen.current_locals.insert(dst, 0);
        codegen.next_local_index = 1;

        let result = codegen
            .generate_instruction(&crate::mir::MirInstruction::Const {
                dst,
                value: ConstValue::Null,
            })
            .expect("null const should lower");

        assert_eq!(result, vec!["i32.const 0".to_string(), "local.set $0".to_string()]);
    }

    #[test]
    fn test_unsupported_extern_call_fails_fast_with_supported_list() {
        let mut codegen = WasmCodegen::new();
        let result = codegen.generate_instruction(&MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.console.trace".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });

        match result {
            Err(WasmError::UnsupportedInstruction(msg)) => {
                assert!(msg.contains("Unsupported extern call: env.console.trace"));
                assert!(msg.contains("supported:"));
                assert!(msg.contains("env.console.log"));
                assert!(msg.contains("env.console.debug"));
            }
            other => panic!("expected unsupported extern error, got: {:?}", other),
        }
    }

    #[test]
    fn test_unsupported_boxcall_method_fails_fast_with_supported_list() {
        let mut codegen = WasmCodegen::new();
        let result = codegen.generate_box_call(None, ValueId::new(0), "trace", &[]);

        match result {
            Err(WasmError::UnsupportedInstruction(msg)) => {
                assert!(msg.contains("Unsupported BoxCall method: trace"));
                assert!(msg.contains("supported:"));
                assert!(msg.contains("log"));
                assert!(msg.contains("info"));
                assert!(msg.contains("debug"));
                assert!(msg.contains("warn"));
                assert!(msg.contains("error"));
            }
            other => panic!("expected unsupported boxcall error, got: {:?}", other),
        }
    }

    #[test]
    fn test_wasm_result_type_accepts_handle_like_returns() {
        for ty in [
            MirType::Integer,
            MirType::Bool,
            MirType::String,
            MirType::Box("StringBox".to_string()),
            MirType::Array(Box::new(MirType::Integer)),
            MirType::Future(Box::new(MirType::Box("StringBox".to_string()))),
            MirType::WeakRef,
        ] {
            assert_eq!(wasm_result_type(&ty).unwrap(), Some("i32"));
        }
        assert_eq!(wasm_result_type(&MirType::Void).unwrap(), None);
    }

    #[test]
    fn test_wasm_result_type_rejects_float() {
        match wasm_result_type(&MirType::Float) {
            Err(WasmError::UnsupportedInstruction(msg)) => {
                assert!(msg.contains("Unsupported return type"));
                assert!(msg.contains("Float"));
            }
            other => panic!("expected unsupported float result type, got: {:?}", other),
        }
    }
}
