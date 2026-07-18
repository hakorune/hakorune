//! Located adapter for one exact Variable-target Assignment statement.
//!
//! Selection happens once at the statement boundary. The adapter carries the
//! already-located statement into the shared ASN0 driver and derives its sole
//! child only through the existing `AssignmentValue` source role. It owns no
//! call-site claim, ledger order, assignment completion, or raw delegation.

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::LegacyStmtInputV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, ValueId};

use super::{
    with_legacy_expression_recursion_guard_v1, LocatedLegacyLoweringErrorV1,
    LocatedLegacyLoweringSessionV1,
};
use crate::mir::builder::stmts::{
    drive_variable_assignment_v1, VariableAssignmentDescentPortV1, VariableAssignmentSyntaxViewV1,
};

pub(in crate::mir::builder) struct LocatedVariableAssignmentInputV1<'plan> {
    statement: LegacyStmtInputV1<'plan>,
    variable_name: &'plan str,
}

pub(super) fn select_exact_variable_assignment_v1<'plan>(
    input: LegacyStmtInputV1<'plan>,
) -> Result<LocatedVariableAssignmentInputV1<'plan>, LegacyStmtInputV1<'plan>> {
    let variable_name = match input.node() {
        ASTNode::Assignment { target, .. } => match target.as_ref() {
            ASTNode::Variable { name, .. } => name.as_str(),
            _ => return Err(input),
        },
        _ => return Err(input),
    };
    Ok(LocatedVariableAssignmentInputV1 {
        statement: input,
        variable_name,
    })
}

pub(super) fn lower_selected_variable_assignment_v1<'plan>(
    session: &mut LocatedLegacyLoweringSessionV1<'plan>,
    builder: &mut MirBuilder,
    selected: &LocatedVariableAssignmentInputV1<'plan>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    let guarded_node_kind = std::mem::discriminant(selected.statement.node());
    let statement_span = selected.statement.node().span();

    builder.metadata_ctx.set_current_span(statement_span);
    with_legacy_expression_recursion_guard_v1(builder, guarded_node_kind, |builder| {
        drive_variable_assignment_v1(builder, session, selected)
    })
    .map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

impl<'plan> VariableAssignmentDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type VariableAssignmentInput = LocatedVariableAssignmentInputV1<'plan>;

    fn variable_assignment_syntax<'input>(
        &self,
        input: &'input Self::VariableAssignmentInput,
    ) -> Result<VariableAssignmentSyntaxViewV1<'input>, String> {
        Ok(VariableAssignmentSyntaxViewV1::new(input.variable_name))
    }

    fn assignment_rhs_expression_input(
        &self,
        input: &Self::VariableAssignmentInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.source
            .child_expr_from_stmt(&input.statement, ExprChildRoleV1::AssignmentValue)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }
}
