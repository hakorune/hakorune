//! Caller-zero syntax observer for the bounded Main0 in-body-step profile.
//!
//! This module is the only owner allowed to inspect the exact source view for
//! the `app_main0_continue_to_portable_migration` cohort's second profile.
//! The published product owns source sites and neutral, as-written shapes
//! only; it never carries AST, names as identity, ValueIds, or Recipe
//! meaning into a downstream consumer.
//!
//! The admitted shape is exactly:
//!
//! ```text
//! local <carrier> = <integer>
//! local <effect>  = <integer>
//! loop(<carrier> < <integer>) {
//!     <carrier> = <carrier> + <integer>
//!     <effect>  = <integer>
//! }
//! return <carrier>
//! ```

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1,
};

use super::callable_single_loop_source_shapes::{
    binary_operator_shape, expr_shape, literal_shape, literal_shape_from_expr,
    SourceExprShapeV1, SourceLiteralShapeV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::main0_continue_syntax_facts::{
    Main0ConditionFactsV1, Main0InitialLocalFactV1, Main0StepFactsV1, Main0TailReturnFactV1,
};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1, FunctionSourceViewV1};

/// One `target = <integer literal>` assignment whose target is a variable
/// reference and whose value is an integer literal. The in-body-step
/// profile's second body statement is this write-only rebind; it is the
/// shape the callable loop handoff refuses as `incomplete-binding-coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0LiteralAssignFactV1 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    target_shape: SourceExprShapeV1,
    value_site: SourceExprSiteV1,
    value_shape: SourceLiteralShapeV1,
}

impl Main0LiteralAssignFactV1 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn target_shape(&self) -> &SourceExprShapeV1 {
        &self.target_shape
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn value_shape(&self) -> &SourceLiteralShapeV1 {
        &self.value_shape
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Main0InBodyStepSyntaxFactsRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    LoopCardinality,
    SourceNavigation,
    LoopShape,
    RootBodyShape,
    InitialLocalShape,
    ConditionShape,
    ConditionOperandShape,
    ConditionBoundNotLiteral,
    LoopBodyShape,
    StepShape,
    StepTargetShape,
    StepOperandShape,
    StepDeltaNotLiteral,
    EffectShape,
    EffectTargetShape,
    EffectValueNotLiteral,
    TailShape,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0InBodyStepSyntaxFactsV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_context: VerifiedCallableLoopMembershipV1,
    locals: [Main0InitialLocalFactV1; 2],
    condition: Main0ConditionFactsV1,
    step: Main0StepFactsV1,
    effect: Main0LiteralAssignFactV1,
    tail: Main0TailReturnFactV1,
    _seal: VerifiedMain0InBodyStepSyntaxFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedMain0InBodyStepSyntaxFactsSealV1;

impl VerifiedMain0InBodyStepSyntaxFactsV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn loop_context(&self) -> &VerifiedCallableLoopMembershipV1 {
        &self.loop_context
    }

    pub(crate) fn locals(&self) -> &[Main0InitialLocalFactV1; 2] {
        &self.locals
    }

    pub(crate) fn condition(&self) -> &Main0ConditionFactsV1 {
        &self.condition
    }

    pub(crate) fn step(&self) -> &Main0StepFactsV1 {
        &self.step
    }

    pub(crate) fn effect(&self) -> &Main0LiteralAssignFactV1 {
        &self.effect
    }

    pub(crate) fn tail(&self) -> &Main0TailReturnFactV1 {
        &self.tail
    }
}

/// Issue the neutral source facts through the resolver-owned exact Loop seam.
///
/// The resolver chooses the unique Loop membership, and the branded source
/// view reopens that exact statement. This entry point never reconstructs a
/// path, ordinal, or name.
pub(crate) fn issue_main0_in_body_step_syntax_facts_from_ledger_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<VerifiedMain0InBodyStepSyntaxFactsV1, Main0InBodyStepSyntaxFactsRejectV1> {
    if input.owner() != ledger.owner() {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::ForeignOwner);
    }
    let loop_context = ledger
        .only_loop_site()
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::LoopCardinality)?;
    let loop_stmt = input
        .source()
        .stmt_at(&loop_context)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    issue_main0_in_body_step_syntax_facts_v1(input, loop_stmt, loop_context)
}

pub(crate) fn issue_main0_in_body_step_syntax_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: LocatedStmtV1<'_>,
    loop_context: VerifiedCallableLoopMembershipV1,
) -> Result<VerifiedMain0InBodyStepSyntaxFactsV1, Main0InBodyStepSyntaxFactsRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::ForeignOwner);
    }
    let function = input.function();
    if function.loop_region_bundle_count() != 1
        || !loop_context.source().matches_identity(
            function.function_origin(),
            function.source_kind(),
            loop_stmt.site(),
        )
    {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if loop_context.scope_region().scope().owner() != input.owner()
        || loop_context.scope_region().region().owner() != input.owner()
    {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if !matches!(loop_stmt.node(), ASTNode::Loop { .. }) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopShape);
    }

    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    // The bounded profile is exactly [Local, Local, Loop, Return].
    if body.statements().len() != 4 {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::RootBodyShape);
    }
    let first_local = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let second_local = source
        .body_stmt(&body, 1)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let loop_statement = source
        .body_stmt(&body, 2)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let tail_statement = source
        .body_stmt(&body, 3)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    for statement in [&first_local, &second_local] {
        if !matches!(statement.node(), ASTNode::Local { .. }) {
            return Err(Main0InBodyStepSyntaxFactsRejectV1::RootBodyShape);
        }
    }
    if !matches!(loop_statement.node(), ASTNode::Loop { .. }) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::RootBodyShape);
    }
    if loop_statement.site() != loop_stmt.site() {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopCardinality);
    }
    if !matches!(tail_statement.node(), ASTNode::Return { .. }) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::TailShape);
    }

    let locals = [
        observe_local(source, &first_local)?,
        observe_local(source, &second_local)?,
    ];
    let condition = observe_condition(source, &loop_stmt)?;
    let (step, effect) = observe_loop_body(source, &loop_stmt)?;
    let tail = observe_tail(source, &tail_statement)?;

    Ok(VerifiedMain0InBodyStepSyntaxFactsV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_context,
        locals,
        condition,
        step,
        effect,
        tail,
        _seal: VerifiedMain0InBodyStepSyntaxFactsSealV1,
    })
}

fn observe_local(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0InitialLocalFactV1, Main0InBodyStepSyntaxFactsRejectV1> {
    let ASTNode::Local {
        variables,
        declared_type_names,
        ..
    } = statement.node()
    else {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::InitialLocalShape);
    };
    if variables.len() != 1 || declared_type_names.iter().any(Option::is_some) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::InitialLocalShape);
    }
    let initializer = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(0))
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::InitialLocalShape)?;
    let ASTNode::Literal { value, .. } = initializer.node() else {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::InitialLocalShape);
    };
    Ok(Main0InitialLocalFactV1::from_parts(
        statement.site().clone(),
        initializer.site().clone(),
        literal_shape(value),
    ))
}

fn observe_condition(
    source: FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<Main0ConditionFactsV1, Main0InBodyStepSyntaxFactsRejectV1> {
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let operator = match condition.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0InBodyStepSyntaxFactsRejectV1::ConditionShape),
    };
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    let rhs_shape = expr_shape(rhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::ConditionOperandShape);
    }
    // Unlike the continue profile, the predicate bound is an integer literal
    // (`i < 3`), not a second declared local.
    match rhs_shape {
        SourceExprShapeV1::Literal(SourceLiteralShapeV1::Integer(_)) => {}
        _ => return Err(Main0InBodyStepSyntaxFactsRejectV1::ConditionBoundNotLiteral),
    }
    Ok(Main0ConditionFactsV1::from_parts(
        condition.site().clone(),
        lhs.site().clone(),
        lhs_shape,
        rhs.site().clone(),
        rhs_shape,
        operator,
    ))
}

fn observe_loop_body(
    source: FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<(Main0StepFactsV1, Main0LiteralAssignFactV1), Main0InBodyStepSyntaxFactsRejectV1>
{
    let body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    // The bounded profile is exactly [<carrier> = <carrier> + <int>,
    // <effect> = <int>] in written order; no if/else/branch/transfer.
    if body.statements().len() != 2 {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopBodyShape);
    }
    let step_stmt = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let effect_stmt = source
        .body_stmt(&body, 1)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(step_stmt.node(), ASTNode::Assignment { .. })
        || !matches!(effect_stmt.node(), ASTNode::Assignment { .. })
    {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::LoopBodyShape);
    }
    let step = observe_step(source, &step_stmt)?;
    let effect = observe_literal_assign(source, &effect_stmt)?;
    Ok((step, effect))
}

fn observe_step(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0StepFactsV1, Main0InBodyStepSyntaxFactsRejectV1> {
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(target.node(), ASTNode::Variable { .. }) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::StepTargetShape);
    }
    let operator = match value.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0InBodyStepSyntaxFactsRejectV1::StepShape),
    };
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::StepOperandShape);
    }
    let rhs_shape = literal_shape_from_expr(&rhs)
        .ok_or(Main0InBodyStepSyntaxFactsRejectV1::StepDeltaNotLiteral)?;
    Ok(Main0StepFactsV1::from_parts(
        statement.site().clone(),
        target.site().clone(),
        expr_shape(target.node()),
        value.site().clone(),
        lhs.site().clone(),
        lhs_shape,
        rhs.site().clone(),
        rhs_shape,
        operator,
    ))
}

fn observe_literal_assign(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0LiteralAssignFactV1, Main0InBodyStepSyntaxFactsRejectV1> {
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::SourceNavigation)?;
    let target_shape = expr_shape(target.node());
    if !matches!(target_shape, SourceExprShapeV1::Variable) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::EffectTargetShape);
    }
    let value_shape = literal_shape_from_expr(&value)
        .ok_or(Main0InBodyStepSyntaxFactsRejectV1::EffectValueNotLiteral)?;
    if !matches!(value_shape, SourceLiteralShapeV1::Integer(_)) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::EffectValueNotLiteral);
    }
    Ok(Main0LiteralAssignFactV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        target_shape,
        value_site: value.site().clone(),
        value_shape,
    })
}

fn observe_tail(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0TailReturnFactV1, Main0InBodyStepSyntaxFactsRejectV1> {
    let ASTNode::Return { .. } = statement.node() else {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::TailShape);
    };
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| Main0InBodyStepSyntaxFactsRejectV1::TailShape)?;
    let value_shape = expr_shape(value.node());
    if !matches!(value_shape, SourceExprShapeV1::Variable) {
        return Err(Main0InBodyStepSyntaxFactsRejectV1::TailShape);
    }
    Ok(Main0TailReturnFactV1::from_parts(
        statement.site().clone(),
        value.site().clone(),
        value_shape,
    ))
}
