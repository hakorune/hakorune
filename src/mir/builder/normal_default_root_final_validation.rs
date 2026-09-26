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

    fn validate(&mut self, module: &MirModule, artifact: bool) -> Result<BTreeSet<String>, String> {
        let Some(key) = self.key() else {
            return Ok(BTreeSet::new());
        };
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
                let covered = ledger.validate_finalized_child_functions(module, artifact)?;
                if artifact && (has_lifecycle(root) || has_exact_field_read(root)) {
                    ledger.validate_artifact_after_compiler_finishing(root)?;
                } else {
                    ledger.validate_after_compiler_finishing(root)?;
                }
                Ok(covered)
            }
            Self::Script { entry, source, .. } => {
                if root.entry_block != *entry {
                    return Err(fault("script-root-owner-drift"));
                }
                source.validate_finished_array_root(root)?;
                Ok(BTreeSet::new())
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
            crate::mir::named_array_obligation::reject_unretained_module(module)?;
            let _callables = self.callables;
            let mut root_validation = self.root_validation;
            let _ = root_validation.validate(module, false)?;
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
            Option<crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
            String,
        >,
    ) {
        let validate = move |module: &MirModule| {
            let mut callables = self.callables;
            let named_arrays = match callables.as_mut() {
                Some(cohort) => cohort.take_named_array_emissions(),
                None => Box::new([]),
            };
            let mut covered = BTreeSet::new();
            let mut root_validation = self.root_validation;
            let retained_root = match &root_validation {
                RootValidation::OrdinaryNew { key, .. } => Some(key.clone()),
                RootValidation::Script { .. } | RootValidation::Absent => None,
            };
            let child_symbols = root_validation.validate(module, true)?;
            covered.extend(child_symbols);
            if let Some(key) = &retained_root {
                covered.insert(key.clone());
            }
            // Only the source-selected Script Array whose full control coverage
            // just passed can own lifecycle sites. Other Script roots stay unowned.
            if let RootValidation::Script { key, source, .. } = &root_validation {
                if source.has_array_lifecycle() {
                    covered.insert(key.clone());
                }
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
            let handoff = match root_validation {
                RootValidation::OrdinaryNew { key, ledger } => ledger
                    .seal_finalized_root_birth_handoff(key, &birth_keys, callables)
                    .map(Some),
                RootValidation::Script { key, entry, source } => {
                    use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
                    Ok(match source.into_array_artifact(entry)? {
                        Some(array) => Some(FinalizedRootHandoffV1::ScriptArray {
                            named_arrays: Box::new([]),
                            root_key: key,
                            array,
                            callables,
                        }),
                        None => callables.map(|callables| FinalizedRootHandoffV1::Module {
                            callables: Some(callables),
                            named_arrays: Box::new([]),
                        }),
                    })
                }
                RootValidation::Absent => Ok(callables.map(|callables| {
                    crate::mir::finalized_root_handoff::FinalizedRootHandoffV1::Module {
                        callables: Some(callables),
                        named_arrays: Box::new([]),
                    }
                })),
            }?;
            match handoff {
                Some(handoff) => {
                    let handoff = handoff.with_named_arrays(named_arrays)?;
                    handoff.validate_named_arrays(module)?;
                    Ok(Some(handoff))
                }
                None if named_arrays.is_empty() => {
                    crate::mir::normal_callable_semantic_package::validate_named_array_coverage(
                        module,
                        &[],
                    )?;
                    Ok(None)
                }
                None => Err(crate::mir::named_array_obligation::fault("handoff-missing")),
            }
        };
        (self.session, self.module, validate)
    }
}

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    /// Document publication finishing: the same consuming shape as
    /// `into_artifact_parts` minus object-compilation admission. The
    /// non-artifact finishing validation and the sole finalized-handoff
    /// seal still run — a `RetainedUnavailable` claim stays claim-owned
    /// and is never promoted or rebound into lifecycle bindings.
    pub(in crate::mir) fn into_document_parts(
        self,
    ) -> (
        ModuleBuilderInvocationSessionV1,
        MirModule,
        impl FnOnce(
            &MirModule,
        ) -> Result<
            Option<crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
            String,
        >,
    ) {
        let validate = move |module: &MirModule| {
            let mut callables = self.callables;
            let named_arrays = match callables.as_mut() {
                Some(cohort) => cohort.take_named_array_emissions(),
                None => Box::new([]),
            };
            let mut root_validation = self.root_validation;
            root_validation.validate(module, false)?;
            let mut birth_keys = BTreeSet::new();
            for (key, validation) in self.construction {
                if key.namespace()
                    != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
                    || !birth_keys.insert(key.clone())
                {
                    return Err(fault("foreign-or-duplicate-birth-key"));
                }
                let definition = module
                    .canonical_callable_definition_symbol(&key)
                    .and_then(|symbol| module.functions.get(symbol))
                    .ok_or_else(|| {
                        "[freeze:contract][construction/finished-definition-missing]".to_owned()
                    })?;
                validation.validate_after_compiler_finishing(definition)?;
            }
            let handoff = match root_validation {
                RootValidation::OrdinaryNew { key, ledger } => ledger
                    .seal_finalized_root_birth_handoff(key, &birth_keys, callables)
                    .map(Some),
                RootValidation::Script { key, entry, source } => {
                    use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
                    Ok(match source.into_array_artifact(entry)? {
                        Some(array) => Some(FinalizedRootHandoffV1::ScriptArray {
                            named_arrays: Box::new([]),
                            root_key: key,
                            array,
                            callables,
                        }),
                        None => callables.map(|callables| FinalizedRootHandoffV1::Module {
                            callables: Some(callables),
                            named_arrays: Box::new([]),
                        }),
                    })
                }
                RootValidation::Absent => Ok(callables.map(|callables| {
                    crate::mir::finalized_root_handoff::FinalizedRootHandoffV1::Module {
                        callables: Some(callables),
                        named_arrays: Box::new([]),
                    }
                })),
            }?;
            match handoff {
                Some(handoff) => {
                    let handoff = handoff.with_named_arrays(named_arrays)?;
                    handoff.validate_named_arrays(module)?;
                    Ok(Some(handoff))
                }
                None if named_arrays.is_empty() => {
                    crate::mir::normal_callable_semantic_package::validate_named_array_coverage(
                        module,
                        &[],
                    )?;
                    Ok(None)
                }
                None => Err(crate::mir::named_array_obligation::fault("handoff-missing")),
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
