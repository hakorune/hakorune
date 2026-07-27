use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::function::{FunctionSignature, MirFunction, MirModule};
use crate::mir::{
    BasicBlockId, Callee, EffectMask, MirBuilder, MirInstruction, MirType, TypeOpKind, ValueId,
};
use crate::parser::NyashParser;

use super::super::function_signature_lookup::FunctionSignatureLookupV1;
use super::super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::call_argument_descent::CallArgumentDescentPortV1;
use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{MethodCallDescentPortV1, MethodCallSyntaxViewV1};
use super::method_call_terminal::MethodCallValueTerminalPortV1;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.to_string(),
        span: Span::unknown(),
    }
}

struct RouteInput {
    receiver: ASTNode,
    method: String,
    arguments: Vec<ASTNode>,
}

enum RouteExpression {
    Receiver(ASTNode),
    Argument(usize, ASTNode),
}

#[derive(Default)]
struct RoutePort {
    events: Vec<String>,
    fail_receiver: bool,
    fail_argument: Option<usize>,
    fail_terminal: bool,
}

impl RecursiveChildLoweringPortV1 for RoutePort {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = RouteExpression;

    fn lower_body(&mut self, _builder: &mut MirBuilder, _input: ()) -> Result<ValueId, String> {
        unreachable!("body descent is outside this fixture")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: (),
    ) -> Result<ValueId, String> {
        unreachable!("statement descent is outside this fixture")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: RouteExpression,
    ) -> Result<ValueId, String> {
        let syntax = match input {
            RouteExpression::Receiver(syntax) => {
                self.events.push("receiver".to_string());
                if self.fail_receiver {
                    return Err("route fixture receiver failure".to_string());
                }
                syntax
            }
            RouteExpression::Argument(index, syntax) => {
                self.events.push(format!("argument:{index}"));
                if self.fail_argument == Some(index) {
                    return Err(format!("route fixture argument failure index={index}"));
                }
                syntax
            }
        };
        builder.build_expression(syntax)
    }
}

impl CallArgumentDescentPortV1 for RoutePort {
    type ArgumentsInput = [ASTNode];

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        input.len()
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        input.get(index)
    }

    fn argument_expression_input(
        &mut self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        input
            .get(index)
            .cloned()
            .map(|syntax| RouteExpression::Argument(index, syntax))
            .ok_or_else(|| format!("missing route argument index={index}"))
    }
}

impl MethodCallDescentPortV1 for RoutePort {
    type MethodCallInput = RouteInput;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String> {
        Ok(MethodCallSyntaxViewV1::new(
            &input.receiver,
            &input.method,
            &input.arguments,
        ))
    }

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(RouteExpression::Receiver(input.receiver.clone()))
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(&input.arguments)
    }
}

impl MeCallHeaderObservationPortV1 for RoutePort {
    fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        MeCallParameterObservationV1::from_optional_lookup(
            MeCallHeaderSourceV1::ModuleCompatibility,
            symbol,
            builder
                .current_module
                .as_ref()
                .map(|module| module as &dyn FunctionSignatureLookupV1),
        )
    }
}

impl MethodCallValueTerminalPortV1 for RoutePort {
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        self.events.push("terminal:typeop".to_string());
        let mut raw = RawLegacyChildLoweringPortV1;
        raw.emit_typeop_value_terminal(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.events.push("terminal:static".to_string());
        let mut raw = RawLegacyChildLoweringPortV1;
        raw.emit_static_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.events.push("terminal:me".to_string());
        let mut raw = RawLegacyChildLoweringPortV1;
        raw.emit_me_lowered_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.events.push("terminal:env".to_string());
        let mut raw = RawLegacyChildLoweringPortV1;
        raw.emit_env_value_terminal(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.events.push("terminal:standard".to_string());
        if self.fail_terminal {
            return Err("route fixture terminal failure".to_string());
        }
        let mut raw = RawLegacyChildLoweringPortV1;
        raw.emit_standard_value_terminal(builder, receiver, method, arguments)
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    let root =
        NyashParser::parse_from_string("static box RouteCatalogSentinel { noop() { return 0 } }")
            .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn ordinary_copy_root(instructions: &[MirInstruction], mut value: ValueId) -> ValueId {
    let mut remaining = instructions.len();
    while remaining > 0 {
        let Some(src) = instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            })
        else {
            break;
        };
        value = src;
        remaining -= 1;
    }
    value
}

fn normalized_const_value(
    instructions: &[MirInstruction],
    value: ValueId,
) -> Option<crate::mir::ConstValue> {
    let root = ordinary_copy_root(instructions, value);
    instructions
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Const { dst, value } if *dst == root => Some(value.clone()),
            _ => None,
        })
}

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
        receiver: variable("RouteStaticBox"),
        method: "call".to_string(),
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
        .any(|instruction| matches!(instruction, MirInstruction::Call { .. })));

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
    let me = builder.build_expression(integer(9)).unwrap();
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
    let me = builder.build_expression(integer(9)).unwrap();
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
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if name == "RouteOwner.routeMethod/2" => Some(args),
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
    assert!(!failed_instructions
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Call { .. })));
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
            .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
            .count(),
        1
    );
}

#[test]
fn materialized_property_receiver_is_forwarded_without_source_redescent() {
    let mut builder = builder("materialized_property_route/0");
    let receiver = builder.build_expression(integer(11)).unwrap();

    let result = builder
        .handle_standard_method_call(receiver, "propertyGetter".to_string(), &[])
        .unwrap();

    let function = builder.function_state.current_function.as_ref().unwrap();
    let emitted = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .cloned()
        .collect::<Vec<_>>();
    let (call_dst, call_receiver) = emitted
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
                        method,
                        receiver: Some(call_receiver),
                        ..
                    }),
                args,
                ..
            } if method == "propertyGetter" && args.is_empty() => Some((*dst, *call_receiver)),
            _ => None,
        })
        .expect("materialized property must emit one completed method call");
    assert_eq!(call_dst, result);
    assert_eq!(
        normalized_const_value(&emitted, call_receiver),
        normalized_const_value(&emitted, receiver),
    );
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
