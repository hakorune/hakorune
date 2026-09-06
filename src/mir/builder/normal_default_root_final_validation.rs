//! Consuming final validation of the existing root/Birth source payloads.
//! Diagnostic compilation and artifact admission share the same built module.
use super::*;
use std::collections::BTreeSet;

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn into_parts(self) -> (
        ModuleBuilderInvocationSessionV1,
        MirModule,
        impl FnOnce(&MirModule) -> Result<(), String>,
    ) {
        let validate = move |module: &MirModule| {
            if let Some((root_key, ledger)) = self.root_new_validation {
                let root = module.functions.get(&root_key).ok_or_else(|| {
                    "[freeze:contract][ordinary-new/finished-root-missing]".to_owned()
                })?;
                ledger.validate_after_compiler_finishing(root)?;
            }
            for (key, validation) in self.construction {
                let definition = module.canonical_callable_definition_symbol(&key)
                    .and_then(|symbol| module.functions.get(symbol))
                    .ok_or_else(|| "[freeze:contract][construction/finished-definition-missing]".to_owned())?;
                validation.validate_after_compiler_finishing(definition)?;
            }
            Ok(())
        };
        (self.session, self.module, validate)
    }
}

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn into_artifact_parts(self) -> (
        ModuleBuilderInvocationSessionV1,
        MirModule,
        impl FnOnce(&MirModule) -> Result<Option<String>, String>,
    ) {
        let validate = move |module: &MirModule| {
            let mut covered = BTreeSet::new();
            let retained_root = self.root_new_validation.as_ref().map(|(key, _)| key.clone());
            if let Some((root_key, ledger)) = self.root_new_validation {
                let root = module.functions.get(&root_key)
                    .ok_or_else(|| fault("root-missing"))?;
                if has_lifecycle(root) || has_exact_field_read(root) {
                    ledger.validate_artifact_after_compiler_finishing(root)?;
                } else {
                    ledger.validate_after_compiler_finishing(root)?;
                }
                covered.insert(root_key);
            }
            let mut birth_keys = BTreeSet::new();
            for (key, validation) in self.construction {
                if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
                    || !birth_keys.insert(key.clone()) {
                    return Err(fault("foreign-or-duplicate-birth-key"));
                }
                let symbol = module.canonical_callable_definition_symbol(&key)
                    .ok_or_else(|| fault("birth-definition-missing"))?;
                if symbol != key.mir_symbol_projection() {
                    return Err(fault("birth-definition-symbol-drift"));
                }
                if !covered.insert(symbol.to_owned()) {
                    return Err(fault("duplicate-function-coverage"));
                }
                let definition = module.functions.get(symbol)
                    .ok_or_else(|| fault("birth-function-missing"))?;
                if definition.signature.name != symbol {
                    return Err(fault("birth-function-symbol-drift"));
                }
                if has_exact_field_read(definition) {
                    return Err(fault("unowned-exact-field-read"));
                }
                validation.validate_artifact_after_compiler_finishing(definition)?;
            }
            for key in module.canonical_callable_definitions.keys() {
                if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
                    && !birth_keys.contains(key) {
                    return Err(fault("uncovered-birth-definition"));
                }
            }
            for (symbol, function) in &module.functions {
                if has_exact_field_read(function) && retained_root.as_ref() != Some(symbol) {
                    return Err(fault("unowned-exact-field-read"));
                }
                if has_lifecycle(function) && !covered.contains(symbol) {
                    return Err(fault("uncovered-lifecycle-function"));
                }
            }
            Ok(retained_root)
        };
        (self.session, self.module, validate)
    }
}

fn has_lifecycle(function: &crate::mir::MirFunction) -> bool {
    function.blocks.values().flat_map(|block| block.all_instructions())
        .any(|instruction| instruction.requires_lifecycle_validation())
}

fn has_exact_field_read(function: &crate::mir::MirFunction) -> bool {
    function.blocks.values().flat_map(|block| block.all_instructions())
        .any(|instruction| matches!(instruction, crate::mir::MirInstruction::ObjectFieldGet { .. }))
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][lifecycle-artifact/{reason}]")
}
