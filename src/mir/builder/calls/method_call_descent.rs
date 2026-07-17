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
