//! Exact AST-free source inventory for the first unchanged Dynamic Loop body.
//!
//! This observer is deliberately pre-Recipe and pre-Builder. It navigates one
//! resolver-backed source view, consumes the exact Loop membership and
//! Completion products, and emits source roles/sites/BindingRefs only.

use std::collections::BTreeSet;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ExprChildRoleV1, ResolvedAssignmentTargetV1, ResolvedExitSiteV1,
    ResolvedLexicalRefV1, ScopeId, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourceStmtSiteV1, VerifiedCallableLoopMembershipV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedExprV1, LocatedStmtV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DynamicFullBodySourceRoleV1 {
    PreludeLocalI,
    PreludeInitializerPos,
    Loop,
    LoopCondition,
    LoopConditionI,
    LoopConditionEnd,
    ChLocal,
    SubstringCall,
    SubstringReceiverSrc,
    SubstringStartI,
    SubstringEndAdd,
    SubstringEndI,
    SubstringEndDelta,
    InnerIf,
    InnerIfCondition,
    IndexOfCall,
    IndexOfReceiverPredChars,
    IndexOfArgumentCh,
    InnerIfZero,
    InnerReturn,
    InnerReturnI,
    StepAssignment,
    StepTargetI,
    StepAdd,
    StepReadI,
    StepDelta,
    OuterReturn,
    OuterReturnI,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicFullBodySourceSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DynamicFullBodySourceRowV1 {
    role: DynamicFullBodySourceRoleV1,
    site: DynamicFullBodySourceSiteV1,
}

impl DynamicFullBodySourceRowV1 {
    pub(crate) const fn role(&self) -> DynamicFullBodySourceRoleV1 {
        self.role
    }

    pub(crate) const fn site(&self) -> &DynamicFullBodySourceSiteV1 {
        &self.site
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DynamicFullBodyBindingRoleV1 {
    Src,
    Pos,
    End,
    PredChars,
    Induction,
    IterationLocalCh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DynamicFullBodyBindingRowV1 {
    role: DynamicFullBodyBindingRoleV1,
    declaration: SourceBindingSiteV1,
    binding: BindingRefV1,
}

impl DynamicFullBodyBindingRowV1 {
    pub(crate) const fn role(&self) -> DynamicFullBodyBindingRoleV1 {
        self.role
    }

    pub(crate) const fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedDynamicLoopFullBodySourceInventoryV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    loop_membership: VerifiedCallableLoopMembershipV1,
    bindings: Box<[DynamicFullBodyBindingRowV1]>,
    rows: Box<[DynamicFullBodySourceRowV1]>,
    completion: VerifiedFunctionCompletionV1,
}

impl VerifiedDynamicLoopFullBodySourceInventoryV1 {
    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_membership(&self) -> &VerifiedCallableLoopMembershipV1 {
        &self.loop_membership
    }

    pub(crate) fn bindings(&self) -> &[DynamicFullBodyBindingRowV1] {
        &self.bindings
    }

    pub(crate) fn rows(&self) -> &[DynamicFullBodySourceRowV1] {
        &self.rows
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedCallableLoopMembershipV1,
        Box<[DynamicFullBodyBindingRowV1]>,
        Box<[DynamicFullBodySourceRowV1]>,
        VerifiedFunctionCompletionV1,
    ) {
        (
            self.loop_membership,
            self.bindings,
            self.rows,
            self.completion,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicFullBodySourceIssueV1 {
    ForeignOwner,
    SourceNavigation,
    RootShape,
    LoopShape,
    BodyShape,
    ExpressionShape,
    MissingResolverEvidence,
    BindingMismatch,
    IterationLocalScopeMismatch,
    IterationLocalUseClosureMismatch,
    DuplicateSourceRole,
    DuplicateSourceSite,
    CompletionMismatch,
    CoverageMismatch,
}

pub(crate) struct DynamicFullBodySourceIssuerV1;

impl DynamicFullBodySourceIssuerV1 {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        loop_membership: VerifiedCallableLoopMembershipV1,
        completion: VerifiedFunctionCompletionV1,
    ) -> Result<VerifiedDynamicLoopFullBodySourceInventoryV1, DynamicFullBodySourceIssueV1> {
        let owner = input.owner();
        if completion.owner() != owner
            || loop_membership.scope_region().scope().owner() != owner
            || loop_membership.scope_region().region().owner() != owner
        {
            return Err(DynamicFullBodySourceIssueV1::ForeignOwner);
        }

        let source = input.source();
        let root = source
            .root_body()
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        if root.statements().len() != 3 {
            return Err(DynamicFullBodySourceIssueV1::RootShape);
        }

        let prelude = source
            .body_stmt(&root, 0)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let loop_stmt = source
            .body_stmt(&root, 1)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let outer_return = source
            .body_stmt(&root, 2)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let exact_loop = source
            .stmt_at(&loop_membership)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        if loop_stmt.site() != exact_loop.site()
            || !matches!(loop_stmt.node(), ASTNode::Loop { .. })
        {
            return Err(DynamicFullBodySourceIssueV1::LoopShape);
        }

        ensure_single_local(prelude.node())?;
        let prelude_value = expr_from_stmt(source, &prelude, ExprChildRoleV1::LocalInitializer(0))?;
        ensure_variable(prelude_value.node())?;

        let condition = expr_from_stmt(source, &loop_stmt, ExprChildRoleV1::LoopCondition)?;
        ensure_binary(condition.node(), BinaryOperator::Less)?;
        let condition_i = expr_from_expr(source, &condition, ExprChildRoleV1::BinaryLeft)?;
        let condition_end = expr_from_expr(source, &condition, ExprChildRoleV1::BinaryRight)?;
        ensure_variable(condition_i.node())?;
        ensure_variable(condition_end.node())?;

        let body = source
            .child_body_from_stmt(
                &loop_stmt,
                crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
            )
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        if body.statements().len() != 3 {
            return Err(DynamicFullBodySourceIssueV1::BodyShape);
        }
        let ch_local = source
            .body_stmt(&body, 0)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let inner_if = source
            .body_stmt(&body, 1)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let step = source
            .body_stmt(&body, 2)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;

        ensure_single_local(ch_local.node())?;
        let substring = expr_from_stmt(source, &ch_local, ExprChildRoleV1::LocalInitializer(0))?;
        ensure_method_call(substring.node(), "substring", 2)?;
        let substring_receiver = expr_from_expr(source, &substring, ExprChildRoleV1::Receiver)?;
        let substring_start = expr_from_expr(source, &substring, ExprChildRoleV1::CallArgument(0))?;
        let substring_end = expr_from_expr(source, &substring, ExprChildRoleV1::CallArgument(1))?;
        ensure_variable(substring_receiver.node())?;
        ensure_variable(substring_start.node())?;
        ensure_binary(substring_end.node(), BinaryOperator::Add)?;
        let substring_end_i = expr_from_expr(source, &substring_end, ExprChildRoleV1::BinaryLeft)?;
        let substring_end_delta =
            expr_from_expr(source, &substring_end, ExprChildRoleV1::BinaryRight)?;
        ensure_variable(substring_end_i.node())?;
        ensure_integer(substring_end_delta.node(), 1)?;

        let ASTNode::If { else_body, .. } = inner_if.node() else {
            return Err(DynamicFullBodySourceIssueV1::BodyShape);
        };
        if else_body.is_some() {
            return Err(DynamicFullBodySourceIssueV1::BodyShape);
        }
        let if_condition = expr_from_stmt(source, &inner_if, ExprChildRoleV1::IfCondition)?;
        ensure_binary(if_condition.node(), BinaryOperator::Less)?;
        let index_of = expr_from_expr(source, &if_condition, ExprChildRoleV1::BinaryLeft)?;
        let if_zero = expr_from_expr(source, &if_condition, ExprChildRoleV1::BinaryRight)?;
        ensure_method_call(index_of.node(), "indexOf", 1)?;
        ensure_integer(if_zero.node(), 0)?;
        let index_receiver = expr_from_expr(source, &index_of, ExprChildRoleV1::Receiver)?;
        let index_ch = expr_from_expr(source, &index_of, ExprChildRoleV1::CallArgument(0))?;
        ensure_variable(index_receiver.node())?;
        ensure_variable(index_ch.node())?;

        let then_body = source
            .child_body_from_stmt(
                &inner_if,
                crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
            )
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        if then_body.statements().len() != 1 {
            return Err(DynamicFullBodySourceIssueV1::BodyShape);
        }
        let inner_return = source
            .body_stmt(&then_body, 0)
            .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)?;
        let inner_return_i = return_value(source, &inner_return)?;
        ensure_variable(inner_return_i.node())?;

        let ASTNode::Assignment { .. } = step.node() else {
            return Err(DynamicFullBodySourceIssueV1::BodyShape);
        };
        let step_target = expr_from_stmt(source, &step, ExprChildRoleV1::AssignmentTarget)?;
        let step_add = expr_from_stmt(source, &step, ExprChildRoleV1::AssignmentValue)?;
        ensure_variable(step_target.node())?;
        ensure_binary(step_add.node(), BinaryOperator::Add)?;
        let step_i = expr_from_expr(source, &step_add, ExprChildRoleV1::BinaryLeft)?;
        let step_delta = expr_from_expr(source, &step_add, ExprChildRoleV1::BinaryRight)?;
        ensure_variable(step_i.node())?;
        ensure_integer(step_delta.node(), 1)?;

        let outer_return_i = return_value(source, &outer_return)?;
        ensure_variable(outer_return_i.node())?;

        let parameter_sites = (0..4)
            .map(|index| SourceBindingSiteV1::Parameter { index })
            .collect::<Vec<_>>();
        let i_site = local_site(prelude.site());
        let ch_site = local_site(ch_local.site());
        let mut bindings = Vec::with_capacity(6);
        for (role, declaration) in [
            (
                DynamicFullBodyBindingRoleV1::Src,
                parameter_sites[0].clone(),
            ),
            (
                DynamicFullBodyBindingRoleV1::Pos,
                parameter_sites[1].clone(),
            ),
            (
                DynamicFullBodyBindingRoleV1::End,
                parameter_sites[2].clone(),
            ),
            (
                DynamicFullBodyBindingRoleV1::PredChars,
                parameter_sites[3].clone(),
            ),
            (DynamicFullBodyBindingRoleV1::Induction, i_site.clone()),
            (
                DynamicFullBodyBindingRoleV1::IterationLocalCh,
                ch_site.clone(),
            ),
        ] {
            let binding = input
                .function()
                .declaration_binding(&declaration)
                .ok_or(DynamicFullBodySourceIssueV1::MissingResolverEvidence)?;
            bindings.push(DynamicFullBodyBindingRowV1 {
                role,
                declaration,
                binding,
            });
        }
        let binding = |role| {
            bindings
                .iter()
                .find(|row| row.role == role)
                .map(|row| row.binding)
                .ok_or(DynamicFullBodySourceIssueV1::MissingResolverEvidence)
        };
        require_local(
            input,
            prelude_value.site(),
            binding(DynamicFullBodyBindingRoleV1::Pos)?,
        )?;
        for site in [
            condition_i.site(),
            substring_start.site(),
            substring_end_i.site(),
            inner_return_i.site(),
            step_i.site(),
            outer_return_i.site(),
        ] {
            require_local(
                input,
                site,
                binding(DynamicFullBodyBindingRoleV1::Induction)?,
            )?;
        }
        require_local(
            input,
            condition_end.site(),
            binding(DynamicFullBodyBindingRoleV1::End)?,
        )?;
        require_local(
            input,
            substring_receiver.site(),
            binding(DynamicFullBodyBindingRoleV1::Src)?,
        )?;
        require_local(
            input,
            index_receiver.site(),
            binding(DynamicFullBodyBindingRoleV1::PredChars)?,
        )?;
        require_local(
            input,
            index_ch.site(),
            binding(DynamicFullBodyBindingRoleV1::IterationLocalCh)?,
        )?;
        verify_iteration_local_source_closure(
            input,
            loop_membership.scope_region().scope(),
            binding(DynamicFullBodyBindingRoleV1::IterationLocalCh)?,
            index_ch.site(),
        )?;
        require_assignment(
            input,
            step_target.site(),
            binding(DynamicFullBodyBindingRoleV1::Induction)?,
        )?;

        let mut rows = Vec::with_capacity(28);
        let mut roles = BTreeSet::new();
        let mut sites = BTreeSet::new();
        macro_rules! stmt_row {
            ($role:expr, $value:expr) => {
                push_row(
                    &mut rows,
                    &mut roles,
                    &mut sites,
                    $role,
                    DynamicFullBodySourceSiteV1::Statement($value.site().clone()),
                )?
            };
        }
        macro_rules! expr_row {
            ($role:expr, $value:expr) => {
                push_row(
                    &mut rows,
                    &mut roles,
                    &mut sites,
                    $role,
                    DynamicFullBodySourceSiteV1::Expression($value.site().clone()),
                )?
            };
        }
        stmt_row!(DynamicFullBodySourceRoleV1::PreludeLocalI, prelude);
        expr_row!(
            DynamicFullBodySourceRoleV1::PreludeInitializerPos,
            prelude_value
        );
        stmt_row!(DynamicFullBodySourceRoleV1::Loop, loop_stmt);
        expr_row!(DynamicFullBodySourceRoleV1::LoopCondition, condition);
        expr_row!(DynamicFullBodySourceRoleV1::LoopConditionI, condition_i);
        expr_row!(DynamicFullBodySourceRoleV1::LoopConditionEnd, condition_end);
        stmt_row!(DynamicFullBodySourceRoleV1::ChLocal, ch_local);
        expr_row!(DynamicFullBodySourceRoleV1::SubstringCall, substring);
        expr_row!(
            DynamicFullBodySourceRoleV1::SubstringReceiverSrc,
            substring_receiver
        );
        expr_row!(
            DynamicFullBodySourceRoleV1::SubstringStartI,
            substring_start
        );
        expr_row!(DynamicFullBodySourceRoleV1::SubstringEndAdd, substring_end);
        expr_row!(DynamicFullBodySourceRoleV1::SubstringEndI, substring_end_i);
        expr_row!(
            DynamicFullBodySourceRoleV1::SubstringEndDelta,
            substring_end_delta
        );
        stmt_row!(DynamicFullBodySourceRoleV1::InnerIf, inner_if);
        expr_row!(DynamicFullBodySourceRoleV1::InnerIfCondition, if_condition);
        expr_row!(DynamicFullBodySourceRoleV1::IndexOfCall, index_of);
        expr_row!(
            DynamicFullBodySourceRoleV1::IndexOfReceiverPredChars,
            index_receiver
        );
        expr_row!(DynamicFullBodySourceRoleV1::IndexOfArgumentCh, index_ch);
        expr_row!(DynamicFullBodySourceRoleV1::InnerIfZero, if_zero);
        stmt_row!(DynamicFullBodySourceRoleV1::InnerReturn, inner_return);
        expr_row!(DynamicFullBodySourceRoleV1::InnerReturnI, inner_return_i);
        stmt_row!(DynamicFullBodySourceRoleV1::StepAssignment, step);
        expr_row!(DynamicFullBodySourceRoleV1::StepTargetI, step_target);
        expr_row!(DynamicFullBodySourceRoleV1::StepAdd, step_add);
        expr_row!(DynamicFullBodySourceRoleV1::StepReadI, step_i);
        expr_row!(DynamicFullBodySourceRoleV1::StepDelta, step_delta);
        stmt_row!(DynamicFullBodySourceRoleV1::OuterReturn, outer_return);
        expr_row!(DynamicFullBodySourceRoleV1::OuterReturnI, outer_return_i);
        if rows.len() != 28 || bindings.len() != 6 {
            return Err(DynamicFullBodySourceIssueV1::CoverageMismatch);
        }

        let expected_returns =
            BTreeSet::from([inner_return.site().clone(), outer_return.site().clone()]);
        let actual_returns = completion
            .explicit_sites()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if expected_returns != actual_returns
            || expected_returns.iter().any(|site| {
                input
                    .function()
                    .resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()))
                    .is_none()
            })
        {
            return Err(DynamicFullBodySourceIssueV1::CompletionMismatch);
        }

        Ok(VerifiedDynamicLoopFullBodySourceInventoryV1 {
            owner,
            loop_membership,
            bindings: bindings.into_boxed_slice(),
            rows: rows.into_boxed_slice(),
            completion,
        })
    }
}

/// Closes the resolver-owned lexical boundary for the iteration-local value.
///
/// The exact source observer has already identified the declaration and I7
/// argument site. This check does not classify Home or lifetime behavior; it
/// only proves that the declaration belongs to the sealed Loop-body scope and
/// that the same binding has one exact read with no write or nested capture.
pub(super) fn verify_iteration_local_source_closure(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_body_scope: ScopeId,
    binding: BindingRefV1,
    expected_read: &SourceExprSiteV1,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    let record = input
        .function()
        .binding(binding)
        .ok_or(DynamicFullBodySourceIssueV1::MissingResolverEvidence)?;
    if record.owner_scope() != loop_body_scope {
        return Err(DynamicFullBodySourceIssueV1::IterationLocalScopeMismatch);
    }

    let reads = input
        .function()
        .variable_refs()
        .filter_map(|(site, resolved)| {
            matches!(resolved, ResolvedLexicalRefV1::Local(actual) if *actual == binding)
                .then_some(site)
        })
        .collect::<Vec<_>>();
    let reassigned = input.function().assignment_targets().any(|(_, target)| {
        matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)
    });
    let captured = input.forest().owners().any(|(owner, _)| {
        input
            .forest()
            .ordered_capture_demands(owner)
            .iter()
            .any(|row| row.source_binding() == binding)
    });
    if reads.as_slice() != [expected_read] || reassigned || captured {
        return Err(DynamicFullBodySourceIssueV1::IterationLocalUseClosureMismatch);
    }
    Ok(())
}

fn expr_from_stmt<'a>(
    source: super::source_view::FunctionSourceViewV1<'a>,
    parent: &LocatedStmtV1<'a>,
    role: ExprChildRoleV1,
) -> Result<LocatedExprV1<'a>, DynamicFullBodySourceIssueV1> {
    source
        .child_expr_from_stmt(parent, role)
        .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)
}

fn expr_from_expr<'a>(
    source: super::source_view::FunctionSourceViewV1<'a>,
    parent: &LocatedExprV1<'a>,
    role: ExprChildRoleV1,
) -> Result<LocatedExprV1<'a>, DynamicFullBodySourceIssueV1> {
    source
        .child_expr_from_expr(parent, role)
        .map_err(|_| DynamicFullBodySourceIssueV1::SourceNavigation)
}

fn return_value<'a>(
    source: super::source_view::FunctionSourceViewV1<'a>,
    statement: &LocatedStmtV1<'a>,
) -> Result<LocatedExprV1<'a>, DynamicFullBodySourceIssueV1> {
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        return Err(DynamicFullBodySourceIssueV1::BodyShape);
    }
    expr_from_stmt(source, statement, ExprChildRoleV1::ReturnValue)
}

fn ensure_single_local(node: &ASTNode) -> Result<(), DynamicFullBodySourceIssueV1> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = node
    else {
        return Err(DynamicFullBodySourceIssueV1::BodyShape);
    };
    if variables.len() != 1 || initial_values.len() != 1 || initial_values[0].is_none() {
        return Err(DynamicFullBodySourceIssueV1::BodyShape);
    }
    Ok(())
}

fn ensure_variable(node: &ASTNode) -> Result<(), DynamicFullBodySourceIssueV1> {
    matches!(node, ASTNode::Variable { .. })
        .then_some(())
        .ok_or(DynamicFullBodySourceIssueV1::ExpressionShape)
}

fn ensure_binary(
    node: &ASTNode,
    expected: BinaryOperator,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    matches!(node, ASTNode::BinaryOp { operator, .. } if *operator == expected)
        .then_some(())
        .ok_or(DynamicFullBodySourceIssueV1::ExpressionShape)
}

fn ensure_integer(node: &ASTNode, expected: i64) -> Result<(), DynamicFullBodySourceIssueV1> {
    matches!(node, ASTNode::Literal { value: LiteralValue::Integer(actual), .. } if *actual == expected)
        .then_some(())
        .ok_or(DynamicFullBodySourceIssueV1::ExpressionShape)
}

fn ensure_method_call(
    node: &ASTNode,
    expected_method: &str,
    expected_arity: usize,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    matches!(node, ASTNode::MethodCall { method, arguments, .. } if method == expected_method && arguments.len() == expected_arity)
        .then_some(())
        .ok_or(DynamicFullBodySourceIssueV1::ExpressionShape)
}

fn local_site(statement: &SourceStmtSiteV1) -> SourceBindingSiteV1 {
    SourceBindingSiteV1::Local {
        statement: statement.clone(),
        ordinal: 0,
    }
}

fn require_local(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    expected: BindingRefV1,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    match input.function().variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(actual)) if actual == expected => Ok(()),
        Some(_) => Err(DynamicFullBodySourceIssueV1::BindingMismatch),
        None => Err(DynamicFullBodySourceIssueV1::MissingResolverEvidence),
    }
}

fn require_assignment(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    expected: BindingRefV1,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    match input.function().assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(actual)) if *actual == expected => Ok(()),
        Some(_) => Err(DynamicFullBodySourceIssueV1::BindingMismatch),
        None => Err(DynamicFullBodySourceIssueV1::MissingResolverEvidence),
    }
}

fn push_row(
    rows: &mut Vec<DynamicFullBodySourceRowV1>,
    roles: &mut BTreeSet<DynamicFullBodySourceRoleV1>,
    sites: &mut BTreeSet<SourceNodeSiteV1>,
    role: DynamicFullBodySourceRoleV1,
    site: DynamicFullBodySourceSiteV1,
) -> Result<(), DynamicFullBodySourceIssueV1> {
    if !roles.insert(role) {
        return Err(DynamicFullBodySourceIssueV1::DuplicateSourceRole);
    }
    let node = match &site {
        DynamicFullBodySourceSiteV1::Statement(site) => site.node(),
        DynamicFullBodySourceSiteV1::Expression(site) => site.node(),
    };
    if !sites.insert(node.clone()) {
        return Err(DynamicFullBodySourceIssueV1::DuplicateSourceSite);
    }
    rows.push(DynamicFullBodySourceRowV1 { role, site });
    Ok(())
}
