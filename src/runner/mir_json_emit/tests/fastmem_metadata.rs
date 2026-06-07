use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::fastmem_access_plan::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemFieldAccessMode, FastMemLocalFreeListPlan, FastMemResolvedFieldPlan,
    FastMemTableAccessPlan, FastMemTableAccessProof, FastMemTableFieldAccessLink,
};
use crate::mir::function::{FastMemTableLengthFact, FastMemTableLengthPolicyKind};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, MirModule, ValueId};

#[test]
fn build_mir_json_root_emits_fastmem_table_length_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.fastmem/0", false);
    function
        .metadata
        .fastmem_table_length_facts
        .push(FastMemTableLengthFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            table_id: "page_table".to_string(),
            table_value: ValueId::new(1),
            length_value: ValueId::new(50),
            resolved_length: Some(64),
            policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
        });
    module
        .functions
        .insert("Main.fastmem/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["fastmem_table_length_facts"]
        .as_array()
        .expect("metadata.fastmem_table_length_facts array");

    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["fact_id"], 0);
    assert_eq!(facts[0]["region"], 0);
    assert_eq!(facts[0]["table_id"], "page_table");
    assert_eq!(facts[0]["table_value"], 1);
    assert_eq!(facts[0]["length_value"], 50);
    assert_eq!(facts[0]["resolved_length"], 64);
    assert_eq!(facts[0]["policy"], "explicit_const_len");
}

#[test]
fn build_mir_json_root_emits_fastmem_range_bounds_proof() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.fastmem/0", false);
    function
        .metadata
        .fastmem_access_plans
        .push(FastMemAccessPlan {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            region: FastMemRegionId::new(0),
            kind: FastMemAccessPlanKind::TableIndex,
            status: FastMemAccessPlanStatus::Rejected,
            failure_reason: Some("verified-table-access-proof-incomplete".to_string()),
            payload: FastMemAccessPlanPayload::Table(FastMemTableAccessPlan {
                table_id: "page_table".to_string(),
                table: ValueId::new(1),
                index: ValueId::new(2),
                result: Some(ValueId::new(10)),
                element_layout_id: Some("PageMetaLayoutV0".to_string()),
                element_repr: Some("pointer_to_element".to_string()),
                element_stride: Some(8),
                element_size: Some(56),
                length: Some(64),
                alignment: Some(8),
                index_policy: Some("explicit_check".to_string()),
                proof: FastMemTableAccessProof {
                    table_length_resolved: true,
                    bounds_proof_valid: true,
                    stride_resolved: true,
                    field_offset_resolved: true,
                    overflow_proof_valid: false,
                    alignment_valid: true,
                    element_layout_verified: true,
                    table_length_policy: Some("explicit_const_len".to_string()),
                    bounds_proof: Some("range_fact:7".to_string()),
                    overflow_proof: None,
                    failure_reason: None,
                },
            }),
        });
    function
        .metadata
        .fastmem_table_field_access_links
        .push(FastMemTableFieldAccessLink {
            table_block: BasicBlockId::new(0),
            table_instruction_index: 0,
            field_block: BasicBlockId::new(0),
            field_instruction_index: 1,
            region: FastMemRegionId::new(0),
            table_result: ValueId::new(10),
            field_base: ValueId::new(10),
            field_id: "capacity".to_string(),
            field_access: FastMemFieldAccessMode::Load,
            byte_offset: 40,
            field_size: 8,
            field_type: "usize".to_string(),
            alignment: 8,
            proof: "table_field_link:0:1".to_string(),
        });
    module
        .functions
        .insert("Main.fastmem/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["fastmem_access_plans"]
        .as_array()
        .expect("metadata.fastmem_access_plans array");

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["kind"], "table_index");
    assert_eq!(plans[0]["table_length_resolved"], true);
    assert_eq!(plans[0]["bounds_proof_valid"], true);
    assert_eq!(plans[0]["field_offset_resolved"], true);
    assert_eq!(plans[0]["element_size"], 56);
    assert_eq!(plans[0]["bounds_proof"], "range_fact:7");
    assert_eq!(plans[0]["overflow_proof_valid"], false);
    assert_eq!(
        plans[0]["failure_reason"],
        "verified-table-access-proof-incomplete"
    );
    let links = root["functions"][0]["metadata"]["fastmem_table_field_access_links"]
        .as_array()
        .expect("metadata.fastmem_table_field_access_links array");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0]["table_instruction_index"], 0);
    assert_eq!(links[0]["field_instruction_index"], 1);
    assert_eq!(links[0]["table_result"], 10);
    assert_eq!(links[0]["field_base"], 10);
    assert_eq!(links[0]["field_id"], "capacity");
    assert_eq!(links[0]["field_access"], "load");
    assert_eq!(links[0]["byte_offset"], 40);
    assert_eq!(links[0]["field_size"], 8);
    assert_eq!(links[0]["proof"], "table_field_link:0:1");
}

#[test]
fn build_mir_json_root_preserves_fastmem_resolved_field_legacy_keys() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.fastmem/0", false);
    function
        .metadata
        .fastmem_access_plans
        .push(FastMemAccessPlan {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            region: FastMemRegionId::new(0),
            kind: FastMemAccessPlanKind::LocalFreePush,
            status: FastMemAccessPlanStatus::Verified,
            failure_reason: None,
            payload: FastMemAccessPlanPayload::LocalFree(FastMemLocalFreeListPlan {
                page: ValueId::new(1),
                block: Some(ValueId::new(2)),
                result: None,
                local_free_head: FastMemResolvedFieldPlan {
                    layout_id: Some("PageMetaLayoutV0".to_string()),
                    field_id: Some("local_free_head".to_string()),
                    field_class: Some("local_free_head".to_string()),
                    byte_offset: Some(24),
                    field_size: Some(8),
                    field_type: Some("usize".to_string()),
                    alignment: Some(8),
                },
                block_next: FastMemResolvedFieldPlan {
                    layout_id: Some("FreeBlockNodeLayoutV0".to_string()),
                    field_id: Some("next".to_string()),
                    field_class: Some("plain_pointer".to_string()),
                    byte_offset: Some(0),
                    field_size: Some(8),
                    field_type: Some("usize".to_string()),
                    alignment: Some(8),
                },
                same_owner_proof_valid: true,
                block_next_proof_valid: true,
                non_empty_proof_valid: false,
                remote_owner_rejected: true,
                lowerable: true,
            }),
        });
    module
        .functions
        .insert("Main.fastmem/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["fastmem_access_plans"]
        .as_array()
        .expect("metadata.fastmem_access_plans array");

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["kind"], "local_free_push");
    assert_eq!(plans[0]["local_free_head_layout_id"], "PageMetaLayoutV0");
    assert_eq!(plans[0]["local_free_head_field_id"], "local_free_head");
    assert_eq!(plans[0]["local_free_head_field_class"], "local_free_head");
    assert_eq!(plans[0]["local_free_head_byte_offset"], 24);
    assert_eq!(plans[0]["local_free_head_field_size"], 8);
    assert_eq!(plans[0]["local_free_head_field_type"], "usize");
    assert_eq!(plans[0]["local_free_head_alignment"], 8);
    assert_eq!(plans[0]["block_next_layout_id"], "FreeBlockNodeLayoutV0");
    assert_eq!(plans[0]["block_next_field_id"], "next");
    assert_eq!(plans[0]["block_next_field_class"], "plain_pointer");
    assert_eq!(plans[0]["block_next_byte_offset"], 0);
    assert_eq!(plans[0]["block_next_field_size"], 8);
    assert_eq!(plans[0]["block_next_field_type"], "usize");
    assert_eq!(plans[0]["block_next_alignment"], 8);
}
