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
    NativeShapeTable,
    BridgeRustBackend,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmNativeShapeEmit {
    pub bytes: Vec<u8>,
    pub shape_id: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmHakoDefaultLaneTrace {
    pub plan: WasmHakoDefaultLanePlan,
    pub shape_id: Option<&'static str>,
}

/// Compile strict native shape-table subset directly to wasm bytes for default hako-lane.
///
/// Returns:
/// - `Ok(Some(bytes))` when supported shape matched and native binary writer emitted.
/// - `Ok(None)` when shape is outside current native contract.
pub fn compile_hako_native_shape_emit(
    mir_module: &MirModule,
) -> Result<Option<WasmNativeShapeEmit>, WasmError> {
    let Some(found) = shape_table::match_native_shape(mir_module) else {
        return Ok(None);
    };
    let bytes = binary_writer::build_minimal_main_i32_const_module(found.value)?;
    Ok(Some(WasmNativeShapeEmit {
        bytes,
        shape_id: found.shape.id(),
    }))
}

pub fn compile_hako_native_shape_bytes(mir_module: &MirModule) -> Result<Option<Vec<u8>>, WasmError> {
    Ok(compile_hako_native_shape_emit(mir_module)?.map(|emitted| emitted.bytes))
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
    /// - `NativeShapeTable`: emit wasm bytes directly for shape-table-matched subset.
    /// - `BridgeRustBackend`: delegate to Rust backend compile pipeline.
    pub fn plan_hako_default_lane(&self, mir_module: &MirModule) -> WasmHakoDefaultLanePlan {
        self.plan_hako_default_lane_trace(mir_module).plan
    }

    /// Decide plan with trace payload for default hako-lane.
    pub fn plan_hako_default_lane_trace(&self, mir_module: &MirModule) -> WasmHakoDefaultLaneTrace {
        if let Some(found) = shape_table::match_native_shape(mir_module) {
            WasmHakoDefaultLaneTrace {
                plan: WasmHakoDefaultLanePlan::NativeShapeTable,
                shape_id: Some(found.shape.id()),
            }
        } else {
            // WSM-P10-min2 analysis-only inventory hook.
            // Keep default route bridge-only while making matcher observable in codepath.
            let _p10_candidate_id = shape_table::detect_p10_loop_extern_call_candidate(mir_module);
            WasmHakoDefaultLaneTrace {
                plan: WasmHakoDefaultLanePlan::BridgeRustBackend,
                shape_id: None,
            }
        }
    }

    /// Compile with explicit default hako-lane planning.
    ///
    /// Current state:
    /// - native path is available for shape-table-matched subset.
    /// - non-matching shapes are bridged to Rust backend path.
    pub fn compile_hako_default_lane(
        &mut self,
        mir_module: MirModule,
    ) -> Result<(Vec<u8>, WasmHakoDefaultLanePlan), WasmError> {
        let (plan, bytes) = match compile_hako_native_shape_emit(&mir_module)? {
            Some(emitted) => (WasmHakoDefaultLanePlan::NativeShapeTable, emitted.bytes),
            None => (
                WasmHakoDefaultLanePlan::BridgeRustBackend,
                self.compile_module(mir_module)?,
            ),
        };
        Ok((bytes, plan))
    }

    /// Compile MIR module to WASM bytes
    pub fn compile_module(&mut self, mir_module: MirModule) -> Result<Vec<u8>, WasmError> {
        // WSM-P5-min6 native shape table:
        // For the native subset (main returns integer const family),
        // bypass WAT and emit wasm binary directly.
        if let Some(found) = shape_table::match_native_shape(&mir_module) {
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

    /// Contract helper for WSM-P10-min3.
    /// Emits loop/branch/call writer skeleton without changing default route.
    pub fn build_loop_extern_call_skeleton_wasm(
        &self,
        iterations: i32,
    ) -> Result<Vec<u8>, WasmError> {
        binary_writer::build_loop_extern_call_skeleton_module(iterations)
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
    fn wasm_binary_writer_loop_extern_skeleton_contract() {
        let backend = WasmBackend::new();
        let wasm = backend
            .build_loop_extern_call_skeleton_wasm(3)
            .expect("loop extern skeleton helper must succeed");
        assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
        assert!(wasm.windows(4).any(|w| w == b"main"));
    }

    #[test]
    fn wasm_binary_writer_extract_min_const_return_contract() {
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

        let found = shape_table::match_native_shape(&module).expect("native shape should match");
        assert_eq!(found.value, 7);
        assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
    }

    #[test]
    fn wasm_hako_default_lane_plan_native_for_shape_table_contract() {
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
        assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
    }

    #[test]
    fn wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract() {
        let module = MirModule::new("test".to_string());
        let backend = WasmBackend::new();
        let plan = backend.plan_hako_default_lane(&module);
        assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
    }

    #[test]
    fn wasm_hako_default_lane_trace_includes_shape_id_for_native_contract() {
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
        let trace = backend.plan_hako_default_lane_trace(&module);
        assert_eq!(trace.plan, WasmHakoDefaultLanePlan::NativeShapeTable);
        assert_eq!(trace.shape_id, Some("wsm.p4.main_return_i32_const.v0"));
    }

    #[test]
    fn wasm_hako_default_lane_trace_has_none_shape_id_for_bridge_contract() {
        let module = MirModule::new("test".to_string());
        let backend = WasmBackend::new();
        let trace = backend.plan_hako_default_lane_trace(&module);
        assert_eq!(trace.plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
        assert_eq!(trace.shape_id, None);
    }

    #[test]
    fn wasm_hako_native_shape_bytes_emits_for_pilot_shape_contract() {
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

        let bytes = compile_hako_native_shape_bytes(&module)
            .expect("native helper should succeed")
            .expect("pilot shape must emit bytes");
        assert!(bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }

    #[test]
    fn wasm_hako_native_shape_bytes_rejects_non_pilot_contract() {
        let module = MirModule::new("test".to_string());
        let bytes = compile_hako_native_shape_bytes(&module)
            .expect("native helper should return Ok(None) for non-pilot");
        assert!(bytes.is_none());
    }

    #[test]
    fn wasm_hako_native_shape_bytes_emits_for_const_copy_return_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        let const_dst = ValueId::new(1);
        let copy_dst = ValueId::new(2);
        block.add_instruction(MirInstruction::Const {
            dst: const_dst,
            value: ConstValue::Integer(8),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: copy_dst,
            src: const_dst,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(copy_dst),
        });

        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let bytes = compile_hako_native_shape_bytes(&module)
            .expect("native helper should succeed")
            .expect("const-copy-return shape must emit bytes");
        assert!(bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }

    #[test]
    fn wasm_hako_native_shape_emit_reports_shape_id_for_const_copy_return_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        let const_dst = ValueId::new(1);
        let copy_dst = ValueId::new(2);
        block.add_instruction(MirInstruction::Const {
            dst: const_dst,
            value: ConstValue::Integer(8),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: copy_dst,
            src: const_dst,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(copy_dst),
        });
        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let emitted = compile_hako_native_shape_emit(&module)
            .expect("native shape emit should succeed")
            .expect("shape should match");
        assert_eq!(emitted.shape_id, "wsm.p5.main_return_i32_const_via_copy.v0");
        assert!(emitted.bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }

    #[test]
    fn wasm_hako_native_shape_emit_reports_shape_id_for_const_binop_return_contract() {
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let block = func.get_block_mut(entry).expect("entry block");
        let lhs = ValueId::new(1);
        let rhs = ValueId::new(2);
        let out = ValueId::new(3);
        block.add_instruction(MirInstruction::Const {
            dst: lhs,
            value: ConstValue::Integer(40),
        });
        block.add_instruction(MirInstruction::Const {
            dst: rhs,
            value: ConstValue::Integer(2),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: out,
            op: crate::mir::BinaryOp::Add,
            lhs,
            rhs,
        });
        block.add_instruction(MirInstruction::Return { value: Some(out) });
        let mut module = MirModule::new("test".to_string());
        module.add_function(func);

        let emitted = compile_hako_native_shape_emit(&module)
            .expect("native shape emit should succeed")
            .expect("shape should match");
        assert_eq!(emitted.shape_id, "wsm.p9.main_return_i32_const_binop.v0");
        assert!(emitted.bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }
}
