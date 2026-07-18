//! Located adapter for one exact statement-position If.
//!
//! This adapter selects only the statement carrier and derives the condition
//! and branch carriers through PATH0. The shared driver retains condition
//! order, post-condition preparation, lazy branch demand, and control-flow
//! authority. A branch is delegated only after the caller ledger proves its
//! complete body domain inactive.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::if_statement_descent::{
    complete_if_statement_v1, drive_if_statement_v1, IfStatementDescentPortV1,
    IfStatementSyntaxViewV1,
};
use crate::mir::callable_result_representation::LegacyStmtInputV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirBuilder, ValueId};

use super::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};

pub(in crate::mir::builder) struct LocatedStatementIfInputV1<'plan> {
    statement: LegacyStmtInputV1<'plan>,
    condition: &'plan ASTNode,
    has_explicit_else: bool,
}

pub(super) fn select_exact_statement_if_v1<'plan>(
    input: LegacyStmtInputV1<'plan>,
) -> Result<LocatedStatementIfInputV1<'plan>, LegacyStmtInputV1<'plan>> {
    let (condition, has_explicit_else) = match input.node() {
        ASTNode::If {
            condition,
            else_body,
            ..
        } => (condition.as_ref(), else_body.is_some()),
        _ => return Err(input),
    };
    Ok(LocatedStatementIfInputV1 {
        statement: input,
        condition,
        has_explicit_else,
    })
}

pub(super) fn lower_selected_statement_if_v1<'plan>(
    session: &mut LocatedLegacyLoweringSessionV1<'plan>,
    builder: &mut MirBuilder,
    selected: &LocatedStatementIfInputV1<'plan>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    builder
        .metadata_ctx
        .set_current_span(selected.statement.node().span());
    let lowering = drive_if_statement_v1(builder, session, selected);
    complete_if_statement_v1(builder, lowering).map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

impl<'plan> IfStatementDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type IfInput = LocatedStatementIfInputV1<'plan>;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String> {
        Ok(IfStatementSyntaxViewV1::new(
            input.condition,
            input.has_explicit_else,
        ))
    }

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.source
            .child_expr_from_stmt(&input.statement, ExprChildRoleV1::IfCondition)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        self.source
            .child_body_from_stmt(&input.statement, BodyChildRoleV1::IfThen)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        self.source
            .child_body_from_stmt(&input.statement, BodyChildRoleV1::IfElse)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }
}
