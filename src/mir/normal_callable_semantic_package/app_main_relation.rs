//! App Main package-side owner/forest relation validation.
//!
//! This is provenance-only.  It does not issue a raw lineage, target, loan,
//! or MIR instruction; it only keeps the install boundary from collapsing
//! distinct Main relation failures into the generic direct-call rejection.

use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    forest_has_unissued_direct_call_observation_v1, ReceiverPolicyV1,
};

#[derive(Debug)]
pub(crate) enum AppMainRootRelationIssueV1 {
    CatalogBrandMismatch,
    CatalogNamespaceMismatch,
    CatalogDeclarationMissing,
    CatalogArityMismatch,
    BatchMainMissing,
    BatchMainDuplicate,
    BatchMainShapeMismatch,
    IdentityMismatch,
    OwnerMismatch,
    InputOwnerMismatch,
    UnissuedDirectCallObservation,
    RootCountMismatch,
    RootMissing,
    RootOwnerMismatch,
    FunctionOriginMismatch,
    SourceInventoryMismatch,
    RootProfileMismatch,
    ForestCompilationBrandMismatch,
    ForestOwnerMismatch,
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    ParameterCountOverflow,
}

/// Validate the retained App/Main identity against the exact semantic batch
/// and its complete owner forest before package-side parameter/install work.
/// This is deliberately provenance-only: it does not construct a raw lineage,
/// target, loan, or MIR instruction.
pub(super) fn validate_app_main_root_owner_relation_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
) -> Result<(), AppMainRootRelationIssueV1> {
    let declaration_catalog = catalog.catalog();
    let Some(app_main) = declaration_catalog.source_backed_app_main() else {
        return Ok(());
    };
    if !declaration_catalog
        .brand()
        .is_same(app_main.catalog_brand())
    {
        return Err(AppMainRootRelationIssueV1::CatalogBrandMismatch);
    }
    if app_main.catalog_key().namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
        return Err(AppMainRootRelationIssueV1::CatalogNamespaceMismatch);
    }
    let Some(catalog_declaration) = declaration_catalog.declaration(app_main.catalog_key()) else {
        return Err(AppMainRootRelationIssueV1::CatalogDeclarationMissing);
    };
    let expected_arity = u32::try_from(catalog_declaration.params().len())
        .map_err(|_| AppMainRootRelationIssueV1::ParameterCountOverflow)?;
    if app_main.catalog_key().arity() != expected_arity {
        return Err(AppMainRootRelationIssueV1::CatalogArityMismatch);
    }

    let mut declarations = batch
        .declarations()
        .filter(|declaration| declaration.identity().same_as(app_main.parser_identity()));
    let Some(declaration) = declarations.next() else {
        return Err(AppMainRootRelationIssueV1::BatchMainMissing);
    };
    if declarations.next().is_some() {
        return Err(AppMainRootRelationIssueV1::BatchMainDuplicate);
    }
    if declaration.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod
        || declaration.parameter_count() != expected_arity
    {
        return Err(AppMainRootRelationIssueV1::BatchMainShapeMismatch);
    }

    let owner = declaration.owner();
    batch
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            if !identity.identity().same_as(app_main.parser_identity()) {
                return Err(AppMainRootRelationIssueV1::IdentityMismatch);
            }
            if identity.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod {
                return Err(AppMainRootRelationIssueV1::BatchMainShapeMismatch);
            }
            if identity.owner() != owner {
                return Err(AppMainRootRelationIssueV1::OwnerMismatch);
            }
            if input.owner() != owner {
                return Err(AppMainRootRelationIssueV1::InputOwnerMismatch);
            }

            let forest = input.forest();
            if forest_has_unissued_direct_call_observation_v1(forest) {
                return Err(AppMainRootRelationIssueV1::UnissuedDirectCallObservation);
            }
            if forest.roots() != std::slice::from_ref(&owner) {
                return Err(AppMainRootRelationIssueV1::RootCountMismatch);
            }
            let Some(root) = forest.owner(owner) else {
                return Err(AppMainRootRelationIssueV1::RootMissing);
            };
            if root.owner() != owner {
                return Err(AppMainRootRelationIssueV1::RootOwnerMismatch);
            }
            if root.function_origin() != declaration.function_origin() {
                return Err(AppMainRootRelationIssueV1::FunctionOriginMismatch);
            }
            if root.source_site_inventory().owner() != owner
                || root.source_site_inventory().function_origin() != root.function_origin()
            {
                return Err(AppMainRootRelationIssueV1::SourceInventoryMismatch);
            }
            if root.root_profile().receiver_policy() != ReceiverPolicyV1::StaticCurrentOwner {
                return Err(AppMainRootRelationIssueV1::RootProfileMismatch);
            }

            let compilation = owner.compilation_brand();
            if forest
                .owners()
                .any(|(candidate, _function)| candidate.compilation_brand() != compilation)
            {
                return Err(AppMainRootRelationIssueV1::ForestCompilationBrandMismatch);
            }
            if forest.owners().any(|(candidate, function)| {
                function.owner() != candidate
                    || function.source_site_inventory().owner() != candidate
                    || function.source_site_inventory().function_origin()
                        != function.function_origin()
            }) {
                return Err(AppMainRootRelationIssueV1::ForestOwnerMismatch);
            }
            Ok(())
        })
        .map_err(AppMainRootRelationIssueV1::BatchLoan)??;
    Ok(())
}
