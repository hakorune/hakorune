//! Behavior-neutral associated-input MethodCall child boundary.
//!
//! This module owns only the syntax/input port and the reusable receiver and
//! argument descent primitives. Route selection, special syntax preflight,
//! emission, effects, result publication, location, and ledger policy remain
//! outside this box. The port is stack-scoped and is never stored in Builder.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1,
};
use super::call_argument_descent::{drive_call_arguments_v1, CallArgumentDescentPortV1};

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

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String>;
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
    let expression = port.argument_expression_input(arguments, index)?;
    drive_legacy_expression_v1(builder, port, expression)
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
    drive_legacy_expression_v1(builder, port, receiver)
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

    pub(super) fn terminal_parts(&mut self) -> (&mut Port, &Port::MethodCallInput) {
        (self.port, self.input)
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
}

/// Existing already-materialized receiver callers do not own a MethodCall
/// source carrier. This adapter preserves their legacy argument behavior
/// without fabricating an AST receiver or re-running route classification.
pub(in crate::mir::builder) struct LegacyMethodCallArgumentsV1<'input> {
    arguments: &'input [ASTNode],
}

impl<'input> LegacyMethodCallArgumentsV1<'input> {
    pub(in crate::mir::builder) const fn new(arguments: &'input [ASTNode]) -> Self {
        Self { arguments }
    }
}

impl MethodCallArgumentDescentV1 for LegacyMethodCallArgumentsV1<'_> {
    fn lower_all(&mut self, builder: &mut MirBuilder) -> Result<Vec<ValueId>, String> {
        builder.build_call_args(self.arguments)
    }

    fn lower_index(&mut self, builder: &mut MirBuilder, index: usize) -> Result<ValueId, String> {
        let argument = self.arguments.get(index).cloned().ok_or_else(|| {
            format!("[method-call-descent/missing-legacy-argument] index={index}")
        })?;
        builder.build_expression(argument)
    }
}

#[allow(dead_code)]
pub(in crate::mir::builder) struct RawLegacyMethodCallInputV1 {
    receiver: ASTNode,
    method: String,
    arguments: Vec<ASTNode>,
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
        }
    }
}

impl MethodCallDescentPortV1 for RawLegacyChildLoweringPortV1 {
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

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(&input.arguments)
    }
}
