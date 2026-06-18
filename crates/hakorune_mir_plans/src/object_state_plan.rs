//! Typed object, direct-state, and record-state passive plan vocabulary.
//!
//! This module defines metadata rows only. It does not infer field storage,
//! scan MIR modules, choose routes, or enable lowering.

use crate::typed_field_storage::TypedObjectFieldStorage;
use hakorune_mir_core::{BasicBlockId, ValueId};

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
/// narrow enough for a later producer/lowering slice to consume.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_object_plan_keeps_field_storage_metadata() {
        let plan = TypedObjectPlan {
            box_name: "Page".to_string(),
            type_id: 7,
            layout_kind: "typed_object_v0".to_string(),
            field_count: 1,
            fields: vec![TypedObjectFieldPlan {
                name: "size".to_string(),
                slot: 0,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::USize,
                is_weak: false,
            }],
        };

        assert_eq!(plan.fields[0].storage.as_str(), "usize");
    }

    #[test]
    fn record_state_field_access_plan_stays_metadata_only() {
        let plan = RecordStateFieldAccessPlan {
            block: BasicBlockId::new(3),
            instruction_index: 9,
            owner_box: "Owner".to_string(),
            candidate_record: "OwnerState".to_string(),
            field_name: "count".to_string(),
            op: "get".to_string(),
            value: None,
            result: Some(ValueId::new(11)),
            route: "record_state_field_access_report_only".to_string(),
            storage: TypedObjectFieldStorage::I64,
            proof_ids: vec!["field_decl_authority".to_string()],
            lowering_enabled: false,
            fallback_policy: "generic_box_fallback".to_string(),
            summary: "ok".to_string(),
        };

        assert_eq!(plan.block, BasicBlockId::new(3));
        assert!(!plan.lowering_enabled);
    }
}
