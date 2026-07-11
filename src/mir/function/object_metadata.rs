pub use hakorune_mir_plans::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan,
    DirectStateFieldPlan, DirectStatePlan, HakoAllocAlignedSmallPackedStorePilotPlan,
    HakoAllocHugePagePackedStorePilotPlan, RecordLayoutFieldPlan, RecordLayoutPlan,
    RecordStateFieldAccessPlan, RecordStateResidenceFieldPlan, RecordStateResidencePlan,
    RecordStateResidenceRejectedFieldPlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, TypedObjectFieldPlan, TypedObjectFieldStorage,
    TypedObjectPlan,
};

/// Typed field declaration metadata carried alongside names-only user box decls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxFieldDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
    pub is_weak: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeakFieldId {
    pub box_schema_fingerprint: String,
    pub field_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeakFieldContractSpec {
    pub contract_id: String,
    pub weak_field_id: WeakFieldId,
    pub diagnostic_box_name: String,
    pub diagnostic_field_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeakFieldWriteContract {
    pub site_id: crate::mir::WeakFieldWriteSiteId,
    pub contract_id: String,
    pub base_value_id: crate::mir::ValueId,
    pub value_id: crate::mir::ValueId,
    pub box_schema_fingerprint: String,
    pub field_index: u32,
    pub runtime_check_required: bool,
    pub proof_elision_allowed: bool,
    pub backend_capability_required: String,
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
    /// Field names with source-owned defaults. Values remain builder-local AST.
    pub default_field_names: Vec<String>,
}
