//! Caller-zero syntax observer for one callable single-loop profile.
//!
//! This module is the only owner allowed to inspect the exact source view for
//! the S1 row. The published product owns source sites and neutral, as-written
//! shapes only; it never carries AST, names as identity, ValueIds, or Recipe
//! meaning into a downstream consumer.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1,
};

use super::callable_single_loop_source_shapes::{
    binary_operator_shape, expr_shape, literal_shape, literal_shape_from_expr, receiver_shape,
    SourceCallBoundaryShapeV1, SourceExprShapeV1, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InitialCarrierSyntaxFactV1 {
    statement_site: SourceStmtSiteV1,
    initializer_site: SourceExprSiteV1,
    shape: SourceLiteralShapeV1,
}

impl InitialCarrierSyntaxFactV1 {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConditionSyntaxFactsV1 {
    site: SourceExprSiteV1,
    lhs_site: SourceExprSiteV1,
    lhs_shape: SourceExprShapeV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceLiteralShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl ConditionSyntaxFactsV1 {
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

    pub(crate) fn rhs_shape(&self) -> &SourceLiteralShapeV1 {
        &self.rhs_shape
    }

    pub(crate) const fn operator(&self) -> SyntaxBinaryOperatorV1 {
        self.operator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StepSyntaxFactsV1 {
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

impl StepSyntaxFactsV1 {
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
pub(crate) struct PrefixBoundarySyntaxFactV1 {
    statement_site: SourceStmtSiteV1,
    initializer_site: SourceExprSiteV1,
    call: SourceCallBoundaryShapeV1,
}

impl PrefixBoundarySyntaxFactV1 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn initializer_site(&self) -> &SourceExprSiteV1 {
        &self.initializer_site
    }

    pub(crate) fn call(&self) -> &SourceCallBoundaryShapeV1 {
        &self.call
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TailReturnSyntaxFactV1 {
    statement_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value_shape: SourceExprShapeV1,
}

impl TailReturnSyntaxFactV1 {
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
pub(crate) enum CallableSyntaxFactsRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    LoopCardinality,
    SourceNavigation,
    LoopShape,
    LoopBodyArity,
    InitialCarrierShape,
    DuplicateInitialCarrier,
    PrefixBoundaryShape,
    DuplicatePrefixBoundary,
    ConditionShape,
    ConditionRhsNotLiteral,
    StepShape,
    StepRhsNotLiteral,
    StepTargetShape,
    TailShape,
    UnexpectedBodyStatement,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedSourceSyntaxFactsV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_context: VerifiedCallableLoopMembershipV1,
    initial: InitialCarrierSyntaxFactV1,
    condition: ConditionSyntaxFactsV1,
    step: StepSyntaxFactsV1,
    prefix: PrefixBoundarySyntaxFactV1,
    tail: TailReturnSyntaxFactV1,
    _seal: VerifiedSourceSyntaxFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedSourceSyntaxFactsSealV1;

impl VerifiedSourceSyntaxFactsV1 {
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

    pub(crate) fn initial(&self) -> &InitialCarrierSyntaxFactV1 {
        &self.initial
    }

    pub(crate) fn condition(&self) -> &ConditionSyntaxFactsV1 {
        &self.condition
    }

    pub(crate) fn step(&self) -> &StepSyntaxFactsV1 {
        &self.step
    }

    pub(crate) fn prefix(&self) -> &PrefixBoundarySyntaxFactV1 {
        &self.prefix
    }

    pub(crate) fn tail(&self) -> &TailReturnSyntaxFactV1 {
        &self.tail
    }
}

pub(crate) fn issue_callable_single_loop_syntax_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: LocatedStmtV1<'_>,
    loop_context: VerifiedCallableLoopMembershipV1,
) -> Result<VerifiedSourceSyntaxFactsV1, CallableSyntaxFactsRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(CallableSyntaxFactsRejectV1::ForeignOwner);
    }
    let function = input.function();
    if function.loop_region_bundle_count() != 1
        || !loop_context.source().matches_identity(
            function.function_origin(),
            function.source_kind(),
            loop_stmt.site(),
        )
    {
        return Err(CallableSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if loop_context.scope_region().scope().owner() != input.owner()
        || loop_context.scope_region().region().owner() != input.owner()
    {
        return Err(CallableSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if !matches!(loop_stmt.node(), ASTNode::Loop { .. }) {
        return Err(CallableSyntaxFactsRejectV1::LoopShape);
    }

    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let mut loops = Vec::new();
    let mut locals = Vec::new();
    let mut returns = Vec::new();
    for index in 0..body.statements().len() {
        let statement = source
            .body_stmt(&body, index)
            .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
        match statement.node() {
            ASTNode::Loop { .. } => loops.push(statement),
            ASTNode::Local { .. } => locals.push(statement),
            ASTNode::Return { .. } => returns.push((index, statement)),
            _ => return Err(CallableSyntaxFactsRejectV1::UnexpectedBodyStatement),
        }
    }
    if loops.len() != 1 || loops[0].site() != loop_stmt.site() {
        return Err(CallableSyntaxFactsRejectV1::LoopCardinality);
    }
    if returns.len() != 1 || returns[0].0 + 1 != body.statements().len() {
        return Err(CallableSyntaxFactsRejectV1::TailShape);
    }

    let (initial, prefix) = observe_locals(source, locals)?;
    let condition = observe_condition(source, &loop_stmt)?;
    let step = observe_step(source, &loop_stmt)?;
    let tail = observe_tail(source, &returns[0].1)?;

    Ok(VerifiedSourceSyntaxFactsV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_context,
        initial,
        condition,
        step,
        prefix,
        tail,
        _seal: VerifiedSourceSyntaxFactsSealV1,
    })
}

/// Issue the neutral source facts through the resolver-owned exact Loop seam.
///
/// This is the production ingress for the source/facts row: the resolver
/// chooses the unique Loop membership, and the branded source view reopens
/// that exact statement. It never reconstructs a path, ordinal, or name.
pub(crate) fn issue_callable_single_loop_syntax_facts_from_ledger_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<VerifiedSourceSyntaxFactsV1, CallableSyntaxFactsRejectV1> {
    if input.owner() != ledger.owner() {
        return Err(CallableSyntaxFactsRejectV1::ForeignOwner);
    }
    let loop_context = ledger
        .only_loop_site()
        .map_err(|_| CallableSyntaxFactsRejectV1::LoopCardinality)?;
    let loop_stmt = input
        .source()
        .stmt_at(&loop_context)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, loop_context)
}

fn observe_locals(
    source: super::source_view::FunctionSourceViewV1<'_>,
    locals: Vec<LocatedStmtV1<'_>>,
) -> Result<(InitialCarrierSyntaxFactV1, PrefixBoundarySyntaxFactV1), CallableSyntaxFactsRejectV1> {
    let mut initial = None;
    let mut prefix = None;
    for statement in locals {
        let initializer = source
            .child_expr_from_stmt(&statement, ExprChildRoleV1::LocalInitializer(0))
            .map_err(|_| CallableSyntaxFactsRejectV1::InitialCarrierShape)?;
        match initializer.node() {
            ASTNode::Literal { value, .. } => {
                if initial.is_some() {
                    return Err(CallableSyntaxFactsRejectV1::DuplicateInitialCarrier);
                }
                initial = Some(InitialCarrierSyntaxFactV1 {
                    statement_site: statement.site().clone(),
                    initializer_site: initializer.site().clone(),
                    shape: literal_shape(value),
                });
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                if prefix.is_some() {
                    return Err(CallableSyntaxFactsRejectV1::DuplicatePrefixBoundary);
                }
                prefix = Some(PrefixBoundarySyntaxFactV1 {
                    statement_site: statement.site().clone(),
                    initializer_site: initializer.site().clone(),
                    call: SourceCallBoundaryShapeV1::method(
                        receiver_shape(object),
                        arguments.len() as u32,
                    ),
                });
            }
            ASTNode::FunctionCall { arguments, .. } => {
                if prefix.is_some() {
                    return Err(CallableSyntaxFactsRejectV1::DuplicatePrefixBoundary);
                }
                prefix = Some(PrefixBoundarySyntaxFactV1 {
                    statement_site: statement.site().clone(),
                    initializer_site: initializer.site().clone(),
                    call: SourceCallBoundaryShapeV1::free_static(arguments.len() as u32),
                });
            }
            _ => return Err(CallableSyntaxFactsRejectV1::PrefixBoundaryShape),
        }
    }
    match (initial, prefix) {
        (Some(initial), Some(prefix)) => Ok((initial, prefix)),
        _ => Err(CallableSyntaxFactsRejectV1::InitialCarrierShape),
    }
}

fn observe_condition(
    source: super::source_view::FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<ConditionSyntaxFactsV1, CallableSyntaxFactsRejectV1> {
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let operator = match condition.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(CallableSyntaxFactsRejectV1::ConditionShape),
    };
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs_shape =
        literal_shape_from_expr(&rhs).ok_or(CallableSyntaxFactsRejectV1::ConditionRhsNotLiteral)?;
    Ok(ConditionSyntaxFactsV1 {
        site: condition.site().clone(),
        lhs_site: lhs.site().clone(),
        lhs_shape: expr_shape(lhs.node()),
        rhs_site: rhs.site().clone(),
        rhs_shape,
        operator,
    })
}

fn observe_step(
    source: super::source_view::FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<StepSyntaxFactsV1, CallableSyntaxFactsRejectV1> {
    let body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    if body.statements().len() != 1 {
        return Err(CallableSyntaxFactsRejectV1::LoopBodyArity);
    }
    let statement = source
        .body_stmt(&body, 0)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(CallableSyntaxFactsRejectV1::StepShape);
    }
    let target = source
        .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let operator = match value.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(CallableSyntaxFactsRejectV1::StepShape),
    };
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| CallableSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs_shape =
        literal_shape_from_expr(&rhs).ok_or(CallableSyntaxFactsRejectV1::StepRhsNotLiteral)?;
    if !matches!(target.node(), ASTNode::Variable { .. }) {
        return Err(CallableSyntaxFactsRejectV1::StepTargetShape);
    }
    Ok(StepSyntaxFactsV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        target_shape: expr_shape(target.node()),
        value_site: value.site().clone(),
        lhs_site: lhs.site().clone(),
        lhs_shape: expr_shape(lhs.node()),
        rhs_site: rhs.site().clone(),
        rhs_shape,
        operator,
    })
}

fn observe_tail(
    source: super::source_view::FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<TailReturnSyntaxFactV1, CallableSyntaxFactsRejectV1> {
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| CallableSyntaxFactsRejectV1::TailShape)?;
    if !matches!(value.node(), ASTNode::Variable { .. }) {
        return Err(CallableSyntaxFactsRejectV1::TailShape);
    }
    Ok(TailReturnSyntaxFactV1 {
        statement_site: statement.site().clone(),
        value_site: value.site().clone(),
        value_shape: expr_shape(value.node()),
    })
}

#[cfg(test)]
#[path = "callable_single_loop_syntax_facts_tests.rs"]
pub(crate) mod tests;
