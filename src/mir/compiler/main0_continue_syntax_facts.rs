//! Caller-zero syntax observer for the bounded Main0 Continue profile.
//!
//! This module is the only owner allowed to inspect the exact source view for
//! the `app_main0_continue_to_portable_migration` row. The published product
//! owns source sites and neutral, as-written shapes only; it never carries
//! AST, names as identity, ValueIds, or Recipe meaning into a downstream
//! consumer.
//!
//! The admitted shape is exactly:
//!
//! ```text
//! local <carrier> = <integer>
//! local <bound>   = <integer>
//! loop(<carrier> < <bound>) {
//!     if <carrier> == <integer> {
//!         <carrier> = <carrier> + <integer>
//!         continue
//!     }
//!     <carrier> = <carrier> + <integer>
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
    SourceExprShapeV1, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1, FunctionSourceViewV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0InitialLocalFactV1 {
    statement_site: SourceStmtSiteV1,
    initializer_site: SourceExprSiteV1,
    shape: SourceLiteralShapeV1,
}

impl Main0InitialLocalFactV1 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn initializer_site(&self) -> &SourceExprSiteV1 {
        &self.initializer_site
    }

    pub(crate) fn shape(&self) -> &SourceLiteralShapeV1 {
        &self.shape
    }
}

/// Loop head predicate facts.  Both operands are lexical variable reads; the
/// source map assigns carrier/bound roles from resolver membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0ConditionFactsV1 {
    site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    lhs_shape: SourceExprShapeV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceExprShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl Main0ConditionFactsV1 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn lhs_site(&self) -> &SourceExprSiteV1 {
        &self.lhs_site
    }

    pub(crate) fn lhs_shape(&self) -> &SourceExprShapeV1 {
        &self.lhs_shape
    }

    pub(crate) fn rhs_site(&self) -> &SourceExprSiteV1 {
        &self.rhs_site
    }

    pub(crate) fn rhs_shape(&self) -> &SourceExprShapeV1 {
        &self.rhs_shape
    }

    pub(crate) const fn operator(&self) -> SyntaxBinaryOperatorV1 {
        self.operator
    }
}

/// In-loop `if` predicate facts.  The left operand is a variable read and the
/// right operand is an integer literal bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0GuardFactsV1 {
    statement_site: SourceStmtSiteV1,
    condition_site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    lhs_shape: SourceExprShapeV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceLiteralShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl Main0GuardFactsV1 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn condition_site(&self) -> &SourceExprSiteV1 {
        &self.condition_site
    }

    pub(crate) fn lhs_site(&self) -> &SourceExprSiteV1 {
        &self.lhs_site
    }

    pub(crate) fn lhs_shape(&self) -> &SourceExprShapeV1 {
        &self.lhs_shape
    }

    pub(crate) fn rhs_site(&self) -> &SourceExprSiteV1 {
        &self.rhs_site
    }

    pub(crate) fn rhs_shape(&self) -> &SourceLiteralShapeV1 {
        &self.rhs_shape
    }

    pub(crate) const fn operator(&self) -> SyntaxBinaryOperatorV1 {
        self.operator
    }
}

/// One `target = lhs + rhs` assignment whose target and left operand are
/// variable references and whose right operand is an integer literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0StepFactsV1 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    target_shape: SourceExprShapeV1,
    value_site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    lhs_shape: SourceExprShapeV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceLiteralShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl Main0StepFactsV1 {
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

    pub(crate) fn lhs_site(&self) -> &SourceExprSiteV1 {
        &self.lhs_site
    }

    pub(crate) fn lhs_shape(&self) -> &SourceExprShapeV1 {
        &self.lhs_shape
    }

    pub(crate) fn rhs_site(&self) -> &SourceExprSiteV1 {
        &self.rhs_site
    }

    pub(crate) fn rhs_shape(&self) -> &SourceLiteralShapeV1 {
        &self.rhs_shape
    }

    pub(crate) const fn operator(&self) -> SyntaxBinaryOperatorV1 {
        self.operator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0TailReturnFactV1 {
    statement_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value_shape: SourceExprShapeV1,
}

impl Main0TailReturnFactV1 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn value_shape(&self) -> &SourceExprShapeV1 {
        &self.value_shape
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Main0ContinueSyntaxFactsRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    LoopCardinality,
    SourceNavigation,
    LoopShape,
    RootBodyShape,
    InitialLocalShape,
    ConditionShape,
    ConditionOperandShape,
    LoopBodyShape,
    GuardShape,
    GuardOperandShape,
    GuardBoundNotLiteral,
    ElsePresent,
    ThenBodyShape,
    StepShape,
    StepTargetShape,
    StepOperandShape,
    StepDeltaNotLiteral,
    TailShape,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0ContinueSyntaxFactsV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_context: VerifiedCallableLoopMembershipV1,
    locals: [Main0InitialLocalFactV1; 2],
    condition: Main0ConditionFactsV1,
    guard: Main0GuardFactsV1,
    then_step: Main0StepFactsV1,
    continue_site: SourceStmtSiteV1,
    normal_step: Main0StepFactsV1,
    tail: Main0TailReturnFactV1,
    _seal: VerifiedMain0ContinueSyntaxFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedMain0ContinueSyntaxFactsSealV1;

impl VerifiedMain0ContinueSyntaxFactsV1 {
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

    pub(crate) fn guard(&self) -> &Main0GuardFactsV1 {
        &self.guard
    }

    pub(crate) fn then_step(&self) -> &Main0StepFactsV1 {
        &self.then_step
    }

    pub(crate) fn continue_site(&self) -> &SourceStmtSiteV1 {
        &self.continue_site
    }

    pub(crate) fn normal_step(&self) -> &Main0StepFactsV1 {
        &self.normal_step
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
pub(crate) fn issue_main0_continue_syntax_facts_from_ledger_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<VerifiedMain0ContinueSyntaxFactsV1, Main0ContinueSyntaxFactsRejectV1> {
    if input.owner() != ledger.owner() {
        return Err(Main0ContinueSyntaxFactsRejectV1::ForeignOwner);
    }
    let loop_context = ledger
        .only_loop_site()
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::LoopCardinality)?;
    let loop_stmt = input
        .source()
        .stmt_at(&loop_context)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    issue_main0_continue_syntax_facts_v1(input, loop_stmt, loop_context)
}

pub(crate) fn issue_main0_continue_syntax_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: LocatedStmtV1<'_>,
    loop_context: VerifiedCallableLoopMembershipV1,
) -> Result<VerifiedMain0ContinueSyntaxFactsV1, Main0ContinueSyntaxFactsRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(Main0ContinueSyntaxFactsRejectV1::ForeignOwner);
    }
    let function = input.function();
    if function.loop_region_bundle_count() != 1
        || !loop_context.source().matches_identity(
            function.function_origin(),
            function.source_kind(),
            loop_stmt.site(),
        )
    {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if loop_context.scope_region().scope().owner() != input.owner()
        || loop_context.scope_region().region().owner() != input.owner()
    {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if !matches!(loop_stmt.node(), ASTNode::Loop { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopShape);
    }

    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    // The bounded profile is exactly [Local, Local, Loop, Return].
    if body.statements().len() != 4 {
        return Err(Main0ContinueSyntaxFactsRejectV1::RootBodyShape);
    }
    let first_local = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let second_local = source
        .body_stmt(&body, 1)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let loop_statement = source
        .body_stmt(&body, 2)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let tail_statement = source
        .body_stmt(&body, 3)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    for statement in [&first_local, &second_local] {
        if !matches!(statement.node(), ASTNode::Local { .. }) {
            return Err(Main0ContinueSyntaxFactsRejectV1::RootBodyShape);
        }
    }
    if !matches!(loop_statement.node(), ASTNode::Loop { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::RootBodyShape);
    }
    if loop_statement.site() != loop_stmt.site() {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopCardinality);
    }
    if !matches!(tail_statement.node(), ASTNode::Return { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::TailShape);
    }

    let locals = [
        observe_local(source, &first_local)?,
        observe_local(source, &second_local)?,
    ];
    let condition = observe_condition(source, &loop_stmt)?;
    let (guard, then_step, continue_site, normal_step) = observe_loop_body(source, &loop_stmt)?;
    let tail = observe_tail(source, &tail_statement)?;

    Ok(VerifiedMain0ContinueSyntaxFactsV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_context,
        locals,
        condition,
        guard,
        then_step,
        continue_site,
        normal_step,
        tail,
        _seal: VerifiedMain0ContinueSyntaxFactsSealV1,
    })
}

fn observe_local(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0InitialLocalFactV1, Main0ContinueSyntaxFactsRejectV1> {
    let ASTNode::Local {
        variables,
        declared_type_names,
        ..
    } = statement.node()
    else {
        return Err(Main0ContinueSyntaxFactsRejectV1::InitialLocalShape);
    };
    if variables.len() != 1 || declared_type_names.iter().any(Option::is_some) {
        return Err(Main0ContinueSyntaxFactsRejectV1::InitialLocalShape);
    }
    let initializer = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(0))
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::InitialLocalShape)?;
    let ASTNode::Literal { value, .. } = initializer.node() else {
        return Err(Main0ContinueSyntaxFactsRejectV1::InitialLocalShape);
    };
    Ok(Main0InitialLocalFactV1 {
        statement_site: statement.site().clone(),
        initializer_site: initializer.site().clone(),
        shape: literal_shape(value),
    })
}

fn observe_condition(
    source: FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<Main0ConditionFactsV1, Main0ContinueSyntaxFactsRejectV1> {
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let operator = match condition.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0ContinueSyntaxFactsRejectV1::ConditionShape),
    };
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    let rhs_shape = expr_shape(rhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable)
        || !matches!(rhs_shape, SourceExprShapeV1::Variable)
    {
        return Err(Main0ContinueSyntaxFactsRejectV1::ConditionOperandShape);
    }
    Ok(Main0ConditionFactsV1 {
        site: condition.site().clone(),
        lhs_site: lhs.site().clone(),
        lhs_shape,
        rhs_site: rhs.site().clone(),
        rhs_shape,
        operator,
    })
}

fn observe_loop_body(
    source: FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<
    (
        Main0GuardFactsV1,
        Main0StepFactsV1,
        SourceStmtSiteV1,
        Main0StepFactsV1,
    ),
    Main0ContinueSyntaxFactsRejectV1,
> {
    let body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopBodyShape);
    }
    let guard_stmt = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let normal_stmt = source
        .body_stmt(&body, 1)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(normal_stmt.node(), ASTNode::Assignment { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopBodyShape);
    }
    let ASTNode::If { else_body, .. } = guard_stmt.node() else {
        return Err(Main0ContinueSyntaxFactsRejectV1::LoopBodyShape);
    };
    if else_body.is_some() {
        return Err(Main0ContinueSyntaxFactsRejectV1::ElsePresent);
    }
    let guard = observe_guard(source, &guard_stmt)?;
    let then_body = source
        .child_body_from_stmt(&guard_stmt, BodyChildRoleV1::IfThen)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    if then_body.statements().len() != 2 {
        return Err(Main0ContinueSyntaxFactsRejectV1::ThenBodyShape);
    }
    let then_stmt = source
        .body_stmt(&then_body, 0)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let continue_stmt = source
        .body_stmt(&then_body, 1)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(then_stmt.node(), ASTNode::Assignment { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::ThenBodyShape);
    }
    if !matches!(continue_stmt.node(), ASTNode::Continue { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::ThenBodyShape);
    }
    let then_step = observe_step(source, &then_stmt)?;
    let normal_step = observe_step(source, &normal_stmt)?;
    Ok((
        guard,
        then_step,
        continue_stmt.site().clone(),
        normal_step,
    ))
}

fn observe_guard(
    source: FunctionSourceViewV1<'_>,
    if_stmt: &LocatedStmtV1<'_>,
) -> Result<Main0GuardFactsV1, Main0ContinueSyntaxFactsRejectV1> {
    let condition = source
        .child_expr_from_stmt(if_stmt, ExprChildRoleV1::IfCondition)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::GuardShape)?;
    let operator = match condition.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0ContinueSyntaxFactsRejectV1::GuardShape),
    };
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0ContinueSyntaxFactsRejectV1::GuardOperandShape);
    }
    let rhs_shape = literal_shape_from_expr(&rhs)
        .ok_or(Main0ContinueSyntaxFactsRejectV1::GuardBoundNotLiteral)?;
    Ok(Main0GuardFactsV1 {
        statement_site: if_stmt.site().clone(),
        condition_site: condition.site().clone(),
        lhs_site: lhs.site().clone(),
        lhs_shape,
        rhs_site: rhs.site().clone(),
        rhs_shape,
        operator,
    })
}

fn observe_step(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0StepFactsV1, Main0ContinueSyntaxFactsRejectV1> {
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(target.node(), ASTNode::Variable { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::StepTargetShape);
    }
    let operator = match value.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0ContinueSyntaxFactsRejectV1::StepShape),
    };
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0ContinueSyntaxFactsRejectV1::StepOperandShape);
    }
    let rhs_shape = literal_shape_from_expr(&rhs)
        .ok_or(Main0ContinueSyntaxFactsRejectV1::StepDeltaNotLiteral)?;
    Ok(Main0StepFactsV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        target_shape: expr_shape(target.node()),
        value_site: value.site().clone(),
        lhs_site: lhs.site().clone(),
        lhs_shape,
        rhs_site: rhs.site().clone(),
        rhs_shape,
        operator,
    })
}

fn observe_tail(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0TailReturnFactV1, Main0ContinueSyntaxFactsRejectV1> {
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| Main0ContinueSyntaxFactsRejectV1::TailShape)?;
    if !matches!(value.node(), ASTNode::Variable { .. }) {
        return Err(Main0ContinueSyntaxFactsRejectV1::TailShape);
    }
    Ok(Main0TailReturnFactV1 {
        statement_site: statement.site().clone(),
        value_site: value.site().clone(),
        value_shape: expr_shape(value.node()),
    })
}
