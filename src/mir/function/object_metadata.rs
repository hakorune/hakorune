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
