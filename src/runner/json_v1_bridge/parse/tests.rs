use super::{mir_call::parse_v1_mir_call, try_parse_v1_to_module};
use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};

fn single_v1_instruction_payload(instruction: &str) -> String {
    format!(
        r#"{{"schema_version":"1.0","functions":[{{"name":"main","blocks":[{{"id":0,"instructions":[{}]}}]}}]}}"#,
        instruction
    )
}

#[test]
fn parse_v1_ownership_transport_requires_exact_boxref_witness() {
    let payload = r#"{
      "schema_version":"1.0",
      "functions":[{
        "name":"main",
        "metadata":{
          "value_types":{
            "1":{"kind":"handle","box_type":"WidgetBox"},
            "2":{"kind":"handle","box_type":"WidgetBox"}
          },
          "storage_classes":{"1":"box_ref","2":"box_ref"}
        },
        "blocks":[{"id":0,"instructions":[
          {"op":"copy_owned","dst":2,"src":1},
          {"op":"destroy_owned","value":2}
        ]}]
      }]
    }"#;

    let module = try_parse_v1_to_module(payload)
        .expect("v1 parse")
        .expect("v1 handled");
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
fn parse_phi_incoming_uses_value_then_pred_order() {
    let payload = r#"{
      "schema_version":"1.0",
      "functions":[
        {
          "name":"main",
          "blocks":[
            { "id":0, "instructions":[
              {"op":"const","dst":1,"value":{"type":"i64","value":1}},
              {"op":"const","dst":2,"value":{"type":"i64","value":100}},
              {"op":"const","dst":3,"value":{"type":"i64","value":200}},
              {"op":"branch","cond":1,"then":1,"else":2}
            ]},
            { "id":1, "instructions":[{"op":"jump","target":3}]},
            { "id":2, "instructions":[{"op":"jump","target":3}]},
            { "id":3, "instructions":[
              {"op":"phi","dst":4,"incoming":[[2,1],[3,2]]},
              {"op":"ret","value":4}
            ]}
          ]
        }
      ]
    }"#;

    let module = try_parse_v1_to_module(payload)
        .expect("v1 parse must succeed")
        .expect("schema_version=1.0 must be handled");
    let func = module.get_function("main").expect("main function");
    let bb3 = func.get_block(BasicBlockId::new(3)).expect("bb3");
    let phi = bb3
        .instructions
        .iter()
        .find_map(|inst| match inst {
            MirInstruction::Phi { inputs, .. } => Some(inputs.clone()),
            _ => None,
        })
        .expect("phi instruction in bb3");

    assert_eq!(
        phi,
        vec![
            (BasicBlockId::new(1), ValueId::new(2)),
            (BasicBlockId::new(2), ValueId::new(3))
        ]
    );
}

#[test]
fn parse_v1_params_array_sets_function_arity() {
    let payload = r#"{
      "schema_version":"1.0",
      "functions":[
        {
          "name":"AddOperator.apply/2",
          "params":[0,1],
          "blocks":[
            { "id":0, "instructions":[
              {"op":"copy","dst":2,"src":0},
              {"op":"copy","dst":3,"src":1},
              {"op":"binop","operation":"+","lhs":2,"rhs":3,"dst":4},
              {"op":"ret","value":4}
            ]}
          ]
        },
        {
          "name":"main",
          "params":[],
          "blocks":[
            { "id":10, "instructions":[
              {"op":"const","dst":1,"value":{"type":"i64","value":2}},
              {"op":"const","dst":2,"value":{"type":"i64","value":3}},
              {"op":"ret","value":1}
            ]}
          ]
        }
      ]
    }"#;

    let module = try_parse_v1_to_module(payload)
        .expect("v1 parse must succeed")
        .expect("schema_version=1.0 must be handled");
    let func = module
        .get_function("AddOperator.apply/2")
        .expect("operator function exists");
    assert_eq!(func.signature.params.len(), 2);
    assert_eq!(func.params, vec![ValueId::new(0), ValueId::new(1)]);
    assert!(func.next_value_id >= 5);
}

#[test]
fn parse_v1_accepts_newbox_and_field_get() {
    let payload = r#"{
      "schema_version":"1.0",
      "functions":[
        {
          "name":"main",
          "params":[],
          "blocks":[
            { "id":0, "instructions":[
              {"op":"newbox","dst":1,"type":"ArrayBox","args":[]},
              {"op":"field_get","dst":2,"box":1,"field":"stringify"},
              {"op":"ret","value":2}
            ]}
          ]
        }
      ]
    }"#;

    let module = try_parse_v1_to_module(payload)
        .expect("v1 parse must succeed")
        .expect("schema_version=1.0 must be handled");
    let func = module.get_function("main").expect("main function");
    let bb0 = func.get_block(BasicBlockId::new(0)).expect("bb0");

    assert!(matches!(
        &bb0.instructions[0],
        MirInstruction::NewBox { dst, box_type, args }
            if *dst == ValueId::new(1) && box_type == "ArrayBox" && args.is_empty()
    ));
    assert!(matches!(
        &bb0.instructions[1],
        MirInstruction::FieldGet { dst, base, field, declared_type }
            if *dst == ValueId::new(2)
                && *base == ValueId::new(1)
                && field == "stringify"
                && declared_type.is_none()
    ));
    assert!(func.next_value_id >= 3);
}

#[test]
fn parse_v1_typed_constructor_preserves_valid_newbox_shape() {
    let payload = single_v1_instruction_payload(
        r#"{"op":"mir_call","dst":1,"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[]}"#,
    );
    let module = try_parse_v1_to_module(&payload)
        .expect("valid Constructor must parse")
        .expect("schema_version=1.0 must be handled");
    let instructions = &module
        .get_function("main")
        .expect("main exists")
        .get_block(BasicBlockId::new(0))
        .expect("bb0 exists")
        .instructions;
    assert!(matches!(
        &instructions[0],
        MirInstruction::NewBox { dst, box_type, args }
            if *dst == ValueId::new(1) && box_type == "ArrayBox" && args.is_empty()
    ));

    let closure_payload = single_v1_instruction_payload(
        r#"{"op":"mir_call","dst":2,"callee":{"type":"Closure","params":[],"captures":[]},"args":[]}"#,
    );
    let closure_module = try_parse_v1_to_module(&closure_payload)
        .expect("valid Closure creation must parse")
        .expect("schema_version=1.0 must be handled");
    let closure_instructions = &closure_module
        .get_function("main")
        .expect("closure main exists")
        .get_block(BasicBlockId::new(0))
        .expect("closure bb0 exists")
        .instructions;
    assert!(matches!(
        &closure_instructions[0],
        MirInstruction::NewClosure { dst, params, captures, .. }
            if *dst == ValueId::new(2) && params.is_empty() && captures.is_empty()
    ));
}

#[test]
fn parse_v1_constructor_rejects_missing_args_before_publication() {
    let payload = single_v1_instruction_payload(
        r#"{"op":"mir_call","dst":1,"callee":{"type":"Constructor","box_type":"ArrayBox"}}"#,
    );
    let error = try_parse_v1_to_module(&payload).expect_err("missing Constructor args must reject");
    assert!(error.contains("[freeze:contract][mir-json-v1/constructor-args-required]"));
}

#[test]
fn parse_v1_constructor_rejects_non_array_args_before_publication() {
    let payload = single_v1_instruction_payload(
        r#"{"op":"mir_call","dst":1,"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":null}"#,
    );
    let error =
        try_parse_v1_to_module(&payload).expect_err("non-array Constructor args must reject");
    assert!(error.contains("[freeze:contract][mir-json-v1/constructor-args-must-be-array]"));
}

#[test]
fn parse_v1_constructor_rejects_conflicting_name_aliases_before_publication() {
    let payload = single_v1_instruction_payload(
        r#"{"op":"mir_call","dst":1,"callee":{"type":"Constructor","name":"ArrayBox","box_type":"MapBox"},"args":[]}"#,
    );
    let error = try_parse_v1_to_module(&payload).expect_err("conflicting aliases must reject");
    assert!(error.contains("[freeze:contract][mir-json-v1/constructor-name-box-type-conflict]"));
}

#[test]
fn parse_v1_constructor_rejects_dual_args_placement_before_publication() {
    let payload = r#"{
      "schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
        {"op":"mir_call","dst":1,"args":[],"mir_call":{"args":[],"callee":{"type":"Constructor","box_type":"ArrayBox"}}}
      ]}]}]
    }"#;
    let error = try_parse_v1_to_module(payload).expect_err("dual args placement must reject");
    assert!(error.contains("[freeze:contract][mir-json-v1/constructor-args-ambiguous]"));
}

#[test]
fn parse_v1_legacy_call_writers_stop_before_block_mutation() {
    let cases = [
        ("Global", r#","name":"helper""#),
        ("Method", r#","name":"step","receiver":1"#),
        ("Extern", r#","name":"env.console.log""#),
        ("Value", r#","value":2"#),
        ("Closure", r#","func":2"#),
    ];

    for (callee_type, fields) in cases {
        let instruction = format!(
            r#"{{"op":"mir_call","callee":{{"type":"{}"{}}},"args":[]}}"#,
            callee_type, fields
        );
        let value: serde_json::Value =
            serde_json::from_str(&instruction).expect("test instruction JSON");
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        let mut max_value_id = 0;
        let error = parse_v1_mir_call(&value, "main", &mut block, &mut max_value_id)
            .expect_err("legacy call-like JSON v1 ingress must stop");
        assert!(
            error.contains("[freeze:contract][mir-json-v1/legacy-call-stopped]"),
            "unexpected error for {callee_type}: {error}"
        );
        assert!(block.instructions.is_empty(), "{callee_type} mutated the block");
        assert_eq!(max_value_id, 0, "{callee_type} changed the value-id cursor");
    }
}
