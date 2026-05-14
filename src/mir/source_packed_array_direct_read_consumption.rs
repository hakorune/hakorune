/*!
 * Source PackedArray<Record> direct-read consumption metadata.
 *
 * PACKED-003 consumes explicit source PackedArray pilot rows into per-record
 * field direct-read plans. It does not enable runtime/backend lowering,
 * materialization, or boxed fallback.
 */

use crate::mir::function::{
    RecordLayoutPlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan,
};
use crate::mir::MirModule;

pub const SOURCE_PACKED_ARRAY_DIRECT_READ_KIND_RECORD_FIELD_V0: &str =
    "source_packed_record_field_direct_read_v0";

pub fn refresh_module_source_packed_array_direct_read_consumption_plans(module: &mut MirModule) {
    module
        .metadata
        .source_packed_array_direct_read_consumption_plans =
        build_source_packed_array_direct_read_consumption_plans(module);
}

pub fn build_source_packed_array_direct_read_consumption_plans(
    module: &MirModule,
) -> Vec<SourcePackedArrayDirectReadConsumptionPlan> {
    let mut out = Vec::new();
    for source in &module.metadata.source_packed_array_autouse_pilot_plans {
        let Some(layout) = module
            .metadata
            .record_layout_plans
            .iter()
            .find(|layout| {
                layout.layout_id == source.layout_id && layout.record_name == source.record_name
            })
        else {
            continue;
        };
        out.extend(classify_source_packed_array_direct_read_consumption(
            source, layout,
        ));
    }
    out
}

pub fn classify_source_packed_array_direct_read_consumption(
    source: &SourcePackedArrayAutoUsePilotPlan,
    layout: &RecordLayoutPlan,
) -> Vec<SourcePackedArrayDirectReadConsumptionPlan> {
    if source.record_name != layout.record_name || source.layout_id != layout.layout_id {
        return Vec::new();
    }
    if !source.source_declared_packed
        || !source.direct_indexed_field_reads_enabled
        || !source.private_runtime_storage_enabled
        || source.public_array_get_materialization_enabled
        || source.backend_lowering_enabled
        || source.boxed_fallback_enabled
    {
        return Vec::new();
    }

    layout
        .fields
        .iter()
        .map(|field| SourcePackedArrayDirectReadConsumptionPlan {
            owner_box: source.owner_box.clone(),
            source_field_name: source.field_name.clone(),
            declared_type_name: source.declared_type_name.clone(),
            record_name: source.record_name.clone(),
            layout_id: source.layout_id,
            record_field_name: field.name.clone(),
            record_field_slot: field.slot,
            storage: field.storage.as_str().to_string(),
            read_kind: SOURCE_PACKED_ARRAY_DIRECT_READ_KIND_RECORD_FIELD_V0.to_string(),
            source_declared_packed: true,
            direct_indexed_field_reads_consumed: true,
            private_runtime_storage_consumed: true,
            public_array_get_materialization_enabled: false,
            backend_lowering_enabled: false,
            boxed_fallback_enabled: false,
        })
        .collect()
}

