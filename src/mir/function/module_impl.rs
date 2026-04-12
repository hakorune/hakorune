use super::{ClosureBodyId, MirFunction, MirModule, ModuleMetadata, ModuleStats};
use crate::mir::ConstValue;
use std::collections::{BTreeMap, HashMap};

impl MirModule {
    /// Create a new MIR module
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: BTreeMap::new(),
            globals: HashMap::new(),
            metadata: ModuleMetadata::default(),
        }
    }

    /// Add a function to the module
    pub fn add_function(&mut self, function: MirFunction) {
        let name = function.signature.name.clone();
        self.functions.insert(name, function);
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&MirFunction> {
        self.functions.get(name)
    }

    /// Get a mutable function by name
    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut MirFunction> {
        self.functions.get_mut(name)
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }

    /// Add a global constant
    pub fn add_global(&mut self, name: String, value: ConstValue) {
        self.globals.insert(name, value);
    }

    /// NCL-1: Store closure body into module metadata and return stable id.
    pub fn intern_closure_body(&mut self, body: Vec<crate::ast::ASTNode>) -> ClosureBodyId {
        let id = self.metadata.next_closure_body_id;
        self.metadata.next_closure_body_id = self.metadata.next_closure_body_id.saturating_add(1);
        self.metadata.closure_bodies.insert(id, body);
        id
    }

    /// NCL-1: Read externalized closure body by id.
    pub fn closure_body(&self, id: ClosureBodyId) -> Option<&[crate::ast::ASTNode]> {
        self.metadata
            .closure_bodies
            .get(&id)
            .map(|body| body.as_slice())
    }

    /// Verify entire module
    pub fn verify(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (name, function) in &self.functions {
            if let Err(e) = function.verify() {
                errors.push(format!("Function '{}': {}", name, e));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get module statistics
    pub fn stats(&self) -> ModuleStats {
        let function_stats: Vec<_> = self.functions.values().map(|f| f.stats()).collect();

        ModuleStats {
            function_count: self.functions.len(),
            global_count: self.globals.len(),
            total_blocks: function_stats.iter().map(|s| s.block_count).sum(),
            total_instructions: function_stats.iter().map(|s| s.instruction_count).sum(),
            total_values: function_stats.iter().map(|s| s.value_count).sum(),
            pure_functions: function_stats.iter().filter(|s| s.is_pure).count(),
        }
    }
}
