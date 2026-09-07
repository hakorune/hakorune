//! Finalization-owned lifecycle admission before the artifact consumer callback.
//!
//! Consumes existing published identity and handoff only; no source meaning is
//! issued here. The callback borrows the selected profile from finalization.

use hakorune_mir_defs::SameModuleCallableNamespaceV1;
use crate::mir::{Callee, MirInstruction, ValueId};
use super::published_backend_view::{
    PublishedMirBackendView, PublishedObjectStorageProfileV1, PublishedStaticMethodRouteV1,
};

pub(super) fn admit_lifecycle<'module>(
    mut view: PublishedMirBackendView<'module>,
    profile: &'module PublishedObjectStorageProfileV1,
) -> Result<PublishedMirBackendView<'module>, String> {
    if view.route() != PublishedStaticMethodRouteV1::UnsupportedBeforeObject
        || view.has_non_lifecycle_unsupported
        || view.lifecycle_instructions.is_empty()
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
    if !matches!(
        view.retained_root_result(),
        Some(crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::IntegerLiteralReturn { .. }
            | crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64FieldReturn { .. })
    ) {
        return Err(fault("retained-root-result-missing"));
    }
    if view.retained_root_source().is_none() {
        return Err(fault("retained-root-source-missing"));
    }
    let module = view.module();
    view.lifecycle_instructions
        .extend(view.return_instructions.iter().copied().filter(|row| {
            row.function_name() == root_name
                || module
                    .canonical_callable_definitions
                    .iter()
                    .any(|(key, symbol)| {
                        retained_births.iter().any(|birth| birth.target() == key)
                            && symbol.as_str() == row.function_name()
                    })
        }));
    for row in &view.lifecycle_instructions {
        if row.function_name() == root_name {
            continue;
        }
        let Some(function) = view.module().functions.get(row.function_name()) else {
            return Err(fault("function-missing"));
        };
        let Some((key, _)) = view
            .module()
            .canonical_callable_definitions
            .iter()
            .find(|(_, symbol)| symbol.as_str() == row.function_name())
        else {
            return Err(fault("function-not-cataloged"));
        };
        if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
            || function.signature.name != key.mir_symbol_projection()
        {
            return Err(fault("function-not-birth"));
        }
    }
    for row in &view.lifecycle_instructions {
        if let MirInstruction::Call(call) = row.instruction() {
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                continue;
            };
            if *receiver == ValueId::INVALID
                || key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                || module
                    .canonical_callable_definition_symbol(key)
                    .is_none()
            {
                return Err(fault("birth-call-drift"));
            }
        }
    }
    view.lifecycle_storage_profile = Some(profile);
    Ok(view)
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/admission-{reason}]")
}
