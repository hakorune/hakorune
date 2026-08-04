//! Caller-zero source projection for the bounded LoopTrue branch shape.
//!
//! `FunctionSourceViewV1` is the only syntax observer. The returned product
//! keeps resolver-owned sites, one BindingRef, and the source-issued frame;
//! it contains no AST, Recipe, route, Builder, CFG, PHI, or physical identity.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, LoopExecutionFrameKeyV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1, VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopTrueBreakContinueProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    SourceBinding(LoopRootSourceBindingRejectV1),
    LoopConditionShape,
    BodyArity,
    BranchShape,
    ExplicitElseRequired,
    BranchBodyArity,
    BranchConditionShape,
    MissingBinding,
    UpvarBinding,
    ConstantShape,
    ExitResolution,
    ExitTargetMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinueSourceShapeV1 {
    pub(crate) loop_site: SourceStmtSiteV1,
    pub(crate) loop_condition_site: SourceExprSiteV1,
    pub(crate) branch_site: SourceStmtSiteV1,
    pub(crate) branch_condition_site: SourceExprSiteV1,
    pub(crate) branch_condition_lhs_site: SourceExprSiteV1,
    pub(crate) branch_condition_rhs_site: SourceExprSiteV1,
    pub(crate) branch_condition_binding: BindingRefV1,
    pub(crate) branch_condition_bound: i64,
    pub(crate) then_exit_site: SourceStmtSiteV1,
    pub(crate) else_exit_site: SourceStmtSiteV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinueSourceProjectionV1 {
    source_binding: VerifiedLoopRootSourceV1,
    shape: VerifiedLoopTrueBreakContinueSourceShapeV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedLoopTrueBreakContinueSourceProjectionV1 {
    pub(crate) fn source_binding(&self) -> &VerifiedLoopRootSourceV1 {
        &self.source_binding
    }

    pub(crate) fn shape(&self) -> &VerifiedLoopTrueBreakContinueSourceShapeV1 {
        &self.shape
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopRootSourceV1,
        VerifiedLoopTrueBreakContinueSourceShapeV1,
        LoopExecutionFrameKeyV1,
    ) {
        (self.source_binding, self.shape, self.root_frame_key)
    }
}

pub(crate) fn issue_loop_true_break_continue_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedLoopTrueBreakContinueSourceProjectionV1, LoopTrueBreakContinueProjectionRejectV1>
{
    if input.owner() != loop_stmt.owner() {
        return Err(LoopTrueBreakContinueProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    let source = input.source();
    verify_source_identity(function, loop_stmt, &resolved_source)?;
    let root_frame_key = resolved_source.frame_key();

    let loop_condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        loop_condition.node(),
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return Err(LoopTrueBreakContinueProjectionRejectV1::LoopConditionShape);
    }

    let loop_body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    if loop_body.statements().len() != 1 {
        return Err(LoopTrueBreakContinueProjectionRejectV1::BodyArity);
    }
    let branch = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    let ASTNode::If { else_body, .. } = branch.node() else {
        return Err(LoopTrueBreakContinueProjectionRejectV1::BranchShape);
    };
    if else_body.is_none() {
        return Err(LoopTrueBreakContinueProjectionRejectV1::ExplicitElseRequired);
    }
    function
        .if_region_bundle(branch.site())
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceLookup)?;

    let then_body = source
        .child_body_from_stmt(&branch, BodyChildRoleV1::IfThen)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    let else_body = source
        .child_body_from_stmt(&branch, BodyChildRoleV1::IfElse)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::ExplicitElseRequired)?;
    if then_body.statements().len() != 1 || else_body.statements().len() != 1 {
        return Err(LoopTrueBreakContinueProjectionRejectV1::BranchBodyArity);
    }
    let then_exit = source
        .body_stmt(&then_body, 0)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    let else_exit = source
        .body_stmt(&else_body, 0)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    if !matches!(then_exit.node(), ASTNode::Break { .. })
        || !matches!(else_exit.node(), ASTNode::Continue { .. })
    {
        return Err(LoopTrueBreakContinueProjectionRejectV1::BranchShape);
    }

    let branch_condition = source
        .child_expr_from_stmt(&branch, ExprChildRoleV1::IfCondition)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    if !matches!(
        branch_condition.node(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            ..
        }
    ) {
        return Err(LoopTrueBreakContinueProjectionRejectV1::BranchConditionShape);
    }
    let lhs = source
        .child_expr_from_expr(&branch_condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    let rhs = source
        .child_expr_from_expr(&branch_condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceNavigation)?;
    let binding = local_binding(function, lhs.site())?;
    let bound = integer_constant(rhs.node())?;

    let loop_region = function
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| LoopTrueBreakContinueProjectionRejectV1::SourceLookup)?
        .loop_pair()
        .region();
    verify_exit(
        function,
        then_exit.site(),
        ResolvedExitOriginV1::ExplicitBreak,
        ResolvedControlTransferV1::Break {
            target_loop: loop_region,
        },
    )?;
    verify_exit(
        function,
        else_exit.site(),
        ResolvedExitOriginV1::ExplicitContinue,
        ResolvedControlTransferV1::Continue {
            target_loop: loop_region,
        },
    )?;

    let source_binding = bind_resolved_loop_root_v1(resolved_source)
        .map_err(LoopTrueBreakContinueProjectionRejectV1::SourceBinding)?;
    Ok(VerifiedLoopTrueBreakContinueSourceProjectionV1 {
        source_binding,
        shape: VerifiedLoopTrueBreakContinueSourceShapeV1 {
            loop_site: loop_stmt.site().clone(),
            loop_condition_site: loop_condition.site().clone(),
            branch_site: branch.site().clone(),
            branch_condition_site: branch_condition.site().clone(),
            branch_condition_lhs_site: lhs.site().clone(),
            branch_condition_rhs_site: rhs.site().clone(),
            branch_condition_binding: binding,
            branch_condition_bound: bound,
            then_exit_site: then_exit.site().clone(),
            else_exit_site: else_exit.site().clone(),
        },
        root_frame_key,
    })
}

fn verify_source_identity(
    function: &VerifiedResolvedFunctionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    source: &VerifiedResolvedLoopSourceV1,
) -> Result<(), LoopTrueBreakContinueProjectionRejectV1> {
    if source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        Ok(())
    } else {
        Err(LoopTrueBreakContinueProjectionRejectV1::SourceLookup)
    }
}

fn verify_exit(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
    expected_origin: ResolvedExitOriginV1,
    expected_transfer: ResolvedControlTransferV1,
) -> Result<(), LoopTrueBreakContinueProjectionRejectV1> {
    let record = function
        .resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()))
        .ok_or(LoopTrueBreakContinueProjectionRejectV1::ExitResolution)?;
    if record.origin() == expected_origin && record.transfer() == expected_transfer {
        Ok(())
    } else {
        Err(LoopTrueBreakContinueProjectionRejectV1::ExitTargetMismatch)
    }
}

fn local_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Result<BindingRefV1, LoopTrueBreakContinueProjectionRejectV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
        Some(ResolvedLexicalRefV1::Upvar(_)) => {
            Err(LoopTrueBreakContinueProjectionRejectV1::UpvarBinding)
        }
        None => Err(LoopTrueBreakContinueProjectionRejectV1::MissingBinding),
    }
}

fn integer_constant(node: &ASTNode) -> Result<i64, LoopTrueBreakContinueProjectionRejectV1> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value),
        _ => Err(LoopTrueBreakContinueProjectionRejectV1::ConstantShape),
    }
}
