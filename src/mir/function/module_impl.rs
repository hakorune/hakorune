use super::{
    ClosureBodyId, FunctionPublicationErrorV1, MirFunction, MirModule, ModuleMetadata, ModuleStats,
};
use crate::mir::ConstValue;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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

    /// Publish a canonical function without replacing an existing draft.
    pub fn try_add_function(
        &mut self,
        function: MirFunction,
    ) -> Result<(), FunctionPublicationErrorV1> {
        let name = function.signature.name.clone();
        if self.functions.contains_key(&name) {
            return Err(FunctionPublicationErrorV1 {
                function_name: name,
            });
        }
        self.functions.insert(name, function);
        Ok(())
    }

    /// Publish a closed draft batch only after every function name is proven
    /// unique against both the module and the batch itself.
    pub(in crate::mir) fn try_add_functions_atomic(
        &mut self,
        functions: Vec<MirFunction>,
    ) -> Result<(), FunctionPublicationErrorV1> {
        self.preflight_add_function_symbols(
            functions
                .iter()
                .map(|function| function.signature.name.as_str()),
        )?;
        for function in functions {
            let name = function.signature.name.clone();
            self.functions.insert(name, function);
        }
        Ok(())
    }

    /// Check an exact function-symbol batch without taking ownership of its
    /// drafts, so a prepared collector can retain them until infallible commit.
    pub(in crate::mir) fn preflight_add_function_symbols<'symbol>(
        &self,
        symbols: impl IntoIterator<Item = &'symbol str>,
    ) -> Result<(), FunctionPublicationErrorV1> {
        let mut names = BTreeSet::new();
        for symbol in symbols {
            if self.functions.contains_key(symbol) || !names.insert(symbol) {
                return Err(FunctionPublicationErrorV1 {
                    function_name: symbol.to_owned(),
                });
            }
        }
        Ok(())
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

    /// Read the exact id that a subsequent closure-body publication would use.
    /// This is intentionally non-mutating so a caller can emit its matching
    /// `NewClosure` instruction before publishing the body metadata.
    pub(in crate::mir) fn reserve_next_closure_body_id(&self) -> ClosureBodyId {
        self.metadata.next_closure_body_id
    }

    /// Publish a body for a previously observed next id.
    ///
    /// The caller must arrange that no fallible work remains between its
    /// reservation and this terminal. A mismatch is an internal lifecycle
    /// violation, not a recoverable source error.
    pub(in crate::mir) fn commit_reserved_closure_body(
        &mut self,
        expected: ClosureBodyId,
        body: Vec<crate::ast::ASTNode>,
    ) {
        assert!(
            !body.is_empty(),
            "[freeze:contract][mir/closure_body_empty_commit]"
        );
        assert_eq!(
            self.metadata.next_closure_body_id, expected,
            "[freeze:contract][mir/closure_body_reservation_drift]"
        );
        let published = self.intern_closure_body(body);
        assert_eq!(
            published, expected,
            "[freeze:contract][mir/closure_body_commit_drift]"
        );
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
