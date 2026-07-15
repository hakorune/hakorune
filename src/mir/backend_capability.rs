use crate::mir::MirModule;

pub(crate) fn enforce_mir_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
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
    crate::mir::array_record_backend_capability::enforce_array_record_backend_supported(
        module, backend,
    )?;
    crate::mir::parameter_entry_backend_capability::enforce_parameter_entry_backend_supported(
        module, backend,
    )?;
    crate::mir::return_exit_backend_capability::enforce_return_exit_backend_supported(
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
    crate::mir::typed_array_backend_capability::enforce_typed_array_backend_supported(
        module, backend,
    )?;
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
