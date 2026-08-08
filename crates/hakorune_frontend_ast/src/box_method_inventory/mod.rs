//! Ordered Box-method source inventory.
//!
//! This model owns selected-declaration order and a private lookup index.
//! Provenance is descriptive: only a later parser-owned seal may authorize a
//! resolver source row. This crate does not resolve names, types, callable
//! contracts, or physical routes.

mod error;
mod generated_batch;
mod prepared_append;
mod roundtrip_v2;
mod transform;

use std::collections::HashMap;

use crate::{ASTNode, Span};

pub use error::BoxMethodInventoryErrorV1;
pub use generated_batch::{PreparedGeneratedBoxMethodBatchV1, PreparedGeneratedBoxMethodV1};
pub use prepared_append::PreparedBoxMethodInventoryAppendV1;
pub use roundtrip_v2::{BoxMethodInventoryRoundtripRowV2, PreparedBoxMethodInventoryRoundtripV2};
pub use transform::BoxMethodDeclarationTransformErrorV1;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxMemberGateSiteV1 {
    box_member_ordinal: u32,
}

impl BoxMemberGateSiteV1 {
    pub const fn from_box_member_ordinal(box_member_ordinal: u32) -> Self {
        Self { box_member_ordinal }
    }

    pub const fn box_member_ordinal(self) -> u32 {
        self.box_member_ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxMethodGateSelectionV1 {
    gate_site: BoxMemberGateSiteV1,
    branch_member_ordinal: u32,
}

impl BoxMethodGateSelectionV1 {
    pub const fn from_parts(gate_site: BoxMemberGateSiteV1, branch_member_ordinal: u32) -> Self {
        Self {
            gate_site,
            branch_member_ordinal,
        }
    }

    pub const fn gate_site(self) -> BoxMemberGateSiteV1 {
        self.gate_site
    }

    pub const fn branch_member_ordinal(self) -> u32 {
        self.branch_member_ordinal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxMethodSourceSelectionV1 {
    Direct,
    SelectedBuildGate {
        path: Box<[BoxMethodGateSelectionV1]>,
    },
}

impl BoxMethodSourceSelectionV1 {
    pub fn selected_build_gate(
        path: impl Into<Box<[BoxMethodGateSelectionV1]>>,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let path = path.into();
        if path.is_empty() {
            return Err(BoxMethodInventoryErrorV1::EmptySelectedBuildGatePath);
        }
        Ok(Self::SelectedBuildGate { path })
    }

    pub(super) fn validate_transport(&self) -> Result<(), BoxMethodInventoryErrorV1> {
        match self {
            Self::Direct => Ok(()),
            Self::SelectedBuildGate { path } if path.is_empty() => {
                Err(BoxMethodInventoryErrorV1::EmptySelectedBuildGatePath)
            }
            Self::SelectedBuildGate { .. } => Ok(()),
        }
    }

    pub(crate) fn prepend_gate(
        &mut self,
        gate_site: BoxMemberGateSiteV1,
        branch_member_ordinal: u32,
    ) {
        let mut path = vec![BoxMethodGateSelectionV1 {
            gate_site,
            branch_member_ordinal,
        }];
        if let Self::SelectedBuildGate { path: existing } = self {
            path.extend(existing.iter().copied());
        }
        *self = Self::SelectedBuildGate {
            path: path.into_boxed_slice(),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxMethodGeneratedProvenanceV1 {
    Property {
        property_name: Box<str>,
        selection: BoxMethodSourceSelectionV1,
    },
    Delegate {
        field_name: Box<str>,
        exposed_name: Box<str>,
        selection: BoxMethodSourceSelectionV1,
    },
    MacroOrImport {
        generator: Box<str>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxMethodCompatibilityOriginV1 {
    LegacyAstConstruction,
    LegacyJsonV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxMethodProvenanceV1 {
    ExplicitSource {
        selection: BoxMethodSourceSelectionV1,
    },
    Generated(BoxMethodGeneratedProvenanceV1),
    CompatibilityOnly {
        origin: BoxMethodCompatibilityOriginV1,
    },
}

impl BoxMethodProvenanceV1 {
    pub fn explicit_source_selection(&self) -> Option<&BoxMethodSourceSelectionV1> {
        match self {
            Self::ExplicitSource { selection } => Some(selection),
            Self::Generated(_) | Self::CompatibilityOnly { .. } => None,
        }
    }

    fn prepend_selected_gate(
        &mut self,
        gate_site: BoxMemberGateSiteV1,
        branch_member_ordinal: u32,
    ) -> Result<(), BoxMethodInventoryErrorV1> {
        match self {
            Self::ExplicitSource { selection }
            | Self::Generated(BoxMethodGeneratedProvenanceV1::Property { selection, .. })
            | Self::Generated(BoxMethodGeneratedProvenanceV1::Delegate { selection, .. }) => {
                selection.prepend_gate(gate_site, branch_member_ordinal);
                Ok(())
            }
            Self::Generated(BoxMethodGeneratedProvenanceV1::MacroOrImport { .. })
            | Self::CompatibilityOnly { .. } => {
                Err(BoxMethodInventoryErrorV1::InvalidSelectedGateProvenance)
            }
        }
    }

    fn validate_transport(&self) -> Result<(), BoxMethodInventoryErrorV1> {
        match self {
            Self::ExplicitSource { selection }
            | Self::Generated(BoxMethodGeneratedProvenanceV1::Property { selection, .. })
            | Self::Generated(BoxMethodGeneratedProvenanceV1::Delegate { selection, .. }) => {
                selection.validate_transport()
            }
            Self::Generated(BoxMethodGeneratedProvenanceV1::MacroOrImport { .. })
            | Self::CompatibilityOnly { .. } => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Selected/generated inventory placement.
///
/// This is deliberately not a source declaration identity. Parser-owned
/// source sites are a later non-Clone authority and must not be reconstructed
/// from this placement ordinal.
pub struct BoxMethodInventoryOrdinalV1 {
    selected_method_ordinal: u32,
}

impl BoxMethodInventoryOrdinalV1 {
    /// Canonical descriptive inventory placement accessor.
    pub const fn inventory_ordinal(self) -> u32 {
        self.selected_method_ordinal
    }

    /// Compatibility accessor preserving the v2 wire vocabulary.
    pub const fn selected_method_ordinal(self) -> u32 {
        self.inventory_ordinal()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxMethodInventoryPlacementReceiptV1 {
    name: Box<str>,
    ordinal: BoxMethodInventoryOrdinalV1,
}

impl BoxMethodInventoryPlacementReceiptV1 {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        self.ordinal
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoxMethodEntryV1 {
    name: Box<str>,
    declaration: ASTNode,
    provenance: BoxMethodProvenanceV1,
    site: BoxMethodInventoryOrdinalV1,
    diagnostic_span: Span,
}

impl BoxMethodEntryV1 {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn declaration(&self) -> &ASTNode {
        &self.declaration
    }

    pub fn provenance(&self) -> &BoxMethodProvenanceV1 {
        &self.provenance
    }

    pub const fn site(&self) -> BoxMethodInventoryOrdinalV1 {
        self.site
    }

    pub const fn diagnostic_span(&self) -> Span {
        self.diagnostic_span
    }

    /// Rebase parser-owned selected-gate provenance without exposing the
    /// inventory placement or source authority to the AST carrier.
    pub fn prepend_selected_gate(
        &mut self,
        gate_site: BoxMemberGateSiteV1,
        branch_member_ordinal: u32,
    ) -> Result<(), BoxMethodInventoryErrorV1> {
        self.provenance
            .prepend_selected_gate(gate_site, branch_member_ordinal)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoxMethodInventoryV1 {
    entries: Vec<BoxMethodEntryV1>,
    lookup: HashMap<Box<str>, usize>,
}

impl BoxMethodInventoryV1 {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter_selected_declaration_order(
        &self,
    ) -> impl ExactSizeIterator<Item = &BoxMethodEntryV1> {
        self.entries.iter()
    }

    pub fn iter_compat_name_order(&self) -> impl Iterator<Item = &BoxMethodEntryV1> {
        let mut entries = self.entries.iter().collect::<Vec<_>>();
        entries.sort_by(|left, right| left.name.cmp(&right.name));
        entries.into_iter()
    }

    pub fn get(&self, name: &str) -> Option<&BoxMethodEntryV1> {
        self.lookup.get(name).map(|index| &self.entries[*index])
    }

    pub fn get_declaration(&self, name: &str) -> Option<&ASTNode> {
        self.get(name).map(BoxMethodEntryV1::declaration)
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.lookup.contains_key(name)
    }

    pub fn declarations_in_selected_order(&self) -> impl ExactSizeIterator<Item = &ASTNode> {
        self.entries.iter().map(BoxMethodEntryV1::declaration)
    }

    pub fn names_in_selected_order(&self) -> impl ExactSizeIterator<Item = &str> {
        self.entries.iter().map(BoxMethodEntryV1::name)
    }

    pub fn into_selected_declaration_order(self) -> Vec<BoxMethodEntryV1> {
        self.entries
    }

    /// Rewrites declaration payloads while preserving the ordered inventory
    /// and each row's descriptive provenance. This is the only canonical
    /// carrier-preserving transform; callers cannot rebuild source identity
    /// from a compatibility map.
    pub fn map_declarations<F>(self, mut rewrite: F) -> Result<Self, BoxMethodInventoryErrorV1>
    where
        F: FnMut(ASTNode) -> ASTNode,
    {
        let mut mapped = Self::empty();
        for entry in self.entries {
            mapped.try_push_with_provenance(
                entry.name,
                rewrite(entry.declaration),
                entry.provenance,
                entry.diagnostic_span,
            )?;
        }
        Ok(mapped)
    }

    /// Consumes the carrier into the historical deterministic name-order
    /// projection used by compatibility-only Builder edges. This is not
    /// source order and must not be used as resolver or semantic authority.
    pub fn into_compatibility_name_order(self) -> Vec<BoxMethodEntryV1> {
        let mut entries = self.entries;
        entries.sort_by(|left, right| left.name.cmp(&right.name));
        entries
    }

    /// Projects the carrier into the legacy lookup shape. The result carries
    /// no order, provenance, or resolver authority.
    pub fn clone_compatibility_map(&self) -> HashMap<String, ASTNode> {
        self.entries
            .iter()
            .map(|entry| (entry.name.to_string(), entry.declaration.clone()))
            .collect()
    }

    /// Consumes the carrier into the legacy lookup shape. The result carries
    /// no order, provenance, or resolver authority.
    pub fn into_compatibility_map(self) -> HashMap<String, ASTNode> {
        self.entries
            .into_iter()
            .map(|entry| (entry.name.into(), entry.declaration))
            .collect()
    }

    /// Adds descriptive explicit-source provenance to the raw AST carrier.
    ///
    /// This does not issue a resolver capability. A future parser-owned seal
    /// must prove complete parsing, selected membership, and exact source
    /// identity before a row may cross into resolved semantics.
    pub fn try_push_explicit_source(
        &mut self,
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, BoxMethodInventoryErrorV1> {
        self.try_push_with_provenance(
            name.into(),
            declaration,
            BoxMethodProvenanceV1::ExplicitSource {
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            diagnostic_span,
        )
    }

    pub fn try_push_generated(
        &mut self,
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        provenance: BoxMethodGeneratedProvenanceV1,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, BoxMethodInventoryErrorV1> {
        self.try_push_with_provenance(
            name.into(),
            declaration,
            BoxMethodProvenanceV1::Generated(provenance),
            diagnostic_span,
        )
    }

    pub fn try_commit_generated_batch(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<(), BoxMethodInventoryErrorV1> {
        self.try_commit_generated_batch_with_ordinals(batch)
            .map(|_| ())
    }

    pub fn try_commit_generated_batch_with_ordinals(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryOrdinalV1]>, BoxMethodInventoryErrorV1> {
        Ok(self
            .try_commit_generated_batch_with_placements(batch)?
            .into_vec()
            .into_iter()
            .map(|placement| placement.inventory_ordinal())
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub fn try_commit_generated_batch_with_placements(
        &mut self,
        batch: PreparedGeneratedBoxMethodBatchV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, BoxMethodInventoryErrorV1> {
        let entries = batch
            .rows
            .into_vec()
            .into_iter()
            .map(|row| BoxMethodEntryV1 {
                name: row.name,
                declaration: row.declaration,
                provenance: BoxMethodProvenanceV1::Generated(row.provenance),
                site: BoxMethodInventoryOrdinalV1 {
                    selected_method_ordinal: 0,
                },
                diagnostic_span: row.diagnostic_span,
            });
        self.commit_prepared_append(PreparedBoxMethodInventoryAppendV1::try_new(entries)?)
    }

    pub fn try_push_compatibility(
        &mut self,
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        origin: BoxMethodCompatibilityOriginV1,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, BoxMethodInventoryErrorV1> {
        self.try_push_with_provenance(
            name.into(),
            declaration,
            BoxMethodProvenanceV1::CompatibilityOnly { origin },
            diagnostic_span,
        )
    }

    pub fn try_from_compatibility_entries<I, N>(
        entries: I,
        origin: BoxMethodCompatibilityOriginV1,
    ) -> Result<Self, BoxMethodInventoryErrorV1>
    where
        I: IntoIterator<Item = (N, ASTNode)>,
        N: Into<Box<str>>,
    {
        let mut inventory = Self::empty();
        for (name, declaration) in entries {
            let diagnostic_span = declaration.span();
            inventory.try_push_compatibility(name, declaration, origin, diagnostic_span)?;
        }
        Ok(inventory)
    }

    /// Imports a legacy method map through an explicitly non-authoritative,
    /// deterministic name-order compatibility boundary.
    pub fn try_from_compatibility_map(
        methods: HashMap<String, ASTNode>,
        origin: BoxMethodCompatibilityOriginV1,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let mut entries = methods.into_iter().collect::<Vec<_>>();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        Self::try_from_compatibility_entries(entries, origin)
    }

    pub fn from_legacy_ast_map(methods: HashMap<String, ASTNode>) -> Self {
        Self::try_from_compatibility_map(
            methods,
            BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
        )
        .expect("legacy AST HashMap compatibility import must be lossless")
    }

    /// Commits an append whose source/gate rebasing was completed by the
    /// parser transaction. This is the only AST-side append authority for the
    /// R6 typed transaction bridge.
    pub fn commit_prepared_append(
        &mut self,
        append: PreparedBoxMethodInventoryAppendV1,
    ) -> Result<Box<[BoxMethodInventoryPlacementReceiptV1]>, BoxMethodInventoryErrorV1> {
        for entry in &append.entries {
            if let Some(first_index) = self.lookup.get(entry.name()) {
                return Err(BoxMethodInventoryErrorV1::DuplicateMethod {
                    name: entry.name.clone(),
                    first_span: self.entries[*first_index].diagnostic_span,
                    duplicate_span: entry.diagnostic_span,
                });
            }
        }

        let base = self.entries.len();
        let final_len = base
            .checked_add(append.entries.len())
            .ok_or(BoxMethodInventoryErrorV1::OrdinalOverflow)?;
        u32::try_from(final_len).map_err(|_| BoxMethodInventoryErrorV1::OrdinalOverflow)?;

        let mut placements = Vec::with_capacity(append.entries.len());
        for (offset, mut entry) in append.into_entries().into_vec().into_iter().enumerate() {
            let ordinal = BoxMethodInventoryOrdinalV1 {
                selected_method_ordinal: u32::try_from(base + offset)
                    .map_err(|_| BoxMethodInventoryErrorV1::OrdinalOverflow)?,
            };
            entry.site = ordinal;
            let index = self.entries.len();
            self.lookup.insert(entry.name.clone(), index);
            self.entries.push(entry);
            placements.push(BoxMethodInventoryPlacementReceiptV1 {
                name: self.entries[index].name.clone(),
                ordinal,
            });
        }
        Ok(placements.into_boxed_slice())
    }

    fn try_push_with_provenance(
        &mut self,
        name: Box<str>,
        declaration: ASTNode,
        provenance: BoxMethodProvenanceV1,
        diagnostic_span: Span,
    ) -> Result<BoxMethodInventoryOrdinalV1, BoxMethodInventoryErrorV1> {
        if !matches!(provenance, BoxMethodProvenanceV1::CompatibilityOnly { .. }) {
            Self::validate_declaration_name(&name, &declaration)?;
        }
        if let Some(first_index) = self.lookup.get(name.as_ref()) {
            return Err(BoxMethodInventoryErrorV1::DuplicateMethod {
                name,
                first_span: self.entries[*first_index].diagnostic_span,
                duplicate_span: diagnostic_span,
            });
        }
        let selected_method_ordinal = u32::try_from(self.entries.len())
            .map_err(|_| BoxMethodInventoryErrorV1::OrdinalOverflow)?;
        let site = BoxMethodInventoryOrdinalV1 {
            selected_method_ordinal,
        };
        let index = self.entries.len();
        self.lookup.insert(name.clone(), index);
        self.entries.push(BoxMethodEntryV1 {
            name,
            declaration,
            provenance,
            site,
            diagnostic_span,
        });
        Ok(site)
    }

    pub(super) fn validate_declaration_name(
        expected: &str,
        declaration: &ASTNode,
    ) -> Result<(), BoxMethodInventoryErrorV1> {
        let ASTNode::FunctionDeclaration { name, .. } = declaration else {
            return Err(BoxMethodInventoryErrorV1::NotFunctionDeclaration);
        };
        if name != expected {
            return Err(BoxMethodInventoryErrorV1::DeclarationNameMismatch {
                inventory_name: expected.into(),
                declaration_name: name.clone().into_boxed_str(),
            });
        }
        Ok(())
    }
}
