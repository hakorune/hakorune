use std::collections::BTreeSet;
use std::sync::Arc;

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    forest_has_unissued_direct_call_observation_v1, issue_resolved_block_expr_expectation_v1,
    CallableHeaderSyntaxViewV1, DeclaredInstanceCallEffectIssueV1,
    DeclaredInstanceCallEffectIssuerV1, DeclaredInstanceCallRelationIssueV1,
    DeclaredInstanceCallRelationIssuerV1, DeclaredInstanceCallSourceRefV1,
    DeclaredInstanceMethodModeV1, DeclaredInstanceMethodSourceRefV1,
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1,
    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1,
    ResolvedBlockExpressionExpectationIssueV1, ResolvedLexicalRefV1,
    ResolvedMethodCallReceiverSourceV1, SelectedCallableResolverDeferredBatchV1,
    SelectedCallableResolverInputV1, SemanticOwnerRootProfileV1, SourceBindingSiteV1,
    SourceBoundSelectedCallableResolverRejectV1,
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
    ParserSyntax {
        _error: FinalCallableSemanticSyntaxLoanErrorV1,
    },
    SourceCoverage,
    UnissuedDirectCallObservation,
    Resolver(SourceBoundSelectedCallableResolverRejectV1),
    ResolverDeferred(SelectedCallableResolverDeferredBatchV1),
    MissingRoot,
    RootProfileMismatch,
    BodyShapeMissing,
    BodyShapeOwnerMismatch,
    BlockExprExpectation {
        _error: ResolvedBlockExpressionExpectationIssueV1,
    },
    DuplicateOwner,
    Projection {
        _error: SourceNavigationErrorV1,
    },
    ParameterCountOverflow,
    DeclaredInstanceCallRelation {
        _error: DeclaredInstanceCallRelationIssueV1,
    },
    DeclaredInstanceCallEffect {
        _error: DeclaredInstanceCallEffectIssueV1,
    },
}

/// Controls whether unissued direct-call observations remain available to a
/// package-owned validation boundary.  The default public batch entry keeps
/// rejecting them; only the Cataloged validation row opts into observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectCallObservationBatchPolicyV1 {
    RejectUnissued,
    ObserveForCatalogedValidation,
}

pub(crate) fn issue_resolved_callable_semantic_batch_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    issue_resolved_callable_semantic_batch_with_policy_v1(
        resolver,
        source,
        None,
        DirectCallObservationBatchPolicyV1::RejectUnissued,
    )
}

pub(crate) fn issue_resolved_callable_semantic_batch_with_brand_catalog_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    issue_resolved_callable_semantic_batch_with_policy_v1(
        resolver,
        source,
        brand_catalog,
        DirectCallObservationBatchPolicyV1::RejectUnissued,
    )
}

pub(crate) fn issue_resolved_callable_semantic_batch_with_policy_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    direct_call_policy: DirectCallObservationBatchPolicyV1,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    issue_resolved_callable_semantic_batch_with_policy_and_main_v1(
        resolver,
        source,
        brand_catalog,
        direct_call_policy,
        None,
    )
}

/// App Main-specific source-bound batch entry.  The target index is issued by
/// the same resolver session as the owner forests; all other public batch
/// entries remain observer-only.
pub(crate) fn issue_resolved_callable_semantic_batch_with_main_freestatic_targets_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    app_main_identity: &crate::parser::CallableDeclarationIdentityV1,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    issue_resolved_callable_semantic_batch_with_policy_and_main_v1(
        resolver,
        source,
        brand_catalog,
        DirectCallObservationBatchPolicyV1::ObserveForCatalogedValidation,
        Some(app_main_identity),
    )
}

fn issue_resolved_callable_semantic_batch_with_policy_and_main_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    direct_call_policy: DirectCallObservationBatchPolicyV1,
    app_main_identity: Option<&crate::parser::CallableDeclarationIdentityV1>,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
    let (rows, callable_index, declared_instance_call_source, declared_instance_call_effect_source) = source
        .with_callable_semantic_syntax(|loan| {
            let mut candidates = Vec::with_capacity(loan.rows().len());
            let mut resolver_inputs = Vec::with_capacity(loan.rows().len());
            for (expected, syntax) in loan.rows().iter().enumerate() {
                let batch_slot = u32::try_from(expected)
                    .map_err(|_| ResolvedCallableSemanticBatchIssueV1::SourceCoverage)?;
                if syntax.batch_slot() != batch_slot {
                    return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                }
                let ASTNode::FunctionDeclaration {
                    name, params, body, ..
                } = syntax.declaration()
                else {
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
                let header = CallableHeaderSyntaxViewV1::from_function_ast(syntax.declaration())
                    .ok_or(ResolvedCallableSemanticBatchIssueV1::SourceCoverage)?;
                candidates.push((
                    batch_slot,
                    syntax.identity().clone(),
                    mode,
                    parameter_count,
                    syntax.declaration(),
                    syntax.method_source_observation().cloned(),
                    view,
                ));
                resolver_inputs.push(SelectedCallableResolverInputV1::callable_with_header(
                    syntax.identity().clone(),
                    syntax.owner_name(),
                    name,
                    view,
                    Some(header),
                ));
            }

            let (forests, mut body_shapes, callable_index) = if let Some(app_main_identity) =
                app_main_identity
            {
                match resolver
                    .resolve_source_bound_selected_callable_forests_with_main_freestatic_targets(
                        &resolver_inputs,
                        brand_catalog,
                        app_main_identity,
                    )
                    .map_err(ResolvedCallableSemanticBatchIssueV1::Resolver)?
                {
                    ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1::Complete {
                        forests,
                        body_shapes,
                        callable_index,
                    } => (forests, body_shapes, callable_index),
                    ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1::Deferred(
                        deferred,
                    ) => {
                        return Err(ResolvedCallableSemanticBatchIssueV1::ResolverDeferred(
                            deferred,
                        ))
                    }
                }
            } else {
                match resolver
                    .resolve_source_bound_selected_callable_forests_with_body_shapes_and_brand_catalog(
                        &resolver_inputs,
                        brand_catalog,
                    )
                    .map_err(ResolvedCallableSemanticBatchIssueV1::Resolver)?
                {
                    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
                        forests,
                        body_shapes,
                    } => (forests, body_shapes, None),
                    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred(
                        deferred,
                    ) => {
                        return Err(ResolvedCallableSemanticBatchIssueV1::ResolverDeferred(
                            deferred,
                        ))
                    }
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
                if matches!(
                    direct_call_policy,
                    DirectCallObservationBatchPolicyV1::RejectUnissued
                ) && forest_has_unissued_direct_call_observation_v1(&forest)
                {
                    return Err(
                        ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
                    );
                }
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
                    issue_resolved_block_expr_expectation_v1(function, &body_shape).map_err(
                        |error| ResolvedCallableSemanticBatchIssueV1::BlockExprExpectation {
                            _error: error,
                        },
                    )?;
                let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                    declaration,
                    &forest,
                    view.root_profile(),
                )
                .map_err(|error| {
                    ResolvedCallableSemanticBatchIssueV1::Projection { _error: error }
                })?;
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

            // Build neutral method declarations from the same borrowed
            // syntax rows and their paired resolved owners. This is still the
            // one final-source HRTB; no parser or resolver is re-entered.
            let mut method_sources = Vec::new();
            let mut call_sources = Vec::new();
            for (syntax, row) in loan.rows().iter().zip(resolved.iter()) {
                if let Some(observation) = syntax.method_source_observation() {
                    let ASTNode::FunctionDeclaration { name, params, .. } = syntax.declaration()
                    else {
                        return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                    };
                    let box_name = syntax
                        .owner_name()
                        .ok_or(ResolvedCallableSemanticBatchIssueV1::SourceCoverage)?;
                    let method_mode = match syntax.mode() {
                        FinalCallableDeclarationModeV1::StaticBoxMethod => {
                            DeclaredInstanceMethodModeV1::Static
                        }
                        FinalCallableDeclarationModeV1::InstanceBoxMethod => {
                            DeclaredInstanceMethodModeV1::Instance
                        }
                        FinalCallableDeclarationModeV1::TopLevel => {
                            return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage)
                        }
                    };
                    let parameter_count = u32::try_from(params.len())
                        .map_err(|_| ResolvedCallableSemanticBatchIssueV1::ParameterCountOverflow)?;
                    method_sources.push(DeclaredInstanceMethodSourceRefV1::new(
                        syntax.identity(),
                        observation.parser_provenance(),
                        observation.source_site(),
                        box_name,
                        name,
                        method_mode,
                        parameter_count,
                        row.owner,
                    ));
                }
                if row.mode != ResolvedCallableDeclarationModeV1::InstanceBoxMethod {
                    continue;
                }
                let Some(caller_observation) = syntax.method_source_observation() else {
                    continue;
                };
                let function = row
                    .forest
                    .owner(row.owner)
                    .ok_or(ResolvedCallableSemanticBatchIssueV1::MissingRoot)?;
                let root_receiver_binding =
                    function.declaration_binding(&SourceBindingSiteV1::Receiver);
                let root_receiver_declaration_count = function
                    .declaration_sites()
                    .filter(|site| matches!(site, SourceBindingSiteV1::Receiver))
                    .count();
                for (call_site, call) in function.method_calls() {
                    let ResolvedMethodCallReceiverSourceV1::Lexical(
                        ResolvedLexicalRefV1::Local(receiver_binding),
                    ) = call.receiver()
                    else {
                        continue;
                    };
                    if call.owner() != row.owner {
                        return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
                    }
                    call_sources.push(DeclaredInstanceCallSourceRefV1::new(
                        syntax.identity(),
                        caller_observation.parser_provenance(),
                        caller_observation.source_site(),
                        row.owner,
                        call_site.clone(),
                        call.receiver_site().clone(),
                        receiver_binding,
                        root_receiver_binding,
                        root_receiver_declaration_count,
                        call.selector(),
                        call.arity(),
                    ));
                }
            }
            let declared_instance_call_source = DeclaredInstanceCallRelationIssuerV1::issue(
                &method_sources,
                &call_sources,
            )
            .map_err(|error| {
                ResolvedCallableSemanticBatchIssueV1::DeclaredInstanceCallRelation {
                    _error: error,
                }
            })?;
            let declared_instance_call_effect_source = DeclaredInstanceCallEffectIssuerV1::issue(
                &declared_instance_call_source,
                loan.rows(),
            )
            .map_err(|error| {
                ResolvedCallableSemanticBatchIssueV1::DeclaredInstanceCallEffect {
                    _error: error,
                }
            })?;
            Ok((
                resolved.into_boxed_slice(),
                callable_index,
                declared_instance_call_source,
                declared_instance_call_effect_source,
            ))
        })
        .map_err(|error| ResolvedCallableSemanticBatchIssueV1::ParserSyntax { _error: error })??;

    let main_callable_index = match (app_main_identity, callable_index) {
        (Some(identity), Some(index)) => {
            let mut matching = rows.iter().filter(|row| row.identity.same_as(identity));
            let Some(main) = matching.next() else {
                return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
            };
            if matching.next().is_some()
                || main.mode != ResolvedCallableDeclarationModeV1::StaticBoxMethod
            {
                return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
            }
            Some((main.batch_slot, index))
        }
        (None, None) => None,
        // A specialized App Main batch with no exact-i64 free-static headers
        // simply has no index to lend; direct observations still fail closed
        // at the package gate. The generic batch must never gain one.
        (Some(_), None) => None,
        (None, Some(_)) => {
            return Err(ResolvedCallableSemanticBatchIssueV1::SourceCoverage);
        }
    };

    Ok(VerifiedResolvedCallableSemanticBatchV1 {
        source,
        rows,
        main_callable_index,
        declared_instance_call_source,
        declared_instance_call_effect_source,
    })
}
