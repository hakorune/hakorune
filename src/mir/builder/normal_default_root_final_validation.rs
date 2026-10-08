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
    #[cfg(test)]
    pub(in crate::mir::builder) fn ordinary_root_ledger_for_test(
        &self,
    ) -> (String, Rc<OrdinaryNewClaimLedgerV1>) {
        match &self.root_validation {
            RootValidation::OrdinaryNew { key, ledger } => (key.clone(), Rc::clone(ledger)),
            _ => panic!("ordinary root fixture"),
        }
    }

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
            let callables = self.callables;
            let mut covered = BTreeSet::new();
            let mut root_validation = self.root_validation;
            let retained_root = match &root_validation {
                RootValidation::OrdinaryNew { key, .. } => Some(key.clone()),
                RootValidation::Script { .. } | RootValidation::Absent => None,
            };
            let child_symbols = root_validation.validate(module, true)?;
            // Finishing-checked children carry ledger-owned exact field
            // reads (`validate_field_reads` ran for each of them inside
            // `validate`), so they own `ObjectFieldGet` the same way the
            // retained root does.
            let exact_read_owners: BTreeSet<_> = child_symbols.iter().cloned().collect();
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
                if has_exact_field_read(function)
                    && retained_root.as_ref() != Some(symbol)
                    && !exact_read_owners.contains(symbol)
                {
                    return Err(fault("unowned-exact-field-read"));
                }
                if has_lifecycle(function) && !covered.contains(symbol) {
                    return Err(fault("uncovered-lifecycle-function"));
                }
            }
            seal_root_handoff(root_validation, module, &birth_keys, callables)
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
            let callables = self.callables;
            let mut root_validation = self.root_validation;
            let birth_keys =
                finish_document_inputs(&mut root_validation, module, self.construction)?;
            seal_root_handoff(root_validation, module, &birth_keys, callables)
        };
        (self.session, self.module, validate)
    }
}

/// The document route and its test checkpoint use the same finishing consumers.
fn finish_document_inputs(
    root: &mut RootValidation,
    module: &crate::mir::MirModule,
    construction: RetainedConstructionDrafts,
) -> Result<BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>, String> {
    root.validate(module, false)?;
    let mut keys = BTreeSet::new();
    for (key, validation) in construction {
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
            || !keys.insert(key.clone())
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
    Ok(keys)
}

#[cfg(test)]
impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    /// Retain the actual cohort and ledger at the original pre-seal checkpoint.
    pub(in crate::mir) fn document_preflight_parts_for_test(
        mut self,
        simplify: bool,
    ) -> Result<(
        String,
        crate::mir::MirModule,
        Rc<OrdinaryNewClaimLedgerV1>,
        BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
        Option<
            crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1,
        >,
    ), String>{
        if simplify {
            crate::mir::passes::simplify_cfg::simplify(&mut self.module);
        }
        let keys =
            finish_document_inputs(&mut self.root_validation, &self.module, self.construction)?;
        let RootValidation::OrdinaryNew { key, ledger } = self.root_validation else {
            panic!("ordinary document fixture");
        };
        Ok((key, self.module, ledger, keys, self.callables))
    }
}

/// Finish each existing root family with all named-array input checks before take.
fn seal_root_handoff(
    root: RootValidation,
    module: &crate::mir::MirModule,
    birth_keys: &BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
    mut callables: Option<
        crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1,
    >,
) -> Result<Option<crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>, String> {
    use crate::mir::finalized_root_handoff::{
        validate_named_array_handoff_inputs, FinalizedRootHandoffV1,
    };
    let (key, array) = match root {
        RootValidation::OrdinaryNew { key, ledger } => {
            return ledger
                .seal_finalized_root_birth_handoff(key, module, birth_keys, callables)
                .map(Some);
        }
        RootValidation::Script { key, entry, source } => {
            validate_named_array_handoff_inputs(
                module,
                callables.as_ref(),
                callables
                    .as_ref()
                    .map_or(&[][..], |cohort| cohort.named_array_emissions()),
            )?;
            (Some(key), source.into_array_artifact(entry)?)
        }
        RootValidation::Absent => {
            validate_named_array_handoff_inputs(
                module,
                callables.as_ref(),
                callables
                    .as_ref()
                    .map_or(&[][..], |cohort| cohort.named_array_emissions()),
            )?;
            (None, None)
        }
    };
    let named_arrays = callables.as_mut().map_or_else(
        || Box::new([]) as Box<[_]>,
        |cohort| cohort.take_named_array_emissions(),
    );
    Ok(match array {
        Some(array) => Some(FinalizedRootHandoffV1::ScriptArray {
            root_key: key.expect("Script source owns the Array root"),
            named_arrays,
            callables,
            array,
        }),
        None => callables.map(|callables| FinalizedRootHandoffV1::Module {
            callables: Some(callables),
            named_arrays,
        }),
    })
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
