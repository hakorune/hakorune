use super::parse_mir_v0_to_module;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

fn single_block_json(instructions: &str) -> String {
    format!(
        r#"{{"functions":[{{"name":"main","blocks":[{{"id":0,"instructions":[{}]}}]}}]}}"#,
        instructions
    )
}

fn ownership_json(value_types: &str, storage_classes: &str) -> String {
    format!(
        r#"{{
          "functions":[{{
            "name":"main",
            "metadata":{{"value_types":{value_types},"storage_classes":{storage_classes}}},
            "blocks":[{{"id":0,"instructions":[
              {{"op":"copy_owned","dst":2,"src":1}},
              {{"op":"destroy_owned","value":2}}
            ]}}]
          }}]
        }}"#
    )
}

#[test]
fn parse_ownership_transport_requires_exact_boxref_witness() {
    let json = ownership_json(
        r#"{"1":{"kind":"handle","box_type":"WidgetBox"},"2":{"kind":"handle","box_type":"WidgetBox"}}"#,
        r#"{"1":"box_ref","2":"box_ref"}"#,
    );
    let module = parse_mir_v0_to_module(&json).expect("exact witness must parse");
    let instructions = &module
        .get_function("main")
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .instructions;
    assert!(matches!(instructions[0], MirInstruction::CopyOwned { .. }));
    assert!(matches!(
        instructions[1],
        MirInstruction::DestroyOwned { .. }
    ));
}

#[test]
fn parse_ownership_transport_rejects_storage_mismatch() {
    let json = ownership_json(
        r#"{"1":{"kind":"handle","box_type":"WidgetBox"},"2":{"kind":"handle","box_type":"WidgetBox"}}"#,
        r#"{"1":"box_ref","2":"opaque"}"#,
    );
    let error = parse_mir_v0_to_module(&json).expect_err("opaque ownership dst must reject");
    assert!(error.contains("[ownership-json-witness]"));
    assert!(error.contains("storage must be box_ref"));
}

#[test]
fn parse_ownership_transport_rejects_type_mismatch() {
    let json = ownership_json(
        r#"{"1":{"kind":"handle","box_type":"WidgetBox"},"2":{"kind":"handle","box_type":"OtherBox"}}"#,
        r#"{"1":"box_ref","2":"box_ref"}"#,
    );
    let error = parse_mir_v0_to_module(&json).expect_err("different box types must reject");
    assert!(error.contains("copy_owned type mismatch"));
}

#[test]
fn parse_direct_newbox_remains_positive() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"newbox","dst":2,"type":"MapBox","args":[]},
            {"op":"ret","value":2}
          ]}
        ]}
      ]
    }"#;

    let module = parse_mir_v0_to_module(json).expect("direct newbox must remain supported");
    let instructions = &module
        .get_function("main")
        .expect("main exists")
        .get_block(BasicBlockId::new(0))
        .expect("bb0 exists")
        .instructions;
    assert!(matches!(
        &instructions[0],
        MirInstruction::NewBox { dst, box_type, args }
            if *dst == ValueId::new(2) && box_type == "MapBox" && args.is_empty()
    ));
}

#[test]
fn parse_nop_is_lowered_away() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"const","dst":1,"value":{"type":"i64","value":7}},
            {"op":"nop"},
            {"op":"ret","value":1}
          ]}
        ]}
      ]
    }"#;

    let module = parse_mir_v0_to_module(json).expect("must parse");
    let func = module.get_function("main").expect("main exists");
    let insts = &func
        .blocks
        .get(&BasicBlockId::new(0))
        .expect("bb0 exists")
        .instructions;
    assert_eq!(insts.len(), 1, "nop must be lowered away");
    assert!(matches!(
        &insts[0],
        MirInstruction::Const { dst, .. } if *dst == ValueId::new(1)
    ));
}

#[test]
fn parse_debug_log_canonicalizes_to_debug_sequence() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"const","dst":1,"value":{"type":"i64","value":7}},
            {"op":"const","dst":2,"value":{"type":"i64","value":8}},
            {"op":"debug_log","message":"probe","values":[1,2]},
            {"op":"ret","value":1}
          ]}
        ]}
      ]
    }"#;

    let module = parse_mir_v0_to_module(json).expect("must parse");
    let func = module.get_function("main").expect("main exists");
    let insts = &func
        .blocks
        .get(&BasicBlockId::new(0))
        .expect("bb0 exists")
        .instructions;

    assert!(matches!(
        &insts[2],
        MirInstruction::Debug { value, message }
            if *value == ValueId::new(1) && message == "probe[0]"
    ));
    assert!(matches!(
        &insts[3],
        MirInstruction::Debug { value, message }
            if *value == ValueId::new(2) && message == "probe[1]"
    ));
}

#[test]
fn parse_params_restores_valueid_zero_as_parameter() {
    let json = r#"{
      "functions":[
        {"name":"AddOperator.apply/2","params":[0,1],"blocks":[
          {"id":0,"instructions":[
            {"op":"copy","dst":2,"src":0},
            {"op":"ret","value":2}
          ]}
        ]}
      ]
    }"#;

    let module = parse_mir_v0_to_module(json).expect("must parse");
    let func = module
        .get_function("AddOperator.apply/2")
        .expect("function exists");

    assert_eq!(
        func.params,
        vec![ValueId::new(0), ValueId::new(1)],
        "params must preserve JSON parameter ids so src=0 is defined"
    );
    assert!(
        func.next_value_id >= 3,
        "next_value_id must be above dst/param range"
    );
}

#[test]
fn parse_params_rejects_non_contiguous_ids() {
    let json = r#"{
      "functions":[
        {"name":"main","params":[1,2],"blocks":[
          {"id":0,"instructions":[
            {"op":"ret","value":1}
          ]}
        ]}
      ]
    }"#;

    let err = parse_mir_v0_to_module(json).expect_err("must reject non-contiguous params");
    assert!(
        err.contains("params must be contiguous [0..N-1]"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_params_rejects_duplicate_ids() {
    let json = r#"{
      "functions":[
        {"name":"main","params":[0,0],"blocks":[
          {"id":0,"instructions":[
            {"op":"ret","value":0}
          ]}
        ]}
      ]
    }"#;

    let err = parse_mir_v0_to_module(json).expect_err("must reject duplicated params");
    assert!(
        err.contains("params contains duplicate"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_stops_before_legacy_carrier_construction() {
    let json = single_block_json(r#"{"op":"call","func":9,"args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("legacy call must stop");
    assert_eq!(err, "[freeze:contract][mir-json-v0/legacy-call-stopped]");
}

#[test]
fn parse_mir_call_stops_before_legacy_carrier_construction() {
    let json = single_block_json(
        r#"{"op":"mir_call","mir_call":{"callee":{"type":"Extern","name":"env.console.log"},"args":[],"effects":[]}}"#,
    );
    let err = parse_mir_v0_to_module(&json).expect_err("legacy mir_call must stop");
    assert_eq!(err, "[freeze:contract][mir-json-v0/legacy-call-stopped]");
}
