use std::collections::BTreeSet;
use std::sync::Arc;

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    issue_resolved_block_expr_expectation_v1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ReceiverPolicyV1, ResolveOwnerForestErrorV1,
    ResolveSelectedCallableForestsWithBodyShapesOutcomeV1,
    ResolvedBlockExpressionExpectationIssueV1, SemanticOwnerRootProfileV1,
};
use crate::parser::{
    FinalCallableDeclarationModeV1, FinalCallableSemanticSyntaxLoanErrorV1,
    VerifiedFinalCallableProgramSourceV1,
};

use super::model::{
    ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableSemanticBatchV1,
    VerifiedResolvedCallableSemanticRowV1,
};

#[derive(Debug)]
pub(crate) enum ResolvedCallableSemanticBatchIssueV1 {
    ParserSyntax(FinalCallableSemanticSyntaxLoanErrorV1),
    SourceCoverage,
    Resolver(ResolveOwnerForestErrorV1),
    ResolverDeferred,
    MissingRoot,
    RootProfileMismatch,
    BodyShapeMissing,
    BodyShapeOwnerMismatch,
    BlockExprExpectation(ResolvedBlockExpressionExpectationIssueV1),
    DuplicateOwner,
    Projection(SourceNavigationErrorV1),
    ParameterCountOverflow,
}

pub(crate) fn issue_resolved_callable_semantic_batch_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    issue_resolved_callable_semantic_batch_with_brand_catalog_v1(resolver, source, None)
}

pub(crate) fn issue_resolved_callable_semantic_batch_with_brand_catalog_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    let rows = source
        .with_callable_semantic_syntax(|loan| {
            let mut candidates = Vec::with_capacity(loan.rows().len());
            let mut views = Vec::with_capacity(loan.rows().len());
            for (expected, syntax) in loan.rows().iter().enumerate() {
                let batch_slot = u32::try_from(expected)
                    .map_err(|_| ResolvedCallableSemanticBatchIssueV1::SourceCoverage)?;
                if syntax.batch_slot() != batch_slot {
                    return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                }
                let ASTNode::FunctionDeclaration { params, body, .. } = syntax.declaration() else {
                    return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                };
                let (mode, receiver) = match syntax.mode() {
                    FinalCallableDeclarationModeV1::TopLevel => (
                        ResolvedCallableDeclarationModeV1::TopLevel,
                        ReceiverPolicyV1::Absent,
                    ),
                    FinalCallableDeclarationModeV1::StaticBoxMethod => (
                        ResolvedCallableDeclarationModeV1::StaticBoxMethod,
                        ReceiverPolicyV1::StaticCurrentOwner,
                    ),
                    FinalCallableDeclarationModeV1::InstanceBoxMethod => (
                        ResolvedCallableDeclarationModeV1::InstanceBoxMethod,
                        ReceiverPolicyV1::DeclaredInstance,
                    ),
                };
                let parameter_count = u32::try_from(params.len())
                    .map_err(|_| ResolvedCallableSemanticBatchIssueV1::ParameterCountOverflow)?;
                let view =
                    FunctionSyntaxViewV1::from_borrowed_function_parts(params, body, receiver);
                candidates.push((
                    batch_slot,
                    syntax.identity().clone(),
                    mode,
                    parameter_count,
                    syntax.declaration(),
                    syntax.method_source_observation().cloned(),
                    view,
                ));
                views.push(view);
            }

            let (forests, mut body_shapes) = match resolver
                .resolve_selected_callable_forests_with_body_shapes_and_brand_catalog(
                    &views,
                    brand_catalog,
                )
                .map_err(ResolvedCallableSemanticBatchIssueV1::Resolver)?
            {
                ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                    forests,
                    body_shapes,
                } => (forests, body_shapes),
                ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred => {
                    return Err(ResolvedCallableSemanticBatchIssueV1::ResolverDeferred)
                }
            };
            if forests.len() != candidates.len() {
                return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
            }

            let mut owners = BTreeSet::new();
            let mut resolved = Vec::with_capacity(forests.len());
            for (
                (
                    batch_slot,
                    identity,
                    mode,
                    parameter_count,
                    declaration,
                    method_source_observation,
                    view,
                ),
                forest,
            ) in candidates.into_iter().zip(forests)
            {
                let [owner] = forest.roots() else {
                    return Err(ResolvedCallableSemanticBatchIssueV1::MissingRoot);
                };
                let function = forest
                    .owner(*owner)
                    .ok_or(ResolvedCallableSemanticBatchIssueV1::MissingRoot)?;
                let body_shape = body_shapes
                    .remove(owner)
                    .ok_or(ResolvedCallableSemanticBatchIssueV1::BodyShapeMissing)?;
                if body_shape.owner() != *owner
                    || body_shape.body_root() != &view.root_profile().body_root()
                {
                    return Err(ResolvedCallableSemanticBatchIssueV1::BodyShapeOwnerMismatch);
                }
                if function.root_profile() != view.root_profile()
                    || !matches!(
                        function.root_profile(),
                        SemanticOwnerRootProfileV1::DeclaredFunction { .. }
                    )
                {
                    return Err(ResolvedCallableSemanticBatchIssueV1::RootProfileMismatch);
                }
                if !owners.insert(*owner) {
                    return Err(ResolvedCallableSemanticBatchIssueV1::DuplicateOwner);
                }
                let block_expr_expectation =
                    issue_resolved_block_expr_expectation_v1(function, &body_shape)
                        .map_err(ResolvedCallableSemanticBatchIssueV1::BlockExprExpectation)?;
                let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                    declaration,
                    &forest,
                    view.root_profile(),
                )
                .map_err(ResolvedCallableSemanticBatchIssueV1::Projection)?;
                resolved.push(VerifiedResolvedCallableSemanticRowV1 {
                    batch_slot,
                    identity,
                    mode,
                    parameter_count,
                    owner: *owner,
                    function_origin: function.function_origin(),
                    forest,
                    body_shape: Arc::new(body_shape),
                    block_expr_expectation,
                    projection,
                    method_source_observation,
                });
            }
            Ok(resolved.into_boxed_slice())
        })
        .map_err(ResolvedCallableSemanticBatchIssueV1::ParserSyntax)??;

    Ok(VerifiedResolvedCallableSemanticBatchV1 { source, rows })
}
