use crate::mir::{BasicBlockId, ValueId};

pub use hakorune_mir_plans::TypedObjectFieldStorage;

/// Typed field declaration metadata carried alongside names-only user box decls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxFieldDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
    pub is_weak: bool,
}

/// Declared record inventory carried separately from ordinary user boxes.
///
/// Records are identity-free aggregate contracts. Keeping them out of
/// `user_box_decls` prevents ordinary box identity semantics from becoming the
/// accidental transport for record lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordDecl {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub fields: Vec<UserBoxFieldDecl>,
}

/// Backend-readable slot layout for one field in a typed user object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedObjectFieldPlan {
    pub name: String,
    pub slot: u32,
    pub declared_type_name: Option<String>,
    pub storage: TypedObjectFieldStorage,
    pub is_weak: bool,
}

/// MIR-owned object layout truth consumed by EXE backends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedObjectPlan {
    pub box_name: String,
    pub type_id: u32,
    pub layout_kind: String,
    pub field_count: u32,
    pub fields: Vec<TypedObjectFieldPlan>,
}

/// Metadata-only direct-state candidate for a user box.
///
/// This is deliberately not a backend layout contract. It records the
/// field-declaration authority and primitive-field surface that a later guard
/// may select for NativeDirect lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectStateFieldPlan {
    pub name: String,
    pub slot: u32,
    pub declared_type_name: Option<String>,
    pub storage: TypedObjectFieldStorage,
}

/// MIR-owned candidate row for direct mutable state representation.
///
/// `state_repr` is the stable report key (`direct_v0`), while implementation
/// details such as object layout and lowering stay closed until a later guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectStatePlan {
    pub box_name: String,
    pub state_repr: String,
    pub field_decl_authority: bool,
    pub selected_field_count: u32,
    pub unsupported_field_count: u32,
    pub materialization_boundary_known: bool,
    pub positive_net_expected: bool,
    pub fields: Vec<DirectStateFieldPlan>,
}

/// Metadata-only box-private record residence candidate field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStateResidenceFieldPlan {
    pub name: String,
    pub slot: u32,
    pub declared_type_name: Option<String>,
    pub storage: TypedObjectFieldStorage,
    pub bucket: String,
}

/// Metadata-only rejected field for record-state residence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStateResidenceRejectedFieldPlan {
    pub name: String,
    pub slot: u32,
    pub declared_type_name: Option<String>,
    pub reason: String,
}

/// MIR-owned report-only candidate for box-private primitive state residence.
///
/// This does not create a source record, rewrite storage, enable lowering, or
/// authorize whole-record ABI. It only exposes which owner-box fields are
/// narrow enough for a later `RecordStateResidencePlanV0` producer/lowering
/// slice to consume.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStateResidencePlan {
    pub owner_box: String,
    pub candidate_record: String,
    pub residence: String,
    pub field_decl_authority: bool,
    pub report_only: bool,
    pub source_migration_allowed: bool,
    pub selected_field_count: u32,
    pub rejected_field_count: u32,
    pub fields: Vec<RecordStateResidenceFieldPlan>,
    pub rejected_fields: Vec<RecordStateResidenceRejectedFieldPlan>,
    pub summary: String,
}

/// Metadata-only access-site view for a record-state residence candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStateFieldAccessPlan {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub owner_box: String,
    pub candidate_record: String,
    pub field_name: String,
    pub op: String,
    pub value: Option<ValueId>,
    pub result: Option<ValueId>,
    pub route: String,
    pub storage: TypedObjectFieldStorage,
    pub proof_ids: Vec<String>,
    pub lowering_enabled: bool,
    pub fallback_policy: String,
    pub summary: String,
}

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
/// This is intentionally separate from `TypedObjectPlan`: records have value
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
/// This consumes C207 eligibility and C208 boundary rows, but only opens the
/// private pilot shape: integer-lane inline-record storage plus direct indexed
/// field reads. Public record materialization, hako_alloc migration, and
/// backend lowering stay closed.
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
/// `field: PackedArray<Meta>` to an already-proven C209 packed ArrayBox pilot
/// row. It still does not rewrite storage, materialize records, enable backend
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
/// This is the PACKED-003 consumer-facing plan derived from explicit source
/// PackedArray declarations plus record layout facts. It remains metadata-only:
/// runtime/backend lowering and public record materialization stay disabled.
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
/// This consumes the C209 private packed ArrayBox pilot for the
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
/// This consumes the C209 private packed ArrayBox pilot for the
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
