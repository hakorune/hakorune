/*!
 * WASM Backend - Phase 8 Implementation
 *
 * Converts MIR instructions to WebAssembly for sandboxed execution
 * Targets browser execution and wasmtime runtime
 */

mod codegen;
mod binary_writer;
mod extern_contract;
mod memory;
mod runtime;
mod shape_table;
// mod executor; // TODO: Fix WASM executor build errors

pub use codegen::{WasmCodegen, WasmModule};
pub use memory::{BoxLayout, MemoryManager};
pub use runtime::RuntimeImports;
// pub use executor::WasmExecutor; // TODO: Fix WASM executor build errors

use crate::mir::MirModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmHakoDefaultLanePlan {
    NativePilotShape,
    BridgeRustBackend,
}

/// Compile strict pilot shape directly to wasm bytes for default hako-lane.
///
/// Returns:
/// - `Ok(Some(bytes))` when pilot shape matched and native binary writer emitted.
/// - `Ok(None)` when shape is outside pilot contract.
pub fn compile_hako_native_pilot_bytes(mir_module: &MirModule) -> Result<Option<Vec<u8>>, WasmError> {
    let Some(found) = shape_table::match_pilot_shape(mir_module) else {
        return Ok(None);
    };
    let bytes = binary_writer::build_minimal_main_i32_const_module(found.value)?;
    Ok(Some(bytes))
}

/// WASM compilation error
#[derive(Debug)]
pub enum WasmError {
    CodegenError(String),
    MemoryError(String),
    UnsupportedInstruction(String),
    WasmValidationError(String),
    IOError(String),
}

impl std::fmt::Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmError::CodegenError(msg) => write!(f, "Codegen error: {}", msg),
            WasmError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            WasmError::UnsupportedInstruction(msg) => write!(f, "Unsupported instruction: {}", msg),
            WasmError::WasmValidationError(msg) => write!(f, "WASM validation error: {}", msg),
            WasmError::IOError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for WasmError {}

/// Main WASM backend compiler
pub struct WasmBackend {
    codegen: WasmCodegen,
    memory_manager: MemoryManager,
    runtime: RuntimeImports,
}

impl WasmBackend {
    /// Create a new WASM backend
    pub fn new() -> Self {
        Self {
            codegen: WasmCodegen::new(),
            memory_manager: MemoryManager::new(),
            runtime: RuntimeImports::new(),
        }
    }

    /// Decide plan for default hako-lane compilation.
    ///
    /// - `NativePilotShape`: emit wasm bytes directly for strict pilot shape.
    /// - `BridgeRustBackend`: delegate to Rust backend compile pipeline.
    pub fn plan_hako_default_lane(&self, mir_module: &MirModule) -> WasmHakoDefaultLanePlan {
        if shape_table::match_pilot_shape(mir_module).is_some() {
            WasmHakoDefaultLanePlan::NativePilotShape
        } else {
            WasmHakoDefaultLanePlan::BridgeRustBackend
        }
    }

    /// Compile with explicit default hako-lane planning.
    ///
    /// Current state:
    /// - native path is available for pilot shape only.
    /// - non-pilot shapes are bridged to Rust backend path.
    pub fn compile_hako_default_lane(
        &mut self,
        mir_module: MirModule,
    ) -> Result<(Vec<u8>, WasmHakoDefaultLanePlan), WasmError> {
        let (plan, bytes) = match compile_hako_native_pilot_bytes(&mir_module)? {
            Some(bytes) => (WasmHakoDefaultLanePlan::NativePilotShape, bytes),
            None => (
                WasmHakoDefaultLanePlan::BridgeRustBackend,
                self.compile_module(mir_module)?,
            ),
        };
        Ok((bytes, plan))
    }

    /// Compile MIR module to WASM bytes
    pub fn compile_module(&mut self, mir_module: MirModule) -> Result<Vec<u8>, WasmError> {
        // WSM-P4-min4 pilot:
        // For the strict minimal shape (main returns integer const),
        // bypass WAT and emit wasm binary directly.
        if let Some(found) = shape_table::match_pilot_shape(&mir_module) {
            return binary_writer::build_minimal_main_i32_const_module(found.value);
        }

        // Generate WAT (WebAssembly Text) first for debugging
        let wat_text = self.compile_to_wat(mir_module)?;

        // Phase 9.77 Task 1.3: Fix UTF-8 encoding error in WAT→WASM conversion
        self.convert_wat_to_wasm(&wat_text)
    }

    /// Contract helper for WSM-P4-min2.
    /// Emits the minimum valid wasm binary without WAT conversion.
    pub fn build_minimal_i32_const_wasm(&self, value: i32) -> Result<Vec<u8>, WasmError> {
        binary_writer::build_minimal_main_i32_const_module(value)
    }

    /// Convert WAT text to WASM binary with proper UTF-8 handling
    pub fn convert_wat_to_wasm(&self, wat_source: &str) -> Result<Vec<u8>, WasmError> {
        // Debug: Print WAT source for analysis
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("🔍 WAT Source Debug (length: {}):", wat_source.len()));
        ring0.log.debug(&format!("WAT Content:\n{}", wat_source));

        // UTF-8 validation to prevent encoding errors
        if !wat_source.is_ascii() {
            ring0.log.debug("❌ WAT source contains non-ASCII characters");
            return Err(WasmError::WasmValidationError(
                "WAT source contains non-ASCII characters".to_string(),
            ));
        }

        ring0.log.debug("✅ WAT source is ASCII-compatible");

        // Convert to bytes as required by wabt::wat2wasm
        ring0.log.debug("🔄 Converting WAT to WASM bytes...");
        let wasm_bytes = wabt::wat2wasm(wat_source.as_bytes()).map_err(|e| {
            ring0.log.debug(&format!("❌ wabt::wat2wasm failed: {}", e));
            WasmError::WasmValidationError(format!("WAT to WASM conversion failed: {}", e))
        })?;

        ring0.log.debug(&format!(
            "✅ WASM conversion successful, {} bytes generated",
            wasm_bytes.len()
        ));
        Ok(wasm_bytes)
    }

    /// Compile MIR module to WAT text format (for debugging)
    pub fn compile_to_wat(&mut self, mir_module: MirModule) -> Result<String, WasmError> {
        let wasm_module =
            self.codegen
                .generate_module(mir_module, &self.memory_manager, &self.runtime)?;
        Ok(wasm_module.to_wat())
    }

    /// Execute WASM bytes using wasmtime (for testing)
    pub fn execute_wasm(&self, wasm_bytes: &[u8]) -> Result<i32, WasmError> {
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, wasm_bytes).map_err(|e| {
            WasmError::WasmValidationError(format!("Module creation failed: {}", e))
        })?;

        let mut store = wasmtime::Store::new(&engine, ());

        // Create print function import
        let print_func = wasmtime::Func::wrap(&mut store, |value: i32| {
            println!("{}", value);
        });

        // Create print_str function import for string debugging
        let print_str_func = wasmtime::Func::wrap(
            &mut store,
            |mut caller: wasmtime::Caller<'_, ()>,
             ptr: i32,
             len: i32|
             -> Result<(), wasmtime::Error> {
                let memory = caller
                    .get_export("memory")
                    .and_then(|export| export.into_memory())
                    .ok_or_else(|| wasmtime::Error::msg("Memory export not found"))?;

                let data = memory.data(&caller);
                let start = ptr as usize;
                let end = start + len as usize;

                if end <= data.len() {
                    let bytes = &data[start..end];
                    if let Ok(s) = std::str::from_utf8(bytes) {
                        println!("String: {}", s);
                    } else {
                        println!("Invalid UTF-8 bytes: {:?}", bytes);
                    }
                } else {
                    println!(
                        "String out of bounds: ptr={}, len={}, memory_size={}",
                        ptr,
                        len,
                        data.len()
                    );
                }

                Ok(())
            },
        );

        let imports = [print_func.into(), print_str_func.into()];
        let instance = wasmtime::Instance::new(&mut store, &module, &imports).map_err(|e| {
            WasmError::WasmValidationError(format!("Instance creation failed: {}", e))
        })?;

        // Call main function
        let main_func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .map_err(|e| {
                WasmError::WasmValidationError(format!("Main function not found: {}", e))
            })?;

        let result = main_func
            .call(&mut store, ())
            .map_err(|e| WasmError::WasmValidationError(format!("Execution failed: {}", e)))?;

        Ok(result)
    }
}

impl Default for WasmBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, ConstValue, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType, ValueId,
    };

    #[test]
    fn test_backend_creation() {
        let _backend = WasmBackend::new();
        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_empty_module_compilation() {
        let mut backend = WasmBackend::new();
        let module = MirModule::new("test".to_string());

        // Should handle empty module gracefully
        let result = backend.compile_to_wat(module);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wat_to_wasm_ascii_guard_fails_fast() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let backend = WasmBackend::new();
        let err = backend
            .convert_wat_to_wasm("(module (func (export \"main\") (result i32) i32.const 0 ;; あ))")
            .expect_err("non-ascii WAT must fail fast");
        let msg = err.to_string();
        assert!(msg.contains("WAT source contains non-ASCII characters"));
    }

    #[test]
    fn test_wat_to_wasm_invalid_wat_fails_fast() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let backend = WasmBackend::new();
        let err = backend
            .convert_wat_to_wasm("(module (func")
            .expect_err("malformed WAT must fail fast");
        let msg = err.to_string();
        assert!(msg.contains("WAT to WASM conversion failed"));
    }

    #[test]
    fn wasm_binary_writer_minimal_module_contract() {
        let backend = WasmBackend::new();
        let wasm = backend
            .build_minimal_i32_const_wasm(7)
            .expect("binary writer helper must succeed");
        assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }

    #[test]
    fn wasm_binary_writer_pilot_extract_min_const_return_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let found = shape_table::match_pilot_shape(&module).expect("pilot shape should match");
        assert_eq!(found.value, 7);
        assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
    }

    #[test]
    fn wasm_hako_default_lane_plan_native_for_pilot_shape_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let backend = WasmBackend::new();
        let plan = backend.plan_hako_default_lane(&module);
        assert_eq!(plan, WasmHakoDefaultLanePlan::NativePilotShape);
    }

    #[test]
    fn wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract() {
        let module = MirModule::new("test".to_string());
        let backend = WasmBackend::new();
        let plan = backend.plan_hako_default_lane(&module);
        assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
    }

    #[test]
    fn wasm_hako_native_pilot_bytes_emits_for_pilot_shape_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let bytes = compile_hako_native_pilot_bytes(&module)
            .expect("native helper should succeed")
            .expect("pilot shape must emit bytes");
        assert!(bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }

    #[test]
    fn wasm_hako_native_pilot_bytes_rejects_non_pilot_contract() {
        let module = MirModule::new("test".to_string());
        let bytes = compile_hako_native_pilot_bytes(&module)
            .expect("native helper should return Ok(None) for non-pilot");
        assert!(bytes.is_none());
    }
}
