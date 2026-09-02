use super::parse_mir_v0_to_module;
use crate::mir::{BasicBlockId, Callee, Effect, MirInstruction, ValueId};

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
fn parse_call_accepts_extern_callee_without_func() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"const","dst":1,"value":{"type":"i64","value":7}},
            {"op":"call","dst":2,"callee":{"type":"Extern","name":"env.console.log"},"args":[1]},
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
        &insts[1],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Extern(name)),
            args,
            dst: Some(dst),
            ..
        } if *func == ValueId::INVALID
            && name == "env.console.log"
            && args == &vec![ValueId::new(1)]
            && *dst == ValueId::new(2)
    ));
}

#[test]
fn parse_typed_constructor_call_rejects_before_call_publication() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"call","dst":2,"callee":{"type":"Constructor","box_type":"MapBox"},"args":[]},
            {"op":"ret","value":2}
          ]}
        ]}
      ]
    }"#;

    let error = parse_mir_v0_to_module(json).expect_err("typed Constructor Call must reject");
    assert!(error.contains("[freeze:contract][mir-json-v0/constructor-call-requires-newbox]"));
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
fn parse_call_accepts_top_level_name_as_global_callee_without_func() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"call","dst":1,"name":"id","args":[]},
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
        &insts[0],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Global(name)),
            args,
            dst: Some(dst),
            ..
        } if *func == ValueId::INVALID
            && name.display_name() == "id/0"
            && args.is_empty()
            && *dst == ValueId::new(1)
    ));
}

#[test]
fn parse_call_accepts_method_callee_without_func() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"call","dst":4,"callee":{"type":"Method","box_name":"StringBox","method":"length","receiver":1},"args":[]},
            {"op":"ret","value":4}
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
        &insts[0],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
            dst: Some(dst),
            ..
        } if *func == ValueId::INVALID
            && box_name == "StringBox"
            && method == "length"
            && *receiver == ValueId::new(1)
            && *dst == ValueId::new(4)
    ));
}

#[test]
fn parse_mir_call_accepts_nested_callee_shape() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Method","box_name":"StringBox","method":"length","receiver":1},"args":[],"effects":[]}},
            {"op":"ret","value":3}
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
        &insts[0],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
            args,
            dst: Some(dst),
            ..
        } if *func == ValueId::INVALID
            && box_name == "StringBox"
            && method == "length"
            && *receiver == ValueId::new(1)
            && args.is_empty()
            && *dst == ValueId::new(3)
    ));
}

#[test]
fn parse_mir_call_parses_effect_tokens() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[
          {"id":0,"instructions":[
            {"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Extern","name":"env.console.log"},"args":[1],"effects":["io","write"]}},
            {"op":"ret","value":3}
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
        &insts[0],
        MirInstruction::LegacyCallV0 { effects, .. }
            if effects.contains(Effect::Io) && effects.contains(Effect::WriteHeap)
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
fn parse_legacy_func_resolves_exact_local_string_const() {
    let json = single_block_json(
        r#"
          {"op":"const","dst":9,"value":{"type":"string","value":"target"}},
          {"op":"call","func":9,"args":[]},
          {"op":"ret"}
        "#,
    );
    let module = parse_mir_v0_to_module(&json).expect("legacy func must resolve");
    let instructions = &module
        .get_function("main")
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .instructions;
    assert!(matches!(
        &instructions[1],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Global(name)),
            ..
        } if *func == ValueId::INVALID && name.display_name() == "target/0"
    ));
}

#[test]
fn parse_legacy_func_resolves_const_from_later_block() {
    let json = r#"{
      "functions":[{"name":"main","blocks":[
        {"id":0,"instructions":[{"op":"call","func":9,"args":[]}]},
        {"id":1,"instructions":[{"op":"const","dst":9,"value":{"type":"string","value":"later"}}]}
      ]}]
    }"#;
    let module = parse_mir_v0_to_module(json).expect("catalog must cover all blocks");
    let instructions = &module
        .get_function("main")
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .instructions;
    assert!(matches!(
        &instructions[0],
        MirInstruction::LegacyCallV0 {
            callee: Some(Callee::Global(name)),
            ..
        } if name.display_name() == "later/0"
    ));
}

#[test]
fn parse_nested_legacy_mir_call_uses_same_catalog_and_outer_dst() {
    let json = single_block_json(
        r#"{"op":"const","dst":9,"value":{"type":"string","value":"nested"}}, {"op":"mir_call","dst":4,"mir_call":{"func":9,"args":[],"effects":[]}}"#,
    );
    let module = parse_mir_v0_to_module(&json).expect("nested legacy call must resolve");
    let instructions = &module
        .get_function("main")
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .instructions;
    assert!(matches!(
        &instructions[1],
        MirInstruction::LegacyCallV0 {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } if *dst == ValueId::new(4) && name.display_name() == "nested/0"
    ));
}

#[test]
fn parse_explicit_callee_ignores_legacy_decoration() {
    let json = single_block_json(
        r#"{"op":"call","callee":{"type":"Global","name":"exact"},"name":7,"func":"ignored","args":[]}"#,
    );
    let module = parse_mir_v0_to_module(&json).expect("explicit callee is authoritative");
    let instructions = &module
        .get_function("main")
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .instructions;
    assert!(matches!(
        &instructions[0],
        MirInstruction::LegacyCallV0 {
            func,
            callee: Some(Callee::Global(name)),
            ..
        } if *func == ValueId::INVALID && name.display_name() == "exact/0"
    ));
}

#[test]
fn parse_call_rejects_missing_target_before_publication() {
    let json = single_block_json(r#"{"op":"call","args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("missing target must reject");
    assert!(
        err.contains("call missing target"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_malformed_explicit_without_legacy_fallback() {
    let json = single_block_json(r#"{"op":"call","callee":null,"name":"fallback","args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("malformed explicit target must reject");
    assert!(
        err.contains("callee must be an object"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_conflicting_legacy_name_and_func() {
    let json = single_block_json(r#"{"op":"call","name":"target","func":9,"args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("conflicting legacy sources must reject");
    assert!(
        err.contains("name and func cannot both"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_undefined_legacy_func() {
    let json = single_block_json(r#"{"op":"call","func":9,"args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("undefined legacy func must reject");
    assert!(
        err.contains("no function-local Const(String)"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_invalid_legacy_func_value_id() {
    let json = single_block_json(r#"{"op":"call","func":4294967295,"args":[]}"#);
    let err = parse_mir_v0_to_module(&json).expect_err("invalid legacy func must reject");
    assert!(err.contains("ValueId::INVALID"), "unexpected error: {err}");
}

#[test]
fn parse_call_rejects_non_string_legacy_func() {
    let json = single_block_json(
        r#"{"op":"const","dst":9,"value":{"type":"i64","value":7}}, {"op":"call","func":9,"args":[]}"#,
    );
    let err = parse_mir_v0_to_module(&json).expect_err("non-string legacy func must reject");
    assert!(
        err.contains("not a Const(String)"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_duplicate_const_relation() {
    let json = single_block_json(
        r#"{"op":"const","dst":9,"value":{"type":"string","value":"a"}}, {"op":"const","dst":9,"value":{"type":"string","value":"b"}}, {"op":"call","func":9,"args":[]}"#,
    );
    let err = parse_mir_v0_to_module(&json).expect_err("duplicate relation must reject");
    assert!(
        err.contains("multiple Const definitions"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_foreign_function_const_relation() {
    let json = r#"{
      "functions":[
        {"name":"main","blocks":[{"id":0,"instructions":[{"op":"call","func":9,"args":[]}]}]},
        {"name":"other","blocks":[{"id":0,"instructions":[{"op":"const","dst":9,"value":{"type":"string","value":"foreign"}}]}]}
      ]
    }"#;
    let err = parse_mir_v0_to_module(json).expect_err("foreign relation must reject");
    assert!(
        err.contains("no function-local Const(String)"),
        "unexpected error: {err}"
    );
}

#[test]
fn parse_call_rejects_explicit_alias_conflict() {
    let json = single_block_json(
        r#"{"op":"call","callee":{"type":"Method","method":"length","name":"size"},"args":[]}"#,
    );
    let err = parse_mir_v0_to_module(&json).expect_err("callee aliases must agree");
    assert!(
        err.contains("method and name conflict"),
        "unexpected error: {err}"
    );
}
