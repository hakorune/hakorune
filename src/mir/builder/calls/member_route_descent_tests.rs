use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::{Callee, MirBuilder, MirInstruction, ValueId};
use crate::parser::NyashParser;

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::call_argument_descent::CallArgumentDescentPortV1;
use super::method_call_descent::{MethodCallDescentPortV1, MethodCallSyntaxViewV1};

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
        &self,
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

    assert_eq!(port.events, ["receiver"]);
    assert!(builder
        .scope_ctx
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

    assert_eq!(port.events, ["argument:0", "argument:1"]);
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

    assert_eq!(port.events, ["receiver", "argument:0", "argument:1"]);
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
    assert_eq!(port.events, ["receiver"]);
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

    assert_eq!(port.events, ["receiver", "argument:0"]);
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

    assert_eq!(port.events, ["argument:0", "argument:1"]);
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
        .variable_ctx
        .variable_map
        .insert("me".to_string(), me);

    builder
        .build_method_call_from_input_v1(&mut port, &input)
        .unwrap();

    assert_eq!(port.events, ["argument:0", "argument:1"]);
}

#[test]
fn materialized_property_receiver_is_forwarded_without_source_redescent() {
    let mut builder = builder("materialized_property_route/0");
    let receiver = builder.build_expression(integer(11)).unwrap();

    builder
        .handle_standard_method_call(receiver, "propertyGetter".to_string(), &[])
        .unwrap();

    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let emitted = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .cloned()
        .collect::<Vec<_>>();
    assert!(emitted.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Call {
            callee: Some(Callee::Method {
                method,
                receiver: Some(_),
                ..
            }),
            args,
            ..
        } if method == "propertyGetter" && args.is_empty()
    )));
}
