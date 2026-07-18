//! Disconnected associated-input boundary for exact Variable assignments.
//!
//! The target selector remains outside this box. Its input contains only one
//! already-selected variable name plus an RHS carrier. This box preserves the
//! existing declared-binding preflight, requests the RHS once, and delegates
//! completion to the existing from-value Assignment owner.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::super::vars::assignment_resolver::AssignmentResolverBox;

pub(in crate::mir::builder) struct RawLegacyVariableAssignmentInputV1 {
    variable_name: String,
    value: ASTNode,
}

impl RawLegacyVariableAssignmentInputV1 {
    pub(in crate::mir::builder) const fn new(variable_name: String, value: ASTNode) -> Self {
        Self {
            variable_name,
            value,
        }
    }
}

pub(in crate::mir::builder) struct VariableAssignmentSyntaxViewV1<'input> {
    variable_name: &'input str,
}

impl<'input> VariableAssignmentSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(variable_name: &'input str) -> Self {
        Self { variable_name }
    }

    pub(in crate::mir::builder) const fn variable_name(&self) -> &'input str {
        self.variable_name
    }
}

pub(in crate::mir::builder) trait VariableAssignmentDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type VariableAssignmentInput;

    fn variable_assignment_syntax<'input>(
        &self,
        input: &'input Self::VariableAssignmentInput,
    ) -> Result<VariableAssignmentSyntaxViewV1<'input>, String>;

    fn assignment_rhs_expression_input(
        &self,
        input: &Self::VariableAssignmentInput,
    ) -> Result<Self::ExpressionInput, String>;
}

impl VariableAssignmentDescentPortV1 for RawLegacyChildLoweringPortV1 {
    type VariableAssignmentInput = RawLegacyVariableAssignmentInputV1;

    fn variable_assignment_syntax<'input>(
        &self,
        input: &'input Self::VariableAssignmentInput,
    ) -> Result<VariableAssignmentSyntaxViewV1<'input>, String> {
        Ok(VariableAssignmentSyntaxViewV1::new(&input.variable_name))
    }

    fn assignment_rhs_expression_input(
        &self,
        input: &Self::VariableAssignmentInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.value.clone())
    }
}

pub(in crate::mir::builder) fn drive_variable_assignment_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::VariableAssignmentInput,
) -> Result<ValueId, String>
where
    Port: VariableAssignmentDescentPortV1,
{
    let variable_name = port
        .variable_assignment_syntax(input)?
        .variable_name()
        .to_string();

    AssignmentResolverBox::ensure_declared(builder, &variable_name)?;
    let rhs_input = port.assignment_rhs_expression_input(input)?;
    let rhs = drive_legacy_expression_v1(builder, port, rhs_input)?;
    builder.build_assignment_from_value(variable_name, rhs)
}

pub(in crate::mir::builder) fn drive_raw_variable_assignment_v1(
    builder: &mut MirBuilder,
    variable_name: String,
    value: ASTNode,
) -> Result<ValueId, String> {
    let input = RawLegacyVariableAssignmentInputV1::new(variable_name, value);
    let mut port = RawLegacyChildLoweringPortV1;
    drive_variable_assignment_v1(builder, &mut port, &input)
}
