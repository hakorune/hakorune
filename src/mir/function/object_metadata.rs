use crate::mir::{BasicBlockId, ValueId};

pub use hakorune_mir_plans::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan,
    HakoAllocAlignedSmallPackedStorePilotPlan, HakoAllocHugePagePackedStorePilotPlan,
    RecordLayoutFieldPlan, RecordLayoutPlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, TypedObjectFieldStorage,
};

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
