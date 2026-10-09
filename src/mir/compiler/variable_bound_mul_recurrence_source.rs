//! Exact resolver-backed source observation for a variable-bound multiply loop.
//!
//! This is source shape only. The package must combine it with the Home
//! scanner's pre-loop Integer classes before issuing executable Facts/Recipe
//! or a post-loop After loan. Neither names nor this observation prove class.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, CallableSemanticSourceLedgerView, ExprChildRoleV1,
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedScopeRegionPairV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1, VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::source_view::FunctionSourceViewV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableBoundMulSourceRejectV1 {
    ForeignOwner,
    SourceIdentity,
    SourceNavigation,
    MissingResolverEvidence,
    Condition,
    Body,
    Update,
    Step,
    BindingConflict,
}

/// A non-Clone membership and its complete ordered source sites. This cannot
/// be substituted for a typed Facts product or a verified Loop After state.
#[derive(Debug)]
pub(crate) struct ObservedVariableBoundMulSourceV1 {
    owner: FunctionOwnerIdV1,
    loop_source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    condition: SourceExprSiteV1,
    update: SourceStmtSiteV1,
    update_value: SourceExprSiteV1,
    step: SourceStmtSiteV1,
    step_value: SourceExprSiteV1,
    scale: BindingRefV1,
    induction: BindingRefV1,
    bound: BindingRefV1,
}

impl ObservedVariableBoundMulSourceV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn loop_source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.loop_source
    }
    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
    pub(crate) fn condition(&self) -> &SourceExprSiteV1 {
        &self.condition
    }
    pub(crate) fn update(&self) -> &SourceStmtSiteV1 {
        &self.update
    }
    pub(crate) fn update_value(&self) -> &SourceExprSiteV1 {
        &self.update_value
    }
    pub(crate) fn step(&self) -> &SourceStmtSiteV1 {
        &self.step
    }
    pub(crate) fn step_value(&self) -> &SourceExprSiteV1 {
        &self.step_value
    }
    pub(crate) const fn bindings(&self) -> [BindingRefV1; 3] {
        [self.scale, self.induction, self.bound]
    }
}

pub(crate) fn observe_variable_bound_mul_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    membership: VerifiedCallableLoopMembershipV1,
) -> Result<ObservedVariableBoundMulSourceV1, VariableBoundMulSourceRejectV1> {
    use VariableBoundMulSourceRejectV1 as Reject;
    if input.owner() != ledger.owner() || membership.scope_region().scope().owner() != input.owner()
    {
        return Err(Reject::ForeignOwner);
    }
    let source = input.source();
    let loop_stmt = source
        .stmt_at(&membership)
        .map_err(|_| Reject::SourceNavigation)?;
    if !membership.source().matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    ) {
        return Err(Reject::SourceIdentity);
    }
    let condition = source
        .child_expr_from_stmt(&loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| Reject::SourceNavigation)?;
    if !matches!(
        condition.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            ..
        }
    ) {
        return Err(Reject::Condition);
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Reject::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Reject::SourceNavigation)?;
    let induction = local(input, lhs.site(), lhs.node(), Reject::Condition)?;
    let bound = local(input, rhs.site(), rhs.node(), Reject::Condition)?;

    let body = source
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| Reject::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Err(Reject::Body);
    }
    let update = source
        .body_stmt(&body, 0)
        .map_err(|_| Reject::SourceNavigation)?;
    let step = source
        .body_stmt(&body, 1)
        .map_err(|_| Reject::SourceNavigation)?;
    let (update_target, update_value, update_lhs, update_rhs) =
        assignment_parts(&source, &update, BinaryOperator::Multiply, Reject::Update)?;
    let scale = rebound(input, update_target.site(), Reject::Update)?;
    if local(input, update_lhs.site(), update_lhs.node(), Reject::Update)? != scale
        || !integer(update_rhs.node(), 2)
    {
        return Err(Reject::Update);
    }

    let (step_target, step_value, step_lhs, step_rhs) =
        assignment_parts(&source, &step, BinaryOperator::Add, Reject::Step)?;
    if rebound(input, step_target.site(), Reject::Step)? != induction
        || local(input, step_lhs.site(), step_lhs.node(), Reject::Step)? != induction
        || !integer(step_rhs.node(), 1)
    {
        return Err(Reject::Step);
    }
    if scale == induction || scale == bound || induction == bound {
        return Err(Reject::BindingConflict);
    }
    let (loop_source, frame, scope_region) = membership.into_parts();
    Ok(ObservedVariableBoundMulSourceV1 {
        owner: input.owner(),
        loop_source,
        frame,
        scope_region,
        condition: condition.site().clone(),
        update: update.site().clone(),
        update_value: update_value.site().clone(),
        step: step.site().clone(),
        step_value: step_value.site().clone(),
        scale,
        induction,
        bound,
    })
}

fn assignment_parts<'a>(
    source: &FunctionSourceViewV1<'a>,
    statement: &LocatedStmtV1<'a>,
    operator: BinaryOperator,
    reject: VariableBoundMulSourceRejectV1,
) -> Result<
    (
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
    ),
    VariableBoundMulSourceRejectV1,
> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(reject);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| reject)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| reject)?;
    if !matches!(value.node(), ASTNode::BinaryOp { operator: found, .. } if *found == operator) {
        return Err(reject);
    }
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| reject)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| reject)?;
    Ok((target, value, lhs, rhs))
}

fn local(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    node: &ASTNode,
    reject: VariableBoundMulSourceRejectV1,
) -> Result<BindingRefV1, VariableBoundMulSourceRejectV1> {
    if !matches!(node, ASTNode::Variable { .. }) {
        return Err(reject);
    }
    match input.function().variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(_) => Err(reject),
        None => Err(VariableBoundMulSourceRejectV1::MissingResolverEvidence),
    }
}

fn rebound(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    reject: VariableBoundMulSourceRejectV1,
) -> Result<BindingRefV1, VariableBoundMulSourceRejectV1> {
    match input.function().assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(_) => Err(reject),
        None => Err(VariableBoundMulSourceRejectV1::MissingResolverEvidence),
    }
}

fn integer(node: &ASTNode, expected: i64) -> bool {
    matches!(node, ASTNode::Literal { value: LiteralValue::Integer(value), .. } if *value == expected)
}
