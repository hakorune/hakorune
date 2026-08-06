//! Natural-source projector for Generic G0 S0A.
//!
//! This module is the only S0A layer that looks at AST nodes. It navigates
//! through the sealed `FunctionSourceViewV1`, resolves binding identity from
//! `VerifiedResolvedFunctionV1`, and hands an AST-free observation to the
//! structural-facts issuer. It never chooses a family or creates Recipe data.

use crate::ast::ASTNode;
use crate::mir::loop_structural_facts::generic_g0::{
    issue_generic_g0_structural_facts_v1 as issue_structural_facts_product_v1,
    GenericG0ConditionSitesV1, GenericG0StructuralObservationV1, GenericG0StructuralRejectV1,
    GenericG0TailSitesV1, GenericG0UpdateSitesV1, VerifiedGenericStructuralFactsG0,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, SourceExprSiteV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ProjectionRejectV1 {
    ForeignOwner,
    SourceNavigation,
    FunctionBodySchedule,
    RootBodySchedule,
    ChildBodySchedule,
    LoopShape,
    ConditionShape,
    UpdateShape,
    TailShape,
    BindingLookup,
    ForestShape,
    Structural(GenericG0StructuralRejectV1),
}

pub(crate) fn issue_generic_g0_structural_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<VerifiedGenericStructuralFactsG0, GenericG0ProjectionRejectV1> {
    let source = input.source();
    let function = input.function();
    let function_body = source
        .root_body()
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if function_body.statements().len() != 2 {
        return Err(GenericG0ProjectionRejectV1::FunctionBodySchedule);
    }
    let function_statements = statement_sites(source, &function_body)?;
    let root_loop = source
        .body_stmt(&function_body, 0)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    let tail_statement = source
        .body_stmt(&function_body, 1)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if !matches!(root_loop.node(), ASTNode::Loop { .. }) {
        return Err(GenericG0ProjectionRejectV1::LoopShape);
    }
    let root_body = source
        .child_body_from_stmt(&root_loop, BodyChildRoleV1::LoopBody)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if root_body.statements().len() != 2 {
        return Err(GenericG0ProjectionRejectV1::RootBodySchedule);
    }
    let root_statements = statement_sites(source, &root_body)?;
    let child_loop = source
        .body_stmt(&root_body, 0)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    let outer_update_statement = source
        .body_stmt(&root_body, 1)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if !matches!(child_loop.node(), ASTNode::Loop { .. }) {
        return Err(GenericG0ProjectionRejectV1::LoopShape);
    }
    let child_body = source
        .child_body_from_stmt(&child_loop, BodyChildRoleV1::LoopBody)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if child_body.statements().len() != 1 {
        return Err(GenericG0ProjectionRejectV1::ChildBodySchedule);
    }
    let child_statements = statement_sites(source, &child_body)?;
    let inner_update_statement = source
        .body_stmt(&child_body, 0)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    let tail = observe_tail(source, function, &tail_statement)?;
    let outer_condition = observe_condition(source, function, &root_loop)?;
    let inner_condition = observe_condition(source, function, &child_loop)?;
    let outer_update = observe_update(source, function, &outer_update_statement)?;
    let inner_update = observe_update(source, function, &inner_update_statement)?;
    let forest = function
        .resolved_loop_source_forest(root_loop.site())
        .map_err(|_| GenericG0ProjectionRejectV1::ForestShape)?;
    let expected_root_frame = function
        .resolved_loop_source(root_loop.site())
        .map_err(|_| GenericG0ProjectionRejectV1::ForestShape)?
        .frame_key();
    if forest.members().len() != 2
        || forest.members()[0].source().site() != root_loop.site()
        || forest.members()[1].source().site() != child_loop.site()
    {
        return Err(GenericG0ProjectionRejectV1::ForestShape);
    }
    let coverage = coverage_sites(
        &function_statements,
        &root_statements,
        &child_statements,
        &outer_condition,
        &inner_condition,
        &outer_update,
        &inner_update,
        &tail,
    );
    issue_structural_facts_product_v1(GenericG0StructuralObservationV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        forest,
        expected_root_frame,
        function_body: function_statements,
        root_body: root_statements,
        child_body: child_statements,
        root_loop: root_loop.site().clone(),
        child_loop: child_loop.site().clone(),
        outer_condition,
        inner_condition,
        outer_update,
        inner_update,
        tail,
        coverage,
    })
    .map_err(GenericG0ProjectionRejectV1::Structural)
}

fn statement_sites(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    body: &crate::mir::compiler::located::LocatedBodyV1<'_>,
) -> Result<Box<[SourceStmtSiteV1]>, GenericG0ProjectionRejectV1> {
    (0..body.statements().len())
        .map(|index| {
            source
                .body_stmt(body, index)
                .map(|statement| statement.site().clone())
                .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

fn observe_condition(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
) -> Result<GenericG0ConditionSitesV1, GenericG0ProjectionRejectV1> {
    let condition = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::LoopCondition)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if !matches!(condition.node(), ASTNode::BinaryOp { .. }) {
        return Err(GenericG0ProjectionRejectV1::ConditionShape);
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    Ok(GenericG0ConditionSitesV1 {
        condition: condition.site().clone(),
        lhs: lhs.site().clone(),
        rhs: rhs.site().clone(),
        binding: local_binding(function, lhs.site())?,
    })
}

fn observe_update(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
) -> Result<GenericG0UpdateSitesV1, GenericG0ProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(GenericG0ProjectionRejectV1::UpdateShape);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if !matches!(value.node(), ASTNode::BinaryOp { .. }) {
        return Err(GenericG0ProjectionRejectV1::UpdateShape);
    }
    let binding = assignment_binding(function, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    if local_binding(function, lhs.site())? != binding {
        return Err(GenericG0ProjectionRejectV1::BindingLookup);
    }
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    Ok(GenericG0UpdateSitesV1 {
        statement: statement.site().clone(),
        target: target.site().clone(),
        value: value.site().clone(),
        lhs: lhs.site().clone(),
        rhs: rhs.site().clone(),
        binding,
    })
}

fn observe_tail(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
) -> Result<GenericG0TailSitesV1, GenericG0ProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        return Err(GenericG0ProjectionRejectV1::TailShape);
    }
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| GenericG0ProjectionRejectV1::SourceNavigation)?;
    Ok(GenericG0TailSitesV1 {
        statement: statement.site().clone(),
        value: value.site().clone(),
        binding: local_binding(function, value.site())?,
    })
}

fn local_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<BindingRefV1, GenericG0ProjectionRejectV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) | None => {
            Err(GenericG0ProjectionRejectV1::BindingLookup)
        }
    }
}

fn assignment_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<BindingRefV1, GenericG0ProjectionRejectV1> {
    match function.assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(ResolvedAssignmentTargetV1::UpvarRebind(_))
        | Some(ResolvedAssignmentTargetV1::FieldWrite { .. })
        | Some(ResolvedAssignmentTargetV1::IndexWrite { .. })
        | None => Err(GenericG0ProjectionRejectV1::BindingLookup),
    }
}

fn coverage_sites(
    function_body: &[SourceStmtSiteV1],
    root_body: &[SourceStmtSiteV1],
    child_body: &[SourceStmtSiteV1],
    outer_condition: &GenericG0ConditionSitesV1,
    inner_condition: &GenericG0ConditionSitesV1,
    outer_update: &GenericG0UpdateSitesV1,
    inner_update: &GenericG0UpdateSitesV1,
    tail: &GenericG0TailSitesV1,
) -> Box<[crate::mir::resolved_semantics::SourceNodeSiteV1]> {
    let mut sites = Vec::new();
    for site in function_body.iter().chain(root_body).chain(child_body) {
        sites.push(site.node().clone());
    }
    for site in [
        &outer_condition.condition,
        &outer_condition.lhs,
        &outer_condition.rhs,
        &inner_condition.condition,
        &inner_condition.lhs,
        &inner_condition.rhs,
        &outer_update.target,
        &outer_update.value,
        &outer_update.lhs,
        &outer_update.rhs,
        &inner_update.target,
        &inner_update.value,
        &inner_update.lhs,
        &inner_update.rhs,
        &tail.value,
    ] {
        sites.push(site.node().clone());
    }
    sites.into_boxed_slice()
}
