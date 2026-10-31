use crate::mir::MirModule;

/// Selected published callers validate the borrowed final image, never a repair clone.
pub(crate) fn enforce_published_backend_supported(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
    backend: &str,
) -> Result<(), String> {
    validate_published_ingress(view)?;
    if backend != "ny-llvmc-obj" {
        crate::mir::named_array_obligation::reject_unretained_module(view.module())?;
    }
    enforce_refreshed_mir_backend_supported(view.module(), backend)
}

/// The lifecycle consumer proves numeric coverage using the exact input it emits.
/// No arbitrary skip flag or alternate backend spelling can select this path.
pub(crate) fn enforce_published_lifecycle_backend_supported(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
    input: &crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    validate_published_ingress(view)?;
    view.validate_lifecycle_input_handoff(input)?;
    verify_source_moves_on_published_mir(input)?;
    crate::mir::ownership_backend_capability::enforce(view.module(), "ny-llvmc-obj")?;
    crate::mir::exact_numeric_backend_capability::enforce_lifecycle_input(view.module(), input)?;
    enforce_before_typed_array_backend_supported_without_parameter_entry_or_return_exit(
        view.module(),
        "ny-llvmc-obj",
    )?;
    crate::mir::return_exit_backend_capability::enforce_lifecycle_return_exit_backend_supported(
        view.module(),
        input,
    )?;
    crate::mir::parameter_entry_backend_capability::enforce_lifecycle_parameter_entry_backend_supported(
        view.module(),
        input,
    )?;
    crate::mir::typed_array_backend_capability::enforce_lifecycle_input(view, input)?;
    enforce_after_typed_array_backend_supported(view.module(), "ny-llvmc-obj")
}

fn verify_source_moves_on_published_mir(
    input: &crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    use crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1 as Role;
    use crate::mir::ownership_ssa::{FunctionResultOwnershipV1, OwnershipFunctionAbiV1, OwnershipFunctionOwnerV1};
    let handoff = input.program().handoff();
    let Some(source) = handoff.root_source() else { return Ok(()) };
    let mut remaining = source.move_ownership_owners_v1();
    for physical in input.program().functions() {
        let owner = match physical.role() {
            Role::Root { .. } => Some(source.owner()),
            Role::BirthUnit { abi } => Some(abi.owner()),
            role => role.ordinary_target().and_then(|key| handoff.callables()?.owner_for_canonical_key(key)),
        };
        let Some(owner) = owner else { continue };
        let Some((aliases, initially_published)) = source.move_ownership_aliases_v1(owner, physical.function())? else { continue };
        // Step 1 currently proves the parameter-free, non-owned-result lane.
        // Do not infer borrowed/owned ABI from a MIR type or physical name.
        if !physical.params().is_empty() || !matches!(physical.role(), Role::Root { .. } | Role::OrdinaryI64 { .. } | Role::OrdinaryUnit { .. } | Role::OrdinaryBool { .. }) {
            return Err("[ownership/move-abi-not-supported]".into());
        }
        let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), Vec::new(), FunctionResultOwnershipV1::None)
            .with_source_aliases(aliases).with_initially_published(initially_published);
        crate::mir::ownership_ssa::verify_ownership_ssa_v1(physical.function(), &abi)
            .map_err(|error| format!("[ownership/published-mir] {error:?}"))?;
        if !remaining.remove(&owner) { return Err("[ownership/move-owner-duplicate]".into()); }
    }
    if !remaining.is_empty() { return Err("[ownership/move-owner-unpublished]".into()); }
    Ok(())
}

fn validate_published_ingress(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
) -> Result<(), String> {
    let module = view.module();
    // Source/physical correspondence is checked here; actual constructor
    // capability is discharged by the same C invocation that compiles it.
    view.validated_named_arrays()?;
    for function in module.functions.values() {
        if !function.metadata.extern_call_routes.is_empty()
            || function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .any(|instruction| {
                    matches!(instruction, crate::mir::MirInstruction::LegacyCallV0 { .. })
                        || matches!(instruction, crate::mir::MirInstruction::Call(call)
                        if matches!(call.callee, crate::mir::Callee::Extern(_)))
                })
        {
            return Err("[freeze:contract][published-backend/compatibility-ingress]".into());
        }
    }
    crate::mir::semantic_refresh::validate_published_contracts(module)
}

pub(crate) fn enforce_mir_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    crate::mir::named_array_obligation::reject_unretained_module(module)?;
    let refreshed = crate::mir::semantic_refresh::refresh_owned_for_boundary(
        module,
        crate::mir::ContractRefreshBoundary::BackendPreflight,
    )?;
    enforce_refreshed_mir_backend_supported(refreshed.module(), backend)
}

fn enforce_refreshed_mir_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    crate::mir::ownership_backend_capability::enforce(module, backend)?;
    crate::mir::exact_numeric_backend_capability::enforce_exact_numeric_backend_supported(
        module, backend,
    )?;
    enforce_before_typed_array_backend_supported(module, backend)?;
    crate::mir::typed_array_backend_capability::enforce_typed_array_backend_supported(
        module, backend,
    )?;
    enforce_after_typed_array_backend_supported(module, backend)
}

fn enforce_before_typed_array_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    enforce_before_typed_array_backend_supported_without_parameter_entry_or_return_exit(
        module, backend,
    )?;
    crate::mir::return_exit_backend_capability::enforce_return_exit_backend_supported(
        module, backend,
    )?;
    crate::mir::parameter_entry_backend_capability::enforce_parameter_entry_backend_supported(
        module, backend,
    )
}

fn enforce_before_typed_array_backend_supported_without_parameter_entry_or_return_exit(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    crate::mir::array_record_backend_capability::enforce_array_record_backend_supported(
        module, backend,
    )?;
    crate::mir::canonical_direct_static_call_backend_capability::enforce(module, backend)?;
    crate::mir::canonical_recursive_callable_module_backend_capability::enforce(module, backend)?;
    crate::mir::local_slot_backend_capability::enforce_local_slot_backend_supported(
        module, backend,
    )?;
    crate::mir::record_value_backend_capability::enforce_record_value_backend_supported(
        module, backend,
    )?;
    crate::mir::static_table_backend_capability::enforce_static_table_backend_supported(
        module, backend,
    )?;
    Ok(())
}

fn enforce_after_typed_array_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    crate::mir::weak_field_backend_capability::enforce_weak_field_backend_supported(
        module, backend,
    )?;
    crate::mir::array_element_write::enforce_backend_supported(module, backend)?;
    crate::mir::decoded_utf8_byte_len_backend_capability::enforce_decoded_utf8_byte_len_backend_supported(
        module, backend,
    )?;
    crate::mir::strict_json_tree_backend_capability::enforce_strict_json_tree_backend_supported(
        module, backend,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
    use crate::mir::function::ArrayRecordPackedAutoUsePilotPlan;
    use crate::mir::{
        array_record_backend_capability::ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG,
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

    fn module_with_packed_route(required: bool) -> MirModule {
        let mut module = MirModule::new("test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(ArrayRecordPackedAutoUsePilotPlan {
                record_name: "Meta".to_string(),
                layout_id: 7,
                pilot_kind: "integer_lane_direct_reads_v0".to_string(),
                source_boundary_kind: "non_escaping_direct_field_reads".to_string(),
                integer_lane_columns: 2,
                direct_indexed_field_reads_enabled: true,
                private_runtime_storage_enabled: true,
                public_array_get_materialization_enabled: false,
                hako_alloc_migration_enabled: false,
                backend_lowering_enabled: required,
            });
        module
    }

    #[test]
    fn shared_gate_accepts_plain_modules() {
        let module = MirModule::new("plain".to_string());
        assert!(enforce_mir_backend_supported(&module, "wasm").is_ok());
    }

    #[test]
    fn shared_gate_includes_packed_record_backend_failfast() {
        let module = module_with_packed_route(true);
        let err = enforce_mir_backend_supported(&module, "wasm").unwrap_err();
        assert!(err.contains(ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG));
    }

    #[test]
    fn shared_gate_includes_canonical_direct_call_failfast() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "countdown/1".to_string(),
                params: vec![MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        CanonicalDirectStaticCallCapabilityV1::install_for_function(
            &mut function.metadata.canonical_direct_static_call_capabilities,
            true,
        )
        .unwrap();
        let mut module = MirModule::new("direct-call-gate".to_string());
        module.add_function(function);

        assert!(enforce_mir_backend_supported(&module, "mir-interpreter").is_ok());
        let error = enforce_mir_backend_supported(&module, "wasm").unwrap_err();
        assert!(error.contains(
            crate::mir::canonical_direct_static_call_backend_capability::
                CANONICAL_DIRECT_STATIC_CALL_BACKEND_UNSUPPORTED_TAG
        ));
    }
}
