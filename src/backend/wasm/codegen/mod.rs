/*!
 * WASM Code Generation - Core MIR to WASM instruction conversion
 *
 * Phase 8.2 PoC1: Basic operations (arithmetic, control flow, print)
 * Phase 8.3 PoC2: Reference operations (RefNew/RefGet/RefSet)
 */

use super::{MemoryManager, RuntimeImports, WasmError};
use crate::mir::{
    BasicBlockId,
    MirFunction,
    MirType,
    MirModule,
    ValueId,
};
use std::collections::HashMap;

mod instructions;
mod builtins;
#[cfg(test)]
mod tests;

/// WASM module representation for WAT generation
pub struct WasmModule {
    pub imports: Vec<String>,
    pub memory: String,
    pub data_segments: Vec<String>,
    pub globals: Vec<String>,
    pub functions: Vec<String>,
    pub exports: Vec<String>,
}

impl WasmModule {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            memory: String::new(),
            data_segments: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
            exports: Vec::new(),
        }
    }

    /// Generate WAT text format
    pub fn to_wat(&self) -> String {
        let mut wat = String::new();
        wat.push_str("(module\n");

        // Add imports first (must come before other definitions in WASM)
        for import in &self.imports {
            wat.push_str(&format!("  {}\n", import));
        }

        // Add memory declaration
        if !self.memory.is_empty() {
            wat.push_str(&format!("  {}\n", self.memory));
        }

        // Add data segments (must come after memory)
        for data_segment in &self.data_segments {
            wat.push_str(&format!("  {}\n", data_segment));
        }

        // Add globals
        for global in &self.globals {
            wat.push_str(&format!("  {}\n", global));
        }

        // Add functions
        for function in &self.functions {
            wat.push_str(&format!("  {}\n", function));
        }

        // Add exports
        for export in &self.exports {
            wat.push_str(&format!("  {}\n", export));
        }

        wat.push_str(")\n");
        wat
    }
}

/// WASM code generator
pub struct WasmCodegen {
    /// Current function context for local variable management
    current_locals: HashMap<ValueId, u32>,
    next_local_index: u32,
    /// String literals and their data segment offsets
    string_literals: HashMap<String, u32>,
    next_data_offset: u32,
    /// Function signature lookup for global call lowering.
    function_param_counts: HashMap<String, usize>,
    function_return_types: HashMap<String, MirType>,
}

impl WasmCodegen {
    pub fn new() -> Self {
        Self {
            current_locals: HashMap::new(),
            next_local_index: 0,
            string_literals: HashMap::new(),
            next_data_offset: 0x1000, // Start data after initial heap space
            function_param_counts: HashMap::new(),
            function_return_types: HashMap::new(),
        }
    }

    /// Generate WASM module from MIR module
    pub fn generate_module(
        &mut self,
        mir_module: MirModule,
        memory_manager: &MemoryManager,
        runtime: &RuntimeImports,
    ) -> Result<WasmModule, WasmError> {
        let mut wasm_module = WasmModule::new();
        self.function_param_counts.clear();
        self.function_return_types.clear();
        for (name, function) in &mir_module.functions {
            self.function_param_counts
                .insert(name.clone(), function.params.len());
            self.function_return_types
                .insert(name.clone(), function.signature.return_type.clone());
        }

        // Add memory declaration (64KB initial)
        wasm_module.memory = "(memory (export \"memory\") 1)".to_string();

        // Add runtime imports (env.print for debugging)
        wasm_module.imports.extend(runtime.get_imports());

        // Add globals (heap pointer)
        wasm_module.globals.extend(memory_manager.get_globals());

        // Add memory management functions
        wasm_module
            .functions
            .push(memory_manager.get_malloc_function());
        wasm_module
            .functions
            .push(memory_manager.get_generic_box_alloc_function());

        // Add Box-specific allocation functions for known types
        for box_type in ["StringBox", "IntegerBox", "BoolBox", "DataBox"] {
            if let Ok(alloc_func) = memory_manager.get_box_alloc_function(box_type) {
                wasm_module.functions.push(alloc_func);
            }
        }

        // Generate functions in deterministic order to keep WAT output stable.
        let mut function_names: Vec<String> = mir_module.functions.keys().cloned().collect();
        function_names.sort();
        for name in function_names {
            let function = mir_module.functions.get(&name).ok_or_else(|| {
                WasmError::CodegenError(format!("Function not found during codegen: {}", name))
            })?;
            let wasm_function = self.generate_function(&name, function.clone())?;
            wasm_module.functions.push(wasm_function);
        }

        // Add string literal data segments
        wasm_module
            .data_segments
            .extend(self.generate_data_segments());

        // Add main function export if it exists
        if mir_module.functions.contains_key("main") {
            wasm_module
                .exports
                .push("(export \"main\" (func $main))".to_string());
        }

        Ok(wasm_module)
    }

    /// Generate WASM function from MIR function
    fn generate_function(
        &mut self,
        name: &str,
        mir_function: MirFunction,
    ) -> Result<String, WasmError> {
        // Reset local variable tracking for this function
        self.current_locals.clear();
        self.next_local_index = 0;

        let mut function_body = String::new();
        function_body.push_str(&format!("(func ${} ", name));

        // Parameters are mapped to i32 handles in current WASM backend.
        for pid in &mir_function.params {
            function_body.push_str(&format!(" (param ${} i32)", pid.as_u32()));
        }

        // Add return type if not void
        match mir_function.signature.return_type {
            crate::mir::MirType::Integer => function_body.push_str(" (result i32)"),
            crate::mir::MirType::Bool => function_body.push_str(" (result i32)"),
            crate::mir::MirType::Void => {} // No return type
            _ => {
                return Err(WasmError::UnsupportedInstruction(format!(
                    "Unsupported return type: {:?}",
                    mir_function.signature.return_type
                )))
            }
        }

        // Collect all local variables needed
        let local_count = self.count_locals(&mir_function)?;
        if local_count > 0 {
            // Declare individual local variables for each ValueId
            for i in mir_function.params.len() as u32..local_count {
                function_body.push_str(&format!(" (local ${} i32)", i));
            }
        }

        function_body.push('\n');

        // Generate body from entry block
        let entry_instructions =
            self.generate_basic_block(&mir_function, mir_function.entry_block)?;
        for instruction in entry_instructions {
            function_body.push_str(&format!("    {}\n", instruction));
        }

        function_body.push_str("  )");
        Ok(function_body)
    }

    /// Count local variables needed for the function
    fn count_locals(&mut self, mir_function: &MirFunction) -> Result<u32, WasmError> {
        let mut value_ids: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
        self.current_locals.clear();
        self.next_local_index = 0;

        // Reserve parameter slots first to match function signature order.
        for pid in &mir_function.params {
            self.current_locals.insert(*pid, self.next_local_index);
            self.next_local_index += 1;
        }

        for block in mir_function.blocks.values() {
            for instruction in &block.instructions {
                if let Some(value_id) = instruction.dst_value() {
                    if value_id != ValueId::INVALID {
                        value_ids.insert(value_id.as_u32());
                    }
                }
                for used_value in instruction.used_values() {
                    if used_value != ValueId::INVALID {
                        value_ids.insert(used_value.as_u32());
                    }
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(value_id) = terminator.dst_value() {
                    if value_id != ValueId::INVALID {
                        value_ids.insert(value_id.as_u32());
                    }
                }
                for used_value in terminator.used_values() {
                    if used_value != ValueId::INVALID {
                        value_ids.insert(used_value.as_u32());
                    }
                }
            }
        }

        // Assign local indices to non-parameter ValueIds in stable order.
        for raw in value_ids {
            let value_id = ValueId::new(raw);
            if self.current_locals.contains_key(&value_id) {
                continue;
            }
            self.current_locals.insert(value_id, self.next_local_index);
            self.next_local_index += 1;
        }

        Ok(self.next_local_index)
    }

    pub(crate) fn get_function_param_count(&self, name: &str) -> Option<usize> {
        self.function_param_counts.get(name).copied()
    }

    pub(crate) fn function_has_return_value(&self, name: &str) -> Result<bool, WasmError> {
        let ty = self.function_return_types.get(name).ok_or_else(|| {
            WasmError::UnsupportedInstruction(format!(
                "Unknown global callee: {}",
                name
            ))
        })?;
        match ty {
            MirType::Integer | MirType::Bool => Ok(true),
            MirType::Void => Ok(false),
            other => Err(WasmError::UnsupportedInstruction(format!(
                "Unsupported global return type for {}: {:?}",
                name, other
            ))),
        }
    }

    pub(crate) fn supported_global_calls_csv(&self) -> String {
        let mut names: Vec<&str> = self.function_param_counts.keys().map(String::as_str).collect();
        names.sort_unstable();
        names.join(", ")
    }

    /// Generate WASM instructions for a basic block
    fn generate_basic_block(
        &mut self,
        mir_function: &MirFunction,
        block_id: BasicBlockId,
    ) -> Result<Vec<String>, WasmError> {
        let block = mir_function.blocks.get(&block_id).ok_or_else(|| {
            WasmError::CodegenError(format!("Basic block {:?} not found", block_id))
        })?;

        let mut instructions = Vec::new();

        // Process regular instructions
        for mir_instruction in &block.instructions {
            let wasm_instructions = self.generate_instruction(mir_instruction)?;
            instructions.extend(wasm_instructions);
        }

        // Process terminator instruction
        if let Some(ref terminator) = block.terminator {
            let wasm_instructions = self.generate_instruction(terminator)?;
            instructions.extend(wasm_instructions);
        }

        Ok(instructions)
    }

    /// Register a string literal and return its data offset
    fn register_string_literal(&mut self, string: &str) -> u32 {
        if let Some(&offset) = self.string_literals.get(string) {
            return offset;
        }

        let offset = self.next_data_offset;
        let string_bytes = string.as_bytes();
        self.string_literals.insert(string.to_string(), offset);
        self.next_data_offset += string_bytes.len() as u32;

        offset
    }

    /// Generate data segments for all registered string literals
    fn generate_data_segments(&self) -> Vec<String> {
        let mut segments = Vec::new();
        let mut ordered: Vec<(&String, &u32)> = self.string_literals.iter().collect();
        ordered.sort_by(|(sa, oa), (sb, ob)| oa.cmp(ob).then_with(|| sa.cmp(sb)));

        for (string, &offset) in ordered {
            let string_bytes = string.as_bytes();

            // Convert to hex-escaped string for WAT
            let byte_string = string_bytes
                .iter()
                .map(|b| format!("\\{:02x}", b))
                .collect::<String>();

            let data_segment = format!("(data (i32.const {}) \"{}\")", offset, byte_string);

            segments.push(data_segment);
        }

        segments
    }

    /// Get WASM local variable index for ValueId
    fn get_local_index(&self, value_id: ValueId) -> Result<u32, WasmError> {
        self.current_locals.get(&value_id).copied().ok_or_else(|| {
            WasmError::CodegenError(format!(
                "Local variable not found for ValueId: {:?}",
                value_id
            ))
        })
    }
}
