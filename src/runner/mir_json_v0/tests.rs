use super::parse_mir_v0_to_module;
use crate::mir::{BasicBlockId, Callee, Effect, MirInstruction, ValueId};

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
        MirInstruction::Call {
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
        MirInstruction::Call {
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
        MirInstruction::Call {
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
        MirInstruction::Call { effects, .. }
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
