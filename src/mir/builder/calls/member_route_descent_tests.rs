use crate::ast::{ASTNode, Span};
use crate::mir::function::{FunctionSignature, MirFunction, MirModule};
use crate::mir::{BasicBlockId, Callee, EffectMask, MirInstruction, MirType};

use super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use super::member_route_descent_testkit::{
    builder, field, integer, string, variable, RouteInput, RoutePort,
};
use super::method_call_descent::{CatalogHelperChildV1, MethodCallArgumentDescentV1};
use super::method_call_terminal::StandardMethodCallCompletionV1;
#[test]
fn typeop_descends_receiver_once_and_keeps_type_string_syntax_only() {
    let input = RouteInput {
        receiver: integer(7),
        method: "is".to_string(),
        arguments: vec![string("Integer")],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("typeop_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["receiver", "terminal:typeop"]);
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| matches!(instruction, MirInstruction::TypeOp { .. })));
}

#[test]
fn static_route_skips_receiver_and_descends_arguments_left_to_right() {
    let input = RouteInput {
        // Generic unissued static routes now retire before descent; Math is
        // the explicitly preserved qualified compatibility owner.
        receiver: variable("Math"),
        method: "abs".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("static_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["argument:0", "argument:1", "terminal:static"]);
}

#[test]
fn standard_route_descends_receiver_before_arguments() {
    let input = RouteInput {
        receiver: integer(7),
        method: "routeMethod".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("standard_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(
        port.events,
        ["receiver", "argument:0", "argument:1", "terminal:standard"]
    );
}

#[test]
fn standard_receiver_failure_descends_no_arguments_and_builder_is_reusable() {
    let failing = RouteInput {
        receiver: integer(7),
        method: "routeMethod".to_string(),
        arguments: vec![integer(1)],
    };
    let valid = RouteInput {
        receiver: integer(8),
        method: "as".to_string(),
        arguments: vec![string("Integer")],
    };
    let mut port = RoutePort {
        fail_receiver: true,
        ..RoutePort::default()
    };
    let mut builder = builder("standard_route_failure/0");

    assert!(builder
        .build_method_call_from_input_v1(&mut port, &failing)
        .is_err());
    assert_eq!(port.events, ["receiver"]);

    port.fail_receiver = false;
    port.events.clear();
    builder
        .build_method_call_from_input_v1(&mut port, &valid)
        .unwrap();
    assert_eq!(port.events, ["receiver", "terminal:typeop"]);
}

#[test]
fn argument_failure_enters_no_terminal_and_builder_reuses() {
    let failing = RouteInput {
        receiver: integer(7),
        method: "routeMethod".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let valid = RouteInput {
        receiver: integer(8),
        method: "is".to_string(),
        arguments: vec![string("Integer")],
    };
    let mut port = RoutePort {
        fail_argument: Some(0),
        ..RoutePort::default()
    };
    let mut builder = builder("standard_argument_failure/0");

    assert_eq!(
        builder
            .build_method_call_from_input_v1(&mut port, &failing)
            .unwrap_err(),
        "route fixture argument failure index=0"
    );
    assert_eq!(port.events, ["receiver", "argument:0"]);
    assert!(!builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| matches!(
            instruction,
            MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
        )));

    port.fail_argument = None;
    port.events.clear();
    builder
        .build_method_call_from_input_v1(&mut port, &valid)
        .unwrap();
    assert_eq!(port.events, ["receiver", "terminal:typeop"]);
}

#[test]
fn static_scalar_fact_returns_const_without_generic_terminal() {
    let body = [ASTNode::Return {
        value: Some(Box::new(integer(41))),
        span: Span::unknown(),
    }];
    let input = RouteInput {
        receiver: variable("ScalarFacts"),
        method: "answer".to_string(),
        arguments: vec![],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("static_scalar_custom_terminal/0");
    assert!(builder
        .comp_ctx
        .register_static_scalar_method_fact_if_verified("ScalarFacts.answer/0", &[], &body));

    let result = builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert!(port.events.is_empty());
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| matches!(
            instruction,
            MirInstruction::Const {
                dst,
                value: crate::mir::ConstValue::Integer(41),
            } if *dst == result
        )));
}

#[test]
fn weak_load_and_upgrade_preflight_bypass_generic_terminal() {
    let weak = RouteInput {
        receiver: integer(7),
        method: "weak_to_strong".to_string(),
        arguments: vec![],
    };
    let upgrade = RouteInput {
        receiver: integer(8),
        method: "upgrade".to_string(),
        arguments: vec![],
    };
    let valid = RouteInput {
        receiver: integer(9),
        method: "as".to_string(),
        arguments: vec![string("Integer")],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("weak_custom_terminal/0");

    builder
        .build_method_call_from_input_v1(&mut port, &weak)
        .unwrap();
    assert_eq!(port.events, ["receiver"]);

    port.events.clear();
    assert_eq!(
        builder
            .build_method_call_from_input_v1(&mut port, &upgrade)
            .unwrap_err(),
        "WeakRef uses weak_to_strong(), not upgrade()"
    );
    assert_eq!(port.events, ["receiver"]);

    port.events.clear();
    builder
        .build_method_call_from_input_v1(&mut port, &valid)
        .unwrap();
    assert_eq!(port.events, ["receiver", "terminal:typeop"]);
}

#[test]
fn malformed_typeop_uses_standard_receiver_then_argument_demand() {
    let input = RouteInput {
        receiver: integer(7),
        method: "is".to_string(),
        arguments: vec![integer(1)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("malformed_typeop_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["receiver", "argument:0", "terminal:standard"]);
}

#[test]
fn env_route_keeps_receiver_syntax_only_and_descends_arguments() {
    let input = RouteInput {
        receiver: field(variable("env"), "console"),
        method: "log".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("env_route/0");

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["argument:0", "argument:1", "terminal:env"]);
}

#[test]
fn bound_me_route_keeps_source_receiver_syntax_only() {
    let input = RouteInput {
        receiver: ASTNode::Me {
            span: Span::unknown(),
        },
        method: "routeMethod".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("RouteOwner.method/0");
    let me = crate::mir::builder::emission::constant::emit_integer(&mut builder, 9).unwrap();
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("me".to_string(), me);

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(
        port.events,
        ["argument:0", "argument:1", "terminal:standard"]
    );
}

#[test]
fn lowered_me_arguments_precede_terminal_and_keep_receiver_prefix() {
    let input = RouteInput {
        receiver: ASTNode::Me {
            span: Span::unknown(),
        },
        method: "routeMethod".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort::default();
    let mut builder = builder("RouteOwner.caller/0");
    let me = crate::mir::builder::emission::constant::emit_integer(&mut builder, 9).unwrap();
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("me".to_string(), me);

    let signature = FunctionSignature {
        name: "RouteOwner.routeMethod/2".to_string(),
        params: vec![
            MirType::Box("RouteOwner".to_string()),
            MirType::Integer,
            MirType::Integer,
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut module = MirModule::new("route-terminal-module".to_string());
    module.add_function(MirFunction::new(signature, BasicBlockId::new(0)));
    builder.current_module = Some(module);

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["argument:0", "argument:1", "terminal:me"]);
    let call = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .find_map(|instruction| match instruction {
            MirInstruction::Call(call)
                if matches!(&call.callee, Callee::Global(name) if name.display_name() == "RouteOwner.routeMethod/2") =>
            {
                Some(call.args.as_slice())
            }
            MirInstruction::LegacyCallV0 {
                callee: Some(Callee::Global(name)), args, ..
            } if name.display_name() == "RouteOwner.routeMethod/2" => Some(args.as_slice()),
            _ => None,
        })
        .expect("lowered me terminal must emit the module global");
    assert_eq!(call.len(), 3);
}

#[test]
fn generic_terminal_failure_follows_children_without_retry_and_builder_reuses() {
    let input = RouteInput {
        receiver: integer(7),
        method: "routeMethod".to_string(),
        arguments: vec![integer(1), integer(2)],
    };
    let mut port = RoutePort {
        fail_terminal: true,
        ..RoutePort::default()
    };
    let mut builder = builder("terminal_failure_route/0");

    let error = builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap_err();
    assert_eq!(error, "route fixture terminal failure");
    assert_eq!(
        port.events,
        ["receiver", "argument:0", "argument:1", "terminal:standard"]
    );
    let failed_instructions = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .cloned()
        .collect::<Vec<_>>();
    assert!(!failed_instructions.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
    )));
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .is_empty());

    port.fail_terminal = false;
    port.events.clear();
    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();
    assert_eq!(
        port.events,
        ["receiver", "argument:0", "argument:1", "terminal:standard"]
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| &block.instructions)
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
            ))
            .count(),
        1
    );
}

#[test]
fn property_completion_uses_selected_catalog_child_but_raw_terminal() {
    use super::super::property_reads::PropertyGetterCompletionV1;

    let mut builder = builder("property_completion/0");
    let receiver = crate::mir::builder::emission::constant::emit_integer(&mut builder, 11).unwrap();
    let mut port = RawLegacyChildLoweringPortV1;
    let mut completion = PropertyGetterCompletionV1::new(&mut port);

    assert!(completion.lower_all(&mut builder).unwrap().is_empty());
    assert_eq!(
        completion.lower_index(&mut builder, 0).unwrap_err(),
        "[property-getter-descent/indexed-argument] index=0 arity=0"
    );
    let catalog_value = completion
        .lower_catalog_helper_child(&mut builder, CatalogHelperChildV1::Expression(integer(12)))
        .unwrap();
    let result = completion
        .finish_standard_value_terminal(
            &mut builder,
            receiver,
            "propertyGetter".to_string(),
            Vec::new(),
        )
        .unwrap();

    let function = builder.function_state.current_function.as_ref().unwrap();
    let instructions = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .collect::<Vec<_>>();
    assert!(instructions.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(12) }
            if *dst == catalog_value
    )));
    let calls = instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call(call)
                if call.dst == Some(result)
                    && matches!(&call.callee, Callee::Method { method, receiver: Some(_), .. }
                        if method == "propertyGetter") =>
            {
                Some((result, call.args.len()))
            }
            MirInstruction::LegacyCallV0 {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
                        method,
                        receiver: Some(_),
                        ..
                    }),
                args,
                ..
            } if method == "propertyGetter" => Some((*dst, args.len())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls, [(result, 0)]);
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&result),
        None
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&result),
        None
    );
}
