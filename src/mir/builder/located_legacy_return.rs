//! Located adapter for one exact value-bearing Return statement.
//!
//! The adapter seals `Return { value: Some(_) }`, preserves the statement
//! span/recursion shell, and derives the value only through the existing
//! `ReturnValue` role. It owns no call-row claim, Return completion, Match
//! policy, raw fallback, or production located root.

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::LegacyStmtInputV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, ValueId};

use super::{
    with_legacy_expression_recursion_guard_v1, LocatedLegacyLoweringErrorV1,
    LocatedLegacyLoweringSessionV1,
};
use crate::mir::builder::recursive_child_lowering::drive_legacy_expression_v1;
use crate::mir::builder::stmts::return_statement_descent::{
    drive_value_return_statement_v1, ReturnStatementDescentPortV1, ReturnStatementSyntaxViewV1,
};
use crate::mir::builder::stmts::return_stmt::try_apply_match_return_optimization;

pub(in crate::mir::builder) struct LocatedValueReturnInputV1<'plan> {
    statement: LegacyStmtInputV1<'plan>,
}

pub(super) fn select_exact_value_return_v1<'plan>(
    input: LegacyStmtInputV1<'plan>,
) -> Result<LocatedValueReturnInputV1<'plan>, LegacyStmtInputV1<'plan>> {
    match input.node() {
        ASTNode::Return { value: Some(_), .. } => {}
        _ => return Err(input),
    }
    Ok(LocatedValueReturnInputV1 { statement: input })
}

pub(super) fn lower_selected_value_return_v1<'plan>(
    session: &mut LocatedLegacyLoweringSessionV1<'plan>,
    builder: &mut MirBuilder,
    selected: LocatedValueReturnInputV1<'plan>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    let guarded_node_kind = std::mem::discriminant(selected.statement.node());
    let statement_span = selected.statement.node().span();

    builder.metadata_ctx.set_current_span(statement_span);
    with_legacy_expression_recursion_guard_v1(builder, guarded_node_kind, |builder| {
        drive_value_return_statement_v1(
            builder,
            session,
            selected,
            lower_located_value_return_after_probe_v1,
        )
    })
    .map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

impl<'plan> ReturnStatementDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type ReturnInput = LocatedValueReturnInputV1<'plan>;

    fn return_value_syntax<'input>(
        &self,
        input: &'input Self::ReturnInput,
    ) -> Result<ReturnStatementSyntaxViewV1<'input>, String> {
        match input.statement.node() {
            ASTNode::Return {
                value: Some(value), ..
            } => Ok(ReturnStatementSyntaxViewV1::new(value)),
            _ => Err("[freeze:contract][located-return/value-input-requires-return]".to_owned()),
        }
    }

    fn try_match_return_optimization(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::ReturnInput,
        value: &ASTNode,
    ) -> Result<Option<ValueId>, String> {
        if !matches!(value, ASTNode::MatchExpr { .. }) {
            return Ok(None);
        }
        self.ledger
            .prove_stmt_inactive(&input.statement)
            .map_err(|error| format!("[located-lowering/ledger] {error:?}"))?;
        try_apply_match_return_optimization(builder, Some(value), true)
    }
}

fn lower_located_value_return_after_probe_v1<'plan>(
    builder: &mut MirBuilder,
    session: &mut LocatedLegacyLoweringSessionV1<'plan>,
    input: LocatedValueReturnInputV1<'plan>,
) -> Result<ValueId, String> {
    let expression = session
        .source
        .child_expr_from_stmt(&input.statement, ExprChildRoleV1::ReturnValue)
        .map_err(|error| format!("[located-lowering/location] {error:?}"))?;
    drive_legacy_expression_v1(builder, session, expression)
}
