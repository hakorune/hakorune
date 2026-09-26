use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::{MirBuilder, MirType, TypeOpKind, ValueId};
use crate::parser::NyashParser;

use super::super::function_signature_lookup::FunctionSignatureLookupV1;
use super::super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use super::super::recursive_child_lowering::{
    drive_raw_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::call_argument_descent::CallArgumentDescentPortV1;
use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{MethodCallDescentPortV1, MethodCallSyntaxViewV1};
use super::method_call_terminal::MethodCallValueTerminalPortV1;

pub(super) fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(super) fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

pub(super) fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

pub(super) fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.to_string(),
        span: Span::unknown(),
    }
}

pub(super) struct RouteInput {
    pub receiver: ASTNode,
    pub method: String,
    pub arguments: Vec<ASTNode>,
}

pub(super) enum RouteExpression {
    Receiver(ASTNode),
    Argument(usize, ASTNode),
}

#[derive(Default)]
pub(super) struct RoutePort {
    pub events: Vec<String>,
    pub fail_receiver: bool,
    pub fail_argument: Option<usize>,
    pub fail_terminal: bool,
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
        drive_raw_legacy_expression_v1(builder, syntax)
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

pub(super) fn builder(name: &str) -> MirBuilder {
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
