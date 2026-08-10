//! Parser-private source authority substrate for Box declarations.
//!
//! R6-S1 introduced this parser-private transaction. R6-S3A now connects its
//! prepared payload to the bounded rich parse product; the final non-Clone
//! seal is still issued only by `source_seal` after all supported postpasses.

#![allow(dead_code)]

mod selected_gate;

use std::sync::Arc;

use super::callable_contract_syntax::CallableContractSyntaxV1;
use super::callable_gate_projection::MemberGateSelectionReceiptV1;
use super::source_member_cursor::{
    ParserBoxMemberSourceCursorErrorV1, ParserBoxMemberSourceCursorV1,
};
use crate::ast::{
    ASTNode, BoxMethodInventoryErrorV1, BoxMethodInventoryOrdinalV1,
    BoxMethodInventoryPlacementReceiptV1, BoxMethodInventoryV1, DelegateDecl,
    PreparedGeneratedBoxMethodBatchV1, Span,
};
use crate::parser::ParseError;

pub(super) use super::source_seal::PreparedBoxSourceSealV1;

#[derive(Debug, Clone)]
pub(super) struct ParserInvocationBrandV1(Arc<()>);

impl ParserInvocationBrandV1 {
    pub(super) fn issue() -> Self {
        Self(Arc::new(()))
    }

    pub(super) fn same_as(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialEq for ParserInvocationBrandV1 {
    fn eq(&self, other: &Self) -> bool {
        self.same_as(other)
    }
}

impl Eq for ParserInvocationBrandV1 {}

pub(super) use super::source_path::{
    SourceBoxDeclarationPathV1, SourceBoxPathCursorV1, SourceBoxPathSegmentV1,
    SourceBuildGateBranchV1, SourceBuildGateIdV1, SourceProgramCallablePathV1,
    SourceProgramDeclarationPathV1, SourceProgramMemberGateStepV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBoxDeclarationSiteV1 {
    path: SourceBoxDeclarationPathV1,
}

impl SourceBoxDeclarationSiteV1 {
    pub(super) fn from_path(path: SourceBoxDeclarationPathV1) -> Self {
        Self { path }
    }

    pub(super) fn statement_ordinal(&self) -> u32 {
        self.path.root_statement_ordinal().unwrap_or_default()
    }

    pub(super) fn path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.path
    }

    fn brand_matches(&self, brand: &ParserInvocationBrandV1) -> bool {
        self.path.brand().same_as(brand)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBoxMemberSiteV1 {
    box_site: SourceBoxDeclarationSiteV1,
    member_ordinal: u32,
}

impl SourceBoxMemberSiteV1 {
    pub(super) fn new(box_site: SourceBoxDeclarationSiteV1, member_ordinal: u32) -> Self {
        Self {
            box_site,
            member_ordinal,
        }
    }

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
    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        self.member().box_site()
    }

    pub(super) fn source_member_ordinal(&self) -> u32 {
        self.member().member_ordinal()
    }

    pub(super) fn is_direct(&self) -> bool {
        matches!(self, Self::Direct { .. })
    }

    pub(super) fn matches_ast_selection(
        &self,
        selection: &crate::ast::BoxMethodSourceSelectionV1,
    ) -> bool {
        match (self, selection) {
            (Self::Direct { .. }, crate::ast::BoxMethodSourceSelectionV1::Direct) => true,
            (
                Self::SelectedBuildGate { path, .. },
                crate::ast::BoxMethodSourceSelectionV1::SelectedBuildGate { path: ast_path },
            ) => {
                path.len() == ast_path.len()
                    && path.iter().zip(ast_path.iter()).all(|(source, ast)| {
                        source.gate_member_ordinal == ast.gate_site().box_member_ordinal()
                            && source.branch_member_ordinal == ast.branch_member_ordinal()
                    })
            }
            _ => false,
        }
    }

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DelegateSourceDeclarationV1 {
    source_site: SourceBoxMethodSiteV1,
    expose_ordinal: u32,
    delegate_field_name: Box<str>,
    source_method_name: Box<str>,
    exposed_method_name: Box<str>,
}

impl DelegateSourceDeclarationV1 {
    pub(super) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
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

    fn prepend_selected_gate(&mut self, gate_member_ordinal: u32, branch_member_ordinal: u32) {
        self.source_site
            .prepend_selected_gate(gate_member_ordinal, branch_member_ordinal);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExplicitMethodSourceRelationV1 {
    source_site: SourceBoxMethodSiteV1,
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    name: Box<str>,
    callable_contract: Option<CallableContractSyntaxV1>,
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

    pub(super) fn callable_contract(&self) -> Option<&CallableContractSyntaxV1> {
        self.callable_contract.as_ref()
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
    pub(super) fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        match self {
            Self::Explicit(relation) => relation.inventory_ordinal(),
            Self::GeneratedProperty {
                inventory_ordinal, ..
            } => *inventory_ordinal,
        }
    }

    pub(super) fn name(&self) -> &str {
        match self {
            Self::Explicit(relation) => relation.name(),
            Self::GeneratedProperty { name, .. } => name,
        }
    }

    pub(super) fn source_site(&self) -> Option<&SourceBoxMethodSiteV1> {
        match self {
            Self::Explicit(relation) => Some(relation.source_site()),
            Self::GeneratedProperty { .. } => None,
        }
    }

    pub(super) fn callable_contract(&self) -> Option<&CallableContractSyntaxV1> {
        match self {
            Self::Explicit(relation) => relation.callable_contract(),
            Self::GeneratedProperty { .. } => None,
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
    ExposeOrdinalOverflow,
    DelegateCompatibilityOnly,
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

/// Explicit declaration commit and callable path must come from one source
/// transaction.  Generated/compatibility sinks intentionally do not implement
/// this capability.
pub(super) trait DirectExplicitMethodSinkV1: ExplicitMethodSink {
    fn current_program_callable_path(&self) -> SourceProgramCallablePathV1;
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
    cursor: ParserBoxMemberSourceCursorV1,
    written_gate_path: Vec<SourceProgramMemberGateStepV1>,
    member_gate_selection_receipts: Vec<MemberGateSelectionReceiptV1>,
    inventory: BoxMethodInventoryV1,
    method_relations: Vec<MethodSourceRelationV1>,
    delegate_source_declarations: Vec<DelegateSourceDeclarationV1>,
}

impl OpenBoxMethodSourceTransactionV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1, statement_ordinal: u32) -> Self {
        Self {
            cursor: ParserBoxMemberSourceCursorV1::open(brand, statement_ordinal),
            written_gate_path: Vec::new(),
            member_gate_selection_receipts: Vec::new(),
            inventory: BoxMethodInventoryV1::empty(),
            method_relations: Vec::new(),
            delegate_source_declarations: Vec::new(),
        }
    }

    pub(super) fn open_with_path(
        brand: ParserInvocationBrandV1,
        path: SourceBoxDeclarationPathV1,
    ) -> Self {
        Self {
            cursor: ParserBoxMemberSourceCursorV1::open_with_path(brand, path),
            written_gate_path: Vec::new(),
            member_gate_selection_receipts: Vec::new(),
            inventory: BoxMethodInventoryV1::empty(),
            method_relations: Vec::new(),
            delegate_source_declarations: Vec::new(),
        }
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        self.cursor.box_site()
    }

    pub(super) fn current_member_site(&self) -> SourceBoxMemberSiteV1 {
        self.cursor.current_member_site()
    }

    pub(super) fn current_gate_site(&self) -> crate::ast::BoxMemberGateSiteV1 {
        self.cursor.current_gate_site()
    }

    pub(super) fn current_member_ordinal(&self) -> u32 {
        self.cursor.current_member_ordinal()
    }

    pub(super) fn branch(&self) -> Self {
        Self {
            cursor: self.cursor.branch(),
            written_gate_path: self.written_gate_path.clone(),
            member_gate_selection_receipts: Vec::new(),
            inventory: BoxMethodInventoryV1::empty(),
            method_relations: Vec::new(),
            delegate_source_declarations: Vec::new(),
        }
    }

    /// Create one as-written member-gate branch transaction.  Selection is a
    /// later projection and never participates in this path identity.
    pub(super) fn branch_at(
        &self,
        gate_site: crate::ast::BoxMemberGateSiteV1,
        branch: SourceBuildGateBranchV1,
    ) -> Self {
        let mut next = self.branch();
        next.written_gate_path
            .push(SourceProgramMemberGateStepV1::new(
                gate_site.box_member_ordinal(),
                branch,
            ));
        next
    }

    pub(super) fn current_program_callable_path(&self) -> SourceProgramCallablePathV1 {
        SourceProgramCallablePathV1::box_method(
            SourceProgramDeclarationPathV1::from_parser_path(self.cursor.box_site().path().clone()),
            self.written_gate_path.clone().into_boxed_slice(),
            self.cursor.current_member_ordinal(),
        )
    }

    pub(super) fn finish_member(&mut self) -> Result<(), SourceAuthorityErrorV1> {
        self.cursor.finish_member().map_err(|error| match error {
            ParserBoxMemberSourceCursorErrorV1::MemberOrdinalOverflow => {
                SourceAuthorityErrorV1::MemberOrdinalOverflow
            }
        })
    }

    pub(super) fn commit_explicit_method(
        &mut self,
        source_site: SourceBoxMethodSiteV1,
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, SourceAuthorityErrorV1> {
        if !source_site.validate_for(
            self.cursor.brand(),
            self.cursor.box_site(),
            self.cursor.current_member_ordinal(),
        ) {
            if source_site.member().box_site() == self.cursor.box_site()
                && source_site.member().member_ordinal() != self.cursor.current_member_ordinal()
            {
                return Err(SourceAuthorityErrorV1::StaleMemberSite);
            }
            return Err(SourceAuthorityErrorV1::ForeignBoxSite);
        }
        let name = name.into();
        let callable_contract = CallableContractSyntaxV1::from_instance_method(&declaration);
        let ordinal = self
            .inventory
            .try_push_explicit_source(name.clone(), declaration, diagnostic_span)
            .map_err(SourceAuthorityErrorV1::Inventory)?;
        self.method_relations.push(MethodSourceRelationV1::Explicit(
            ExplicitMethodSourceRelationV1 {
                source_site,
                inventory_ordinal: ordinal,
                name,
                callable_contract,
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

    pub(super) fn record_delegate_source_at_current(
        &mut self,
        delegate: &DelegateDecl,
    ) -> Result<(), ParseError> {
        if delegate.explicit_source_selection().is_none() {
            return Err(source_authority_to_parse_error(
                SourceAuthorityErrorV1::DelegateCompatibilityOnly,
            ));
        }
        let source_site = SourceBoxMethodSiteV1::Direct {
            member: self.current_member_site(),
        };
        let mut rows = Vec::with_capacity(delegate.exposes.len());
        for (expose_ordinal, expose) in delegate.exposes.iter().enumerate() {
            let expose_ordinal = u32::try_from(expose_ordinal).map_err(|_| {
                source_authority_to_parse_error(SourceAuthorityErrorV1::ExposeOrdinalOverflow)
            })?;
            rows.push(DelegateSourceDeclarationV1 {
                source_site: source_site.clone(),
                expose_ordinal,
                delegate_field_name: delegate.field_name.clone().into_boxed_str(),
                source_method_name: expose.source_name.clone().into_boxed_str(),
                exposed_method_name: expose.exposed_name.clone().into_boxed_str(),
            });
        }
        self.delegate_source_declarations.extend(rows);
        Ok(())
    }

    pub(super) fn inventory(&self) -> &BoxMethodInventoryV1 {
        &self.inventory
    }

    pub(super) fn method_relations(&self) -> &[MethodSourceRelationV1] {
        &self.method_relations
    }

    pub(super) fn delegate_source_declarations(&self) -> &[DelegateSourceDeclarationV1] {
        &self.delegate_source_declarations
    }

    pub(super) fn finish(self) -> PreparedBoxSourceSealV1 {
        let (brand, box_site) = self.cursor.into_parts();
        PreparedBoxSourceSealV1 {
            brand,
            box_site,
            inventory: self.inventory,
            method_relations: self.method_relations.into_boxed_slice(),
            delegate_source_declarations: self.delegate_source_declarations.into_boxed_slice(),
            member_gate_selection_receipts: self.member_gate_selection_receipts.into_boxed_slice(),
            generated_delegate_source_relations: Box::new([]),
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

impl DirectExplicitMethodSinkV1 for OpenBoxMethodSourceTransactionV1 {
    fn current_program_callable_path(&self) -> SourceProgramCallablePathV1 {
        OpenBoxMethodSourceTransactionV1::current_program_callable_path(self)
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
        SourceAuthorityErrorV1::ExposeOrdinalOverflow => {
            "delegate expose ordinal exceeds u32".to_owned()
        }
        SourceAuthorityErrorV1::DelegateCompatibilityOnly => {
            "compatibility-only delegate cannot issue parser source declarations".to_owned()
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
#[path = "source_authority_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "delegate_source_tests.rs"]
mod delegate_source_tests;
