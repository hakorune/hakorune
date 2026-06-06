use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::fastmem_access_plan::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemTableAccessPlan, FastMemTableAccessProof,
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
                length: Some(64),
                alignment: Some(8),
                index_policy: Some("explicit_check".to_string()),
                proof: FastMemTableAccessProof {
                    table_length_resolved: true,
                    bounds_proof_valid: true,
                    stride_resolved: true,
                    field_offset_resolved: false,
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
    assert_eq!(plans[0]["bounds_proof"], "range_fact:7");
    assert_eq!(plans[0]["overflow_proof_valid"], false);
    assert_eq!(
        plans[0]["failure_reason"],
        "verified-table-access-proof-incomplete"
    );
}
