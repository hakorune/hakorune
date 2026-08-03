//! Caller-zero AST-to-structural projection for Direct Accum.
//!
//! This is the only layer allowed to inspect the source AST for the S0 facts
//! product. It navigates through `FunctionSourceViewV1`, resolves bindings
//! through `VerifiedResolvedFunctionV1`, and then discards the AST view.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::loop_structural_facts::{
    issue_direct_accum_structural_facts_v1, DirectAccumObservedShapeV1,
    DirectAccumStructuralShapeV1, DirectAccumUpdateShapeV1, VerifiedLoopStructuralFactsV1,
};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, VerifiedResolvedFunctionV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedExprV1, LocatedStmtV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    BodyArity,
    ConditionShape,
    UpdateShape,
    StepShape,
    MissingBinding,
    UpvarBinding,
    NonBindingTarget,
    ConstantShape,
    BindingMismatch,
}

pub(crate) fn issue_direct_accum_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<VerifiedLoopStructuralFactsV1, DirectAccumProjectionRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(DirectAccumProjectionRejectV1::ForeignOwner);
    }
    let source = input.source();
    let function = input.function();
    let resolved_source = function
        .resolved_loop_source(loop_stmt.site())
        .map_err(|_| DirectAccumProjectionRejectV1::SourceLookup)?;
    let body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Err(DirectAccumProjectionRejectV1::BodyArity);
    }
    let update_stmt = source
        .body_stmt(&body, 0)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let step_stmt = source
        .body_stmt(&body, 1)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;

    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let (condition_lhs_site, condition_binding, condition_bound) =
        observe_condition(source, function, &condition)?;
    let update = observe_update(
        source,
        function,
        &update_stmt,
        DirectAccumProjectionRejectV1::UpdateShape,
    )?;
    let step = observe_update(
        source,
        function,
        &step_stmt,
        DirectAccumProjectionRejectV1::StepShape,
    )?;

    if condition_binding != step.binding {
        return Err(DirectAccumProjectionRejectV1::BindingMismatch);
    }

    let shape = DirectAccumStructuralShapeV1 {
        condition_site: condition.site().clone(),
        condition_lhs_site,
        condition_binding,
        condition_bound,
        induction: condition_binding,
        accumulator: update.binding,
        update,
        step,
    };
    let observed = DirectAccumObservedShapeV1 {
        function_origin: function.function_origin(),
        owner_source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        frame_key: resolved_source.frame_key(),
        shape,
    };
    Ok(issue_direct_accum_structural_facts_v1(observed))
}

fn observe_condition(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    condition: &LocatedExprV1<'_>,
) -> Result<
    (
        SourceExprSiteV1,
        crate::mir::resolved_semantics::BindingRefV1,
        i64,
    ),
    DirectAccumProjectionRejectV1,
> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        ..
    } = condition.node()
    else {
        return Err(DirectAccumProjectionRejectV1::ConditionShape);
    };
    let lhs = source
        .child_expr_from_expr(condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let binding = local_binding(function, lhs.site())?;
    let bound = integer_constant(rhs.node())?;
    Ok((lhs.site().clone(), binding, bound))
}

fn observe_update(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
    shape_error: DirectAccumProjectionRejectV1,
) -> Result<DirectAccumUpdateShapeV1, DirectAccumProjectionRejectV1> {
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(shape_error);
    }
    if !matches!(
        value.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) {
        return Err(shape_error);
    }
    let binding = assignment_binding(function, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| DirectAccumProjectionRejectV1::SourceNavigation)?;
    let lhs_binding = local_binding(function, lhs.site())?;
    if lhs_binding != binding {
        return Err(DirectAccumProjectionRejectV1::BindingMismatch);
    }
    Ok(DirectAccumUpdateShapeV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        value_site: value.site().clone(),
        lhs_site: lhs.site().clone(),
        rhs_site: rhs.site().clone(),
        binding,
        delta: integer_constant(rhs.node())?,
    })
}

fn local_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, DirectAccumProjectionRejectV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) => Err(DirectAccumProjectionRejectV1::UpvarBinding),
        None => Err(DirectAccumProjectionRejectV1::MissingBinding),
    }
}

fn assignment_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<crate::mir::resolved_semantics::BindingRefV1, DirectAccumProjectionRejectV1> {
    match function.assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(ResolvedAssignmentTargetV1::UpvarRebind(_)) => {
            Err(DirectAccumProjectionRejectV1::UpvarBinding)
        }
        Some(_) => Err(DirectAccumProjectionRejectV1::NonBindingTarget),
        None => Err(DirectAccumProjectionRejectV1::MissingBinding),
    }
}

fn integer_constant(node: &ASTNode) -> Result<i64, DirectAccumProjectionRejectV1> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value),
        _ => Err(DirectAccumProjectionRejectV1::ConstantShape),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, Span};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn add(name: &str, delta: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable(name)),
                right: Box::new(integer(delta)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn function() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "accum".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into(), "sum".into()],
                    initial_values: vec![Some(Box::new(integer(0))), Some(Box::new(integer(0)))],
                    declared_type_names: vec![None, None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(3)),
                        span: Span::unknown(),
                    }),
                    body: vec![add("sum", 1), add("i", 1)],
                    span: Span::unknown(),
                },
            ],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn direct_accum_projection_is_ast_free_after_navigation() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
        let input = unit.root_function_input().unwrap();
        let body = input.source().root_body().unwrap();
        let loop_stmt = input.source().body_stmt(&body, 1).unwrap();
        let facts = issue_direct_accum_facts_v1(input, &loop_stmt).unwrap();
        let shape = facts.direct_accum_shape().expect("Direct Accum payload");
        assert_eq!(shape.condition_bound, 3);
        assert_eq!(shape.update.delta, 1);
        assert_eq!(shape.step.delta, 1);
        assert_eq!(shape.update.binding, shape.accumulator);
        assert_eq!(shape.step.binding, shape.induction);

        let source = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .unwrap();
        let frame_key = source.frame_key();
        crate::mir::loop_structural_facts::issue_selected_loop_recipe_demand_v1(
            crate::mir::loop_route_policy::issue_policy_winner_for_test_with_frame(4, &frame_key),
            facts,
            source,
        )
        .expect("Direct Accum facts/source/winner frame must seal");
    }

    #[test]
    fn non_accum_body_fails_before_any_builder_effect() {
        let mut tree = function();
        let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
            unreachable!();
        };
        let ASTNode::Loop { body, .. } = &mut body[1] else {
            unreachable!();
        };
        body.pop();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).unwrap();
        let input = unit.root_function_input().unwrap();
        let function_body = input.source().root_body().unwrap();
        let loop_stmt = input.source().body_stmt(&function_body, 1).unwrap();
        assert_eq!(
            issue_direct_accum_facts_v1(input, &loop_stmt),
            Err(DirectAccumProjectionRejectV1::BodyArity)
        );
    }
}
