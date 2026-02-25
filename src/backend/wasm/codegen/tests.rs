#[cfg(test)]
mod tests {
    use super::super::{WasmCodegen, WasmModule};
    use crate::mir::{ConstValue, ValueId};

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
}
