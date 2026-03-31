use super::try_parse_v1_to_module;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

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
