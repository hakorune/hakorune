/*!
 * Metadata-only record-state residence candidates.
 *
 * This module does not rewrite box storage, create source records, or enable
 * backend lowering. It reports box-private primitive state fields that are
 * narrow enough for a later RecordStateResidencePlanV0 slice.
 */

use crate::mir::declared_type_storage::storage_for_declared_type;
use crate::mir::function::{
    ModuleMetadata, RecordStateFieldAccessPlan, RecordStateResidenceFieldPlan,
    RecordStateResidencePlan, RecordStateResidenceRejectedFieldPlan, TypedObjectFieldStorage,
};
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, UserBoxFieldDecl};
use std::collections::BTreeMap;

pub const RECORD_STATE_RESIDENCE_V0: &str = "box_private_record_state_v0";

pub fn refresh_function_record_state_field_access_plans(
    function: &mut MirFunction,
    module_metadata: &ModuleMetadata,
) {
    function.metadata.record_state_field_access_plans =
        build_record_state_field_access_plans(function, module_metadata);
}

pub fn refresh_module_record_state_residence_plans(module: &mut MirModule) {
    module.metadata.record_state_residence_plans = build_record_state_residence_plans(module);
}

pub fn build_record_state_field_access_plans(
    function: &MirFunction,
    module_metadata: &ModuleMetadata,
) -> Vec<RecordStateFieldAccessPlan> {
    let candidate_fields = record_state_candidate_fields(module_metadata);
    if candidate_fields.is_empty() {
        return Vec::new();
    }

    let mut plans = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            match instruction {
                MirInstruction::FieldGet {
                    dst,
                    field,
                    declared_type,
                    ..
                } => {
                    let Some((owner_box, candidate_record, storage)) =
                        candidate_fields.get(field).cloned()
                    else {
                        continue;
                    };
                    if declared_type_supported(declared_type, storage) {
                        plans.push(RecordStateFieldAccessPlan {
                            block: block_id,
                            instruction_index,
                            owner_box,
                            candidate_record,
                            field_name: field.clone(),
                            op: "load".to_string(),
                            value: None,
                            result: Some(*dst),
                            route: format!("record_state_{}_load", storage.as_str()),
                            storage,
                            proof_ids: vec!["record_state_residence_plan".to_string()],
                            lowering_enabled: false,
                            fallback_policy: "report_only".to_string(),
                            summary: "ok".to_string(),
                        });
                    }
                }
                MirInstruction::FieldSet {
                    field,
                    value,
                    declared_type,
                    ..
                } => {
                    let Some((owner_box, candidate_record, storage)) =
                        candidate_fields.get(field).cloned()
                    else {
                        continue;
                    };
                    if declared_type_supported(declared_type, storage) {
                        plans.push(RecordStateFieldAccessPlan {
                            block: block_id,
                            instruction_index,
                            owner_box,
                            candidate_record,
                            field_name: field.clone(),
                            op: "store".to_string(),
                            value: Some(*value),
                            result: None,
                            route: format!("record_state_{}_store", storage.as_str()),
                            storage,
                            proof_ids: vec!["record_state_residence_plan".to_string()],
                            lowering_enabled: false,
                            fallback_policy: "report_only".to_string(),
                            summary: "ok".to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
    }
    plans
}

pub fn build_record_state_residence_plans(module: &MirModule) -> Vec<RecordStateResidencePlan> {
    let mut names: Vec<_> = module
        .metadata
        .user_box_field_decls
        .keys()
        .cloned()
        .collect();
    names.sort();

    names
        .into_iter()
        .filter_map(|box_name| {
            let fields = module.metadata.user_box_field_decls.get(&box_name)?;
            build_record_state_residence_plan(module, box_name, fields)
        })
        .collect()
}

fn record_state_candidate_fields(
    module_metadata: &ModuleMetadata,
) -> BTreeMap<String, (String, String, TypedObjectFieldStorage)> {
    let mut fields = BTreeMap::new();
    let mut duplicate_names = std::collections::BTreeSet::new();
    for plan in &module_metadata.record_state_residence_plans {
        if !plan.report_only || plan.source_migration_allowed {
            continue;
        }
        for field in &plan.fields {
            if fields.contains_key(&field.name) {
                duplicate_names.insert(field.name.clone());
            } else {
                fields.insert(
                    field.name.clone(),
                    (
                        plan.owner_box.clone(),
                        plan.candidate_record.clone(),
                        field.storage,
                    ),
                );
            }
        }
    }
    for name in duplicate_names {
        fields.remove(&name);
    }
    fields
}

fn declared_type_supported(
    declared_type: &Option<MirType>,
    storage: TypedObjectFieldStorage,
) -> bool {
    match declared_type {
        Some(MirType::Integer) => storage.uses_integer_lane(),
        Some(_) => false,
        None => true,
    }
}

fn build_record_state_residence_plan(
    module: &MirModule,
    owner_box: String,
    decls: &[UserBoxFieldDecl],
) -> Option<RecordStateResidencePlan> {
    let mut fields = Vec::new();
    let mut rejected_fields = Vec::new();

    for (slot, decl) in decls.iter().enumerate() {
        let bucket = field_bucket(&decl.name);
        let storage =
            storage_for_declared_type(&module.metadata, decl.declared_type_name.as_deref());
        if bucket == "primitive_hot_state"
            && !decl.is_weak
            && storage.is_some_and(record_state_storage_supported)
        {
            fields.push(RecordStateResidenceFieldPlan {
                name: decl.name.clone(),
                slot: slot as u32,
                declared_type_name: decl.declared_type_name.clone(),
                storage: storage.expect("checked above"),
                bucket: bucket.to_string(),
            });
        } else if bucket != "unknown" {
            rejected_fields.push(RecordStateResidenceRejectedFieldPlan {
                name: decl.name.clone(),
                slot: slot as u32,
                declared_type_name: decl.declared_type_name.clone(),
                reason: bucket.to_string(),
            });
        }
    }

    if fields.is_empty() {
        return None;
    }

    Some(RecordStateResidencePlan {
        candidate_record: candidate_record_for_owner(&owner_box),
        owner_box,
        residence: RECORD_STATE_RESIDENCE_V0.to_string(),
        field_decl_authority: true,
        report_only: true,
        source_migration_allowed: false,
        selected_field_count: fields.len() as u32,
        rejected_field_count: rejected_fields.len() as u32,
        fields,
        rejected_fields,
        summary: "ok".to_string(),
    })
}

fn record_state_storage_supported(storage: TypedObjectFieldStorage) -> bool {
    matches!(
        storage,
        TypedObjectFieldStorage::I64
            | TypedObjectFieldStorage::ISize
            | TypedObjectFieldStorage::U64
            | TypedObjectFieldStorage::USize
    )
}

fn candidate_record_for_owner(owner_box: &str) -> String {
    match owner_box {
        "HakoAllocPageModel" => "PageState".to_string(),
        _ => format!("{owner_box}State"),
    }
}

fn field_bucket(name: &str) -> &'static str {
    match name {
        "used" | "free_top" | "local_free_top" | "retired" | "decommitted" | "peak_used" => {
            "primitive_hot_state"
        }
        "page_id" | "block_size" | "capacity" | "reserved" => "public_semantics",
        "requested_bytes" => "public_semantics_proof_evidence",
        "alloc_count"
        | "local_free_count"
        | "release_count"
        | "reject_count"
        | "retire_count"
        | "reactivate_count"
        | "local_free_collect_count"
        | "local_free_collected_blocks"
        | "decommit_count"
        | "recommit_count"
        | "reuse_count"
        | "lifecycle_reject_count"
        | "reactivate_reject_count" => "observer_counter",
        "free" | "local_free" | "block_used" => "direct_array_owner",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field(name: &str, ty: &str) -> UserBoxFieldDecl {
        UserBoxFieldDecl {
            name: name.to_string(),
            declared_type_name: Some(ty.to_string()),
            is_weak: false,
        }
    }

    #[test]
    fn reports_page_model_primitive_state_candidates_without_source_migration() {
        let mut module = MirModule::new("record-state-test".to_string());
        module.metadata.user_box_field_decls.insert(
            "HakoAllocPageModel".to_string(),
            vec![
                field("used", "i64"),
                field("free_top", "i64"),
                field("requested_bytes", "usize"),
                field("free", "DirectArrayI64"),
            ],
        );

        refresh_module_record_state_residence_plans(&mut module);

        assert_eq!(module.metadata.record_state_residence_plans.len(), 1);
        let plan = &module.metadata.record_state_residence_plans[0];
        assert_eq!(plan.owner_box, "HakoAllocPageModel");
        assert_eq!(plan.candidate_record, "PageState");
        assert!(plan.report_only);
        assert!(!plan.source_migration_allowed);
        assert_eq!(plan.selected_field_count, 2);
        assert_eq!(plan.rejected_field_count, 2);
        assert_eq!(plan.fields[0].name, "used");
        assert_eq!(plan.fields[1].name, "free_top");
        assert_eq!(
            plan.rejected_fields[0].reason,
            "public_semantics_proof_evidence"
        );
    }
}
