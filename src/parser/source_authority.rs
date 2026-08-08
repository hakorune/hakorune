//! Parser-private source authority substrate for Box declarations.
//!
//! R6-S1 deliberately stops before connecting this transaction to the public
//! AST parse output, build-gate pruning, or delegate lowering. The transaction
//! owns the only fresh invocation brand and the unpublished ordered inventory;
//! a later R6-S3 finalizer will issue the non-Clone seal after all postpasses.

#![allow(dead_code)]

use std::sync::Arc;

use crate::ast::{
    ASTNode, BoxMethodInventoryErrorV1, BoxMethodInventoryOrdinalV1, BoxMethodInventoryV1, Span,
};

#[derive(Debug, Clone)]
pub(super) struct ParserInvocationBrandV1(Arc<()>);

impl ParserInvocationBrandV1 {
    pub(super) fn issue() -> Self {
        Self(Arc::new(()))
    }

    fn same_as(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialEq for ParserInvocationBrandV1 {
    fn eq(&self, other: &Self) -> bool {
        self.same_as(other)
    }
}

impl Eq for ParserInvocationBrandV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBoxDeclarationSiteV1 {
    brand: ParserInvocationBrandV1,
    statement_ordinal: u32,
}

impl SourceBoxDeclarationSiteV1 {
    pub(super) fn statement_ordinal(&self) -> u32 {
        self.statement_ordinal
    }

    fn brand_matches(&self, brand: &ParserInvocationBrandV1) -> bool {
        self.brand.same_as(brand)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBoxMemberSiteV1 {
    box_site: SourceBoxDeclarationSiteV1,
    member_ordinal: u32,
}

impl SourceBoxMemberSiteV1 {
    pub(super) fn member_ordinal(&self) -> u32 {
        self.member_ordinal
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SourceBoxGateSelectionV1 {
    gate_member_ordinal: u32,
    branch_member_ordinal: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceBoxMethodSiteV1 {
    Direct {
        member: SourceBoxMemberSiteV1,
    },
    SelectedBuildGate {
        member: SourceBoxMemberSiteV1,
        path: Box<[SourceBoxGateSelectionV1]>,
    },
}

impl SourceBoxMethodSiteV1 {
    fn member(&self) -> &SourceBoxMemberSiteV1 {
        match self {
            Self::Direct { member } | Self::SelectedBuildGate { member, .. } => member,
        }
    }

    fn validate_for(
        &self,
        brand: &ParserInvocationBrandV1,
        box_site: &SourceBoxDeclarationSiteV1,
    ) -> bool {
        self.member().box_site() == box_site && box_site.brand_matches(brand)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExplicitMethodSourceRelationV1 {
    source_site: SourceBoxMethodSiteV1,
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    name: Box<str>,
}

impl ExplicitMethodSourceRelationV1 {
    pub(super) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(super) fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        self.inventory_ordinal
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum SourceAuthorityErrorV1 {
    ForeignBoxSite,
    MemberOrdinalOverflow,
    Inventory(BoxMethodInventoryErrorV1),
}

#[derive(Debug)]
pub(super) struct OpenBoxMethodSourceTransactionV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    inventory: BoxMethodInventoryV1,
    explicit_relations: Vec<ExplicitMethodSourceRelationV1>,
    next_member_ordinal: u32,
}

impl OpenBoxMethodSourceTransactionV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1, statement_ordinal: u32) -> Self {
        let box_site = SourceBoxDeclarationSiteV1 {
            brand: brand.clone(),
            statement_ordinal,
        };
        Self {
            brand,
            box_site,
            inventory: BoxMethodInventoryV1::empty(),
            explicit_relations: Vec::new(),
            next_member_ordinal: 0,
        }
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(super) fn current_member_site(&self) -> SourceBoxMemberSiteV1 {
        SourceBoxMemberSiteV1 {
            box_site: self.box_site.clone(),
            member_ordinal: self.next_member_ordinal,
        }
    }

    pub(super) fn finish_member(&mut self) -> Result<(), SourceAuthorityErrorV1> {
        self.next_member_ordinal = self
            .next_member_ordinal
            .checked_add(1)
            .ok_or(SourceAuthorityErrorV1::MemberOrdinalOverflow)?;
        Ok(())
    }

    pub(super) fn commit_explicit_method(
        &mut self,
        source_site: SourceBoxMethodSiteV1,
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, SourceAuthorityErrorV1> {
        if !source_site.validate_for(&self.brand, &self.box_site) {
            return Err(SourceAuthorityErrorV1::ForeignBoxSite);
        }
        let name = name.into();
        let ordinal = self
            .inventory
            .try_push_explicit_source(name.clone(), declaration, diagnostic_span)
            .map_err(SourceAuthorityErrorV1::Inventory)?;
        self.explicit_relations
            .push(ExplicitMethodSourceRelationV1 {
                source_site,
                inventory_ordinal: ordinal,
                name,
            });
        Ok(ordinal)
    }

    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn finish(self) -> PreparedBoxSourceSealV1 {
        PreparedBoxSourceSealV1 {
            brand: self.brand,
            box_site: self.box_site,
            inventory: self.inventory,
            explicit_relations: self.explicit_relations.into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
pub(super) struct PreparedBoxSourceSealV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    inventory: BoxMethodInventoryV1,
    explicit_relations: Box<[ExplicitMethodSourceRelationV1]>,
}

impl PreparedBoxSourceSealV1 {
    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn explicit_relations(&self) -> &[ExplicitMethodSourceRelationV1] {
        &self.explicit_relations
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }
}

/// Final authority. It intentionally has no public constructor and no Clone.
/// R6-S3 will issue it only from the final rich parse product after prune and
/// delegate postpass have completed.
#[derive(Debug)]
pub(super) struct ParserBoxSourceSealV1 {
    prepared: PreparedBoxSourceSealV1,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, Span};

    fn function(name: &str) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            contracts: Vec::new(),
            uses: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn transaction_co_seals_explicit_source_with_inventory_placement() {
        let brand = ParserInvocationBrandV1::issue();
        let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 4);
        let site = SourceBoxMethodSiteV1::Direct {
            member: transaction.current_member_site(),
        };
        let ordinal = transaction
            .commit_explicit_method(site, "length", function("length"), Span::unknown())
            .unwrap();
        transaction.finish_member().unwrap();

        let prepared = transaction.finish();
        assert_eq!(prepared.box_site().statement_ordinal(), 4);
        assert_eq!(prepared.inventory().len(), 1);
        assert_eq!(prepared.explicit_relations().len(), 1);
        assert_eq!(
            prepared.explicit_relations()[0].inventory_ordinal(),
            ordinal
        );
        assert_eq!(prepared.explicit_relations()[0].name(), "length");
    }

    #[test]
    fn foreign_invocation_site_is_rejected_before_inventory_mutation() {
        let brand = ParserInvocationBrandV1::issue();
        let foreign = ParserInvocationBrandV1::issue();
        let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 1);
        let foreign_site = SourceBoxMethodSiteV1::Direct {
            member: SourceBoxMemberSiteV1 {
                box_site: SourceBoxDeclarationSiteV1 {
                    brand: foreign,
                    statement_ordinal: 1,
                },
                member_ordinal: 0,
            },
        };

        assert_eq!(
            transaction
                .commit_explicit_method(
                    foreign_site,
                    "length",
                    function("length"),
                    Span::unknown(),
                )
                .unwrap_err(),
            SourceAuthorityErrorV1::ForeignBoxSite
        );
        assert!(transaction.inventory().is_empty());
    }

    #[test]
    fn brand_is_identity_not_value_equality() {
        let left = ParserInvocationBrandV1::issue();
        let right = ParserInvocationBrandV1::issue();
        assert_ne!(left, right);
        assert_eq!(left, left.clone());
    }

    #[test]
    fn duplicate_source_name_is_rejected_without_partial_relation() {
        let brand = ParserInvocationBrandV1::issue();
        let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 2);
        let site = SourceBoxMethodSiteV1::Direct {
            member: transaction.current_member_site(),
        };
        transaction
            .commit_explicit_method(site.clone(), "length", function("length"), Span::unknown())
            .unwrap();

        let error = transaction
            .commit_explicit_method(site, "length", function("length"), Span::unknown())
            .unwrap_err();
        assert!(matches!(
            error,
            SourceAuthorityErrorV1::Inventory(BoxMethodInventoryErrorV1::DuplicateMethod { .. })
        ));
        assert_eq!(transaction.inventory().len(), 1);
    }
}
