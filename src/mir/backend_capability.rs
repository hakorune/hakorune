use crate::mir::MirModule;

pub(crate) fn enforce_mir_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    crate::mir::exact_numeric_backend_capability::enforce_exact_numeric_backend_supported(
        module, backend,
    )?;
    crate::mir::array_record_backend_capability::enforce_array_record_backend_supported(
        module, backend,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        array_record_backend_capability::ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG,
        ArrayRecordPackedAutoUsePilotPlan, MirModule,
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
}
