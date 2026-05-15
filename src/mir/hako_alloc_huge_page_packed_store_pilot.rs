/*!
 * hako_alloc huge-page metadata packed-store pilot.
 *
 * This C211 owner consumes the C209 packed ArrayBox pilot only for the
 * `HakoAllocHugePageMeta` shape. It records that huge-page metadata can use the
 * private packed i64-column seam while preserving live-flag and released
 * sentinel contracts.
 */

use crate::mir::array_record_packed_autouse_pilot::ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0;
use crate::mir::function::{
    ArrayRecordPackedAutoUsePilotPlan, HakoAllocHugePagePackedStorePilotPlan, RecordLayoutPlan,
};
use crate::mir::MirModule;

pub const HAKO_ALLOC_HUGE_PAGE_META_RECORD: &str = "HakoAllocHugePageMeta";
pub const HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER: &str = "HakoAllocHugePageMetaStore";
pub const HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND: &str = "huge_page_metadata_i64_columns_v0";
pub const HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL: i64 = -1;
pub const HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL: i64 = 0;

const PAGE_ID_FIELD: &str = "page_id";
const PTR_FIELD: &str = "ptr";
const REQUESTED_SIZE_FIELD: &str = "requested_size";
const COMMITTED_SIZE_FIELD: &str = "committed_size";
const LIVE_FIELD: &str = "live";

pub fn refresh_module_hako_alloc_huge_page_packed_store_pilot_plans(module: &mut MirModule) {
    module
        .metadata
        .hako_alloc_huge_page_packed_store_pilot_plans =
        build_hako_alloc_huge_page_packed_store_pilot_plans(module);
}

pub fn build_hako_alloc_huge_page_packed_store_pilot_plans(
    module: &MirModule,
) -> Vec<HakoAllocHugePagePackedStorePilotPlan> {
    module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .iter()
        .filter_map(|pilot| {
            let layout = module.metadata.record_layout_plans.iter().find(|layout| {
                layout.layout_id == pilot.layout_id && layout.record_name == pilot.record_name
            })?;
            classify_hako_alloc_huge_page_packed_store_pilot(pilot, layout)
        })
        .collect()
}

pub fn classify_hako_alloc_huge_page_packed_store_pilot(
    pilot: &ArrayRecordPackedAutoUsePilotPlan,
    layout: &RecordLayoutPlan,
) -> Option<HakoAllocHugePagePackedStorePilotPlan> {
    if pilot.record_name != HAKO_ALLOC_HUGE_PAGE_META_RECORD {
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

    let columns = huge_page_field_columns(layout)?;

    Some(HakoAllocHugePagePackedStorePilotPlan {
        record_name: pilot.record_name.clone(),
        store_owner: HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER.to_string(),
        layout_id: pilot.layout_id,
        pilot_kind: HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND.to_string(),
        page_id_column: columns.page_id,
        ptr_column: columns.ptr,
        requested_size_column: columns.requested_size,
        committed_size_column: columns.committed_size,
        live_column: columns.live,
        released_page_id_sentinel: HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL,
        released_size_sentinel: HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL,
        private_runtime_storage_enabled: true,
        hako_alloc_source_mentions_compiler: false,
        live_scalar_columns_retained: true,
        public_array_get_materialization_enabled: false,
        backend_lowering_enabled: false,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HugePageFieldColumns {
    page_id: u32,
    ptr: u32,
    requested_size: u32,
    committed_size: u32,
    live: u32,
}

fn huge_page_field_columns(layout: &RecordLayoutPlan) -> Option<HugePageFieldColumns> {
    if layout.record_name != HAKO_ALLOC_HUGE_PAGE_META_RECORD || layout.field_count != 5 {
        return None;
    }

    let columns = HugePageFieldColumns {
        page_id: field_column(layout, PAGE_ID_FIELD)?,
        ptr: field_column(layout, PTR_FIELD)?,
        requested_size: field_column(layout, REQUESTED_SIZE_FIELD)?,
        committed_size: field_column(layout, COMMITTED_SIZE_FIELD)?,
        live: field_column(layout, LIVE_FIELD)?,
    };

    if columns
        != (HugePageFieldColumns {
            page_id: 0,
            ptr: 1,
            requested_size: 2,
            committed_size: 3,
            live: 4,
        })
    {
        return None;
    }

    Some(columns)
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
    use crate::mir::function::{
        ArrayRecordPackedAutoUsePilotPlan, RecordLayoutFieldPlan, RecordLayoutPlan,
        TypedObjectFieldStorage,
    };
    use crate::mir::MirModule;

    fn packed_pilot(record_name: &str) -> ArrayRecordPackedAutoUsePilotPlan {
        ArrayRecordPackedAutoUsePilotPlan {
            record_name: record_name.to_string(),
            layout_id: 29,
            pilot_kind: ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0
                .to_string(),
            source_boundary_kind: "non_escaping_direct_field_reads_v0".to_string(),
            integer_lane_columns: 5,
            direct_indexed_field_reads_enabled: true,
            private_runtime_storage_enabled: true,
            public_array_get_materialization_enabled: false,
            hako_alloc_migration_enabled: false,
            backend_lowering_enabled: false,
        }
    }

    fn huge_layout() -> RecordLayoutPlan {
        RecordLayoutPlan {
            record_name: HAKO_ALLOC_HUGE_PAGE_META_RECORD.to_string(),
            layout_id: 29,
            layout_kind: "record_value_aggregate_v0".to_string(),
            field_count: 5,
            fields: vec![
                RecordLayoutFieldPlan {
                    name: PAGE_ID_FIELD.to_string(),
                    slot: 0,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: PTR_FIELD.to_string(),
                    slot: 1,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: REQUESTED_SIZE_FIELD.to_string(),
                    slot: 2,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: COMMITTED_SIZE_FIELD.to_string(),
                    slot: 3,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: LIVE_FIELD.to_string(),
                    slot: 4,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
            ],
        }
    }

    #[test]
    fn huge_page_packed_store_pilot_consumes_c209_row() {
        let mut module = MirModule::new("huge-page-packed-store-pilot-test".to_string());
        module
            .metadata
            .array_record_packed_autouse_pilot_plans
            .push(packed_pilot(HAKO_ALLOC_HUGE_PAGE_META_RECORD));
        module.metadata.record_layout_plans.push(huge_layout());

        let plans = build_hako_alloc_huge_page_packed_store_pilot_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].record_name, HAKO_ALLOC_HUGE_PAGE_META_RECORD);
        assert_eq!(plans[0].store_owner, HAKO_ALLOC_HUGE_PAGE_META_STORE_OWNER);
        assert_eq!(
            plans[0].pilot_kind,
            HAKO_ALLOC_HUGE_PAGE_PACKED_STORE_PILOT_KIND
        );
        assert_eq!(plans[0].page_id_column, 0);
        assert_eq!(plans[0].ptr_column, 1);
        assert_eq!(plans[0].requested_size_column, 2);
        assert_eq!(plans[0].committed_size_column, 3);
        assert_eq!(plans[0].live_column, 4);
        assert_eq!(
            plans[0].released_page_id_sentinel,
            HAKO_ALLOC_HUGE_PAGE_RELEASED_PAGE_ID_SENTINEL
        );
        assert_eq!(
            plans[0].released_size_sentinel,
            HAKO_ALLOC_HUGE_PAGE_RELEASED_SIZE_SENTINEL
        );
        assert!(plans[0].private_runtime_storage_enabled);
        assert!(!plans[0].hako_alloc_source_mentions_compiler);
        assert!(plans[0].live_scalar_columns_retained);
        assert!(!plans[0].public_array_get_materialization_enabled);
        assert!(!plans[0].backend_lowering_enabled);
    }

    #[test]
    fn huge_page_packed_store_pilot_rejects_other_records() {
        let plan = classify_hako_alloc_huge_page_packed_store_pilot(
            &packed_pilot("OtherMeta"),
            &huge_layout(),
        );

        assert!(plan.is_none());
    }

    #[test]
    fn huge_page_packed_store_pilot_requires_integer_lane_fields() {
        let pilot = packed_pilot(HAKO_ALLOC_HUGE_PAGE_META_RECORD);
        let mut layout = huge_layout();
        layout.fields[4].storage = TypedObjectFieldStorage::Handle;

        let plan = classify_hako_alloc_huge_page_packed_store_pilot(&pilot, &layout);

        assert!(plan.is_none());
    }

    #[test]
    fn huge_page_packed_store_pilot_requires_fixed_column_order() {
        let pilot = packed_pilot(HAKO_ALLOC_HUGE_PAGE_META_RECORD);
        let mut layout = huge_layout();
        layout.fields[0].slot = 1;
        layout.fields[1].slot = 0;

        let plan = classify_hako_alloc_huge_page_packed_store_pilot(&pilot, &layout);

        assert!(plan.is_none());
    }
}
