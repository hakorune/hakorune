//! Final generated-delegate coverage for the parser source seal.
//!
//! This module is deliberately parser-private.  It consumes the C-I0 relation
//! rows and the postpass inventory placement receipts; it does not inspect AST
//! names, reconstruct source identity from ordinals, or issue later semantic
//! target/Recipe products.

use crate::ast::{BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1, BoxMethodProvenanceV1};

use super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::source_seal::PreparedBoxSourceSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GeneratedDelegateCoverageErrorV1 {
    ForeignHostPath,
    ForeignTargetPath,
    HostMemberPathMismatch,
    TargetMethodSourcePathMismatch,
    TargetMethodNameMismatch,
    DuplicateRelationKey,
    RelationPlacementMissing,
    RelationPlacementNotGenerated,
    RelationNameMismatch,
    RelationProvenanceMismatch,
    RelationSelectionMismatch,
    MissingRelationForGeneratedPlacement,
}

pub(super) fn validate_generated_delegate_coverage(
    prepared: &PreparedBoxSourceSealV1,
    final_inventory: &BoxMethodInventoryV1,
) -> Result<(), GeneratedDelegateCoverageErrorV1> {
    let final_entries = final_inventory.clone().into_selected_declaration_order();
    let relations = prepared.generated_delegate_source_relations();
    let mut relation_keys = Vec::with_capacity(relations.len());

    for relation in relations {
        validate_relation_identity(prepared, relation)?;
        let key = (
            relation.host_box_path().clone(),
            relation.host_delegate_member().clone(),
            relation.expose_ordinal(),
        );
        if relation_keys.iter().any(|previous| previous == &key) {
            return Err(GeneratedDelegateCoverageErrorV1::DuplicateRelationKey);
        }
        relation_keys.push(key);

        let Some(entry) = final_entries.iter().find(|entry| {
            entry.site() == relation.generated_inventory_placement().inventory_ordinal()
        }) else {
            return Err(GeneratedDelegateCoverageErrorV1::RelationPlacementMissing);
        };
        validate_entry_relation(entry, relation)?;
    }

    for entry in &final_entries {
        let BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate { .. }) =
            entry.provenance()
        else {
            continue;
        };
        let matching = relations
            .iter()
            .filter(|relation| {
                relation.generated_inventory_placement().inventory_ordinal() == entry.site()
            })
            .count();
        if matching == 0 {
            return Err(GeneratedDelegateCoverageErrorV1::MissingRelationForGeneratedPlacement);
        }
        if matching > 1 {
            return Err(GeneratedDelegateCoverageErrorV1::DuplicateRelationKey);
        }
    }

    if relations.iter().any(|relation| {
        !matches!(
            final_entries
                .iter()
                .find(|entry| {
                    entry.site() == relation.generated_inventory_placement().inventory_ordinal()
                })
                .map(|entry| entry.provenance()),
            Some(BoxMethodProvenanceV1::Generated(
                BoxMethodGeneratedProvenanceV1::Delegate { .. }
            ))
        )
    }) {
        return Err(GeneratedDelegateCoverageErrorV1::RelationPlacementNotGenerated);
    }

    Ok(())
}

fn validate_relation_identity(
    prepared: &PreparedBoxSourceSealV1,
    relation: &GeneratedDelegateSourceRelationV1,
) -> Result<(), GeneratedDelegateCoverageErrorV1> {
    let host_path = prepared.box_site().path();
    if relation.host_box_path() != host_path || relation.host_box_path().brand() != &prepared.brand
    {
        return Err(GeneratedDelegateCoverageErrorV1::ForeignHostPath);
    }
    if relation.host_delegate_member().box_site().path() != host_path {
        return Err(GeneratedDelegateCoverageErrorV1::HostMemberPathMismatch);
    }
    if relation.target_box_path().brand() != &prepared.brand
        || relation
            .target_method_source_ref()
            .target_box_path()
            .brand()
            != &prepared.brand
    {
        return Err(GeneratedDelegateCoverageErrorV1::ForeignTargetPath);
    }
    if relation.target_method_source_ref().target_box_path() != relation.target_box_path()
        || relation
            .target_method_source_ref()
            .source_site()
            .box_site()
            .path()
            != relation.target_box_path()
    {
        return Err(GeneratedDelegateCoverageErrorV1::TargetMethodSourcePathMismatch);
    }
    if relation.target_method_source_ref().name() != relation.source_method_name() {
        return Err(GeneratedDelegateCoverageErrorV1::TargetMethodNameMismatch);
    }
    Ok(())
}

fn validate_entry_relation(
    entry: &crate::ast::BoxMethodEntryV1,
    relation: &GeneratedDelegateSourceRelationV1,
) -> Result<(), GeneratedDelegateCoverageErrorV1> {
    let placement = relation.generated_inventory_placement();
    if entry.name() != placement.name() {
        return Err(GeneratedDelegateCoverageErrorV1::RelationNameMismatch);
    }
    let BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate {
        field_name,
        exposed_name,
        selection,
    }) = entry.provenance()
    else {
        return Err(GeneratedDelegateCoverageErrorV1::RelationPlacementNotGenerated);
    };
    if entry.provenance()
        != &BoxMethodProvenanceV1::Generated(relation.generated_name_provenance().clone())
    {
        return Err(GeneratedDelegateCoverageErrorV1::RelationProvenanceMismatch);
    }
    if field_name.as_ref() != relation.delegate_field_name()
        || exposed_name.as_ref() != relation.exposed_method_name()
    {
        return Err(GeneratedDelegateCoverageErrorV1::RelationProvenanceMismatch);
    }
    if relation
        .host_delegate_member()
        .matches_ast_selection(selection)
    {
        Ok(())
    } else {
        Err(GeneratedDelegateCoverageErrorV1::RelationSelectionMismatch)
    }
}
