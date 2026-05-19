/*!
 * Source PackedArray<T> auto-use pilot metadata.
 *
 * This owner connects explicit source declarations such as
 * `field: PackedArray<Meta>` to the existing non-escaping packed ArrayBox pilot
 * metadata. It does not rewrite runtime storage, enable backend lowering, or
 * allow fallback to ordinary ArrayBox storage.
 */

use crate::mir::array_record_packed_autouse_pilot::ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0;
use crate::mir::function::{ArrayRecordPackedAutoUsePilotPlan, SourcePackedArrayAutoUsePilotPlan};
use crate::mir::MirModule;

pub const SOURCE_PACKED_ARRAY_AUTOUSE_PILOT_KIND_DECLARED_PACKED_RECORD_ARRAY_V0: &str =
    "declared_packed_record_array_v0";

pub fn refresh_module_source_packed_array_autouse_pilot_plans(module: &mut MirModule) {
    module.metadata.source_packed_array_autouse_pilot_plans =
        build_source_packed_array_autouse_pilot_plans(module);
}

pub fn build_source_packed_array_autouse_pilot_plans(
    module: &MirModule,
) -> Vec<SourcePackedArrayAutoUsePilotPlan> {
    let mut out = Vec::new();
    for (owner_box, fields) in &module.metadata.user_box_field_decls {
        for field in fields {
            let Some(declared_type_name) = field.declared_type_name.as_deref() else {
                continue;
            };
            let Some(record_name) = packed_array_record_name(declared_type_name) else {
                continue;
            };
            let Some(pilot) = module
                .metadata
                .array_record_packed_autouse_pilot_plans
                .iter()
                .find(|plan| plan.record_name == record_name)
            else {
                continue;
            };
            if let Some(plan) = classify_source_packed_array_autouse_pilot(
                owner_box,
                &field.name,
                declared_type_name,
                record_name,
                pilot,
            ) {
                out.push(plan);
            }
        }
    }
    out
}

pub fn classify_source_packed_array_autouse_pilot(
    owner_box: &str,
    field_name: &str,
    declared_type_name: &str,
    record_name: &str,
    pilot: &ArrayRecordPackedAutoUsePilotPlan,
) -> Option<SourcePackedArrayAutoUsePilotPlan> {
    if pilot.record_name != record_name {
        return None;
    }
    if pilot.pilot_kind != ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0 {
        return None;
    }
    if !pilot.direct_indexed_field_reads_enabled
        || !pilot.private_runtime_storage_enabled
        || pilot.public_array_get_materialization_enabled
        || pilot.hako_alloc_migration_enabled
        || pilot.backend_lowering_enabled
    {
        return None;
    }

    Some(SourcePackedArrayAutoUsePilotPlan {
        owner_box: owner_box.to_string(),
        field_name: field_name.to_string(),
        declared_type_name: declared_type_name.to_string(),
        record_name: record_name.to_string(),
        layout_id: pilot.layout_id,
        pilot_kind: SOURCE_PACKED_ARRAY_AUTOUSE_PILOT_KIND_DECLARED_PACKED_RECORD_ARRAY_V0
            .to_string(),
        source_boundary_kind: pilot.source_boundary_kind.clone(),
        source_declared_packed: true,
        direct_indexed_field_reads_enabled: true,
        private_runtime_storage_enabled: true,
        public_array_get_materialization_enabled: false,
        backend_lowering_enabled: false,
        boxed_fallback_enabled: false,
    })
}

fn packed_array_record_name(type_name: &str) -> Option<&str> {
    let inner = type_name
        .trim()
        .strip_prefix("PackedArray<")?
        .strip_suffix('>')?
        .trim();
    if inner.is_empty()
        || inner.contains('<')
        || inner.contains('>')
        || inner.contains(',')
        || inner.contains(char::is_whitespace)
    {
        return None;
    }
    Some(inner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{
        ArrayRecordPackedAutoUsePilotPlan, SourcePackedArrayAutoUsePilotPlan,
    };
    use crate::mir::{MirModule, UserBoxFieldDecl};

    fn packed_pilot(record_name: &str) -> ArrayRecordPackedAutoUsePilotPlan {
        ArrayRecordPackedAutoUsePilotPlan {
            record_name: record_name.to_string(),
            layout_id: 41,
            pilot_kind: ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0
                .to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            integer_lane_columns: 2,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            hako_alloc_migration_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    #[test]
    fn source_packed_array_autouse_pilot_consumes_declared_packed_fields() {
        let mut module = MirModule::new("source-packed-array-pilot-test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(packed_pilot("Meta"));
        module.metadata.user_box_field_decls.insert(
            "Store".to_string(),
            vec![UserBoxFieldDecl {
                name: "metas".to_string(),
                declared_type_name: Some("PackedArray<Meta>".to_string()),
                is_weak: false,
            }],
        );

        let plans = build_source_packed_array_autouse_pilot_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].owner_box, "Store");
        assert_eq!(plans[0].field_name, "metas");
        assert_eq!(plans[0].declared_type_name, "PackedArray<Meta>");
        assert_eq!(plans[0].record_name, "Meta");
        assert_eq!(plans[0].layout_id, 41);
        assert_eq!(
            plans[0].pilot_kind,
            SOURCE_PACKED_ARRAY_AUTOUSE_PILOT_KIND_DECLARED_PACKED_RECORD_ARRAY_V0
        );
        assert!(plans[0].source_declared_packed);
        assert!(plans[0].direct_indexed_field_reads_enabled);
        assert!(plans[0].private_runtime_storage_enabled);
        assert!(!plans[0].public_array_get_materialization_enabled);
        assert!(!plans[0].backend_lowering_enabled);
        assert!(!plans[0].boxed_fallback_enabled);
    }

    #[test]
    fn source_packed_array_autouse_pilot_rejects_materializing_pilot_rows() {
        let mut pilot = packed_pilot("Meta");
        pilot.public_array_get_materialization_enabled = true;

        let plan: Option<SourcePackedArrayAutoUsePilotPlan> =
            classify_source_packed_array_autouse_pilot(
                "Store",
                "metas",
                "PackedArray<Meta>",
                "Meta",
                &pilot,
            );

        assert!(plan.is_none());
    }

    #[test]
    fn source_packed_array_autouse_pilot_ignores_noncanonical_type_text() {
        let mut module = MirModule::new("source-packed-array-pilot-test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(packed_pilot("Meta"));
        module.metadata.user_box_field_decls.insert(
            "Store".to_string(),
            vec![UserBoxFieldDecl {
                name: "metas".to_string(),
                declared_type_name: Some("PackedArray<Meta<PageId>>".to_string()),
                is_weak: false,
            }],
        );

        let plans = build_source_packed_array_autouse_pilot_plans(&module);

        assert!(plans.is_empty());
    }
}
