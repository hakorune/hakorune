//! Consuming final validation of the existing root/Birth source payloads.
//! Diagnostic compilation and artifact admission share the same built module.
use super::*;
use std::collections::BTreeSet;

/// One completed root owns either the callable or Script source payload.
/// This retention slot does not issue Script artifact ABI or lifecycle permission.
#[derive(Debug)]
pub(in crate::mir::builder) enum RootValidation {
    Absent,
    OrdinaryNew {
        key: String,
        ledger: Rc<OrdinaryNewClaimLedgerV1>,
    },
    Script {
        key: String,
        entry: crate::mir::BasicBlockId,
        source: super::super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState,
    },
}

impl RootValidation {
    fn key(&self) -> Option<&str> {
        match self {
            Self::Absent => None,
            Self::OrdinaryNew { key, .. } | Self::Script { key, .. } => Some(key),
        }
    }

    fn validate(&mut self, module: &MirModule, artifact: bool) -> Result<(), String> {
        let Some(key) = self.key() else { return Ok(()) };
        let root = module
            .functions
            .get(key)
            .ok_or_else(|| fault("root-missing"))?;
        if root.signature.name != key {
            return Err(fault("root-key-drift"));
        }
        match self {
            Self::Absent => unreachable!(),
            Self::OrdinaryNew { ledger, .. } => {
                if artifact && (has_lifecycle(root) || has_exact_field_read(root)) {
                    ledger.validate_artifact_after_compiler_finishing(root)
                } else {
                    ledger.validate_after_compiler_finishing(root)
                }
            }
            Self::Script { entry, source, .. } => {
                if root.entry_block != *entry {
                    return Err(fault("script-root-owner-drift"));
                }
                source.validate_finished_array_root(root)
            }
        }
    }
}

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleBuilderInvocationSessionV1,
        MirModule,
        impl FnOnce(&MirModule) -> Result<(), String>,
    ) {
        let validate = move |module: &MirModule| {
            let mut root_validation = self.root_validation;
            root_validation.validate(module, false)?;
            for (key, validation) in self.construction {
                let definition = module
                    .canonical_callable_definition_symbol(&key)
                    .and_then(|symbol| module.functions.get(symbol))
                    .ok_or_else(|| {
                        "[freeze:contract][construction/finished-definition-missing]".to_owned()
                    })?;
                validation.validate_after_compiler_finishing(definition)?;
            }
            Ok(())
        };
        (self.session, self.module, validate)
    }
}

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn into_artifact_parts(
        self,
    ) -> (
        ModuleBuilderInvocationSessionV1,
        MirModule,
        impl FnOnce(
            &MirModule,
        ) -> Result<
            Option<crate::mir::normal_callable_semantic_package::FinalizedRootBirthHandoffV1>,
            String,
        >,
    ) {
        let validate = move |module: &MirModule| {
            let mut covered = BTreeSet::new();
            let mut root_validation = self.root_validation;
            let retained_root = match &root_validation {
                RootValidation::OrdinaryNew { key, .. } => Some(key.clone()),
                RootValidation::Script { .. } | RootValidation::Absent => None,
            };
            root_validation.validate(module, true)?;
            if let Some(key) = &retained_root {
                covered.insert(key.clone());
            }
            let mut birth_keys = BTreeSet::new();
            for (key, validation) in self.construction {
                if key.namespace()
                    != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
                    || !birth_keys.insert(key.clone())
                {
                    return Err(fault("foreign-or-duplicate-birth-key"));
                }
                let symbol = module
                    .canonical_callable_definition_symbol(&key)
                    .ok_or_else(|| fault("birth-definition-missing"))?;
                if symbol != key.mir_symbol_projection() {
                    return Err(fault("birth-definition-symbol-drift"));
                }
                if !covered.insert(symbol.to_owned()) {
                    return Err(fault("duplicate-function-coverage"));
                }
                let definition = module
                    .functions
                    .get(symbol)
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
                if key.namespace()
                    == hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
                    && !birth_keys.contains(key)
                {
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
            match root_validation {
                RootValidation::OrdinaryNew { key, ledger } => ledger
                    .seal_finalized_root_birth_handoff(key, &birth_keys)
                    .map(Some),
                RootValidation::Script { .. } | RootValidation::Absent => Ok(None),
            }
        };
        (self.session, self.module, validate)
    }
}

fn has_lifecycle(function: &crate::mir::MirFunction) -> bool {
    function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| instruction.requires_lifecycle_validation())
}

fn has_exact_field_read(function: &crate::mir::MirFunction) -> bool {
    function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::ObjectFieldGet { .. }
            )
        })
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][lifecycle-artifact/{reason}]")
}

#[cfg(test)]
#[path = "normal_default_script_source_handoff_tests.rs"]
mod script_source_tests;
