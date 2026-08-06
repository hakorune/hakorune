//! Behavior-neutral associated-input MethodCall child boundary.
//!
//! This module owns only the syntax/input port and the reusable receiver and
//! argument descent primitives. Route selection, special syntax preflight,
//! emission, effects, result publication, location, and ledger policy remain
//! outside this box. The port is stack-scoped and is never stored in Builder.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallParameterObservationV1, MethodCallLoweringPortV1,
};
use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_legacy_statement_v1, RawAstChildLoweringPortV1,
};
use super::super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::call_argument_descent::{
    drive_call_arguments_v1, lower_call_argument_v1, CallArgumentDescentPortV1,
};

pub(in crate::mir::builder) struct MethodCallSyntaxViewV1<'input> {
    receiver: &'input ASTNode,
    method: &'input str,
    arguments: &'input [ASTNode],
}

impl<'input> MethodCallSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(
        receiver: &'input ASTNode,
        method: &'input str,
        arguments: &'input [ASTNode],
    ) -> Self {
        Self {
            receiver,
            method,
            arguments,
        }
    }

    pub(in crate::mir::builder) const fn receiver(&self) -> &'input ASTNode {
        self.receiver
    }

    pub(in crate::mir::builder) const fn method(&self) -> &'input str {
        self.method
    }

    pub(in crate::mir::builder) const fn arguments(&self) -> &'input [ASTNode] {
        self.arguments
    }
}

pub(in crate::mir::builder) enum CatalogHelperChildV1 {
    Statement(ASTNode),
    Expression(ASTNode),
}

pub(in crate::mir::builder) trait MethodCallDescentPortV1:
    CallArgumentDescentPortV1
{
    type MethodCallInput;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String>;

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String>;

    fn with_receiver_expression_source_v1<R>(
        &mut self,
        _input: &Self::MethodCallInput,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        execute(self)
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String>;

    fn lower_catalog_helper_child(
        &mut self,
        _builder: &mut MirBuilder,
        _child: CatalogHelperChildV1,
    ) -> Result<ValueId, String> {
        Err("[method-call-descent/catalog-helper-child-unsupported]".to_string())
    }
}

pub(in crate::mir::builder) fn lower_method_call_argument_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
    index: usize,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let arguments = port.call_arguments_input(input)?;
    lower_call_argument_v1(builder, port, arguments, index)
}

pub(in crate::mir::builder) fn lower_method_call_receiver_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let receiver = port.receiver_expression_input(input)?;
    port.with_receiver_expression_source_v1(input, |port| {
        drive_legacy_expression_v1(builder, port, receiver)
    })
}

pub(in crate::mir::builder) fn lower_method_call_arguments_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
) -> Result<Vec<ValueId>, String>
where
    Port: MethodCallDescentPortV1,
{
    let arguments = port.call_arguments_input(input)?;
    drive_call_arguments_v1(builder, port, arguments)
}

/// Stack-scoped argument capability consumed by route handlers after their
/// syntax/preflight decisions. It carries no route, result, or terminal
/// authority; it only selects full ARG0 or one indexed E0 descent.
pub(in crate::mir::builder) trait MethodCallArgumentDescentV1 {
    fn lower_all(&mut self, builder: &mut MirBuilder) -> Result<Vec<ValueId>, String>;

    fn lower_index(&mut self, builder: &mut MirBuilder, index: usize) -> Result<ValueId, String>;

    fn lower_catalog_helper_child(
        &mut self,
        builder: &mut MirBuilder,
        child: CatalogHelperChildV1,
    ) -> Result<ValueId, String>;
}

pub(in crate::mir::builder) struct AssociatedMethodCallArgumentsV1<'port, 'input, Port>
where
    Port: MethodCallDescentPortV1,
{
    port: &'port mut Port,
    input: &'input Port::MethodCallInput,
}

impl<'port, 'input, Port> AssociatedMethodCallArgumentsV1<'port, 'input, Port>
where
    Port: MethodCallDescentPortV1,
{
    pub(in crate::mir::builder) const fn new(
        port: &'port mut Port,
        input: &'input Port::MethodCallInput,
    ) -> Self {
        Self { port, input }
    }

    pub(in crate::mir::builder) fn terminal_port(&mut self) -> &mut Port {
        self.port
    }
}

impl<Port> AssociatedMethodCallArgumentsV1<'_, '_, Port>
where
    Port: MethodCallLoweringPortV1,
{
    pub(in crate::mir::builder) fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        self.port.observe_me_call_parameters(builder, symbol)
    }
}

impl<Port> MethodCallArgumentDescentV1 for AssociatedMethodCallArgumentsV1<'_, '_, Port>
where
    Port: MethodCallDescentPortV1,
{
    fn lower_all(&mut self, builder: &mut MirBuilder) -> Result<Vec<ValueId>, String> {
        lower_method_call_arguments_v1(builder, self.port, self.input)
    }

    fn lower_index(&mut self, builder: &mut MirBuilder, index: usize) -> Result<ValueId, String> {
        lower_method_call_argument_v1(builder, self.port, self.input, index)
    }

    fn lower_catalog_helper_child(
        &mut self,
        builder: &mut MirBuilder,
        child: CatalogHelperChildV1,
    ) -> Result<ValueId, String> {
        self.port.lower_catalog_helper_child(builder, child)
    }
}

#[allow(dead_code)]
pub(in crate::mir::builder) struct RawLegacyMethodCallInputV1 {
    receiver: ASTNode,
    method: String,
    arguments: Vec<ASTNode>,
    receiver_source: Option<PreparedRawChildSourceV1>,
}

#[allow(dead_code)]
impl RawLegacyMethodCallInputV1 {
    pub(in crate::mir::builder) fn new(
        receiver: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Self {
        Self {
            receiver,
            method,
            arguments,
            receiver_source: None,
        }
    }

    pub(in crate::mir::builder) fn with_receiver_source(
        receiver: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
        receiver_source: PreparedRawChildSourceV1,
    ) -> Self {
        Self {
            receiver,
            method,
            arguments,
            receiver_source: Some(receiver_source),
        }
    }
}

impl<Port> MethodCallDescentPortV1 for Port
where
    Port: RawAstChildLoweringPortV1,
{
    type MethodCallInput = RawLegacyMethodCallInputV1;

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
        Ok(input.receiver.clone())
    }

    fn with_receiver_expression_source_v1<R>(
        &mut self,
        input: &Self::MethodCallInput,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match input.receiver_source.clone() {
            Some(source) => self.with_prepared_child_source_v1(source, execute),
            None => execute(self),
        }
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(&input.arguments)
    }

    fn lower_catalog_helper_child(
        &mut self,
        builder: &mut MirBuilder,
        child: CatalogHelperChildV1,
    ) -> Result<ValueId, String> {
        match child {
            CatalogHelperChildV1::Statement(statement) => {
                drive_legacy_statement_v1(builder, self, statement)
            }
            CatalogHelperChildV1::Expression(expression) => {
                drive_legacy_expression_v1(builder, self, expression)
            }
        }
    }
}
