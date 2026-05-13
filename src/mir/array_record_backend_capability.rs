use crate::mir::MirModule;

pub(crate) const ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG: &str =
    "[freeze:backend][array-record/packed-route-unsupported]";
pub(crate) const ARRAY_RECORD_BACKEND_CONSUMER_CAPABILITY: &str =
    "arraybox.inline_record_columns_v0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArrayRecordBackendCapabilityReport {
    pub packed_record_required_routes: usize,
}

pub(crate) fn inspect_array_record_backend_capability(
    module: &MirModule,
) -> ArrayRecordBackendCapabilityReport {
    ArrayRecordBackendCapabilityReport {
        packed_record_required_routes: packed_record_required_route_count(module),
    }
}

pub(crate) fn enforce_array_record_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let report = inspect_array_record_backend_capability(module);
    if report.packed_record_required_routes == 0
        || backend_supports_packed_record_inline_columns(backend)
    {
        return Ok(());
    }

    Err(format!(
        "{} backend={} required_routes={} require={} silent_fallback_allowed=false",
        ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG,
        backend,
        report.packed_record_required_routes,
        ARRAY_RECORD_BACKEND_CONSUMER_CAPABILITY
    ))
}

fn packed_record_required_route_count(module: &MirModule) -> usize {
    module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .iter()
        .filter(|plan| plan.backend_lowering_enabled)
        .count()
        + module
            .metadata
            .hako_alloc_aligned_small_packed_store_pilot_plans
            .iter()
            .filter(|plan| plan.backend_lowering_enabled)
            .count()
        + module
            .metadata
            .hako_alloc_huge_page_packed_store_pilot_plans
            .iter()
            .filter(|plan| plan.backend_lowering_enabled)
            .count()
}

fn backend_supports_packed_record_inline_columns(backend: &str) -> bool {
    matches!(backend, "pyvm-harness")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{ArrayRecordPackedAutoUsePilotPlan, MirModule};

    fn module_with_packed_route_required(required: bool) -> MirModule {
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
    fn default_metadata_only_pilots_do_not_require_backend_lowering() {
        let module = module_with_packed_route_required(false);
        assert_eq!(
            inspect_array_record_backend_capability(&module).packed_record_required_routes,
            0
        );
        assert!(enforce_array_record_backend_supported(&module, "wasm").is_ok());
    }

    #[test]
    fn required_packed_record_route_rejects_unsupported_backend() {
        let module = module_with_packed_route_required(true);
        let err = enforce_array_record_backend_supported(&module, "wasm").unwrap_err();
        assert!(err.contains(ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG));
        assert!(err.contains("required_routes=1"));
        assert!(err.contains(ARRAY_RECORD_BACKEND_CONSUMER_CAPABILITY));
        assert!(err.contains("silent_fallback_allowed=false"));
    }

    #[test]
    fn required_packed_record_route_accepts_reference_executor() {
        let module = module_with_packed_route_required(true);
        assert!(enforce_array_record_backend_supported(&module, "pyvm-harness").is_ok());
    }
}
