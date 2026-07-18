//! Disconnected associated-input boundary for value-bearing Return statements.
//!
//! This box owns only the ordering between the existing cleanup preflight,
//! match-return hook, one value-expression demand, and the existing Return
//! completion. Void returns, Match/CorePlan semantics, cleanup/defer policy,
//! source location, and callable-result consumption remain outside it.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::return_stmt::{
    emit_return_from_value, ensure_return_allowed, try_apply_match_return_optimization,
};

pub(in crate::mir::builder) struct RawLegacyValueReturnInputV1 {
    value: ASTNode,
}

impl RawLegacyValueReturnInputV1 {
    pub(in crate::mir::builder) const fn new(value: ASTNode) -> Self {
        Self { value }
    }
}

pub(in crate::mir::builder) struct ReturnStatementSyntaxViewV1<'input> {
    value: &'input ASTNode,
}

impl<'input> ReturnStatementSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(value: &'input ASTNode) -> Self {
        Self { value }
    }

    pub(in crate::mir::builder) const fn value(&self) -> &'input ASTNode {
        self.value
    }
}

pub(in crate::mir::builder) trait ReturnStatementDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type ReturnInput;

    fn return_value_syntax<'input>(
        &self,
        input: &'input Self::ReturnInput,
    ) -> Result<ReturnStatementSyntaxViewV1<'input>, String>;

    fn try_match_return_optimization(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::ReturnInput,
        value: &ASTNode,
    ) -> Result<Option<ValueId>, String>;

    fn return_value_expression_input(
        &self,
        input: &Self::ReturnInput,
    ) -> Result<Self::ExpressionInput, String>;
}

impl ReturnStatementDescentPortV1 for RawLegacyChildLoweringPortV1 {
    type ReturnInput = RawLegacyValueReturnInputV1;

    fn return_value_syntax<'input>(
        &self,
        input: &'input Self::ReturnInput,
    ) -> Result<ReturnStatementSyntaxViewV1<'input>, String> {
        Ok(ReturnStatementSyntaxViewV1::new(&input.value))
    }

    fn try_match_return_optimization(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::ReturnInput,
        value: &ASTNode,
    ) -> Result<Option<ValueId>, String> {
        try_apply_match_return_optimization(builder, Some(value), true)
    }

    fn return_value_expression_input(
        &self,
        input: &Self::ReturnInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.value.clone())
    }
}

pub(in crate::mir::builder) fn drive_value_return_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::ReturnInput,
) -> Result<ValueId, String>
where
    Port: ReturnStatementDescentPortV1,
{
    ensure_return_allowed(builder)?;
    let value = port.return_value_syntax(input)?.value();
    if let Some(result) = port.try_match_return_optimization(builder, input, value)? {
        return Ok(result);
    }

    let expression_input = port.return_value_expression_input(input)?;
    let return_value = drive_legacy_expression_v1(builder, port, expression_input)?;
    emit_return_from_value(builder, return_value)
}

pub(in crate::mir::builder) fn drive_raw_value_return_statement_v1(
    builder: &mut MirBuilder,
    value: ASTNode,
) -> Result<ValueId, String> {
    let input = RawLegacyValueReturnInputV1::new(value);
    let mut port = RawLegacyChildLoweringPortV1;
    drive_value_return_statement_v1(builder, &mut port, &input)
}
