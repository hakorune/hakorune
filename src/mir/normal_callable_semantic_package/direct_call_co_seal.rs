//! Private source/target co-seal validation for the normal semantic package.
//!
//! This module owns only the existing validation helper. It issues no semantic
//! product and does not create a second direct-call authority.

use std::collections::BTreeSet;

use crate::mir::builder::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableSemanticBatchIssueV1, VerifiedResolvedCallableSemanticBatchV1,
};

use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;

/// Validate the expected Cataloged owner/site/provenance relation against the
/// resolver-issued source-unit index. Actual raw lineage remains a later
/// Builder boundary; an observation without an exact target/header still
/// fails closed at this package gate.
pub(super) fn validate_cataloged_source_co_seal_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
) -> Result<(), ResolvedCallableSemanticBatchIssueV1> {
    let declaration_catalog = catalog.catalog();
    if !declaration_catalog
        .brand()
        .is_same(declaration_catalog.selected_source_inventory().brand())
    {
        return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
    }

    let mut owned_sites = BTreeSet::new();
    let app_main_identity = declaration_catalog
        .source_backed_app_main()
        .map(|main| main.parser_identity());
    for declaration in batch.declarations() {
        if app_main_identity.is_some_and(|identity| declaration.identity().same_as(identity)) {
            // App Main has no selected catalog row.  Its exact owner/forest
            // relation is validated by the dedicated pre-install gate below.
            continue;
        }
        let slot = declaration.batch_slot();
        let Some(key) = selected.key_for_batch_slot(slot) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some(identity) = selected.identity_for_batch_slot(slot) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some((catalog_key, catalog_identity, _role)) = catalog
            .selected_identities()
            .find(|(candidate, _, _)| **candidate == *key)
        else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        if !identity.same_as(catalog_identity)
            || !declaration.same_declaration_identity(catalog_identity)
        {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        }

        let Some(_source_site) = declaration_catalog
            .selected_source_inventory()
            .site(catalog_key)
        else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some(catalog_key) = (match key {
            SelectedNormalCallableKeyV1::Cataloged(key)
                if key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod =>
            {
                Some(key)
            }
            _ => None,
        }) else {
            // A non-Cataloged row has no expected source-backed provenance.
            // It is accepted only when its forest contains no observation.
            let has_observation = batch
                .with_lowering_input(slot, |input| {
                    input
                        .forest()
                        .owners()
                        .any(|(_, function)| function.direct_call_observations().next().is_some())
                })
                .map_err(|_| ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?;
            if has_observation {
                return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
            }
            continue;
        };
        let Some(declaration_row) = declaration_catalog.declaration(catalog_key) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let expected_parameter_count =
            u32::try_from(declaration_row.params().len()).unwrap_or(u32::MAX);
        if catalog_key.arity() != expected_parameter_count
            || declaration.parameter_count() != expected_parameter_count
        {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        }

        let callable_index = batch.callable_index();
        batch
            .with_lowering_input(slot, |input| {
                let forest = input.forest();
                if forest.semantic_owners().any(|(owner, product)| {
                    product.as_function().is_none() || product.owner() != owner
                }) {
                    return None;
                }
                let compilation = input.owner().compilation_brand();
                for (owner, function) in forest.owners() {
                    if owner.compilation_brand() != compilation
                        || function.owner() != owner
                        || function.source_site_inventory().owner() != owner
                        || function.source_site_inventory().function_origin()
                            != function.function_origin()
                    {
                        return None;
                    }
                    for (site, _observation) in function.direct_call_observations() {
                        let Some(target) = function.direct_call_target(site) else {
                            return None;
                        };
                        let Some(callable_index) = callable_index else {
                            return None;
                        };
                        let Ok(header) = callable_index.header_for_callable(target.callable())
                        else {
                            return None;
                        };
                        // An unannotated target is callable only through
                        // the map-result lane; the callee's sealed
                        // terminal must prove a Map result, otherwise
                        // the observation has no admissible route.
                        if header.signature().result().is_none()
                            && !super::super::direct_call_loan::lifecycle::map_result_callee(
                                batch,
                                target.callable(),
                            )
                        {
                            return None;
                        }
                        if !function.source_site_inventory().contains_expression(site)
                            || !owned_sites.insert(
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    owner,
                                    site.clone(),
                                ),
                            )
                        {
                            return None;
                        }
                    }
                }
                Some(())
            })
            .map_err(|_| ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?
            .ok_or(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?;
    }
    Ok(())
}
