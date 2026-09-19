//! Caller-zero source projection for the bounded LoopCond branch shape.
//!
//! `FunctionSourceViewV1` is the only syntax observer. The returned product
//! keeps resolver-owned sites and typed exit evidence; it contains no AST,
//! Recipe, route, Builder, CFG, PHI, or physical identity.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, LoopSourceForestBindingRejectV1,
    VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    VerifiedLoopCondBreakContinueSourceProjectionV1, VerifiedLoopCondBreakContinueSourceShapeV1,
    VerifiedLoopCondSourceExitV1,
};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
    VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopCondBreakContinueProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    LoopTrueCondition,
    BodyArity,
    BranchShape,
    ExplicitElseRequired,
    BranchBodyArity,
    ExitResolution,
    ExitTargetMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopCondBreakContinueForestProjectionRejectV1 {
    ForeignOwner,
    ForestLookup,
    ForestBinding(LoopSourceForestBindingRejectV1),
    ForestEmpty,
    RootIdentity,
    MemberNotLoop,
    ExitSiteOutsideOwner,
}

/// Co-seal the resolver's complete nested Loop forest and all exits below its
/// root. The forest and exit inventory come from one verified function; no
/// source path is inferred from an AST ordinal or line number.
pub(crate) fn issue_loop_cond_break_continue_source_forest_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root_loop: &LocatedStmtV1<'_>,
) -> Result<
    VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    LoopCondBreakContinueForestProjectionRejectV1,
> {
    if input.owner() != root_loop.owner() {
        return Err(LoopCondBreakContinueForestProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root_loop.site())
        .map_err(|_| LoopCondBreakContinueForestProjectionRejectV1::ForestLookup)?;
    let member_sites = forest
        .members()
        .iter()
        .map(|member| member.source().site().clone())
        .collect::<Vec<_>>();
    let Some(root_site) = member_sites.first() else {
        return Err(LoopCondBreakContinueForestProjectionRejectV1::ForestEmpty);
    };
    if root_site != root_loop.site() {
        return Err(LoopCondBreakContinueForestProjectionRejectV1::RootIdentity);
    }
    for site in &member_sites {
        let located = input
            .source()
            .exact_stmt(site)
            .map_err(|_| LoopCondBreakContinueForestProjectionRejectV1::MemberNotLoop)?;
        if !matches!(located.node(), ASTNode::Loop { .. }) {
            return Err(LoopCondBreakContinueForestProjectionRejectV1::MemberNotLoop);
        }
    }

    let root_segments = root_site.node().segments();
    let mut exits = Vec::new();
    for (exit_site, record) in function.resolved_exits() {
        let ResolvedExitSiteV1::Statement(site) = exit_site else {
            continue;
        };
        if !site.node().segments().starts_with(root_segments) {
            continue;
        }
        if !function.source_site_inventory().contains_statement(site) {
            return Err(LoopCondBreakContinueForestProjectionRejectV1::ExitSiteOutsideOwner);
        }
        exits.push(VerifiedLoopCondSourceExitV1::new(site.clone(), *record));
    }

    let root_source = function
        .resolved_loop_source(root_site)
        .map_err(|_| LoopCondBreakContinueForestProjectionRejectV1::ForestLookup)?;
    let root_frame_key = root_source.frame_key();
    let forest_binding = bind_resolved_loop_source_forest_v1(forest)
        .map_err(LoopCondBreakContinueForestProjectionRejectV1::ForestBinding)?;

    Ok(VerifiedLoopCondBreakContinueSourceForestProjectionV1::new(
        input.owner(),
        forest_binding,
        member_sites.into_boxed_slice(),
        exits.into_boxed_slice(),
        function.function_origin(),
        function.source_kind(),
        root_frame_key,
    ))
}

pub(crate) fn issue_loop_cond_break_continue_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedLoopCondBreakContinueSourceProjectionV1, LoopCondBreakContinueProjectionRejectV1>
{
    if input.owner() != loop_stmt.owner() {
        return Err(LoopCondBreakContinueProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    let source = input.source();
    verify_source_identity(function, loop_stmt, &resolved_source)?;
    let root_frame_key = resolved_source.frame_key();

    let loop_condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    if matches!(
        loop_condition.node(),
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return Err(LoopCondBreakContinueProjectionRejectV1::LoopTrueCondition);
    }

    let loop_body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    if loop_body.statements().len() != 1 {
        return Err(LoopCondBreakContinueProjectionRejectV1::BodyArity);
    }
    let branch = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    let ASTNode::If { else_body, .. } = branch.node() else {
        return Err(LoopCondBreakContinueProjectionRejectV1::BranchShape);
    };
    if else_body.is_none() {
        return Err(LoopCondBreakContinueProjectionRejectV1::ExplicitElseRequired);
    }
    function
        .if_region_bundle(branch.site())
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceLookup)?;

    let then_body = source
        .child_body_from_stmt(&branch, BodyChildRoleV1::IfThen)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    let else_body = source
        .child_body_from_stmt(&branch, BodyChildRoleV1::IfElse)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::ExplicitElseRequired)?;
    if then_body.statements().len() != 1 || else_body.statements().len() != 1 {
        return Err(LoopCondBreakContinueProjectionRejectV1::BranchBodyArity);
    }
    let then_exit = source
        .body_stmt(&then_body, 0)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    let else_exit = source
        .body_stmt(&else_body, 0)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;
    if !matches!(then_exit.node(), ASTNode::Break { .. })
        || !matches!(else_exit.node(), ASTNode::Continue { .. })
    {
        return Err(LoopCondBreakContinueProjectionRejectV1::BranchShape);
    }

    let branch_condition = source
        .child_expr_from_stmt(&branch, ExprChildRoleV1::IfCondition)
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceNavigation)?;

    let loop_region = function
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| LoopCondBreakContinueProjectionRejectV1::SourceLookup)?
        .loop_pair()
        .region();
    let then_exit_evidence = verify_exit(
        function,
        then_exit.site(),
        ResolvedExitOriginV1::ExplicitBreak,
        ResolvedControlTransferV1::Break {
            target_loop: loop_region,
        },
    )?;
    let else_exit_evidence = verify_exit(
        function,
        else_exit.site(),
        ResolvedExitOriginV1::ExplicitContinue,
        ResolvedControlTransferV1::Continue {
            target_loop: loop_region,
        },
    )?;

    Ok(VerifiedLoopCondBreakContinueSourceProjectionV1::new(
        input.owner(),
        VerifiedLoopCondBreakContinueSourceShapeV1 {
            loop_site: loop_stmt.site().clone(),
            loop_condition_site: loop_condition.site().clone(),
            branch_site: branch.site().clone(),
            branch_condition_site: branch_condition.site().clone(),
            then_exit_site: then_exit.site().clone(),
            then_exit_origin: then_exit_evidence.origin(),
            then_exit_transfer: then_exit_evidence.transfer(),
            else_exit_site: else_exit.site().clone(),
            else_exit_origin: else_exit_evidence.origin(),
            else_exit_transfer: else_exit_evidence.transfer(),
        },
        function.function_origin(),
        function.source_kind(),
        root_frame_key,
    ))
}

fn verify_source_identity(
    function: &VerifiedResolvedFunctionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    source: &VerifiedResolvedLoopSourceV1,
) -> Result<(), LoopCondBreakContinueProjectionRejectV1> {
    if source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        Ok(())
    } else {
        Err(LoopCondBreakContinueProjectionRejectV1::SourceLookup)
    }
}

fn verify_exit(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
    expected_origin: ResolvedExitOriginV1,
    expected_transfer: ResolvedControlTransferV1,
) -> Result<ResolvedExitRecordV1, LoopCondBreakContinueProjectionRejectV1> {
    let record = function
        .resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()))
        .ok_or(LoopCondBreakContinueProjectionRejectV1::ExitResolution)?;
    if record.origin() == expected_origin && record.transfer() == expected_transfer {
        Ok(*record)
    } else {
        Err(LoopCondBreakContinueProjectionRejectV1::ExitTargetMismatch)
    }
}
