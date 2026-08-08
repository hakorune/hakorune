//! Resolver-backed source observer for the bounded break recurrence fixture.
//!
//! This is the only module in the slice that navigates `FunctionSourceViewV1`.
//! Its output is neutral Facts; Recipe keys and JoinSig edges are issued by
//! the producer and shared logical elaborator.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::loop_structural_facts::{
    issue_variable_accum_break_facts_v1, VariableAccumBreakAssignmentObservationV1,
    VariableAccumBreakBindingObservationV1, VariableAccumBreakBindingRoleV1,
    VariableAccumBreakCompareV1, VariableAccumBreakConditionObservationV1,
    VariableAccumBreakCoverageV1, VariableAccumBreakFactsIssueV1,
    VariableAccumBreakInputObservationV1, VariableAccumBreakInputRoleV1,
    VariableAccumBreakObservationCoverageV1, VariableAccumBreakOperationRoleV1,
    VariableAccumBreakSourceAttemptOutcomeV1, VariableAccumBreakSourceDeclineV1,
    VariableAccumBreakSourceIdentityV1, VariableAccumBreakSourceRejectV1,
    VariableAccumBreakSourceUnresolvedV1, VariableAccumBreakValueClassV1,
    VerifiedVariableAccumBreakFactsV1, VerifiedVariableAccumBreakSourceAttemptV1,
};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, CallableSemanticSourceLedgerView, ExprChildRoleV1, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1, VerifiedCallableLoopMembershipV1,
    VerifiedResolvedFunctionV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableAccumBreakProjectionRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    SourceNavigation,
    MissingResolverEvidence,
    RootShape,
    LoopBodyShape,
    BranchShape,
    BranchBodyShape,
    LoopConditionShape,
    BranchConditionShape,
    AssignmentShape,
    BindingShape,
    ConstantShape,
    BindingMismatch,
    ExitResolution,
    ExitTargetMismatch,
    Facts(VariableAccumBreakFactsIssueV1),
}

pub(crate) fn issue_variable_accum_break_source_attempt_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    membership: VerifiedCallableLoopMembershipV1,
    coverage: VariableAccumBreakObservationCoverageV1,
) -> VerifiedVariableAccumBreakSourceAttemptV1 {
    let identity =
        VariableAccumBreakSourceIdentityV1::from_source(input.owner(), membership.source());
    if coverage == VariableAccumBreakObservationCoverageV1::Incomplete {
        return VerifiedVariableAccumBreakSourceAttemptV1::new(
            VariableAccumBreakSourceAttemptOutcomeV1::Unresolved(
                VariableAccumBreakSourceUnresolvedV1::IncompleteCoverage,
            ),
            identity,
            coverage,
        );
    }
    let outcome =
        match issue_variable_accum_break_facts_from_membership_v1(input, ledger, membership) {
            Ok(facts) => VariableAccumBreakSourceAttemptOutcomeV1::Candidate(facts),
            Err(reject) => map_projection_reject(reject),
        };
    VerifiedVariableAccumBreakSourceAttemptV1::new(outcome, identity, coverage)
}

fn map_projection_reject(
    reject: VariableAccumBreakProjectionRejectV1,
) -> VariableAccumBreakSourceAttemptOutcomeV1 {
    match reject {
        VariableAccumBreakProjectionRejectV1::ForeignOwner => {
            VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
                VariableAccumBreakSourceRejectV1::ForeignOwner,
            )
        }
        VariableAccumBreakProjectionRejectV1::SourceIdentityMismatch => {
            VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
                VariableAccumBreakSourceRejectV1::SourceIdentityMismatch,
            )
        }
        VariableAccumBreakProjectionRejectV1::ExitTargetMismatch => {
            VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
                VariableAccumBreakSourceRejectV1::ExitTargetMismatch,
            )
        }
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::ForeignOwner,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::ForeignOwner,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::ForeignFrame,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::ForeignFrame,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::SourceSiteConflict,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::SourceSiteConflict,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::RoleConflict,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::DuplicateRole,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::BindingConflict,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::BindingConflict,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::InputConflict,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::InputConflict,
        ),
        VariableAccumBreakProjectionRejectV1::Facts(
            VariableAccumBreakFactsIssueV1::CoverageConflict,
        ) => VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::CoverageConflict,
        ),
        VariableAccumBreakProjectionRejectV1::SourceNavigation
        | VariableAccumBreakProjectionRejectV1::MissingResolverEvidence
        | VariableAccumBreakProjectionRejectV1::ExitResolution => {
            VariableAccumBreakSourceAttemptOutcomeV1::Unresolved(
                VariableAccumBreakSourceUnresolvedV1::MissingEvidence,
            )
        }
        VariableAccumBreakProjectionRejectV1::RootShape
        | VariableAccumBreakProjectionRejectV1::LoopBodyShape
        | VariableAccumBreakProjectionRejectV1::BranchShape
        | VariableAccumBreakProjectionRejectV1::BranchBodyShape
        | VariableAccumBreakProjectionRejectV1::LoopConditionShape
        | VariableAccumBreakProjectionRejectV1::BranchConditionShape
        | VariableAccumBreakProjectionRejectV1::AssignmentShape
        | VariableAccumBreakProjectionRejectV1::BindingShape
        | VariableAccumBreakProjectionRejectV1::ConstantShape
        | VariableAccumBreakProjectionRejectV1::BindingMismatch => {
            VariableAccumBreakSourceAttemptOutcomeV1::Declined(
                VariableAccumBreakSourceDeclineV1::NotVariableAccumBreakShape,
            )
        }
    }
}

pub(crate) fn issue_variable_accum_break_facts_from_membership_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    membership: VerifiedCallableLoopMembershipV1,
) -> Result<VerifiedVariableAccumBreakFactsV1, VariableAccumBreakProjectionRejectV1> {
    if input.owner() != ledger.owner() || membership.scope_region().scope().owner() != input.owner()
    {
        return Err(VariableAccumBreakProjectionRejectV1::ForeignOwner);
    }
    let source = input.source();
    let loop_stmt = source
        .stmt_at(&membership)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if !membership.source().matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    ) {
        return Err(VariableAccumBreakProjectionRejectV1::SourceIdentityMismatch);
    }
    let root = source
        .root_body()
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if root.statements().len() != 4 {
        return Err(VariableAccumBreakProjectionRejectV1::RootShape);
    }
    let sum_stmt = source
        .body_stmt(&root, 0)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let induction_stmt = source
        .body_stmt(&root, 1)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let loop_at_root = source
        .body_stmt(&root, 2)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if loop_at_root.site() != loop_stmt.site() {
        return Err(VariableAccumBreakProjectionRejectV1::SourceIdentityMismatch);
    }
    let loop_body = source
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if loop_body.statements().len() != 3 {
        return Err(VariableAccumBreakProjectionRejectV1::LoopBodyShape);
    }
    let branch = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let ASTNode::If { else_body, .. } = branch.node() else {
        return Err(VariableAccumBreakProjectionRejectV1::BranchShape);
    };
    if else_body.is_some() {
        return Err(VariableAccumBreakProjectionRejectV1::BranchShape);
    }
    input
        .function()
        .if_region_bundle(branch.site())
        .map_err(|_| VariableAccumBreakProjectionRejectV1::MissingResolverEvidence)?;
    let then_body = source
        .child_body_from_stmt(&branch, BodyChildRoleV1::IfThen)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if then_body.statements().len() != 2 {
        return Err(VariableAccumBreakProjectionRejectV1::BranchBodyShape);
    }
    let terminal_stmt = source
        .body_stmt(&then_body, 0)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let break_stmt = source
        .body_stmt(&then_body, 1)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if !matches!(break_stmt.node(), ASTNode::Break { .. }) {
        return Err(VariableAccumBreakProjectionRejectV1::BranchShape);
    }
    let normal_stmt = source
        .body_stmt(&loop_body, 1)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let step_stmt = source
        .body_stmt(&loop_body, 2)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;

    let sum = declaration_binding(ledger, sum_stmt.site())?;
    let induction = declaration_binding(ledger, induction_stmt.site())?;
    ensure_single_local(sum_stmt.node())?;
    ensure_single_local(induction_stmt.node())?;
    let inputs = [
        VariableAccumBreakInputObservationV1::new(
            VariableAccumBreakInputRoleV1::InductionInitial,
            local_site(induction_stmt.site()),
            initializer(&source, &induction_stmt)?,
            induction,
            VariableAccumBreakValueClassV1::I64,
        ),
        VariableAccumBreakInputObservationV1::new(
            VariableAccumBreakInputRoleV1::AccumulatorInitial,
            local_site(sum_stmt.site()),
            initializer(&source, &sum_stmt)?,
            sum,
            VariableAccumBreakValueClassV1::I64,
        ),
    ];
    let loop_condition = observe_condition(
        &source,
        input,
        &loop_stmt,
        BinaryOperator::Less,
        VariableAccumBreakCompareV1::Less,
    )?;
    let branch_condition = observe_condition(
        &source,
        input,
        &branch,
        BinaryOperator::Equal,
        VariableAccumBreakCompareV1::Equal,
    )?;
    let terminal_update = observe_assignment(&source, input, &terminal_stmt, 10)?;
    let normal_update = observe_assignment(&source, input, &normal_stmt, 1)?;
    let induction_step = observe_assignment(&source, input, &step_stmt, 1)?;
    if loop_condition.binding() != induction
        || branch_condition.binding() != induction
        || terminal_update.target_binding() != sum
        || normal_update.target_binding() != sum
        || induction_step.target_binding() != induction
    {
        return Err(VariableAccumBreakProjectionRejectV1::BindingMismatch);
    }
    let loop_region = input
        .function()
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| VariableAccumBreakProjectionRejectV1::MissingResolverEvidence)?
        .loop_pair()
        .region();
    verify_break(input.function(), break_stmt.site(), loop_region)?;
    let covered = vec![
        branch.site().clone(),
        terminal_stmt.site().clone(),
        break_stmt.site().clone(),
        normal_stmt.site().clone(),
        step_stmt.site().clone(),
    ]
    .into_boxed_slice();
    let coverage = VariableAccumBreakCoverageV1::new(
        root.statements().len() as u32,
        covered,
        VariableAccumBreakOperationRoleV1::ALL.into_iter().collect(),
    );
    let (loop_source, _frame, scope_region) = membership.into_parts();
    issue_variable_accum_break_facts_v1(
        input.owner(),
        loop_source,
        scope_region,
        [
            VariableAccumBreakBindingObservationV1::new(
                VariableAccumBreakBindingRoleV1::Induction,
                induction,
                local_site(induction_stmt.site()),
                VariableAccumBreakValueClassV1::I64,
            ),
            VariableAccumBreakBindingObservationV1::new(
                VariableAccumBreakBindingRoleV1::Accumulator,
                sum,
                local_site(sum_stmt.site()),
                VariableAccumBreakValueClassV1::I64,
            ),
        ],
        inputs,
        loop_condition,
        branch_condition,
        terminal_update,
        normal_update,
        induction_step,
        branch.site().clone(),
        break_stmt.site().clone(),
        coverage,
    )
    .map_err(VariableAccumBreakProjectionRejectV1::Facts)
}

fn observe_condition(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    expected: BinaryOperator,
    operator: VariableAccumBreakCompareV1,
) -> Result<VariableAccumBreakConditionObservationV1, VariableAccumBreakProjectionRejectV1> {
    let condition = source
        .child_expr_from_stmt(
            statement,
            if matches!(statement.node(), ASTNode::Loop { .. }) {
                ExprChildRoleV1::LoopCondition
            } else {
                ExprChildRoleV1::IfCondition
            },
        )
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if !matches!(condition.node(), ASTNode::BinaryOp { operator, .. } if *operator == expected) {
        return Err(VariableAccumBreakProjectionRejectV1::LoopConditionShape);
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let binding = lexical_local(input, lhs.site())?;
    let bound = integer(rhs.node())?;
    Ok(VariableAccumBreakConditionObservationV1::new(
        condition.site().clone(),
        lhs.site().clone(),
        rhs.site().clone(),
        binding,
        bound,
        operator,
    ))
}

fn observe_assignment(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    delta: i64,
) -> Result<VariableAccumBreakAssignmentObservationV1, VariableAccumBreakProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(VariableAccumBreakProjectionRejectV1::AssignmentShape);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        value.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) {
        return Err(VariableAccumBreakProjectionRejectV1::AssignmentShape);
    }
    let target_binding = assignment_binding(input, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)?;
    let lhs_binding = lexical_local(input, lhs.site())?;
    if integer(rhs.node())? != delta || lhs_binding != target_binding {
        return Err(VariableAccumBreakProjectionRejectV1::BindingMismatch);
    }
    Ok(VariableAccumBreakAssignmentObservationV1::new(
        statement.site().clone(),
        target.site().clone(),
        value.site().clone(),
        lhs.site().clone(),
        rhs.site().clone(),
        target_binding,
        lhs_binding,
        delta,
    ))
}

fn verify_break(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
    target_loop: crate::mir::resolved_semantics::RegionId,
) -> Result<(), VariableAccumBreakProjectionRejectV1> {
    let record = function
        .resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()))
        .ok_or(VariableAccumBreakProjectionRejectV1::ExitResolution)?;
    if record.origin() == ResolvedExitOriginV1::ExplicitBreak
        && record.transfer() == (ResolvedControlTransferV1::Break { target_loop })
    {
        Ok(())
    } else {
        Err(VariableAccumBreakProjectionRejectV1::ExitTargetMismatch)
    }
}

fn declaration_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    statement: &SourceStmtSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumBreakProjectionRejectV1> {
    let site = local_site(statement);
    ledger
        .declaration_sites()
        .any(|candidate| candidate == &site)
        .then(|| {
            ledger
                .declaration_binding(&site)
                .ok_or(VariableAccumBreakProjectionRejectV1::MissingResolverEvidence)
        })
        .ok_or(VariableAccumBreakProjectionRejectV1::MissingResolverEvidence)?
}

fn local_site(statement: &SourceStmtSiteV1) -> SourceBindingSiteV1 {
    SourceBindingSiteV1::Local {
        statement: statement.clone(),
        ordinal: 0,
    }
}

fn initializer(
    source: &super::source_view::FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<SourceExprSiteV1, VariableAccumBreakProjectionRejectV1> {
    source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(0))
        .map(|expr| expr.site().clone())
        .map_err(|_| VariableAccumBreakProjectionRejectV1::SourceNavigation)
}

fn ensure_single_local(node: &ASTNode) -> Result<(), VariableAccumBreakProjectionRejectV1> {
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = node
    else {
        return Err(VariableAccumBreakProjectionRejectV1::RootShape);
    };
    if variables.len() != 1
        || initial_values.len() != 1
        || declared_type_names.len() != 1
        || initial_values[0].is_none()
    {
        return Err(VariableAccumBreakProjectionRejectV1::RootShape);
    }
    Ok(())
}

fn lexical_local(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumBreakProjectionRejectV1> {
    match input.function().variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) => {
            Err(VariableAccumBreakProjectionRejectV1::BindingShape)
        }
        None => Err(VariableAccumBreakProjectionRejectV1::MissingResolverEvidence),
    }
}

fn assignment_binding(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, VariableAccumBreakProjectionRejectV1> {
    match input.function().assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(_) => Err(VariableAccumBreakProjectionRejectV1::BindingShape),
        None => Err(VariableAccumBreakProjectionRejectV1::MissingResolverEvidence),
    }
}

fn integer(node: &ASTNode) -> Result<i64, VariableAccumBreakProjectionRejectV1> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value),
        _ => Err(VariableAccumBreakProjectionRejectV1::ConstantShape),
    }
}

#[cfg(test)]
#[path = "variable_accum_break_projection_tests.rs"]
mod variable_accum_break_projection_tests;
