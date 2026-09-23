//! Caller-zero source projection for the bounded Generic residual profile.
//!
//! `FunctionSourceViewV1` is the only syntax observer. The returned product
//! keeps resolver-owned sites for the single-induction integer-progression
//! `GenericLoopV1` source shape; it contains no AST, Recipe, route,
//! Builder, CFG, PHI, or physical identity.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_structural_facts::{
    GenericResidualBodyStatementKindV1, GenericResidualBodyStatementV1,
    VerifiedGenericResidualSourceProjectionV1, VerifiedGenericResidualSourceShapeV1,
};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, VerifiedResolvedFunctionV1, VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GenericResidualProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    LoopTrueCondition,
    BodyEmpty,
    TerminalStepMissing,
    BodyStatementShape,
}

pub(crate) fn issue_generic_residual_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedGenericResidualSourceProjectionV1, GenericResidualProjectionRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(GenericResidualProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    let source = input.source();
    verify_source_identity(function, loop_stmt, &resolved_source)?;
    let root_frame_key = resolved_source.frame_key();

    let loop_condition = source
        .child_expr_from_stmt(loop_stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|_| GenericResidualProjectionRejectV1::SourceNavigation)?;
    if matches!(
        loop_condition.node(),
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return Err(GenericResidualProjectionRejectV1::LoopTrueCondition);
    }

    let loop_body = source
        .child_body_from_stmt(loop_stmt, BodyChildRoleV1::LoopBody)
        .map_err(|_| GenericResidualProjectionRejectV1::SourceNavigation)?;
    if loop_body.statements().is_empty() {
        return Err(GenericResidualProjectionRejectV1::BodyEmpty);
    }

    let mut body_statements = Vec::with_capacity(loop_body.statements().len());
    for (index, _) in loop_body.statements().iter().enumerate() {
        let stmt = source
            .body_stmt(&loop_body, index)
            .map_err(|_| GenericResidualProjectionRejectV1::SourceNavigation)?;
        let kind = match stmt.node() {
            ASTNode::Local { .. } => GenericResidualBodyStatementKindV1::LocalDeclaration,
            ASTNode::Assignment { target, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return Err(GenericResidualProjectionRejectV1::BodyStatementShape);
                }
                GenericResidualBodyStatementKindV1::Rebind
            }
            _ => return Err(GenericResidualProjectionRejectV1::BodyStatementShape),
        };
        body_statements.push(GenericResidualBodyStatementV1 {
            site: stmt.site().clone(),
            kind,
        });
    }
    if body_statements.last().map(|stmt| stmt.kind)
        != Some(GenericResidualBodyStatementKindV1::Rebind)
    {
        return Err(GenericResidualProjectionRejectV1::TerminalStepMissing);
    }

    Ok(VerifiedGenericResidualSourceProjectionV1::new(
        input.owner(),
        VerifiedGenericResidualSourceShapeV1 {
            loop_site: loop_stmt.site().clone(),
            loop_condition_site: loop_condition.site().clone(),
            body_statements: body_statements.into_boxed_slice(),
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
) -> Result<(), GenericResidualProjectionRejectV1> {
    if source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        Ok(())
    } else {
        Err(GenericResidualProjectionRejectV1::SourceLookup)
    }
}
