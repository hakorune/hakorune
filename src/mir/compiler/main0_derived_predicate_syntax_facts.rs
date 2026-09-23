//! Caller-zero syntax observer for the bounded Main0 derived-predicate
//! profile.
//!
//! This module is the only owner allowed to inspect the exact source view for
//! the `app_main0_continue_to_portable_migration` cohort's third profile.
//! The published product owns source sites and neutral, as-written shapes
//! only; it never carries AST, names as identity, ValueIds, or Recipe
//! meaning into a downstream consumer.
//!
//! The admitted shape is exactly:
//!
//! ```text
//! local <carrier> = <integer>
//! local <operand> = <integer>
//! local <bound>   = <integer>
//! loop(<carrier> + <operand> <= <bound>) {
//!     <carrier> = <carrier> + <integer>
//! }
//! return <carrier>
//! ```
//!
//! The predicate's compare LHS is one derived add-expression over two
//! declared locals; the bound is a third declared local read. Which local
//! holds which role is decided later by the source-map join, not by this
//! observer.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1,
};

use super::callable_single_loop_source_shapes::{
    binary_operator_shape, expr_shape, literal_shape, literal_shape_from_expr,
    SourceExprShapeV1, SyntaxBinaryOperatorV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::main0_continue_syntax_facts::{
    Main0InitialLocalFactV1, Main0StepFactsV1, Main0TailReturnFactV1,
};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1, FunctionSourceViewV1};

/// Loop-head `<add> <= <bound>` facts. The outer operator is recorded
/// neutrally; the source map enforces the bounded `LessEqual` admission.
/// `add` is the compare LHS and must be one binary expression over two
/// variable reads; `bound` is the compare RHS variable read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Main0DerivedConditionFactsV1 {
    site: SourceExprSiteV1,
    add_site: SourceExprSiteV1,
    add_lhs_site: SourceExprSiteV1,
    add_lhs_shape: SourceExprShapeV1,
    add_rhs_site: SourceExprSiteV1,
    add_rhs_shape: SourceExprShapeV1,
    add_operator: SyntaxBinaryOperatorV1,
    rhs_site: SourceExprSiteV1,
    rhs_shape: SourceExprShapeV1,
    operator: SyntaxBinaryOperatorV1,
}

impl Main0DerivedConditionFactsV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn from_parts(
        site: SourceExprSiteV1,
        add_site: SourceExprSiteV1,
        add_lhs_site: SourceExprSiteV1,
        add_lhs_shape: SourceExprShapeV1,
        add_rhs_site: SourceExprSiteV1,
        add_rhs_shape: SourceExprShapeV1,
        add_operator: SyntaxBinaryOperatorV1,
        rhs_site: SourceExprSiteV1,
        rhs_shape: SourceExprShapeV1,
        operator: SyntaxBinaryOperatorV1,
    ) -> Self {
        Self {
            site,
            add_site,
            add_lhs_site,
            add_lhs_shape,
            add_rhs_site,
            add_rhs_shape,
            add_operator,
            rhs_site,
            rhs_shape,
            operator,
        }
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn add_site(&self) -> &SourceExprSiteV1 {
        &self.add_site
    }

    pub(crate) fn add_lhs_site(&self) -> &SourceExprSiteV1 {
        &self.add_lhs_site
    }

    pub(crate) fn add_lhs_shape(&self) -> &SourceExprShapeV1 {
        &self.add_lhs_shape
    }

    pub(crate) fn add_rhs_site(&self) -> &SourceExprSiteV1 {
        &self.add_rhs_site
    }

    pub(crate) fn add_rhs_shape(&self) -> &SourceExprShapeV1 {
        &self.add_rhs_shape
    }

    pub(crate) const fn add_operator(&self) -> SyntaxBinaryOperatorV1 {
        self.add_operator
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Main0DerivedPredicateSyntaxFactsRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    LoopCardinality,
    SourceNavigation,
    LoopShape,
    RootBodyShape,
    InitialLocalShape,
    ConditionShape,
    ConditionOperandShape,
    ConditionBoundShape,
    LoopBodyShape,
    StepShape,
    StepTargetShape,
    StepOperandShape,
    StepDeltaNotLiteral,
    TailShape,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedMain0DerivedPredicateSyntaxFactsV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_context: VerifiedCallableLoopMembershipV1,
    locals: [Main0InitialLocalFactV1; 3],
    condition: Main0DerivedConditionFactsV1,
    step: Main0StepFactsV1,
    tail: Main0TailReturnFactV1,
    _seal: VerifiedMain0DerivedPredicateSyntaxFactsSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedMain0DerivedPredicateSyntaxFactsSealV1;

impl VerifiedMain0DerivedPredicateSyntaxFactsV1 {
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

    pub(crate) fn locals(&self) -> &[Main0InitialLocalFactV1; 3] {
        &self.locals
    }

    pub(crate) fn condition(&self) -> &Main0DerivedConditionFactsV1 {
        &self.condition
    }

    pub(crate) fn step(&self) -> &Main0StepFactsV1 {
        &self.step
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
pub(crate) fn issue_main0_derived_predicate_syntax_facts_from_ledger_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<
    VerifiedMain0DerivedPredicateSyntaxFactsV1,
    Main0DerivedPredicateSyntaxFactsRejectV1,
> {
    if input.owner() != ledger.owner() {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ForeignOwner);
    }
    let loop_context = ledger
        .only_loop_site()
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::LoopCardinality)?;
    let loop_stmt = input
        .source()
        .stmt_at(&loop_context)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    issue_main0_derived_predicate_syntax_facts_v1(input, loop_stmt, loop_context)
}

pub(crate) fn issue_main0_derived_predicate_syntax_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: LocatedStmtV1<'_>,
    loop_context: VerifiedCallableLoopMembershipV1,
) -> Result<
    VerifiedMain0DerivedPredicateSyntaxFactsV1,
    Main0DerivedPredicateSyntaxFactsRejectV1,
> {
    if input.owner() != loop_stmt.owner() {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ForeignOwner);
    }
    let function = input.function();
    if function.loop_region_bundle_count() != 1
        || !loop_context.source().matches_identity(
            function.function_origin(),
            function.source_kind(),
            loop_stmt.site(),
        )
    {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if loop_context.scope_region().scope().owner() != input.owner()
        || loop_context.scope_region().region().owner() != input.owner()
    {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopContextMismatch);
    }
    if !matches!(loop_stmt.node(), ASTNode::Loop { .. }) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopShape);
    }

    let source = input.source();
    let body = source
        .root_body()
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    // The bounded profile is exactly [Local, Local, Local, Loop, Return].
    if body.statements().len() != 5 {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::RootBodyShape);
    }
    let first_local = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let second_local = source
        .body_stmt(&body, 1)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let third_local = source
        .body_stmt(&body, 2)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let loop_statement = source
        .body_stmt(&body, 3)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let tail_statement = source
        .body_stmt(&body, 4)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    for statement in [&first_local, &second_local, &third_local] {
        if !matches!(statement.node(), ASTNode::Local { .. }) {
            return Err(Main0DerivedPredicateSyntaxFactsRejectV1::RootBodyShape);
        }
    }
    if !matches!(loop_statement.node(), ASTNode::Loop { .. }) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::RootBodyShape);
    }
    if loop_statement.site() != loop_stmt.site() {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopCardinality);
    }
    if !matches!(tail_statement.node(), ASTNode::Return { .. }) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::TailShape);
    }

    let locals = [
        observe_local(source, &first_local)?,
        observe_local(source, &second_local)?,
        observe_local(source, &third_local)?,
    ];
    let condition = observe_condition(source, &loop_stmt)?;
    let step = observe_loop_body(source, &loop_stmt)?;
    let tail = observe_tail(source, &tail_statement)?;

    Ok(VerifiedMain0DerivedPredicateSyntaxFactsV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_context,
        locals,
        condition,
        step,
        tail,
        _seal: VerifiedMain0DerivedPredicateSyntaxFactsSealV1,
    })
}

fn observe_local(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0InitialLocalFactV1, Main0DerivedPredicateSyntaxFactsRejectV1> {
    let ASTNode::Local {
        variables,
        declared_type_names,
        ..
    } = statement.node()
    else {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::InitialLocalShape);
    };
    if variables.len() != 1 || declared_type_names.iter().any(Option::is_some) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::InitialLocalShape);
    }
    let initializer = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LocalInitializer(0))
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::InitialLocalShape)?;
    let ASTNode::Literal { value, .. } = initializer.node() else {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::InitialLocalShape);
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
) -> Result<Main0DerivedConditionFactsV1, Main0DerivedPredicateSyntaxFactsRejectV1> {
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let operator = match condition.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ConditionShape),
    };
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    // The compare LHS is the profile's derived expression: exactly one
    // binary add over two variable reads. The operator is recorded
    // neutrally here; the source map enforces `Add`.
    let add_operator = match lhs.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => {
            return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ConditionOperandShape)
        }
    };
    let add_lhs = source
        .child_expr_from_expr(&lhs, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let add_rhs = source
        .child_expr_from_expr(&lhs, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let add_lhs_shape = expr_shape(add_lhs.node());
    let add_rhs_shape = expr_shape(add_rhs.node());
    if !matches!(add_lhs_shape, SourceExprShapeV1::Variable)
        || !matches!(add_rhs_shape, SourceExprShapeV1::Variable)
    {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ConditionOperandShape);
    }
    // The bound is a declared local read (`<= n`), not a literal.
    let rhs_shape = expr_shape(rhs.node());
    if !matches!(rhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::ConditionBoundShape);
    }
    Ok(Main0DerivedConditionFactsV1::from_parts(
        condition.site().clone(),
        lhs.site().clone(),
        add_lhs.site().clone(),
        add_lhs_shape,
        add_rhs.site().clone(),
        add_rhs_shape,
        add_operator,
        rhs.site().clone(),
        rhs_shape,
        operator,
    ))
}

fn observe_loop_body(
    source: FunctionSourceViewV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<Main0StepFactsV1, Main0DerivedPredicateSyntaxFactsRejectV1> {
    let body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    // The bounded profile is exactly [<carrier> = <carrier> + <int>];
    // no if/else/branch/transfer and no second body statement.
    if body.statements().len() != 1 {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopBodyShape);
    }
    let step_stmt = source
        .body_stmt(&body, 0)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(step_stmt.node(), ASTNode::Assignment { .. }) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::LoopBodyShape);
    }
    observe_step(source, &step_stmt)
}

fn observe_step(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0StepFactsV1, Main0DerivedPredicateSyntaxFactsRejectV1> {
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    if !matches!(target.node(), ASTNode::Variable { .. }) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::StepTargetShape);
    }
    let operator = match value.node() {
        ASTNode::BinaryOp { operator, .. } => binary_operator_shape(operator),
        _ => return Err(Main0DerivedPredicateSyntaxFactsRejectV1::StepShape),
    };
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::SourceNavigation)?;
    let lhs_shape = expr_shape(lhs.node());
    if !matches!(lhs_shape, SourceExprShapeV1::Variable) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::StepOperandShape);
    }
    let rhs_shape = literal_shape_from_expr(&rhs)
        .ok_or(Main0DerivedPredicateSyntaxFactsRejectV1::StepDeltaNotLiteral)?;
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

fn observe_tail(
    source: FunctionSourceViewV1<'_>,
    statement: &LocatedStmtV1<'_>,
) -> Result<Main0TailReturnFactV1, Main0DerivedPredicateSyntaxFactsRejectV1> {
    let ASTNode::Return { .. } = statement.node() else {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::TailShape);
    };
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| Main0DerivedPredicateSyntaxFactsRejectV1::TailShape)?;
    let value_shape = expr_shape(value.node());
    if !matches!(value_shape, SourceExprShapeV1::Variable) {
        return Err(Main0DerivedPredicateSyntaxFactsRejectV1::TailShape);
    }
    Ok(Main0TailReturnFactV1::from_parts(
        statement.site().clone(),
        value.site().clone(),
        value_shape,
    ))
}
