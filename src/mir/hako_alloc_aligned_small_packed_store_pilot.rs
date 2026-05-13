/*!
 * hako_alloc aligned-small metadata packed-store pilot.
 *
 * This C210 owner consumes the C209 packed ArrayBox pilot only for the
 * `HakoAllocAlignedSmallMeta` shape. It records that aligned-small metadata can
 * use the private packed i64-column seam, while `.hako` source storage, public
 * record materialization, huge metadata, and backend lowering stay closed.
 */

use crate::mir::array_record_packed_autouse_pilot::ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0;
use crate::mir::function::{
    ArrayRecordPackedAutoUsePilotPlan, HakoAllocAlignedSmallPackedStorePilotPlan, RecordLayoutPlan,
};
use crate::mir::MirModule;

pub const HAKO_ALLOC_ALIGNED_SMALL_META_RECORD: &str = "HakoAllocAlignedSmallMeta";
pub const HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER: &str = "HakoAllocAlignedSmallMetaStore";
pub const HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND: &str =
    "aligned_small_metadata_i64_columns_v0";

const PTR_FIELD: &str = "ptr";
const ALIGNMENT_FIELD: &str = "alignment";
const PADDED_SIZE_FIELD: &str = "padded_size";

pub fn refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans(module: &mut MirModule) {
    module
        .metadata
        .hako_alloc_aligned_small_packed_store_pilot_plans =
        build_hako_alloc_aligned_small_packed_store_pilot_plans(module);
}

pub fn build_hako_alloc_aligned_small_packed_store_pilot_plans(
    module: &MirModule,
) -> Vec<HakoAllocAlignedSmallPackedStorePilotPlan> {
    module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .iter()
        .filter_map(|pilot| {
            let layout = module.metadata.record_layout_plans.iter().find(|layout| {
                layout.layout_id == pilot.layout_id && layout.record_name == pilot.record_name
            })?;
            classify_hako_alloc_aligned_small_packed_store_pilot(pilot, layout)
        })
        .collect()
}

pub fn classify_hako_alloc_aligned_small_packed_store_pilot(
    pilot: &ArrayRecordPackedAutoUsePilotPlan,
    layout: &RecordLayoutPlan,
) -> Option<HakoAllocAlignedSmallPackedStorePilotPlan> {
    if pilot.record_name != HAKO_ALLOC_ALIGNED_SMALL_META_RECORD {
        return None;
    }
    if pilot.pilot_kind != ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0 {
        return None;
    }
    if !pilot.private_runtime_storage_enabled
        || pilot.public_array_get_materialization_enabled
        || pilot.hako_alloc_migration_enabled
        || pilot.backend_lowering_enabled
    {
        return None;
    }

    let (ptr_column, alignment_column, padded_size_column) = aligned_small_field_columns(layout)?;

    Some(HakoAllocAlignedSmallPackedStorePilotPlan {
        record_name: pilot.record_name.clone(),
        store_owner: HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER.to_string(),
        layout_id: pilot.layout_id,
        pilot_kind: HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND.to_string(),
        ptr_column,
        alignment_column,
        padded_size_column,
        private_runtime_storage_enabled: true,
        hako_alloc_source_mentions_compiler: false,
        live_scalar_columns_retained: true,
        public_array_get_materialization_enabled: false,
        backend_lowering_enabled: false,
    })
}

fn aligned_small_field_columns(layout: &RecordLayoutPlan) -> Option<(u32, u32, u32)> {
    if layout.record_name != HAKO_ALLOC_ALIGNED_SMALL_META_RECORD || layout.field_count != 3 {
        return None;
    }

    let ptr = field_column(layout, PTR_FIELD)?;
    let alignment = field_column(layout, ALIGNMENT_FIELD)?;
    let padded_size = field_column(layout, PADDED_SIZE_FIELD)?;
    if (ptr, alignment, padded_size) != (0, 1, 2) {
        return None;
    }
    Some((ptr, alignment, padded_size))
}

fn field_column(layout: &RecordLayoutPlan, field_name: &str) -> Option<u32> {
    let field = layout
        .fields
        .iter()
        .find(|field| field.name == field_name)?;
    if field.storage.uses_integer_lane() {
        Some(field.slot)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::array_record_packed_autouse_pilot::ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0;
    use crate::mir::{
        ArrayRecordPackedAutoUsePilotPlan, MirModule, RecordLayoutFieldPlan, RecordLayoutPlan,
        TypedObjectFieldStorage,
    };

    fn packed_pilot(record_name: &str) -> ArrayRecordPackedAutoUsePilotPlan {
        ArrayRecordPackedAutoUsePilotPlan {
            record_name: record_name.to_string(),
            layout_id: 17,
            pilot_kind: ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0
                .to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            integer_lane_columns: 3,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            hako_alloc_migration_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    fn aligned_layout() -> RecordLayoutPlan {
        RecordLayoutPlan {
            record_name: HAKO_ALLOC_ALIGNED_SMALL_META_RECORD.to_string(),
            layout_id: 17,
            layout_kind: "record_value_aggregate_v0".to_string(),
            field_count: 3,
            fields: vec![
                RecordLayoutFieldPlan {
                    name: PTR_FIELD.to_string(),
                    slot: 0,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: ALIGNMENT_FIELD.to_string(),
                    slot: 1,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: PADDED_SIZE_FIELD.to_string(),
                    slot: 2,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
            ],
        }
    }

    #[test]
    fn aligned_small_packed_store_pilot_consumes_c209_row() {
        let mut module = MirModule::new("aligned-small-packed-store-pilot-test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(packed_pilot(HAKO_ALLOC_ALIGNED_SMALL_META_RECORD));
        module.metadata.record_layout_plans.push(aligned_layout());

        let plans = build_hako_alloc_aligned_small_packed_store_pilot_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, HAKO_ALLOC_ALIGNED_SMALL_META_RECORD);
        assert_eq!(
            plans[0].store_owner,
            HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER
        );
        assert_eq!(
            plans[0].pilot_kind,
            HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND
        );
        assert_eq!(plans[0].ptr_column, 0);
        assert_eq!(plans[0].alignment_column, 1);
        assert_eq!(plans[0].padded_size_column, 2);
        assert!(plans[0].private_runtime_storage_enabled);
        assert!(!plans[0].hako_alloc_source_mentions_compiler);
        assert!(plans[0].live_scalar_columns_retained);
        assert!(!plans[0].public_array_get_materialization_enabled);
        assert!(!plans[0].backend_lowering_enabled);
    }

    #[test]
    fn aligned_small_packed_store_pilot_rejects_other_records() {
        let plan = classify_hako_alloc_aligned_small_packed_store_pilot(
            &packed_pilot("OtherMeta"),
            &aligned_layout(),
        );

        assert!(plan.is_none());
    }

    #[test]
    fn aligned_small_packed_store_pilot_requires_integer_lane_fields() {
        let pilot = packed_pilot(HAKO_ALLOC_ALIGNED_SMALL_META_RECORD);
        let mut layout = aligned_layout();
        layout.fields[1].storage = TypedObjectFieldStorage::Handle;

        let plan = classify_hako_alloc_aligned_small_packed_store_pilot(&pilot, &layout);

        assert!(plan.is_none());
    }

    #[test]
    fn aligned_small_packed_store_pilot_requires_fixed_column_order() {
        let pilot = packed_pilot(HAKO_ALLOC_ALIGNED_SMALL_META_RECORD);
        let mut layout = aligned_layout();
        layout.fields[0].slot = 1;
        layout.fields[1].slot = 0;

        let plan = classify_hako_alloc_aligned_small_packed_store_pilot(&pilot, &layout);

        assert!(plan.is_none());
    }
}
