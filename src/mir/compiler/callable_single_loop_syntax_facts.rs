//! Caller-zero syntax observer for one callable single-loop profile.
//!
//! This module is the only owner allowed to inspect the exact source view for
//! the S1 row. The published product owns source sites and neutral, as-written
//! shapes only; it never carries AST, names as identity, ValueIds, or Recipe
//! meaning into a downstream consumer.

#![cfg(test)]

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedCallableLoopMembershipV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedExprV1, LocatedStmtV1};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SyntaxBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLiteralShapeV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceReceiverShapeV1 {
    Me,
    This,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceCallBoundaryShapeV1 {
    receiver: SourceReceiverShapeV1,
    argument_count: u32,
}

impl SourceCallBoundaryShapeV1 {
    pub(crate) const fn receiver(&self) -> SourceReceiverShapeV1 {
        self.receiver
    }

    pub(crate) const fn argument_count(&self) -> u32 {
        self.argument_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceExprShapeV1 {
    Variable,
    Literal(SourceLiteralShapeV1),
    MethodCall(SourceCallBoundaryShapeV1),
    Other,
}

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
                    call: SourceCallBoundaryShapeV1 {
                        receiver: receiver_shape(object),
                        argument_count: arguments.len() as u32,
                    },
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

fn literal_shape_from_expr(expr: &LocatedExprV1<'_>) -> Option<SourceLiteralShapeV1> {
    match expr.node() {
        ASTNode::Literal { value, .. } => Some(literal_shape(value)),
        _ => None,
    }
}

fn literal_shape(value: &LiteralValue) -> SourceLiteralShapeV1 {
    match value {
        LiteralValue::Integer(value) => SourceLiteralShapeV1::Integer(*value),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => SourceLiteralShapeV1::TypedInteger {
            value: *value,
            declared_type_name: declared_type_name.clone().into_boxed_str(),
        },
        _ => SourceLiteralShapeV1::Other,
    }
}

fn expr_shape(node: &ASTNode) -> SourceExprShapeV1 {
    match node {
        ASTNode::Variable { .. } => SourceExprShapeV1::Variable,
        ASTNode::Literal { value, .. } => SourceExprShapeV1::Literal(literal_shape(value)),
        ASTNode::MethodCall {
            object, arguments, ..
        } => SourceExprShapeV1::MethodCall(SourceCallBoundaryShapeV1 {
            receiver: receiver_shape(object),
            argument_count: arguments.len() as u32,
        }),
        _ => SourceExprShapeV1::Other,
    }
}

fn receiver_shape(node: &ASTNode) -> SourceReceiverShapeV1 {
    match node {
        ASTNode::Me { .. } | ASTNode::MeField { .. } => SourceReceiverShapeV1::Me,
        ASTNode::This { .. } | ASTNode::ThisField { .. } => SourceReceiverShapeV1::This,
        _ => SourceReceiverShapeV1::Other,
    }
}

fn binary_operator_shape(operator: &BinaryOperator) -> SyntaxBinaryOperatorV1 {
    match operator {
        BinaryOperator::Add => SyntaxBinaryOperatorV1::Add,
        BinaryOperator::Subtract => SyntaxBinaryOperatorV1::Subtract,
        BinaryOperator::Multiply => SyntaxBinaryOperatorV1::Multiply,
        BinaryOperator::Divide => SyntaxBinaryOperatorV1::Divide,
        BinaryOperator::Modulo => SyntaxBinaryOperatorV1::Modulo,
        BinaryOperator::BitAnd => SyntaxBinaryOperatorV1::BitAnd,
        BinaryOperator::BitOr => SyntaxBinaryOperatorV1::BitOr,
        BinaryOperator::BitXor => SyntaxBinaryOperatorV1::BitXor,
        BinaryOperator::Shl => SyntaxBinaryOperatorV1::Shl,
        BinaryOperator::Shr => SyntaxBinaryOperatorV1::Shr,
        BinaryOperator::Equal => SyntaxBinaryOperatorV1::Equal,
        BinaryOperator::NotEqual => SyntaxBinaryOperatorV1::NotEqual,
        BinaryOperator::Less => SyntaxBinaryOperatorV1::Less,
        BinaryOperator::Greater => SyntaxBinaryOperatorV1::Greater,
        BinaryOperator::LessEqual => SyntaxBinaryOperatorV1::LessEqual,
        BinaryOperator::GreaterEqual => SyntaxBinaryOperatorV1::GreaterEqual,
        BinaryOperator::And => SyntaxBinaryOperatorV1::And,
        BinaryOperator::Or => SyntaxBinaryOperatorV1::Or,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, Span};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn method(object: ASTNode, name: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: name.into(),
            arguments,
            span: Span::unknown(),
        }
    }

    fn assignment(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn function(extra_root_statement: Option<ASTNode>, condition_rhs: ASTNode) -> ASTNode {
        let mut body = vec![
            ASTNode::Local {
                variables: vec!["value".into()],
                initial_values: vec![Some(Box::new(method(
                    variable("helper"),
                    "to_i64",
                    vec![variable("n")],
                )))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(condition_rhs),
                    span: Span::unknown(),
                }),
                body: vec![assignment(
                    "i",
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(1)),
                        span: Span::unknown(),
                    },
                )],
                span: Span::unknown(),
            },
        ];
        if let Some(statement) = extra_root_statement {
            body.push(statement);
        }
        body.push(ASTNode::Return {
            value: Some(Box::new(variable("value"))),
            span: Span::unknown(),
        });
        ASTNode::FunctionDeclaration {
            name: "int_to_str".into(),
            params: vec!["n".into(), "helper".into()],
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    pub(crate) fn unit(
        extra_root_statement: Option<ASTNode>,
        condition_rhs: ASTNode,
    ) -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(function(
            extra_root_statement,
            condition_rhs,
        ))
        .expect("syntax facts fixture resolves")
    }

    pub(crate) fn input_loop_and_context(
        unit: &VerifiedResolvedSourceUnitV1,
    ) -> (
        ResolvedFunctionLoweringInputV1<'_>,
        LocatedStmtV1<'_>,
        VerifiedCallableLoopMembershipV1,
    ) {
        let input = unit.root_function_input().expect("root function input");
        let body = input.source().root_body().expect("function body");
        let loop_stmt = input.source().body_stmt(&body, 2).expect("loop statement");
        let context = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("callable ledger")
            .resolved_loop_source(loop_stmt.site())
            .expect("loop context");
        (input, loop_stmt, context)
    }

    #[test]
    fn issues_exact_nine_rows_plus_prefix_boundary() {
        let unit = unit(None, integer(1));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        let facts = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
            .expect("syntax facts");

        assert_eq!(facts.initial().shape(), &SourceLiteralShapeV1::Integer(0));
        assert_eq!(facts.condition().operator(), SyntaxBinaryOperatorV1::Less);
        assert_eq!(
            facts.condition().rhs_shape(),
            &SourceLiteralShapeV1::Integer(1)
        );
        assert_eq!(facts.step().operator(), SyntaxBinaryOperatorV1::Add);
        assert_eq!(facts.step().rhs_shape(), &SourceLiteralShapeV1::Integer(1));
        assert_eq!(facts.prefix().call().argument_count(), 1);
        assert!(matches!(
            facts.tail().value_shape(),
            SourceExprShapeV1::Variable
        ));
        let scope_region = facts.loop_context().scope_region();
        assert_eq!(scope_region.scope().owner(), facts.owner());
        assert_eq!(scope_region.region().owner(), facts.owner());
    }

    #[test]
    fn product_survives_source_unit_drop() {
        let facts = {
            let unit = unit(None, integer(1));
            let (input, loop_stmt, context) = input_loop_and_context(&unit);
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
                .expect("syntax facts")
        };
        assert_eq!(facts.condition().operator(), SyntaxBinaryOperatorV1::Less);
        assert_eq!(facts.prefix().call().argument_count(), 1);
    }

    #[test]
    fn loop_membership_parts_retain_scope_region_brand() {
        let unit = unit(None, integer(1));
        let (input, _, context) = input_loop_and_context(&unit);
        let (_, _, scope_region) = context.into_parts();
        assert_eq!(scope_region.scope().owner(), input.owner());
        assert_eq!(scope_region.region().owner(), input.owner());
    }

    #[test]
    fn rejects_foreign_loop_context() {
        let first = unit(None, integer(1));
        let second = unit(None, integer(1));
        let (input, loop_stmt, _) = input_loop_and_context(&first);
        let (_, _, foreign_context) = input_loop_and_context(&second);
        assert_eq!(
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, foreign_context),
            Err(CallableSyntaxFactsRejectV1::LoopContextMismatch)
        );
    }

    #[test]
    fn rejects_unknown_root_statement_instead_of_skipping_it() {
        let unit = unit(Some(assignment("helper", variable("helper"))), integer(1));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        assert_eq!(
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context),
            Err(CallableSyntaxFactsRejectV1::UnexpectedBodyStatement)
        );
    }

    #[test]
    fn rejects_non_literal_condition_rhs() {
        let unit = unit(None, variable("n"));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        assert_eq!(
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context),
            Err(CallableSyntaxFactsRejectV1::ConditionRhsNotLiteral)
        );
    }
}
