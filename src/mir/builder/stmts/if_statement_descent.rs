//! Disconnected associated-input boundary for statement-position If control.
//!
//! This box owns only exact If syntax observation, one condition demand,
//! ordered branch-body carrier demand, and delegation to the existing IfForm
//! owner. CFG layout, variable snapshots, PHIs, branch termination, JoinIR
//! diagnostics, statement Void publication, source location, and callable-
//! result consumption remain outside it.

use crate::ast::ASTNode;
use crate::mir::builder::if_form::IfBranchKindV1;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, drive_raw_legacy_expression_v1,
    drive_raw_legacy_statement_v1, RecursiveChildLoweringPortV1,
};

/// Production raw-If port preserving the retired facade's branch Program shell.
///
/// The shared raw child port lowers a body directly. Statement-position If
/// historically lowered each branch as an expression-position `Program` with
/// an unknown span. Keeping that shell here preserves its recursion boundary
/// and span publication without making the generic If driver own either law.
struct RawLegacyIfStatementPortV1;

impl RecursiveChildLoweringPortV1 for RawLegacyIfStatementPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        drive_raw_legacy_expression_v1(
            builder,
            ASTNode::Program {
                statements: input,
                span: crate::ast::Span::unknown(),
            },
        )
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        drive_raw_legacy_statement_v1(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        drive_raw_legacy_expression_v1(builder, input)
    }
}

pub(in crate::mir::builder) struct RawLegacyIfStatementInputV1 {
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
}

impl RawLegacyIfStatementInputV1 {
    pub(in crate::mir::builder) const fn new(
        condition: ASTNode,
        then_body: Vec<ASTNode>,
        else_body: Option<Vec<ASTNode>>,
    ) -> Self {
        Self {
            condition,
            then_body,
            else_body,
        }
    }
}

#[cfg(test)]
pub(super) fn raw_syntax_for_tests(
    input: &RawLegacyIfStatementInputV1,
) -> IfStatementSyntaxViewV1<'_> {
    IfStatementSyntaxViewV1::new(&input.condition, input.else_body.is_some())
}

#[cfg(test)]
pub(super) fn raw_condition_for_tests(input: &RawLegacyIfStatementInputV1) -> ASTNode {
    input.condition.clone()
}

#[cfg(test)]
pub(super) fn raw_then_body_for_tests(input: &RawLegacyIfStatementInputV1) -> Vec<ASTNode> {
    input.then_body.clone()
}

#[cfg(test)]
pub(super) fn raw_else_body_for_tests(input: &RawLegacyIfStatementInputV1) -> Option<Vec<ASTNode>> {
    input.else_body.clone()
}

pub(in crate::mir::builder) struct IfStatementSyntaxViewV1<'input> {
    condition: &'input ASTNode,
    has_explicit_else: bool,
}

impl<'input> IfStatementSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(
        condition: &'input ASTNode,
        has_explicit_else: bool,
    ) -> Self {
        Self {
            condition,
            has_explicit_else,
        }
    }

    pub(in crate::mir::builder) const fn condition(&self) -> &'input ASTNode {
        self.condition
    }

    pub(in crate::mir::builder) const fn has_explicit_else(&self) -> bool {
        self.has_explicit_else
    }
}

pub(in crate::mir::builder) trait IfStatementDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type IfInput;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String>;

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String>;

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String>;

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String>;
}

impl IfStatementDescentPortV1 for RawLegacyIfStatementPortV1 {
    type IfInput = RawLegacyIfStatementInputV1;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String> {
        Ok(IfStatementSyntaxViewV1::new(
            &input.condition,
            input.else_body.is_some(),
        ))
    }

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.condition.clone())
    }

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        Ok(input.then_body.clone())
    }

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        input
            .else_body
            .clone()
            .ok_or_else(|| "[if-statement-descent/raw-else-input-missing]".to_string())
    }
}

pub(in crate::mir::builder) fn drive_if_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::IfInput,
) -> Result<(), String>
where
    Port: IfStatementDescentPortV1,
{
    let (condition_syntax, has_explicit_else) = {
        let syntax = port.if_syntax(input)?;
        (syntax.condition().clone(), syntax.has_explicit_else())
    };
    let condition_input = port.if_condition_expression_input(input)?;
    let condition_value = drive_legacy_expression_v1(builder, port, condition_input)?;
    let condition_debug =
        prepare_if_statement_condition_value_v1(builder, condition_value, &condition_syntax)?;

    // Branch carrier demand stays at the existing IfForm execution points.
    // In particular, an else carrier is not requested unless the then branch
    // was requested and lowered successfully.
    let mut then_demanded = false;
    let mut else_demanded = false;

    builder.lower_if_form_with_condition_value_and_branch_lowerer(
        condition_value,
        condition_debug,
        has_explicit_else,
        |builder, branch| {
            let body = match branch {
                IfBranchKindV1::Then => {
                    if std::mem::replace(&mut then_demanded, true) {
                        return Err("[if-statement-descent/then-demanded-twice]".to_string());
                    }
                    port.if_then_body_input(input)?
                }
                IfBranchKindV1::Else => {
                    if std::mem::replace(&mut else_demanded, true) {
                        return Err("[if-statement-descent/else-demanded-twice]".to_string());
                    }
                    port.if_else_body_input(input)?
                }
            };
            drive_legacy_body_v1(builder, port, body)
        },
    )?;
    Ok(())
}

pub(in crate::mir::builder) fn drive_raw_if_statement_v1(
    builder: &mut MirBuilder,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<(), String> {
    let input = RawLegacyIfStatementInputV1::new(condition, then_body, else_body);
    let mut port = RawLegacyIfStatementPortV1;
    drive_if_statement_v1(builder, &mut port, &input)
}

fn prepare_if_statement_condition_value_v1(
    builder: &mut MirBuilder,
    condition_value: ValueId,
    condition: &ASTNode,
) -> Result<Option<ASTNode>, String> {
    let Some(region) = builder.current_fastmem_region() else {
        return Ok(None);
    };
    crate::mir::builder::fastmem::branch::ensure_fastmem_owner_eq_condition(
        builder,
        region,
        condition_value,
    )?;
    builder.add_fastmem_branch_condition_fact(
        region,
        condition_value,
        crate::mir::function::FastMemBranchConditionProofKind::SourceAssumeOwnerEq,
        true,
    )?;
    Ok(Some(condition.clone()))
}
