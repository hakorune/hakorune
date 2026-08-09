//! AST-free resolver body-source issuer for the bounded instance-method row.
//!
//! This module seals source identity and ordered body coverage only. It does
//! not observe behavior, issue FunctionOwnerId, or infer Query/Home/ABI facts.

use std::collections::BTreeSet;

use crate::parser::{
    ParserBoxBodySourceEnvelopeV1, ParserBoxMethodBodySourceRowV1,
    ResolverSourceInvocationProvenanceV1,
};

use super::{
    ResolverCatalogBrandV1, ResolverNominalBoxTypeIdV1,
    VerifiedInstanceMethodDeclarationCatalogV1, VerifiedInstanceMethodDeclarationV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InstanceMethodBodySourceIssueV1 {
    ParserProvenanceMismatch,
    ResolverBrandMismatch,
    ForeignBodyRow {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    DuplicateBodyRow {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    BodyNameMismatch {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    MissingBodyRow {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    BodyItemPathNotContiguous {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceMethodBodySourceCatalogV1 {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    rows: Box<[VerifiedInstanceMethodBodySourceRowV1]>,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceMethodBodySourceRowV1 {
    resolver_brand: ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
    name: Box<str>,
    body_item_ordinals: Box<[u32]>,
}

impl VerifiedInstanceMethodBodySourceCatalogV1 {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        &self.parser_provenance
    }

    pub(crate) fn rows(&self) -> &[VerifiedInstanceMethodBodySourceRowV1] {
        &self.rows
    }
}

impl VerifiedInstanceMethodBodySourceRowV1 {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) const fn nominal_box_type(&self) -> ResolverNominalBoxTypeIdV1 {
        self.nominal_box_type
    }

    pub(crate) const fn box_statement_ordinal(&self) -> u32 {
        self.box_statement_ordinal
    }

    pub(crate) const fn method_member_ordinal(&self) -> u32 {
        self.method_member_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn body_item_ordinals(&self) -> &[u32] {
        &self.body_item_ordinals
    }
}

pub(crate) struct InstanceMethodBodySourceIssuerV1;

impl InstanceMethodBodySourceIssuerV1 {
    pub(crate) fn issue(
        envelope: ParserBoxBodySourceEnvelopeV1,
        declarations: &VerifiedInstanceMethodDeclarationCatalogV1,
    ) -> Result<VerifiedInstanceMethodBodySourceCatalogV1, InstanceMethodBodySourceIssueV1> {
        if declarations.declarations().is_empty() {
            return Err(InstanceMethodBodySourceIssueV1::MissingBodyRow {
                box_statement_ordinal: 0,
                member_ordinal: 0,
            });
        }
        envelope.consume_with(|parser_provenance, rows| {
            issue_rows(parser_provenance, rows, declarations)
        })
    }
}

fn issue_rows(
    parser_provenance: &ResolverSourceInvocationProvenanceV1,
    rows: &[ParserBoxMethodBodySourceRowV1],
    declarations: &VerifiedInstanceMethodDeclarationCatalogV1,
) -> Result<VerifiedInstanceMethodBodySourceCatalogV1, InstanceMethodBodySourceIssueV1> {
    if !parser_provenance.same_as(declarations.parser_provenance()) {
        return Err(InstanceMethodBodySourceIssueV1::ParserProvenanceMismatch);
    }
    let resolver_brand = declarations.resolver_brand();
    let declarations = declarations.declarations();
    let mut seen_sites = BTreeSet::new();
    let mut normalized = Vec::with_capacity(rows.len());
    for row in rows {
        let site = row.source_site();
        let key = (site.box_statement_ordinal(), site.member_ordinal());
        if !seen_sites.insert(key) {
            return Err(InstanceMethodBodySourceIssueV1::DuplicateBodyRow {
                box_statement_ordinal: key.0,
                member_ordinal: key.1,
            });
        }
        let Some(declaration) = declarations.iter().find(|declaration| {
            declaration.box_statement_ordinal() == key.0
                && declaration.method_member_ordinal() == key.1
        }) else {
            return Err(InstanceMethodBodySourceIssueV1::ForeignBodyRow {
                box_statement_ordinal: key.0,
                member_ordinal: key.1,
            });
        };
        if declaration.resolver_brand() != resolver_brand
            || declaration.nominal_box_type().brand() != resolver_brand
        {
            return Err(InstanceMethodBodySourceIssueV1::ResolverBrandMismatch);
        }
        if declaration.name() != row.name() {
            return Err(InstanceMethodBodySourceIssueV1::BodyNameMismatch {
                box_statement_ordinal: key.0,
                member_ordinal: key.1,
            });
        }
        if !is_contiguous(row.body_item_ordinals()) {
            return Err(InstanceMethodBodySourceIssueV1::BodyItemPathNotContiguous {
                box_statement_ordinal: key.0,
                member_ordinal: key.1,
            });
        }
        normalized.push(normalize_row(resolver_brand, declaration, row));
    }

    for declaration in declarations {
        let key = (declaration.box_statement_ordinal(), declaration.method_member_ordinal());
        if !normalized.iter().any(|row| {
            row.box_statement_ordinal == key.0 && row.method_member_ordinal == key.1
        }) {
            return Err(InstanceMethodBodySourceIssueV1::MissingBodyRow {
                box_statement_ordinal: key.0,
                member_ordinal: key.1,
            });
        }
    }

    Ok(VerifiedInstanceMethodBodySourceCatalogV1 {
        resolver_brand,
        parser_provenance: parser_provenance.clone(),
        rows: normalized.into_boxed_slice(),
    })
}

fn normalize_row(
    resolver_brand: ResolverCatalogBrandV1,
    declaration: &VerifiedInstanceMethodDeclarationV1,
    row: &ParserBoxMethodBodySourceRowV1,
) -> VerifiedInstanceMethodBodySourceRowV1 {
    VerifiedInstanceMethodBodySourceRowV1 {
        resolver_brand,
        nominal_box_type: declaration.nominal_box_type(),
        box_statement_ordinal: declaration.box_statement_ordinal(),
        method_member_ordinal: declaration.method_member_ordinal(),
        name: row.name().to_owned().into_boxed_str(),
        body_item_ordinals: row.body_item_ordinals().to_vec().into_boxed_slice(),
    }
}

fn is_contiguous(ordinals: &[u32]) -> bool {
    ordinals
        .iter()
        .copied()
        .enumerate()
        .all(|(index, ordinal)| u32::try_from(index) == Ok(ordinal))
}

#[cfg(test)]
#[path = "instance_method_body_source_tests.rs"]
mod tests;
