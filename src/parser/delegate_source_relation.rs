//! Parser-private generated-delegate source relations.
//!
//! These rows are transport evidence for the R6-S3B-C postpass. They are not
//! resolver products and deliberately remain outside `ParserBoxSourceSealV1`
//! until the later R6-S3B-D complete-coverage issuer.

use crate::ast::{BoxMethodGeneratedProvenanceV1, BoxMethodInventoryPlacementReceiptV1};

use super::source_authority::SourceBoxMethodSiteV1;
use super::source_path::SourceBoxDeclarationPathV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExistingTargetMethodSourceRefV1 {
    target_box_path: SourceBoxDeclarationPathV1,
    source_site: SourceBoxMethodSiteV1,
    inventory_ordinal: crate::ast::BoxMethodInventoryOrdinalV1,
    name: Box<str>,
}

impl ExistingTargetMethodSourceRefV1 {
    pub(super) fn new(
        target_box_path: SourceBoxDeclarationPathV1,
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: crate::ast::BoxMethodInventoryOrdinalV1,
        name: impl Into<Box<str>>,
    ) -> Self {
        Self {
            target_box_path,
            source_site,
            inventory_ordinal,
            name: name.into(),
        }
    }

    pub(super) fn target_box_path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.target_box_path
    }

    pub(super) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(super) fn inventory_ordinal(self) -> crate::ast::BoxMethodInventoryOrdinalV1 {
        self.inventory_ordinal
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GeneratedDelegateSourceRelationV1 {
    host_box_path: SourceBoxDeclarationPathV1,
    host_delegate_member: SourceBoxMethodSiteV1,
    expose_ordinal: u32,
    delegate_field_name: Box<str>,
    source_method_name: Box<str>,
    exposed_method_name: Box<str>,
    target_box_path: SourceBoxDeclarationPathV1,
    target_method_source_ref: ExistingTargetMethodSourceRefV1,
    generated_inventory_placement: BoxMethodInventoryPlacementReceiptV1,
    generated_name_provenance: BoxMethodGeneratedProvenanceV1,
}

impl GeneratedDelegateSourceRelationV1 {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        host_box_path: SourceBoxDeclarationPathV1,
        host_delegate_member: SourceBoxMethodSiteV1,
        expose_ordinal: u32,
        delegate_field_name: impl Into<Box<str>>,
        source_method_name: impl Into<Box<str>>,
        exposed_method_name: impl Into<Box<str>>,
        target_box_path: SourceBoxDeclarationPathV1,
        target_method_source_ref: ExistingTargetMethodSourceRefV1,
        generated_inventory_placement: BoxMethodInventoryPlacementReceiptV1,
        generated_name_provenance: BoxMethodGeneratedProvenanceV1,
    ) -> Self {
        Self {
            host_box_path,
            host_delegate_member,
            expose_ordinal,
            delegate_field_name: delegate_field_name.into(),
            source_method_name: source_method_name.into(),
            exposed_method_name: exposed_method_name.into(),
            target_box_path,
            target_method_source_ref,
            generated_inventory_placement,
            generated_name_provenance,
        }
    }

    pub(super) fn host_box_path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.host_box_path
    }

    pub(super) fn host_delegate_member(&self) -> &SourceBoxMethodSiteV1 {
        &self.host_delegate_member
    }

    pub(super) fn expose_ordinal(&self) -> u32 {
        self.expose_ordinal
    }

    pub(super) fn delegate_field_name(&self) -> &str {
        &self.delegate_field_name
    }

    pub(super) fn source_method_name(&self) -> &str {
        &self.source_method_name
    }

    pub(super) fn exposed_method_name(&self) -> &str {
        &self.exposed_method_name
    }

    pub(super) fn target_box_path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.target_box_path
    }

    pub(super) fn target_method_source_ref(&self) -> &ExistingTargetMethodSourceRefV1 {
        &self.target_method_source_ref
    }

    pub(super) fn generated_inventory_placement(&self) -> &BoxMethodInventoryPlacementReceiptV1 {
        &self.generated_inventory_placement
    }

    pub(super) fn generated_name_provenance(&self) -> &BoxMethodGeneratedProvenanceV1 {
        &self.generated_name_provenance
    }
}
