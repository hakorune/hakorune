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
}

impl WasmCodegen {
    pub fn new() -> Self {
        Self {
            current_locals: HashMap::new(),
            next_local_index: 0,
            string_literals: HashMap::new(),
            next_data_offset: 0x1000, // Start data after initial heap space
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

        // Generate functions
        for (name, function) in &mir_module.functions {
            let wasm_function = self.generate_function(name, function.clone())?;
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
            for i in 0..local_count {
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
        let mut max_value_id = 0;

        for (_, block) in &mir_function.blocks {
            for instruction in &block.instructions {
                if let Some(value_id) = instruction.dst_value() {
                    max_value_id = max_value_id.max(value_id.as_u32());
                }
                for used_value in instruction.used_values() {
                    max_value_id = max_value_id.max(used_value.as_u32());
                }
            }
        }

        // Assign local indices to value IDs
        for i in 0..=max_value_id {
            let value_id = ValueId::new(i);
            self.current_locals.insert(value_id, self.next_local_index);
            self.next_local_index += 1;
        }

        Ok(self.next_local_index)
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

        for (string, &offset) in &self.string_literals {
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
