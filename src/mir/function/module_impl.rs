use super::{
    CanonicalCallableDefinitionPublicationErrorV1, FunctionPublicationErrorV1,
    MirFunction, MirModule, ModuleMetadata, ModuleStats,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use std::collections::{BTreeMap, BTreeSet, HashMap};

impl MirModule {
    /// Create a new MIR module
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: BTreeMap::new(),
            globals: HashMap::new(),
            metadata: ModuleMetadata::default(),
            canonical_callable_definitions: BTreeMap::new(),
            canonical_object_definitions: None,
        }
    }

    /// Check and publish one cataloged function together with its exact
    /// source key.  The key is retained until this atomic publication point;
    /// consumers never recover it from the physical symbol.
    pub(crate) fn add_cataloged_box_method(
        &mut self,
        key: CanonicalSameModuleCallableKeyV1,
        function: MirFunction,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        self.check_constructor_namespace(&key, false)?;
        self.add_cataloged_callable(key, function)
    }

    pub(crate) fn add_cataloged_constructor(
        &mut self,
        key: CanonicalSameModuleCallableKeyV1,
        function: MirFunction,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        self.check_constructor_namespace(&key, true)?;
        self.add_cataloged_callable(key, function)
    }

    fn check_constructor_namespace(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        constructor: bool,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        if (key.namespace() == SameModuleCallableNamespaceV1::BirthConstructor) != constructor {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::KeyNamespaceMismatch {
                    key: key.clone(),
                },
            );
        }
        Ok(())
    }

    fn add_cataloged_callable(
        &mut self,
        key: CanonicalSameModuleCallableKeyV1,
        function: MirFunction,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        let symbol = function.signature.name.clone();
        let expected_symbol = key.mir_symbol_projection();
        if symbol != expected_symbol {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::KeySymbolMismatch { key, symbol },
            );
        }
        let expected_arity = match key.namespace() {
            SameModuleCallableNamespaceV1::FreeFunction => key.arity() as usize,
            SameModuleCallableNamespaceV1::StaticBoxMethod => key.arity() as usize,
            SameModuleCallableNamespaceV1::InstanceBoxMethod
            | SameModuleCallableNamespaceV1::BirthConstructor => {
                (key.arity() as usize).checked_add(1).ok_or_else(|| {
                    CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch {
                        key: key.clone(),
                        expected: usize::MAX,
                        actual: function.signature.params.len(),
                    }
                })?
            }
        };
        if function.signature.params.len() != expected_arity {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch {
                    key,
                    expected: expected_arity,
                    actual: function.signature.params.len(),
                },
            );
        }
        if self.canonical_callable_definitions.contains_key(&key)
            || self.functions.contains_key(&symbol)
        {
            return Err(CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { key });
        }
        self.functions.insert(symbol.clone(), function);
        self.canonical_callable_definitions.insert(key, symbol);
        Ok(())
    }

    /// Preflight the key/physical pair before a normal collector commits any
    /// function.  Batch duplicate detection remains with the collector.
    pub(crate) fn preflight_cataloged_box_method(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        symbol: &str,
        physical_arity: usize,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        self.check_constructor_namespace(key, false)?;
        self.preflight_cataloged_callable(key, symbol, physical_arity)
    }

    pub(crate) fn preflight_cataloged_constructor(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        symbol: &str,
        physical_arity: usize,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        self.check_constructor_namespace(key, true)?;
        self.preflight_cataloged_callable(key, symbol, physical_arity)
    }

    fn preflight_cataloged_callable(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        symbol: &str,
        physical_arity: usize,
    ) -> Result<(), CanonicalCallableDefinitionPublicationErrorV1> {
        if symbol != key.mir_symbol_projection() {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::KeySymbolMismatch {
                    key: key.clone(),
                    symbol: symbol.to_owned(),
                },
            );
        }
        let expected_arity = match key.namespace() {
            SameModuleCallableNamespaceV1::FreeFunction => key.arity() as usize,
            SameModuleCallableNamespaceV1::StaticBoxMethod => key.arity() as usize,
            SameModuleCallableNamespaceV1::InstanceBoxMethod
            | SameModuleCallableNamespaceV1::BirthConstructor => {
                (key.arity() as usize).checked_add(1).ok_or_else(|| {
                    CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch {
                        key: key.clone(),
                        expected: usize::MAX,
                        actual: physical_arity,
                    }
                })?
            }
        };
        if physical_arity != expected_arity {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch {
                    key: key.clone(),
                    expected: expected_arity,
                    actual: physical_arity,
                },
            );
        }
        if self.canonical_callable_definitions.contains_key(key)
            || self.functions.contains_key(symbol)
        {
            return Err(
                CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { key: key.clone() },
            );
        }
        Ok(())
    }

    /// Borrow the published physical definition symbol for a source key.
    /// This is a one-way publication lookup, not a resolver.
    pub(crate) fn canonical_callable_definition_symbol(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&str> {
        self.canonical_callable_definitions
            .get(key)
            .map(String::as_str)
    }

    pub(crate) fn canonical_callable_definition_count(&self) -> usize {
        self.canonical_callable_definitions.len()
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
