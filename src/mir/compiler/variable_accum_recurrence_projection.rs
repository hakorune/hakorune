//! Resolver-backed source observation for the bounded variable recurrence.
//!
//! This module is the only S6A layer which opens the source view.  It consumes
//! resolver-issued Loop membership and BindingRef rows, then emits one neutral
//! `VerifiedVariableAccumRecurrenceFactsV1`.  It never creates Recipe keys or
//! physical identities.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::loop_structural_facts::{
    issue_variable_accum_recurrence_facts_v1, VariableAccumRecurrenceAccumulatorUpdateV1,
    VariableAccumRecurrenceBindingObservationV1, VariableAccumRecurrenceBindingRoleV1,
    VariableAccumRecurrenceConditionObservationV1, VariableAccumRecurrenceConditionOperatorV1,
    VariableAccumRecurrenceCoverageV1, VariableAccumRecurrenceFactsIssueV1,
    VariableAccumRecurrenceInductionStepV1, VariableAccumRecurrenceInputObservationV1,
    VariableAccumRecurrenceInputRoleV1, VariableAccumRecurrenceSourceRoleV1,
    VariableAccumRecurrenceValueClassV1, VerifiedVariableAccumRecurrenceFactsV1,
};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, CallableSemanticSourceLedgerView, ExprChildRoleV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumRecurrenceProjectionRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    SourceNavigation,
    MissingResolverEvidence,
    RootShape,
    BodyShape,
    ConditionShape,
    UpdateShape,
    StepShape,
    BindingShape,
    ConstantShape,
    BindingMismatch,
    Facts(VariableAccumRecurrenceFactsIssueV1),
}

pub(crate) fn issue_variable_accum_recurrence_facts_from_membership_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    membership: VerifiedCallableLoopMembershipV1,
) -> Result<VerifiedVariableAccumRecurrenceFactsV1, VariableAccumRecurrenceProjectionRejectV1> {
    if input.owner() != ledger.owner() || membership.scope_region().scope().owner() != input.owner()
    {
        return Err(VariableAccumRecurrenceProjectionRejectV1::ForeignOwner);
    }
    let source = input.source();
    let loop_stmt = source
        .stmt_at(&membership)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if !membership.source().matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    ) {
        return Err(VariableAccumRecurrenceProjectionRejectV1::SourceIdentityMismatch);
    }
    let root = source
        .root_body()
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if root.statements().len() != 5 {
        return Err(VariableAccumRecurrenceProjectionRejectV1::RootShape);
    }
    let induction_stmt = source
        .body_stmt(&root, 0)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let accumulator_stmt = source
        .body_stmt(&root, 1)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let loop_at_root = source
        .body_stmt(&root, 2)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if loop_at_root.site() != loop_stmt.site() {
        return Err(VariableAccumRecurrenceProjectionRejectV1::SourceIdentityMismatch);
    }
    let body = source
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Err(VariableAccumRecurrenceProjectionRejectV1::BodyShape);
    }
    let update_stmt = source
        .body_stmt(&body, 0)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let step_stmt = source
        .body_stmt(&body, 1)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;

    let induction = declaration_binding(
        ledger,
        induction_stmt.site(),
        VariableAccumRecurrenceProjectionRejectV1::BindingShape,
    )?;
    let accumulator = declaration_binding(
        ledger,
        accumulator_stmt.site(),
        VariableAccumRecurrenceProjectionRejectV1::BindingShape,
    )?;
    ensure_single_local(induction_stmt.node())?;
    ensure_single_local(accumulator_stmt.node())?;
    let induction_initializer = initializer(&source, &induction_stmt)?;
    let accumulator_initializer = initializer(&source, &accumulator_stmt)?;

    let condition = observe_condition(&source, input, &loop_stmt)?;
    let update = observe_update(
        &source,
        input,
        &update_stmt,
        VariableAccumRecurrenceProjectionRejectV1::UpdateShape,
    )?;
    let step = observe_step(
        &source,
        input,
        &step_stmt,
        VariableAccumRecurrenceProjectionRejectV1::StepShape,
    )?;
    if induction == accumulator
        || condition.induction() != induction
        || update.accumulator() != accumulator
        || update.induction() != induction
        || step.induction() != induction
    {
        return Err(VariableAccumRecurrenceProjectionRejectV1::BindingMismatch);
    }

    let bindings = [
        VariableAccumRecurrenceBindingObservationV1::new(
            VariableAccumRecurrenceBindingRoleV1::Induction,
            induction,
            local_site(induction_stmt.site()),
            VariableAccumRecurrenceValueClassV1::I64,
        ),
        VariableAccumRecurrenceBindingObservationV1::new(
            VariableAccumRecurrenceBindingRoleV1::Accumulator,
            accumulator,
            local_site(accumulator_stmt.site()),
            VariableAccumRecurrenceValueClassV1::I64,
        ),
    ];
    let inputs = [
        VariableAccumRecurrenceInputObservationV1::new(
            VariableAccumRecurrenceInputRoleV1::InductionInitial,
            local_site(induction_stmt.site()),
            induction_initializer,
            induction,
            VariableAccumRecurrenceValueClassV1::I64,
        ),
        VariableAccumRecurrenceInputObservationV1::new(
            VariableAccumRecurrenceInputRoleV1::AccumulatorInitial,
            local_site(accumulator_stmt.site()),
            accumulator_initializer,
            accumulator,
            VariableAccumRecurrenceValueClassV1::I64,
        ),
    ];
    let coverage = VariableAccumRecurrenceCoverageV1::new(
        root.statements().len() as u32,
        vec![update.statement().clone(), step.statement().clone()].into_boxed_slice(),
        VariableAccumRecurrenceSourceRoleV1::ALL
            .into_iter()
            .collect(),
    );
    let (loop_source, _frame, scope_region) = membership.into_parts();
    issue_variable_accum_recurrence_facts_v1(
        input.owner(),
        loop_source,
        scope_region,
        bindings,
        inputs,
        condition,
        update,
        step,
        coverage,
    )
    .map_err(VariableAccumRecurrenceProjectionRejectV1::Facts)
}

fn observe_condition(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<VariableAccumRecurrenceConditionObservationV1, VariableAccumRecurrenceProjectionRejectV1>
{
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        condition.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            ..
        }
    ) {
        return Err(VariableAccumRecurrenceProjectionRejectV1::ConditionShape);
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let induction = lexical_local(input, lhs.site())?;
    let bound = integer(rhs.node())?;
    Ok(VariableAccumRecurrenceConditionObservationV1::new(
        condition.site().clone(),
        lhs.site().clone(),
        rhs.site().clone(),
        induction,
        bound,
        VariableAccumRecurrenceConditionOperatorV1::Less,
    ))
}

fn observe_update(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    error: VariableAccumRecurrenceProjectionRejectV1,
) -> Result<VariableAccumRecurrenceAccumulatorUpdateV1, VariableAccumRecurrenceProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(error);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        value.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) {
        return Err(error);
    }
    let accumulator = assignment_binding(input, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let lhs_binding = lexical_local(input, lhs.site())?;
    let induction = lexical_local(input, rhs.site())?;
    if lhs_binding != accumulator {
        return Err(VariableAccumRecurrenceProjectionRejectV1::BindingMismatch);
    }
    Ok(VariableAccumRecurrenceAccumulatorUpdateV1::new(
        statement.site().clone(),
        target.site().clone(),
        value.site().clone(),
        lhs.site().clone(),
        rhs.site().clone(),
        accumulator,
        induction,
    ))
}

fn observe_step(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    error: VariableAccumRecurrenceProjectionRejectV1,
) -> Result<VariableAccumRecurrenceInductionStepV1, VariableAccumRecurrenceProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(error);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        value.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) {
        return Err(error);
    }
    let induction = assignment_binding(input, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)?;
    if lexical_local(input, lhs.site())? != induction {
        return Err(VariableAccumRecurrenceProjectionRejectV1::BindingMismatch);
    }
    Ok(VariableAccumRecurrenceInductionStepV1::new(
        statement.site().clone(),
        target.site().clone(),
        value.site().clone(),
        lhs.site().clone(),
        rhs.site().clone(),
        induction,
        integer(rhs.node())?,
    ))
}

fn initializer(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<SourceExprSiteV1, VariableAccumRecurrenceProjectionRejectV1> {
    source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(0))
        .map(|expr| expr.site().clone())
        .map_err(|_| VariableAccumRecurrenceProjectionRejectV1::SourceNavigation)
}

fn ensure_single_local(
    statement: &ASTNode,
) -> Result<(), VariableAccumRecurrenceProjectionRejectV1> {
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = statement
    else {
        return Err(VariableAccumRecurrenceProjectionRejectV1::RootShape);
    };
    if variables.len() != 1
        || initial_values.len() != 1
        || declared_type_names.len() != 1
        || initial_values[0].is_none()
    {
        return Err(VariableAccumRecurrenceProjectionRejectV1::RootShape);
    }
    Ok(())
}

fn declaration_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    statement: &SourceStmtSiteV1,
    error: VariableAccumRecurrenceProjectionRejectV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumRecurrenceProjectionRejectV1>
{
    let site = local_site(statement);
    ledger
        .declaration_sites()
        .any(|candidate| candidate == &site)
        .then(|| ledger.declaration_binding(&site).ok_or(error))
        .ok_or(VariableAccumRecurrenceProjectionRejectV1::MissingResolverEvidence)?
}

fn local_site(statement: &SourceStmtSiteV1) -> SourceBindingSiteV1 {
    SourceBindingSiteV1::Local {
        statement: statement.clone(),
        ordinal: 0,
    }
}

fn lexical_local(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumRecurrenceProjectionRejectV1>
{
    match input.function().variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) => {
            Err(VariableAccumRecurrenceProjectionRejectV1::BindingShape)
        }
        None => Err(VariableAccumRecurrenceProjectionRejectV1::MissingResolverEvidence),
    }
}

fn assignment_binding(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumRecurrenceProjectionRejectV1>
{
    match input.function().assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(_) => Err(VariableAccumRecurrenceProjectionRejectV1::BindingShape),
        None => Err(VariableAccumRecurrenceProjectionRejectV1::MissingResolverEvidence),
    }
}

fn integer(node: &ASTNode) -> Result<i64, VariableAccumRecurrenceProjectionRejectV1> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value),
        _ => Err(VariableAccumRecurrenceProjectionRejectV1::ConstantShape),
    }
}
