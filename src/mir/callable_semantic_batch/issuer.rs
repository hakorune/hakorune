use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveOwnerForestErrorV1, ResolveSelectedCallableForestsOutcomeV1, SemanticOwnerRootProfileV1,
};
use crate::parser::{ParserCallableSyntaxLoanErrorV1, VerifiedFinalCallableProgramSourceV1};

use super::model::{
    ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableSemanticBatchV1,
    VerifiedResolvedCallableSemanticRowV1,
};

#[derive(Debug)]
pub(crate) enum ResolvedCallableSemanticBatchIssueV1 {
    ParserSyntax(ParserCallableSyntaxLoanErrorV1),
    ParameterSourceUnavailable,
    SourceCoverage,
    Resolver(ResolveOwnerForestErrorV1),
    ResolverDeferred,
    MissingRoot,
    RootProfileMismatch,
    DuplicateOwner,
    Projection(SourceNavigationErrorV1),
    ParameterCountOverflow,
}

pub(crate) fn issue_resolved_callable_semantic_batch_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    let rows = source
        .with_callable_parameter_syntax(|catalog, loan| {
            if catalog.declarations().len() != loan.declarations().len() {
                return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
            }
            let mut candidates = Vec::with_capacity(loan.declarations().len());
            let mut views = Vec::with_capacity(loan.declarations().len());
            for (expected, (source_row, syntax)) in catalog
                .declarations()
                .iter()
                .zip(loan.declarations())
                .enumerate()
            {
                let source_row_index = u32::try_from(expected)
                    .map_err(|_| ResolvedCallableSemanticBatchIssueV1::SourceCoverage)?;
                if syntax.source_row_index() != source_row_index {
                    return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                }
                let ASTNode::FunctionDeclaration { params, body, .. } = syntax.declaration() else {
                    return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                };
                let (mode, receiver) = if source_row.is_static() {
                    (
                        ResolvedCallableDeclarationModeV1::StaticBoxMethod,
                        ReceiverPolicyV1::StaticCurrentOwner,
                    )
                } else {
                    (
                        ResolvedCallableDeclarationModeV1::InstanceBoxMethod,
                        ReceiverPolicyV1::DeclaredInstance,
                    )
                };
                let parameter_count = u32::try_from(source_row.parameters().len())
                    .map_err(|_| ResolvedCallableSemanticBatchIssueV1::ParameterCountOverflow)?;
                let view =
                    FunctionSyntaxViewV1::from_borrowed_function_parts(params, body, receiver);
                candidates.push((
                    source_row_index,
                    mode,
                    parameter_count,
                    syntax.declaration(),
                    view,
                ));
                views.push(view);
            }

            let forests = match resolver
                .resolve_selected_callable_forests(&views)
                .map_err(ResolvedCallableSemanticBatchIssueV1::Resolver)?
            {
                ResolveSelectedCallableForestsOutcomeV1::Complete(forests) => forests,
                ResolveSelectedCallableForestsOutcomeV1::Deferred => {
                    return Err(ResolvedCallableSemanticBatchIssueV1::ResolverDeferred)
                }
            };
            if forests.len() != candidates.len() {
                return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
            }

            let mut owners = BTreeSet::new();
            let mut resolved = Vec::with_capacity(forests.len());
            for ((source_row_index, mode, parameter_count, declaration, view), forest) in
                candidates.into_iter().zip(forests)
            {
                let [owner] = forest.roots() else {
                    return Err(ResolvedCallableSemanticBatchIssueV1::MissingRoot);
                };
                let function = forest
                    .owner(*owner)
                    .ok_or(ResolvedCallableSemanticBatchIssueV1::MissingRoot)?;
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
                let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                    declaration,
                    &forest,
                    view.root_profile(),
                )
                .map_err(ResolvedCallableSemanticBatchIssueV1::Projection)?;
                resolved.push(VerifiedResolvedCallableSemanticRowV1 {
                    source_row_index,
                    mode,
                    parameter_count,
                    owner: *owner,
                    function_origin: function.function_origin(),
                    forest,
                    projection,
                });
            }
            Ok(resolved.into_boxed_slice())
        })
        .map_err(ResolvedCallableSemanticBatchIssueV1::ParserSyntax)?
        .ok_or(ResolvedCallableSemanticBatchIssueV1::ParameterSourceUnavailable)??;

    Ok(VerifiedResolvedCallableSemanticBatchV1 { source, rows })
}
