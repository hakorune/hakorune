//! Caller-zero source observation for the bounded NestedLoopMinimal shape.
//!
//! Syntax is observed once through `FunctionSourceViewV1`; the returned
//! product retains only resolver-owned sites, bindings, and source facts.
//! Downstream semantic and physical ownership begins in later phases.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, LoopSourceForestBindingRejectV1,
    VerifiedLoopSourceForestBindingV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, FunctionOriginV1, LoopExecutionFrameKeyV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ScopeId, ScopeKindV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedExprV1, LocatedStmtV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedPredicateProjectionRejectV1 {
    ForeignOwner,
    Forest(LoopSourceForestBindingRejectV1),
    ForestShape,
    SourceNavigation,
    RootPredicateShape,
    ChildPredicateShape,
    RootBodySchedule,
    ChildBodySchedule,
    MissingBinding,
    UpvarBinding,
    NonBindingTarget,
    BindingMismatch,
    LexicalScopeMismatch,
    ConstantShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedObservedRecurrenceOwnerV1 {
    Root,
    Child,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedBindingEvidenceV1 {
    pub(crate) binding: BindingRefV1,
    pub(crate) lexical_scope: ScopeId,
    pub(crate) recurrence_owner: NestedObservedRecurrenceOwnerV1,
    pub(crate) parent_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPredicateConditionEvidenceV1 {
    pub(crate) site: SourceExprSiteV1,
    pub(crate) lhs_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) bound: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPredicateUpdateEvidenceV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) target_site: SourceExprSiteV1,
    pub(crate) value_site: SourceExprSiteV1,
    pub(crate) lhs_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) delta: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedRootBodyRoleV1 {
    LocalJ,
    InitializeJ,
    ChildLoop,
    IncrementRoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedChildBodyRoleV1 {
    IncrementAncestor,
    IncrementChild,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedLoopSourceShapeV1 {
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) root_site: SourceStmtSiteV1,
    pub(crate) child_site: SourceStmtSiteV1,
    pub(crate) root_condition: NestedPredicateConditionEvidenceV1,
    pub(crate) child_condition: NestedPredicateConditionEvidenceV1,
    pub(crate) initialize_child: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_root: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_ancestor: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_child: NestedPredicateUpdateEvidenceV1,
    pub(crate) bindings: [NestedBindingEvidenceV1; 3],
    pub(crate) root_body_roles: [NestedRootBodyRoleV1; 4],
    pub(crate) child_body_roles: [NestedChildBodyRoleV1; 2],
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedLoopSourceProjectionV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    shape: VerifiedNestedLoopSourceShapeV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedNestedLoopSourceProjectionV1 {
    pub(crate) fn forest_binding(&self) -> &VerifiedLoopSourceForestBindingV1 {
        &self.forest_binding
    }

    pub(crate) fn shape(&self) -> &VerifiedNestedLoopSourceShapeV1 {
        &self.shape
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopSourceForestBindingV1,
        VerifiedNestedLoopSourceShapeV1,
        LoopExecutionFrameKeyV1,
    ) {
        (self.forest_binding, self.shape, self.root_frame_key)
    }
}

pub(crate) fn issue_nested_predicate_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root_loop: &LocatedStmtV1<'_>,
) -> Result<VerifiedNestedLoopSourceProjectionV1, NestedPredicateProjectionRejectV1> {
    if input.owner() != root_loop.owner() {
        return Err(NestedPredicateProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    let source = input.source();
    let forest = function
        .resolved_loop_source_forest(root_loop.site())
        .map_err(|_| NestedPredicateProjectionRejectV1::ForestShape)?;
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
        || forest.members()[0].source().site() != root_loop.site()
    {
        return Err(NestedPredicateProjectionRejectV1::ForestShape);
    }
    let child_site = forest.members()[1].source().site().clone();
    let root_frame_key = forest.members()[0].source().frame_key();
    let forest_binding = bind_resolved_loop_source_forest_v1(forest)
        .map_err(NestedPredicateProjectionRejectV1::Forest)?;

    let root_body = source
        .child_body_from_stmt(root_loop, BodyChildRoleV1::LoopBody)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if root_body.statements().len() != 4 {
        return Err(NestedPredicateProjectionRejectV1::RootBodySchedule);
    }
    let local_j = source
        .body_stmt(&root_body, 0)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let initialize_j = source
        .body_stmt(&root_body, 1)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let child_loop = source
        .body_stmt(&root_body, 2)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let increment_root = source
        .body_stmt(&root_body, 3)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if !is_local_j(local_j.node())
        || !matches!(child_loop.node(), ASTNode::Loop { .. })
        || child_loop.site() != &child_site
    {
        return Err(NestedPredicateProjectionRejectV1::RootBodySchedule);
    }

    let child_body = source
        .child_body_from_stmt(&child_loop, BodyChildRoleV1::LoopBody)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if child_body.statements().len() != 2 {
        return Err(NestedPredicateProjectionRejectV1::ChildBodySchedule);
    }
    let increment_ancestor = source
        .body_stmt(&child_body, 0)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let increment_child = source
        .body_stmt(&child_body, 1)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let root_condition = observe_condition(
        source,
        function,
        root_loop,
        NestedPredicateProjectionRejectV1::RootPredicateShape,
    )?;
    let child_condition = observe_condition(
        source,
        function,
        &child_loop,
        NestedPredicateProjectionRejectV1::ChildPredicateShape,
    )?;
    let initialize_child = observe_initialization(source, function, &initialize_j)?;
    let increment_root = observe_update(source, function, &increment_root)?;
    let increment_ancestor = observe_update(source, function, &increment_ancestor)?;
    let increment_child = observe_update(source, function, &increment_child)?;
    let root_binding = root_condition.binding;
    let sum_binding = increment_ancestor.binding;

    let child_binding = declaration_binding(function, local_j.site())?;
    if initialize_child.binding != child_binding || increment_child.binding != child_binding {
        return Err(NestedPredicateProjectionRejectV1::BindingMismatch);
    }
    if root_binding != increment_root.binding {
        return Err(NestedPredicateProjectionRejectV1::BindingMismatch);
    }
    if child_condition.binding != child_binding {
        return Err(NestedPredicateProjectionRejectV1::BindingMismatch);
    }
    if sum_binding == child_binding || increment_ancestor.binding == child_binding {
        return Err(NestedPredicateProjectionRejectV1::BindingMismatch);
    }
    let i_scope = binding_scope(function, root_binding)?;
    let sum_scope = binding_scope(function, sum_binding)?;
    let j_scope = binding_scope(function, child_binding)?;
    if !matches!(
        function.scope(i_scope).map(|scope| scope.kind()),
        Some(ScopeKindV1::Function | ScopeKindV1::LexicalBlock)
    ) || !matches!(
        function.scope(sum_scope).map(|scope| scope.kind()),
        Some(ScopeKindV1::Function | ScopeKindV1::LexicalBlock)
    ) || function.scope(j_scope).map(|scope| scope.kind()) != Some(ScopeKindV1::LoopBody)
    {
        return Err(NestedPredicateProjectionRejectV1::LexicalScopeMismatch);
    }

    Ok(VerifiedNestedLoopSourceProjectionV1 {
        forest_binding,
        shape: VerifiedNestedLoopSourceShapeV1 {
            function_origin: function.function_origin(),
            root_site: root_loop.site().clone(),
            child_site,
            root_condition,
            child_condition,
            initialize_child,
            increment_root,
            increment_ancestor,
            increment_child,
            bindings: [
                binding_evidence(
                    function,
                    root_binding,
                    NestedObservedRecurrenceOwnerV1::Root,
                    true,
                )?,
                binding_evidence(
                    function,
                    sum_binding,
                    NestedObservedRecurrenceOwnerV1::Root,
                    true,
                )?,
                binding_evidence(
                    function,
                    child_binding,
                    NestedObservedRecurrenceOwnerV1::Child,
                    false,
                )?,
            ],
            root_body_roles: [
                NestedRootBodyRoleV1::LocalJ,
                NestedRootBodyRoleV1::InitializeJ,
                NestedRootBodyRoleV1::ChildLoop,
                NestedRootBodyRoleV1::IncrementRoot,
            ],
            child_body_roles: [
                NestedChildBodyRoleV1::IncrementAncestor,
                NestedChildBodyRoleV1::IncrementChild,
            ],
        },
        root_frame_key,
    })
}

fn is_local_j(node: &ASTNode) -> bool {
    matches!(
        node,
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } if variables == &["j"] && initial_values.len() == 1 && initial_values[0].is_none()
    )
}

fn observe_condition(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    shape_reject: NestedPredicateProjectionRejectV1,
) -> Result<NestedPredicateConditionEvidenceV1, NestedPredicateProjectionRejectV1> {
    let condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        condition.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            ..
        }
    ) {
        return Err(shape_reject);
    }
    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    Ok(NestedPredicateConditionEvidenceV1 {
        site: condition.site().clone(),
        lhs_site: lhs.site().clone(),
        binding: local_binding(function, lhs.site())?,
        bound: integer_constant(rhs.node())?,
    })
}

fn observe_update(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
) -> Result<NestedPredicateUpdateEvidenceV1, NestedPredicateProjectionRejectV1> {
    let ASTNode::Assignment { .. } = statement.node() else {
        return Err(NestedPredicateProjectionRejectV1::ChildBodySchedule);
    };
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        value.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) {
        return Err(NestedPredicateProjectionRejectV1::ChildBodySchedule);
    }
    let binding = assignment_binding(function, target.site())?;
    let lhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    if local_binding(function, lhs.site())? != binding {
        return Err(NestedPredicateProjectionRejectV1::BindingMismatch);
    }
    let rhs = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    Ok(NestedPredicateUpdateEvidenceV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        value_site: value.site().clone(),
        lhs_site: lhs.site().clone(),
        binding,
        delta: integer_constant(rhs.node())?,
    })
}

fn observe_initialization(
    source: crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    function: &VerifiedResolvedFunctionV1,
    statement: &LocatedStmtV1<'_>,
) -> Result<NestedPredicateUpdateEvidenceV1, NestedPredicateProjectionRejectV1> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Err(NestedPredicateProjectionRejectV1::RootBodySchedule);
    }
    let target = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let value = source
        .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| NestedPredicateProjectionRejectV1::SourceNavigation)?;
    let binding = assignment_binding(function, target.site())?;
    Ok(NestedPredicateUpdateEvidenceV1 {
        statement_site: statement.site().clone(),
        target_site: target.site().clone(),
        value_site: value.site().clone(),
        lhs_site: value.site().clone(),
        binding,
        delta: integer_constant(value.node())?,
    })
}

fn local_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<BindingRefV1, NestedPredicateProjectionRejectV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) => {
            Err(NestedPredicateProjectionRejectV1::UpvarBinding)
        }
        None => Err(NestedPredicateProjectionRejectV1::MissingBinding),
    }
}

fn assignment_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<BindingRefV1, NestedPredicateProjectionRejectV1> {
    match function.assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Ok(*binding),
        Some(ResolvedAssignmentTargetV1::UpvarRebind(_)) => {
            Err(NestedPredicateProjectionRejectV1::UpvarBinding)
        }
        Some(_) => Err(NestedPredicateProjectionRejectV1::NonBindingTarget),
        None => Err(NestedPredicateProjectionRejectV1::MissingBinding),
    }
}

fn declaration_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
) -> Result<BindingRefV1, NestedPredicateProjectionRejectV1> {
    function
        .declaration_binding(
            &crate::mir::resolved_semantics::SourceBindingSiteV1::Local {
                statement: site.clone(),
                ordinal: 0,
            },
        )
        .ok_or(NestedPredicateProjectionRejectV1::MissingBinding)
}

fn binding_scope(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Result<ScopeId, NestedPredicateProjectionRejectV1> {
    function
        .binding(binding)
        .map(|record| record.owner_scope())
        .ok_or(NestedPredicateProjectionRejectV1::MissingBinding)
}

fn binding_evidence(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    recurrence_owner: NestedObservedRecurrenceOwnerV1,
    parent_visible: bool,
) -> Result<NestedBindingEvidenceV1, NestedPredicateProjectionRejectV1> {
    Ok(NestedBindingEvidenceV1 {
        binding,
        lexical_scope: binding_scope(function, binding)?,
        recurrence_owner,
        parent_visible,
    })
}

fn integer_constant(node: &ASTNode) -> Result<i64, NestedPredicateProjectionRejectV1> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value),
        _ => Err(NestedPredicateProjectionRejectV1::ConstantShape),
    }
}
