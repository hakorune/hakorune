use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
use crate::mir::fastmem_access_plan::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemTableAccessPlan, FastMemTableAccessProof,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, Callee, EffectMask, MirInstruction, MirModule, ValueId};

fn method_call(
    dst: Option<u32>,
    box_name: &str,
    method: &str,
    receiver: u32,
    args: Vec<u32>,
) -> MirInstruction {
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    MirInstruction::Call {
        dst: dst.map(ValueId::new),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: args.into_iter().map(ValueId::new).collect(),
        effects: EffectMask::PURE,
    }
}

#[test]
fn build_mir_json_root_emits_direct_array_proof_envelopes() {
    let mut function = make_function("main", true);
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![1, 3]));

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

    let mut module = MirModule::new("json_direct_array_proof_envelope_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let envelopes = root["functions"][0]["metadata"]["proof_envelopes"]
        .as_array()
        .expect("proof_envelopes");
    assert_eq!(envelopes.len(), 2);

    let first = &envelopes[0];
    assert_eq!(first["profile"], "direct_array");
    assert_eq!(first["producer"], "mir_json");
    assert_eq!(first["site"]["block"], 0);
    assert_eq!(first["site"]["instruction_index"], 0);
    assert_eq!(
        first["obligation_ids"],
        serde_json::json!(["exact_front_contract"])
    );
    assert_eq!(
        first["proof_ids"],
        serde_json::json!(["exact_front_contract"])
    );
    assert_eq!(first["verifier_flags"]["bounds_policy"], "checked");
    assert_eq!(first["verifier_flags"]["fallback_policy"], "allow_checked");
    assert_eq!(first["failure_reason"], serde_json::Value::Null);
}

#[test]
fn build_mir_json_root_emits_fastmem_proof_envelopes() {
    let mut module = MirModule::new("json_fastmem_proof_envelope_test".to_string());
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
    module
        .functions
        .insert("Main.fastmem/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let envelopes = root["functions"][0]["metadata"]["proof_envelopes"]
        .as_array()
        .expect("proof_envelopes");
    assert_eq!(envelopes.len(), 1);

    let envelope = &envelopes[0];
    assert_eq!(envelope["profile"], "fastmem");
    assert_eq!(envelope["producer"], "mir_json");
    assert_eq!(envelope["site"]["kind"], "table_index");
    assert_eq!(envelope["site"]["block"], 0);
    assert_eq!(envelope["site"]["instruction_index"], 0);
    assert_eq!(
        envelope["obligation_ids"],
        serde_json::json!(["table_index"])
    );
    assert_eq!(
        envelope["proof_ids"],
        serde_json::json!([
            "table_length_resolved",
            "bounds_proof_valid",
            "stride_resolved",
            "field_offset_resolved",
            "alignment_valid",
            "element_layout_verified"
        ])
    );
    assert_eq!(envelope["verifier_flags"]["status"], "rejected");
    assert_eq!(envelope["verifier_flags"]["verified"], false);
    assert_eq!(
        envelope["failure_reason"],
        "verified-table-access-proof-incomplete"
    );
}
