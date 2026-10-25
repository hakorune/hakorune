//! Exact resolver-backed source observation for a variable-bound multiply loop.
//!
//! This is source shape only. The package must combine it with the Home
//! scanner's pre-loop Integer classes before issuing executable Facts/Recipe
//! or a post-loop After loan. Neither names nor this observation prove class.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, CallableSemanticSourceLedgerView, ExprChildRoleV1,
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedScopeRegionPairV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
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
    InputSource,
}

/// Only a morphology mismatch is unselected. Once the whole source shape is
/// selected, missing or contradictory resolver evidence is a terminal error.
#[derive(Debug)]
pub(crate) enum VariableBoundMulSourceAttemptV1 {
    Selected(ObservedVariableBoundMulSourceV1),
    Unselected(VariableBoundMulSourceRejectV1),
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
    condition_operands: [SourceExprSiteV1; 2],
    update: SourceStmtSiteV1,
    update_value: SourceExprSiteV1,
    step: SourceStmtSiteV1,
    step_value: SourceExprSiteV1,
    operation_operands: [[SourceExprSiteV1; 3]; 2],
    scale: BindingRefV1,
    induction: BindingRefV1,
    bound: BindingRefV1,
    inputs: [(SourceBindingSiteV1, SourceExprSiteV1); 3],
}

impl ObservedVariableBoundMulSourceV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        VerifiedResolvedLoopSourceV1,
        LoopExecutionFrameKeyV1,
        ResolvedScopeRegionPairV1,
        SourceExprSiteV1,
        [SourceExprSiteV1; 2],
        [(SourceStmtSiteV1, SourceExprSiteV1); 2],
        [[SourceExprSiteV1; 3]; 2],
        [BindingRefV1; 3],
        [(SourceBindingSiteV1, SourceExprSiteV1); 3],
    ) {
        (
            self.owner,
            self.loop_source,
            self.frame,
            self.scope_region,
            self.condition,
            self.condition_operands,
            [
                (self.update, self.update_value),
                (self.step, self.step_value),
            ],
            self.operation_operands,
            [self.scale, self.induction, self.bound],
            self.inputs,
        )
    }

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

pub(crate) fn attempt_variable_bound_mul_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    membership: VerifiedCallableLoopMembershipV1,
) -> Result<VariableBoundMulSourceAttemptV1, VariableBoundMulSourceRejectV1> {
    use VariableBoundMulSourceAttemptV1 as Attempt;
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
        return Ok(Attempt::Unselected(Reject::Condition));
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Reject::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Reject::SourceNavigation)?;
    if !matches!(lhs.node(), ASTNode::Variable { .. }) || !matches!(rhs.node(), ASTNode::Variable { .. }) {
        return Ok(Attempt::Unselected(Reject::Condition));
    }

    let body = source
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| Reject::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Ok(Attempt::Unselected(Reject::Body));
    }
    let update = source
        .body_stmt(&body, 0)
        .map_err(|_| Reject::SourceNavigation)?;
    let step = source
        .body_stmt(&body, 1)
        .map_err(|_| Reject::SourceNavigation)?;
    let Some((update_target, update_value, update_lhs, update_rhs)) =
        assignment_parts(&source, &update, BinaryOperator::Multiply)? else {
            return Ok(Attempt::Unselected(Reject::Update));
        };
    if !integer(update_rhs.node(), 2) { return Ok(Attempt::Unselected(Reject::Update)); }
    let Some((step_target, step_value, step_lhs, step_rhs)) =
        assignment_parts(&source, &step, BinaryOperator::Add)? else {
            return Ok(Attempt::Unselected(Reject::Step));
        };
    if !integer(step_rhs.node(), 1) { return Ok(Attempt::Unselected(Reject::Step)); }

    // Complete morphology before asking for binding evidence. A later shape
    // mismatch cannot conceal missing/contradictory evidence on a selected row.
    let induction = local(input, lhs.site(), lhs.node(), Reject::Condition)?;
    let bound = local(input, rhs.site(), rhs.node(), Reject::Condition)?;
    let scale = rebound(input, update_target.site(), Reject::Update)?;
    if local(input, update_lhs.site(), update_lhs.node(), Reject::Update)? != scale {
        return Err(Reject::Update);
    }
    if rebound(input, step_target.site(), Reject::Step)? != induction
        || local(input, step_lhs.site(), step_lhs.node(), Reject::Step)? != induction {
        return Err(Reject::Step);
    }
    if scale == induction || scale == bound || induction == bound {
        return Err(Reject::BindingConflict);
    }
    // This producer owns initialized LOCAL inputs only. An original formal
    // is a different supported source purpose, not missing resolver evidence;
    // the common source Loop owner must retain its entry/read correspondence.
    if [scale, induction, bound].iter().any(|binding| matches!(
        input.function().binding(*binding).map(|row| row.kind()),
        Some(crate::mir::resolved_semantics::BindingKindV1::Parameter { .. })
    )) {
        return Ok(Attempt::Unselected(Reject::InputSource));
    }
    let inputs = [
        input_source(input, scale)?,
        input_source(input, induction)?,
        input_source(input, bound)?,
    ];
    let (loop_source, frame, scope_region) = membership.into_parts();
    Ok(Attempt::Selected(ObservedVariableBoundMulSourceV1 {
        owner: input.owner(),
        loop_source,
        frame,
        scope_region,
        condition: condition.site().clone(),
        condition_operands: [lhs.site().clone(), rhs.site().clone()],
        update: update.site().clone(),
        update_value: update_value.site().clone(),
        step: step.site().clone(),
        step_value: step_value.site().clone(),
        operation_operands: [
            [
                update_target.site().clone(),
                update_lhs.site().clone(),
                update_rhs.site().clone(),
            ],
            [
                step_target.site().clone(),
                step_lhs.site().clone(),
                step_rhs.site().clone(),
            ],
        ],
        scale,
        induction,
        bound,
        inputs,
    }))
}

fn input_source(
    input: ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> Result<(SourceBindingSiteV1, SourceExprSiteV1), VariableBoundMulSourceRejectV1> {
    let function = input.function();
    let mut declarations = function
        .declaration_sites()
        .filter(|site| function.declaration_binding(site) == Some(binding));
    let declaration = match (declarations.next(), declarations.next()) {
        (Some(site @ SourceBindingSiteV1::Local { .. }), None) => site.clone(),
        _ => return Err(VariableBoundMulSourceRejectV1::InputSource),
    };
    let initializer = function
        .expression_source()
        .initializer(&declaration)
        .and_then(|relation| relation.initializer_site().cloned())
        .ok_or(VariableBoundMulSourceRejectV1::InputSource)?;
    Ok((declaration, initializer))
}

fn assignment_parts<'a>(
    source: &FunctionSourceViewV1<'a>,
    statement: &LocatedStmtV1<'a>,
    operator: BinaryOperator,
) -> Result<
    Option<(
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
        super::located::LocatedExprV1<'a>,
    )>,
    VariableBoundMulSourceRejectV1,
> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Ok(None);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| VariableBoundMulSourceRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| VariableBoundMulSourceRejectV1::SourceNavigation)?;
    if !matches!(value.node(), ASTNode::BinaryOp { operator: found, .. } if *found == operator) {
        return Ok(None);
    }
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| VariableBoundMulSourceRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| VariableBoundMulSourceRejectV1::SourceNavigation)?;
    if !matches!(target.node(), ASTNode::Variable { .. }) || !matches!(lhs.node(), ASTNode::Variable { .. }) {
        return Ok(None);
    }
    Ok(Some((target, value, lhs, rhs)))
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
