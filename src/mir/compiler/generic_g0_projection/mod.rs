//! Natural-source projector for Generic G0 S0A.
//!
//! This module is the only S0A layer that looks at AST nodes. It navigates
//! through the sealed `FunctionSourceViewV1`, resolves binding identity from
//! `VerifiedResolvedFunctionV1`, and hands an AST-free observation to the
//! structural-facts issuer. It never chooses a family or creates Recipe data.

use crate::ast::{ASTNode, LiteralValue, ParamDecl};
use crate::mir::loop_structural_facts::generic_g0::{
    issue_generic_g0_structural_facts_v1 as issue_structural_facts_product_v1,
    GenericG0ConditionOperatorV1, GenericG0ConditionSitesV1, GenericG0StructuralObservationV1,
    GenericG0StructuralRejectV1, GenericG0TailSitesV1, GenericG0UpdateOperatorV1,
    GenericG0UpdateSitesV1, VerifiedGenericStructuralFactsG0,
};
use crate::mir::resolved_semantics::{
    generic_g0::{
        binding_origin_is_parameter, issue_generic_g0_source_type_inventory_v1,
        GenericG0LiteralRoleV1, GenericG0LiteralSyntaxV1, GenericG0LiteralTypeRowV1,
        GenericG0ParameterTypeRowV1, GenericG0ResultTypeRowV1, GenericG0SourceTypeIssueV1,
        GenericG0SourceTypeObservationV1, VerifiedGenericSourceTypeInventoryG0,
    },
    BindingRefV1, BodyChildRoleV1, CallableHeaderSyntaxViewV1, ExprChildRoleV1, OwnedExprSiteV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceHeaderSiteV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

mod numeric;
pub(crate) use crate::mir::loop_structural_facts::generic_g0::{
    VerifiedGenericSourceBundleG0, VerifiedGenericTypedSourceBundleG0,
};
pub(crate) use numeric::{
    issue_generic_g0_typed_source_bundle_v1, GenericG0NumericProjectionRejectV1,
};

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

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceTypeProjectionRejectV1 {
    Structural(GenericG0ProjectionRejectV1),
    HeaderShape,
    ParameterShape,
    BindingLookup { index: u32 },
    SourceNavigation,
    Type(GenericG0SourceTypeIssueV1),
    StructuralRelation,
}

pub(crate) fn issue_generic_g0_source_type_bundle_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<VerifiedGenericSourceBundleG0, GenericG0SourceTypeProjectionRejectV1> {
    let structural = issue_generic_g0_structural_facts_v1(input)
        .map_err(GenericG0SourceTypeProjectionRejectV1::Structural)?;
    let observation = project_source_type_observation(input, &structural)?;
    let source_types = issue_generic_g0_source_type_inventory_v1(observation)
        .map_err(GenericG0SourceTypeProjectionRejectV1::Type)?;
    if !matches_structural_relations(&structural, &source_types) {
        return Err(GenericG0SourceTypeProjectionRejectV1::StructuralRelation);
    }
    Ok(VerifiedGenericSourceBundleG0::new(structural, source_types))
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
        operator: condition_operator(condition.node()),
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
        operator: update_operator(value.node()),
        statement: statement.site().clone(),
        target: target.site().clone(),
        value: value.site().clone(),
        lhs: lhs.site().clone(),
        rhs: rhs.site().clone(),
        binding,
    })
}

fn condition_operator(node: &ASTNode) -> GenericG0ConditionOperatorV1 {
    let ASTNode::BinaryOp { operator, .. } = node else {
        return GenericG0ConditionOperatorV1::Other;
    };
    match operator {
        crate::ast::BinaryOperator::Less => GenericG0ConditionOperatorV1::Less,
        crate::ast::BinaryOperator::LessEqual => GenericG0ConditionOperatorV1::LessEqual,
        crate::ast::BinaryOperator::Greater => GenericG0ConditionOperatorV1::Greater,
        crate::ast::BinaryOperator::GreaterEqual => GenericG0ConditionOperatorV1::GreaterEqual,
        crate::ast::BinaryOperator::Equal => GenericG0ConditionOperatorV1::Equal,
        crate::ast::BinaryOperator::NotEqual => GenericG0ConditionOperatorV1::NotEqual,
        _ => GenericG0ConditionOperatorV1::Other,
    }
}

fn update_operator(node: &ASTNode) -> GenericG0UpdateOperatorV1 {
    let ASTNode::BinaryOp { operator, .. } = node else {
        return GenericG0UpdateOperatorV1::Other;
    };
    match operator {
        crate::ast::BinaryOperator::Add => GenericG0UpdateOperatorV1::Add,
        crate::ast::BinaryOperator::Subtract => GenericG0UpdateOperatorV1::Subtract,
        _ => GenericG0UpdateOperatorV1::Other,
    }
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

fn project_source_type_observation(
    input: ResolvedFunctionLoweringInputV1<'_>,
    structural: &VerifiedGenericStructuralFactsG0,
) -> Result<GenericG0SourceTypeObservationV1, GenericG0SourceTypeProjectionRejectV1> {
    let source = input.source();
    let function = input.function();
    let header = CallableHeaderSyntaxViewV1::from_function_ast(source.root())
        .ok_or(GenericG0SourceTypeProjectionRejectV1::HeaderShape)?;
    let parameter_decls = ParamDecl::with_name_fallback(header.param_decls(), header.params());
    if parameter_decls.len() != header.params().len() {
        return Err(GenericG0SourceTypeProjectionRejectV1::ParameterShape);
    }

    let mut parameters = Vec::with_capacity(parameter_decls.len());
    for (index, declaration) in parameter_decls.iter().enumerate() {
        let index = index as u32;
        if declaration.name != header.params()[index as usize] {
            return Err(GenericG0SourceTypeProjectionRejectV1::ParameterShape);
        }
        let binding_site = SourceBindingSiteV1::Parameter { index };
        let binding = function
            .declaration_binding(&binding_site)
            .ok_or(GenericG0SourceTypeProjectionRejectV1::BindingLookup { index })?;
        let record = function
            .binding(binding)
            .ok_or(GenericG0SourceTypeProjectionRejectV1::BindingLookup { index })?;
        if !binding_origin_is_parameter(record.origin(), record.kind(), index) {
            return Err(GenericG0SourceTypeProjectionRejectV1::BindingLookup { index });
        }
        parameters.push(GenericG0ParameterTypeRowV1 {
            index,
            header: crate::mir::resolved_semantics::OwnedHeaderSiteV1::new(
                input.owner(),
                SourceHeaderSiteV1::Parameter { index },
            ),
            binding,
            binding_kind: record.kind(),
            binding_origin: record.origin().clone(),
            declared_type_name: declaration.declared_type_name.as_deref().map(Into::into),
        });
    }

    let result = GenericG0ResultTypeRowV1 {
        header: crate::mir::resolved_semantics::OwnedHeaderSiteV1::new(
            input.owner(),
            SourceHeaderSiteV1::ReturnAnnotation,
        ),
        declared_type_name: header.return_type_name().map(Into::into),
    };

    let literals = [
        (
            GenericG0LiteralRoleV1::OuterConditionRhs,
            &structural.outer_condition().rhs,
            &structural.outer_condition().condition,
            structural.outer_condition().binding,
        ),
        (
            GenericG0LiteralRoleV1::InnerConditionRhs,
            &structural.inner_condition().rhs,
            &structural.inner_condition().condition,
            structural.inner_condition().binding,
        ),
        (
            GenericG0LiteralRoleV1::OuterUpdateRhs,
            &structural.outer_update().rhs,
            &structural.outer_update().value,
            structural.outer_update().binding,
        ),
        (
            GenericG0LiteralRoleV1::InnerUpdateRhs,
            &structural.inner_update().rhs,
            &structural.inner_update().value,
            structural.inner_update().binding,
        ),
    ]
    .into_iter()
    .map(|(role, site, context, binding)| {
        project_literal_type_row(source, input.owner(), role, site, context, binding)
    })
    .collect::<Result<Vec<_>, _>>()?
    .into_boxed_slice();

    Ok(GenericG0SourceTypeObservationV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        parameters: parameters.into_boxed_slice(),
        result,
        literals,
    })
}

fn project_literal_type_row(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    role: GenericG0LiteralRoleV1,
    site: &SourceExprSiteV1,
    context: &SourceExprSiteV1,
    binding: BindingRefV1,
) -> Result<GenericG0LiteralTypeRowV1, GenericG0SourceTypeProjectionRejectV1> {
    let owned_site = OwnedExprSiteV1::new(owner, site.clone());
    let owned_context = OwnedExprSiteV1::new(owner, context.clone());
    let expr = source
        .expr_at(&owned_site)
        .map_err(|_| GenericG0SourceTypeProjectionRejectV1::SourceNavigation)?;
    let syntax = match expr.node() {
        ASTNode::Literal { value, .. } => match value {
            LiteralValue::Integer(value) => GenericG0LiteralSyntaxV1::PlainInteger(*value),
            LiteralValue::TypedInteger {
                value,
                declared_type_name,
            } => GenericG0LiteralSyntaxV1::TypedInteger {
                value: *value,
                declared_type_name: declared_type_name.clone().into_boxed_str(),
            },
            LiteralValue::String(_) => GenericG0LiteralSyntaxV1::Other(
                crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::String,
            ),
            LiteralValue::Float(_) => GenericG0LiteralSyntaxV1::Other(
                crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::Float,
            ),
            LiteralValue::Bool(_) => GenericG0LiteralSyntaxV1::Other(
                crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::Bool,
            ),
            LiteralValue::Null => GenericG0LiteralSyntaxV1::Other(
                crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::Null,
            ),
            LiteralValue::Void => GenericG0LiteralSyntaxV1::Other(
                crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::Void,
            ),
        },
        _ => GenericG0LiteralSyntaxV1::Other(
            crate::mir::resolved_semantics::generic_g0::GenericG0LiteralKindV1::NonLiteral,
        ),
    };
    Ok(GenericG0LiteralTypeRowV1 {
        role,
        site: owned_site,
        context: owned_context,
        binding,
        syntax,
    })
}

fn matches_structural_relations(
    structural: &VerifiedGenericStructuralFactsG0,
    source_types: &VerifiedGenericSourceTypeInventoryG0,
) -> bool {
    if source_types.owner() != structural.owner()
        || source_types.origin() != structural.origin()
        || source_types.source_kind() != structural.source_kind()
    {
        return false;
    }
    let parameters = source_types.parameters();
    if parameters.len() != 2
        || parameters[0].binding != structural.outer_condition().binding
        || parameters[1].binding != structural.inner_condition().binding
    {
        return false;
    }
    let expected = [
        (
            GenericG0LiteralRoleV1::OuterConditionRhs,
            &structural.outer_condition().rhs,
            &structural.outer_condition().condition,
            structural.outer_condition().binding,
        ),
        (
            GenericG0LiteralRoleV1::InnerConditionRhs,
            &structural.inner_condition().rhs,
            &structural.inner_condition().condition,
            structural.inner_condition().binding,
        ),
        (
            GenericG0LiteralRoleV1::OuterUpdateRhs,
            &structural.outer_update().rhs,
            &structural.outer_update().value,
            structural.outer_update().binding,
        ),
        (
            GenericG0LiteralRoleV1::InnerUpdateRhs,
            &structural.inner_update().rhs,
            &structural.inner_update().value,
            structural.inner_update().binding,
        ),
    ];
    source_types.literals().iter().all(|row| {
        expected.iter().any(|(role, site, context, binding)| {
            row.role == *role
                && row.site.site() == *site
                && row.context.site() == *context
                && row.binding == *binding
        })
    })
}
