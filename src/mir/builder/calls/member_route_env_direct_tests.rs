// MIRBUILDER-EXE-ACCEPTANCE-ENV-DIRECT-RECEIVER-S0: bare `env.<method>`
// receivers classify into the existing `EnvMethod` route before static
// receiver resolution; bound `env` locals keep the Standard route and
// unknown env methods fail with a named error.

use super::member_route_descent_testkit::{
    builder, integer, string, variable, RouteInput, RoutePort,
};

#[test]
fn env_direct_receiver_uses_env_terminal_and_emits_extern_call() {
    let input = RouteInput {
        receiver: variable("env"),
        method: "get".to_string(),
        arguments: vec![string("HAKO_K")],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("env_direct_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["argument:0", "terminal:env"]);
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| instruction.extern_name() == Some("env.get")));
}

#[test]
fn env_direct_receiver_rejects_unknown_method_with_named_error() {
    let input = RouteInput {
        receiver: variable("env"),
        method: "noSuch".to_string(),
        arguments: vec![],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("env_direct_unknown/0");

    let error = builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap_err();

    assert!(
        error.contains("env method not supported"),
        "unexpected error: {error}"
    );
    assert!(port.events.is_empty());
}

#[test]
fn bound_env_receiver_keeps_standard_route() {
    let input = RouteInput {
        receiver: variable("env"),
        method: "get".to_string(),
        arguments: vec![integer(1)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("env_bound_route/0");
    let bound = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("env".to_string(), bound);

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["receiver", "argument:0", "terminal:standard"]);
}
