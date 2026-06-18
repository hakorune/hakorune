//! Record layout and ArrayRecord passive plan vocabulary.
//!
//! This module is passive. It stores metadata rows for record layout,
//! packed ArrayBox residence, and source PackedArray pilots. It does not scan
//! MIR modules, infer declared storage, mutate runtime ArrayBox storage, or
//! enable backend lowering.

use crate::typed_field_storage::TypedObjectFieldStorage;

/// Backend-readable slot layout for one field in an identity-free record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordLayoutFieldPlan {
    pub name: String,
    pub slot: u32,
    pub declared_type_name: Option<String>,
    pub storage: TypedObjectFieldStorage,
}

/// MIR-owned record layout truth derived from `record_decls`.
///
/// This is intentionally separate from typed object plans: records have value
/// aggregate semantics and must not acquire ordinary user-box identity by
/// sharing the user-box layout lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordLayoutPlan {
    pub record_name: String,
    pub layout_id: u32,
    pub layout_kind: String,
    pub field_count: u32,
    pub fields: Vec<RecordLayoutFieldPlan>,
}

/// Planned column for future ArrayBox inline-record residence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRecordStorageColumnPlan {
    pub name: String,
    pub column: u32,
    pub storage: TypedObjectFieldStorage,
}

/// Metadata-only packed record storage plan for ArrayBox residence.
///
/// This is a descriptor, not a runtime storage mutation. ArrayBox public
/// behavior remains unchanged until a later row installs an explicit storage
/// owner and promotion contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRecordStoragePlan {
    pub record_name: String,
    pub layout_id: u32,
    pub storage_kind: String,
    pub field_count: u32,
    pub columns: Vec<ArrayRecordStorageColumnPlan>,
}

/// Metadata-only eligibility decision for future ArrayBox inline-record auto-use.
///
/// A positive row is still not production runtime auto-use. It only means the
/// MIR-side storage facts are eligible for a later row to consume once escape
/// and backend capability gates are also proven.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRecordAutoUseEligibilityPlan {
    pub record_name: String,
    pub layout_id: u32,
    pub storage_kind: String,
    pub decision: String,
    pub reason: String,
    pub field_count: u32,
    pub integer_lane_columns: u32,
    pub required_backend_capability: Option<String>,
    pub production_auto_use_enabled: bool,
}

/// Metadata-only escape/materialization boundary for future ArrayBox auto-use.
///
/// This row lets later compiler auto-use consume only non-escaping direct field
/// read candidates. Visible record values still require explicit materializer
/// support and must fail fast while this row is the latest owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRecordMaterializationBoundaryPlan {
    pub record_name: String,
    pub layout_id: u32,
    pub boundary_kind: String,
    pub source_decision: String,
    pub direct_indexed_field_reads_allowed: bool,
    pub visible_record_materialization_enabled: bool,
    pub public_array_get_action: String,
    pub returned_element_action: String,
    pub host_backend_escape_action: String,
    pub diagnostic: String,
    pub runtime_auto_use_enabled: bool,
}

/// Metadata-only pilot plan for non-escaping packed ArrayBox auto-use.
///
/// This consumes eligibility and boundary rows, but only opens the private
/// pilot shape: integer-lane inline-record storage plus direct indexed field
/// reads. Public record materialization, hako_alloc migration, and backend
/// lowering stay closed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRecordPackedAutoUsePilotPlan {
    pub record_name: String,
    pub layout_id: u32,
    pub pilot_kind: String,
    pub source_boundary_kind: String,
    pub integer_lane_columns: u32,
    pub direct_indexed_field_reads_enabled: bool,
    pub private_runtime_storage_enabled: bool,
    pub public_array_get_materialization_enabled: bool,
    pub hako_alloc_migration_enabled: bool,
    pub backend_lowering_enabled: bool,
}

/// Metadata-only source `PackedArray<T>` auto-use pilot row.
///
/// This connects an explicit source declaration site such as
/// `field: PackedArray<Meta>` to an already-proven packed ArrayBox pilot row.
/// It still does not rewrite storage, materialize records, enable backend
/// lowering, or allow boxed fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePackedArrayAutoUsePilotPlan {
    pub owner_box: String,
    pub field_name: String,
    pub declared_type_name: String,
    pub record_name: String,
    pub layout_id: u32,
    pub pilot_kind: String,
    pub source_boundary_kind: String,
    pub source_declared_packed: bool,
    pub direct_indexed_field_reads_enabled: bool,
    pub private_runtime_storage_enabled: bool,
    pub public_array_get_materialization_enabled: bool,
    pub backend_lowering_enabled: bool,
    pub boxed_fallback_enabled: bool,
}

/// Source PackedArray<Record> direct-read consumption row.
///
/// This row remains metadata-only: runtime/backend lowering and public record
/// materialization stay disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePackedArrayDirectReadConsumptionPlan {
    pub owner_box: String,
    pub source_field_name: String,
    pub declared_type_name: String,
    pub record_name: String,
    pub layout_id: u32,
    pub record_field_name: String,
    pub record_field_slot: u32,
    pub storage: String,
    pub read_kind: String,
    pub source_declared_packed: bool,
    pub direct_indexed_field_reads_consumed: bool,
    pub private_runtime_storage_consumed: bool,
    pub public_array_get_materialization_enabled: bool,
    pub backend_lowering_enabled: bool,
    pub boxed_fallback_enabled: bool,
}

/// Metadata-only pilot plan for aligned-small hako_alloc metadata packed store.
///
/// This consumes the private packed ArrayBox pilot for the
/// `HakoAllocAlignedSmallMeta` record shape. It does not rewrite `.hako`
/// storage, materialize records, or add backend lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HakoAllocAlignedSmallPackedStorePilotPlan {
    pub record_name: String,
    pub store_owner: String,
    pub layout_id: u32,
    pub pilot_kind: String,
    pub ptr_column: u32,
    pub alignment_column: u32,
    pub padded_size_column: u32,
    pub private_runtime_storage_enabled: bool,
    pub hako_alloc_source_mentions_compiler: bool,
    pub live_scalar_columns_retained: bool,
    pub public_array_get_materialization_enabled: bool,
    pub backend_lowering_enabled: bool,
}

/// Metadata-only pilot plan for huge-page hako_alloc metadata packed store.
///
/// This consumes the private packed ArrayBox pilot for the
/// `HakoAllocHugePageMeta` record shape. It preserves the live flag and
/// released-sentinel contract without rewriting `.hako` storage or adding
/// backend lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HakoAllocHugePagePackedStorePilotPlan {
    pub record_name: String,
    pub store_owner: String,
    pub layout_id: u32,
    pub pilot_kind: String,
    pub page_id_column: u32,
    pub ptr_column: u32,
    pub requested_size_column: u32,
    pub committed_size_column: u32,
    pub live_column: u32,
    pub released_page_id_sentinel: i64,
    pub released_size_sentinel: i64,
    pub private_runtime_storage_enabled: bool,
    pub hako_alloc_source_mentions_compiler: bool,
    pub live_scalar_columns_retained: bool,
    pub public_array_get_materialization_enabled: bool,
    pub backend_lowering_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_layout_plan_keeps_identity_free_storage_metadata() {
        let plan = RecordLayoutPlan {
            record_name: "Meta".to_string(),
            layout_id: 3,
            layout_kind: "record_value_aggregate_v0".to_string(),
            field_count: 2,
            fields: vec![
                RecordLayoutFieldPlan {
                    name: "ptr".to_string(),
                    slot: 0,
                    declared_type_name: Some("i64".to_string()),
                    storage: TypedObjectFieldStorage::I64,
                },
                RecordLayoutFieldPlan {
                    name: "size".to_string(),
                    slot: 1,
                    declared_type_name: Some("usize".to_string()),
                    storage: TypedObjectFieldStorage::USize,
                },
            ],
        };

        assert_eq!(plan.record_name, "Meta");
        assert_eq!(plan.fields[0].storage.as_str(), "i64");
        assert_eq!(plan.fields[1].storage.as_str(), "usize");
    }

    #[test]
    fn array_record_storage_plan_is_metadata_only() {
        let plan = ArrayRecordStoragePlan {
            record_name: "Meta".to_string(),
            layout_id: 3,
            storage_kind: "inline_record_columns_v0".to_string(),
            field_count: 1,
            columns: vec![ArrayRecordStorageColumnPlan {
                name: "ptr".to_string(),
                column: 0,
                storage: TypedObjectFieldStorage::I64,
            }],
        };

        assert_eq!(plan.storage_kind, "inline_record_columns_v0");
        assert_eq!(plan.columns[0].column, 0);
    }
}
