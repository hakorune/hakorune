//! Finalization-owned lifecycle admission before the artifact consumer callback.
//!
//! Consumes existing published identity and handoff only; no source meaning is
//! issued here. The callback borrows the selected profile from finalization.

use super::published_backend_view::{
    is_lifecycle_instruction, PublishedMirBackendView, PublishedObjectStorageProfileV1,
    PublishedStaticMethodRouteV1,
};
use crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1;
use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, ValueId};
use hakorune_mir_defs::SameModuleCallableNamespaceV1;

pub(super) fn admit_lifecycle<'module>(
    mut view: PublishedMirBackendView<'module>,
    profile: &'module PublishedObjectStorageProfileV1,
) -> Result<PublishedMirBackendView<'module>, String> {
    if view.retained_script_array().is_some() {
        return Err(fault("script-root-not-callable"));
    }
    if view.route() != PublishedStaticMethodRouteV1::UnsupportedBeforeObject
        || view.has_non_lifecycle_unsupported
        || !view.has_lifecycle_instructions()
    {
        return Err(fault("candidate-unavailable"));
    }
    let root = view
        .retained_root()
        .ok_or_else(|| fault("retained-root-missing"))?;
    let root_name = root.signature.name.as_str();
    let retained_births = view
        .retained_birth_abi()
        .ok_or_else(|| fault("retained-birth-handoff-missing"))?;
    let root_source = view
        .retained_root_source()
        .ok_or_else(|| fault("retained-root-source-missing"))?;
    let ordinary_call = super::published_backend_view::issued_ordinary_call(root_source)?;
    let result_is_retained = matches!(
        view.retained_root_result(),
        Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::IntegerLiteralReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64FieldReturn { .. })
    );
    // A root ordinary Call has no scalar terminal result handoff of its own;
    // the call's i64 result is the physical root result. The source handoff
    // carries the call entry, and physical-program issuance validates it.
    if !result_is_retained && root_source.call_entry().is_none() {
        return Err(fault("retained-root-result-missing"));
    }
    let ordinary_name = ordinary_call.as_ref().map(|call| {
        super::published_backend_view::ordinary_callable_key(&call.callee)
            .map(|key| key.mir_symbol_projection())
    });
    let ordinary_name = ordinary_name.transpose()?;
    validate_functions(view.module(), root_name, retained_births, ordinary_name.as_deref())?;
    view.lifecycle_storage_profile = Some(profile);
    Ok(view)
}

// Preserve namespace validation before call validation. Return-only retained
// Birth functions participated in the former appended rows and remain checked.
fn validate_functions(
    module: &MirModule,
    root_name: &str,
    retained_births: &[BirthAbiHandoffV1],
    ordinary_name: Option<&str>,
) -> Result<(), String> {
    for (name, function) in &module.functions {
        if name != root_name && has_lifecycle(function) {
            if ordinary_name == Some(name.as_str()) {
                continue;
            }
            validate_birth_function(module, name, function)?;
        }
    }
    for (name, function) in &module.functions {
        if name == root_name || has_lifecycle(function) {
            continue;
        }
        let retained = module
            .canonical_callable_definitions
            .iter()
            .any(|(key, symbol)| {
                symbol == name && retained_births.iter().any(|birth| birth.target() == key)
            });
        if retained
            && function.blocks.values().any(|block| {
                block
                    .all_instructions()
                    .any(|instruction| matches!(instruction, MirInstruction::Return { .. }))
            })
        {
            validate_birth_function(module, name, function)?;
        }
    }
    for function in module.functions.values() {
        for instruction in function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
        {
            let MirInstruction::Call(call) = instruction else {
                continue;
            };
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                continue;
            };
            if *receiver == ValueId::INVALID
                || key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                || module.canonical_callable_definition_symbol(key).is_none()
            {
                return Err(fault("birth-call-drift"));
            }
        }
    }
    Ok(())
}

fn has_lifecycle(function: &MirFunction) -> bool {
    function
        .blocks
        .values()
        .any(|block| block.all_instructions().any(is_lifecycle_instruction))
}

fn validate_birth_function(
    module: &MirModule,
    name: &str,
    function: &MirFunction,
) -> Result<(), String> {
    let Some((key, _)) = module
        .canonical_callable_definitions
        .iter()
        .find(|(_, symbol)| symbol.as_str() == name)
    else {
        return Err(fault("function-not-cataloged"));
    };
    if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
        || function.signature.name != key.mir_symbol_projection()
    {
        return Err(fault("function-not-birth"));
    }
    Ok(())
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/admission-{reason}]")
}

#[cfg(test)]
#[path = "lifecycle_admission_tests.rs"]
mod tests;
