//! Parser-private source authority substrate for Box declarations.
//!
//! R6-S1 introduced this parser-private transaction. R6-S3A now connects its
//! prepared payload to the bounded rich parse product; the final non-Clone
//! seal is still issued only by `source_seal` after all supported postpasses.

#![allow(dead_code)]

use std::sync::Arc;

use crate::ast::{
    ASTNode, BoxMethodInventoryErrorV1, BoxMethodInventoryOrdinalV1,
    BoxMethodInventoryPlacementReceiptV1, BoxMethodInventoryV1, PreparedGeneratedBoxMethodBatchV1,
    Span,
};
use crate::parser::ParseError;

pub(super) use super::source_seal::PreparedBoxSourceSealV1;

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
        expected_member_ordinal: u32,
    ) -> bool {
        self.member().box_site() == box_site
            && box_site.brand_matches(brand)
            && self.member().member_ordinal() == expected_member_ordinal
    }

    fn prepend_selected_gate(&mut self, gate_member_ordinal: u32, branch_member_ordinal: u32) {
        let selection = SourceBoxGateSelectionV1 {
            gate_member_ordinal,
            branch_member_ordinal,
        };
        match self {
            Self::Direct { member } => {
                *self = Self::SelectedBuildGate {
                    member: member.clone(),
                    path: vec![selection].into_boxed_slice(),
                };
            }
            Self::SelectedBuildGate { path, .. } => {
                let mut rebased = Vec::with_capacity(path.len() + 1);
                rebased.push(selection);
                rebased.extend(path.iter().copied());
                *path = rebased.into_boxed_slice();
            }
        }
    }

    fn source_member_ordinal(&self) -> u32 {
        self.member().member_ordinal()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MethodSourceRelationV1 {
    Explicit(ExplicitMethodSourceRelationV1),
    GeneratedProperty {
        source_member: SourceBoxMemberSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        name: Box<str>,
    },
}

impl MethodSourceRelationV1 {
    fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        match self {
            Self::Explicit(relation) => relation.inventory_ordinal(),
            Self::GeneratedProperty {
                inventory_ordinal, ..
            } => *inventory_ordinal,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Explicit(relation) => relation.name(),
            Self::GeneratedProperty { name, .. } => name,
        }
    }

    fn source_member_ordinal(&self) -> u32 {
        match self {
            Self::Explicit(relation) => relation.source_site().source_member_ordinal(),
            Self::GeneratedProperty { source_member, .. } => source_member.member_ordinal(),
        }
    }

    fn prepend_selected_gate(&mut self, gate_member_ordinal: u32, branch_member_ordinal: u32) {
        match self {
            Self::Explicit(relation) => relation
                .source_site
                .prepend_selected_gate(gate_member_ordinal, branch_member_ordinal),
            Self::GeneratedProperty { .. } => {
                // Generated property source identity remains its originating
                // member. Its selected path is carried by the AST provenance;
                // the relation only needs the exact member site here.
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum SourceAuthorityErrorV1 {
    ForeignBoxSite,
    StaleMemberSite,
    MemberOrdinalOverflow,
    MissingMethodSourceRelation { inventory_ordinal: u32 },
    MethodSourceRelationMismatch { name: Box<str> },
    Inventory(BoxMethodInventoryErrorV1),
}

pub(super) trait ExplicitMethodSink {
    fn commit_explicit_method_at_current(
        &mut self,
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, ParseError>;
}

pub(super) trait GeneratedPropertySink {
    fn commit_generated_property_batch_at_current(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, ParseError>;
}

// Compatibility sink for parser lanes that still build a standalone method
// inventory (interfaces/static boxes). Ordinary `box` declarations use the
// source transaction above; these legacy lanes do not yet publish a source
// seal and are intentionally kept out of the R6 source-authority boundary.
impl ExplicitMethodSink for BoxMethodInventoryV1 {
    fn commit_explicit_method_at_current(
        &mut self,
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, ParseError> {
        self.try_push_explicit_source(name, declaration, diagnostic_span)
            .map_err(
                crate::parser::declarations::box_def::members::pending_method::map_inventory_error,
            )
    }
}

impl GeneratedPropertySink for BoxMethodInventoryV1 {
    fn commit_generated_property_batch_at_current(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, ParseError> {
        self.try_commit_generated_batch_with_placements(batch)
            .map_err(
                crate::parser::declarations::box_def::members::pending_method::map_inventory_error,
            )
    }
}

#[derive(Debug)]
pub(super) struct OpenBoxMethodSourceTransactionV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    inventory: BoxMethodInventoryV1,
    method_relations: Vec<MethodSourceRelationV1>,
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
            method_relations: Vec::new(),
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

    pub(super) fn current_gate_site(&self) -> crate::ast::BoxMemberGateSiteV1 {
        crate::ast::BoxMemberGateSiteV1::from_box_member_ordinal(self.next_member_ordinal)
    }

    pub(super) fn current_member_ordinal(&self) -> u32 {
        self.next_member_ordinal
    }

    pub(super) fn branch(&self) -> Self {
        Self {
            brand: self.brand.clone(),
            box_site: self.box_site.clone(),
            inventory: BoxMethodInventoryV1::empty(),
            method_relations: Vec::new(),
            next_member_ordinal: 0,
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
        if !source_site.validate_for(&self.brand, &self.box_site, self.next_member_ordinal) {
            if source_site.member().box_site() == &self.box_site
                && source_site.member().member_ordinal() != self.next_member_ordinal
            {
                return Err(SourceAuthorityErrorV1::StaleMemberSite);
            }
            return Err(SourceAuthorityErrorV1::ForeignBoxSite);
        }
        let name = name.into();
        let ordinal = self
            .inventory
            .try_push_explicit_source(name.clone(), declaration, diagnostic_span)
            .map_err(SourceAuthorityErrorV1::Inventory)?;
        self.method_relations.push(MethodSourceRelationV1::Explicit(
            ExplicitMethodSourceRelationV1 {
                source_site,
                inventory_ordinal: ordinal,
                name,
            },
        ));
        Ok(ordinal)
    }

    pub(super) fn commit_explicit_at_current(
        &mut self,
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, ParseError> {
        self.commit_explicit_method(
            SourceBoxMethodSiteV1::Direct {
                member: self.current_member_site(),
            },
            name,
            declaration,
            diagnostic_span,
        )
        .map_err(source_authority_to_parse_error)
    }

    pub(super) fn commit_generated_property_batch_at_current(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, ParseError> {
        let source_member = self.current_member_site();
        let names = batch
            .names_in_order()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let placements = self
            .inventory
            .try_commit_generated_batch_with_placements(batch)
            .map_err(inventory_error_to_parse_error)?;
        if names.len() != placements.len() {
            return Err(ParseError::BuildCfg {
                message: "generated property placement receipt count mismatch".to_owned(),
                line: 0,
            });
        }
        for (name, placement) in names.into_iter().zip(placements.iter()) {
            self.method_relations
                .push(MethodSourceRelationV1::GeneratedProperty {
                    source_member: source_member.clone(),
                    inventory_ordinal: placement.inventory_ordinal(),
                    name: name.into_boxed_str(),
                });
        }
        Ok(placements)
    }

    pub(super) fn try_merge_selected_gate(
        &mut self,
        selected: Self,
        gate_site: crate::ast::BoxMemberGateSiteV1,
    ) -> Result<(), ParseError> {
        let mut entries = selected.inventory.into_selected_declaration_order();
        let mut relations = selected.method_relations;
        if entries.len() != relations.len() {
            return Err(ParseError::BuildCfg {
                message: "selected Box source relation coverage is incomplete".to_owned(),
                line: 0,
            });
        }
        let gate_member_ordinal = gate_site.box_member_ordinal();
        let mut rebased_relations = Vec::with_capacity(relations.len());
        for (entry, relation) in entries.iter_mut().zip(relations.iter_mut()) {
            if entry.site() != relation.inventory_ordinal() || entry.name() != relation.name() {
                return Err(ParseError::BuildCfg {
                    message: "selected Box source relation does not match inventory".to_owned(),
                    line: 0,
                });
            }
            let branch_member_ordinal = relation.source_member_ordinal();
            entry
                .prepend_selected_gate(gate_site, branch_member_ordinal)
                .map_err(inventory_error_to_parse_error)?;
            relation.prepend_selected_gate(gate_member_ordinal, branch_member_ordinal);
            rebased_relations.push(relation.clone());
        }

        let placements = self
            .inventory
            .commit_prepared_append(
                crate::ast::PreparedBoxMethodInventoryAppendV1::try_new(entries)
                    .map_err(inventory_error_to_parse_error)?,
            )
            .map_err(inventory_error_to_parse_error)?;
        for (relation, placement) in rebased_relations.into_iter().zip(placements.iter()) {
            let relation = match relation {
                MethodSourceRelationV1::Explicit(mut relation) => {
                    relation.inventory_ordinal = placement.inventory_ordinal();
                    MethodSourceRelationV1::Explicit(relation)
                }
                MethodSourceRelationV1::GeneratedProperty {
                    source_member,
                    name,
                    ..
                } => MethodSourceRelationV1::GeneratedProperty {
                    source_member,
                    inventory_ordinal: placement.inventory_ordinal(),
                    name,
                },
            };
            self.method_relations.push(relation);
        }
        Ok(())
    }

    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.method_relations
    }

    pub(super) fn finish(self) -> PreparedBoxSourceSealV1 {
        PreparedBoxSourceSealV1 {
            brand: self.brand,
            box_site: self.box_site,
            inventory: self.inventory,
            method_relations: self.method_relations.into_boxed_slice(),
        }
    }
}

impl ExplicitMethodSink for OpenBoxMethodSourceTransactionV1 {
    fn commit_explicit_method_at_current(
        &mut self,
        name: String,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, ParseError> {
        self.commit_explicit_at_current(name, declaration, diagnostic_span)
    }
}

impl GeneratedPropertySink for OpenBoxMethodSourceTransactionV1 {
    fn commit_generated_property_batch_at_current(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, ParseError> {
        self.commit_generated_property_batch_at_current(batch)
    }
}

fn source_authority_to_parse_error(error: SourceAuthorityErrorV1) -> ParseError {
    let message = match error {
        SourceAuthorityErrorV1::ForeignBoxSite => {
            "Box source site belongs to another parser invocation".to_owned()
        }
        SourceAuthorityErrorV1::StaleMemberSite => "Box source member site is stale".to_owned(),
        SourceAuthorityErrorV1::MemberOrdinalOverflow => {
            "Box member ordinal exceeds u32".to_owned()
        }
        SourceAuthorityErrorV1::MissingMethodSourceRelation { inventory_ordinal } => {
            format!("Box source relation missing for inventory ordinal {inventory_ordinal}")
        }
        SourceAuthorityErrorV1::MethodSourceRelationMismatch { name } => {
            format!("Box source relation does not match method `{name}`")
        }
        SourceAuthorityErrorV1::Inventory(error) => {
            return crate::parser::declarations::box_def::members::pending_method::map_inventory_error(
                error,
            );
        }
    };
    ParseError::BuildCfg { message, line: 0 }
}

fn inventory_error_to_parse_error(error: BoxMethodInventoryErrorV1) -> ParseError {
    crate::parser::declarations::box_def::members::pending_method::map_inventory_error(error)
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
        assert_eq!(prepared.method_relations().len(), 1);
        let MethodSourceRelationV1::Explicit(relation) = &prepared.method_relations()[0] else {
            panic!("direct method must produce an explicit source relation")
        };
        assert_eq!(relation.inventory_ordinal(), ordinal);
        assert_eq!(relation.name(), "length");
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
        transaction.finish_member().unwrap();
        let duplicate_site = SourceBoxMethodSiteV1::Direct {
            member: transaction.current_member_site(),
        };

        let error = transaction
            .commit_explicit_method(
                duplicate_site,
                "length",
                function("length"),
                Span::unknown(),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            SourceAuthorityErrorV1::Inventory(BoxMethodInventoryErrorV1::DuplicateMethod { .. })
        ));
        assert_eq!(transaction.inventory().len(), 1);
    }

    #[test]
    fn stale_same_box_member_site_is_rejected_before_inventory_mutation() {
        let brand = ParserInvocationBrandV1::issue();
        let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 3);
        let stale_site = SourceBoxMethodSiteV1::Direct {
            member: transaction.current_member_site(),
        };
        transaction.finish_member().unwrap();

        assert_eq!(
            transaction
                .commit_explicit_method(stale_site, "length", function("length"), Span::unknown())
                .unwrap_err(),
            SourceAuthorityErrorV1::StaleMemberSite
        );
        assert!(transaction.inventory().is_empty());
    }
}
