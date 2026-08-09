//! Borrowed Query body-source projection.
//!
//! The general body-source catalog owns all supported direct method rows. This
//! module only borrows that catalog and the aggregate-owned selected
//! declaration/Home/Query view; it never reissues semantic receipts or
//! consumes the general source authority.

use std::collections::BTreeSet;

use crate::parser::ResolverSourceInvocationProvenanceV1;

use super::{
    DeclaredInstanceMethodContractRefV1, ResolverCatalogBrandV1,
    VerifiedDeclaredInstanceMethodContractCatalogV1, VerifiedInstanceMethodBodySourceCatalogV1,
    VerifiedInstanceMethodBodySourceRowV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredQueryBodySourceIssueV1 {
    ResolverBrandMismatch,
    ParserProvenanceMismatch,
    MissingSelectedBodyRow {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    DuplicateSelectedBodyRow {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
}

/// One selected Query body row, retaining both source and declaration
/// relations by borrow rather than copying either authority.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedDeclaredQueryBodySourceRowRefV1<'body, 'contract> {
    body: &'body VerifiedInstanceMethodBodySourceRowV1,
    contract: DeclaredInstanceMethodContractRefV1<'contract>,
}

impl<'body, 'contract> VerifiedDeclaredQueryBodySourceRowRefV1<'body, 'contract> {
    pub(crate) fn body(&self) -> &'body VerifiedInstanceMethodBodySourceRowV1 {
        self.body
    }

    pub(crate) fn contract(&self) -> DeclaredInstanceMethodContractRefV1<'contract> {
        self.contract
    }
}

/// Sparse Query projection over the reusable general body-source catalog.
///
/// The two lifetimes intentionally keep the body source and declared contract
/// owners visible to the next owner-binding step.
#[derive(Debug)]
pub(crate) struct VerifiedDeclaredQueryBodySourceCatalogV1<'body, 'contract> {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: &'body ResolverSourceInvocationProvenanceV1,
    rows: Box<[VerifiedDeclaredQueryBodySourceRowRefV1<'body, 'contract>]>,
}

impl<'body, 'contract> VerifiedDeclaredQueryBodySourceCatalogV1<'body, 'contract> {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &'body ResolverSourceInvocationProvenanceV1 {
        self.parser_provenance
    }

    pub(crate) fn rows(&self) -> &[VerifiedDeclaredQueryBodySourceRowRefV1<'body, 'contract>] {
        &self.rows
    }
}

pub(crate) struct DeclaredQueryBodySourceIssuerV1;

impl DeclaredQueryBodySourceIssuerV1 {
    pub(crate) fn issue<'body, 'contract>(
        body_catalog: &'body VerifiedInstanceMethodBodySourceCatalogV1,
        contract_catalog: &'contract VerifiedDeclaredInstanceMethodContractCatalogV1,
    ) -> Result<
        VerifiedDeclaredQueryBodySourceCatalogV1<'body, 'contract>,
        DeclaredQueryBodySourceIssueV1,
    > {
        if body_catalog.resolver_brand() != contract_catalog.resolver_brand() {
            return Err(DeclaredQueryBodySourceIssueV1::ResolverBrandMismatch);
        }
        if !body_catalog
            .parser_provenance()
            .same_as(contract_catalog.parser_provenance())
        {
            return Err(DeclaredQueryBodySourceIssueV1::ParserProvenanceMismatch);
        }

        let selected = contract_catalog.selected_contracts().collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        let mut rows = Vec::with_capacity(selected.len());

        // Iterating the general catalog preserves its source order.  The
        // selected view only answers membership and carries the declaration
        // relation; it never supplies a second ordering authority.
        for body in body_catalog.rows() {
            let Some(contract) = selected.iter().find(|contract| {
                same_declaration_identity(contract, body, body_catalog.resolver_brand())
            }) else {
                continue;
            };

            let key = (
                body.box_statement_ordinal(),
                body.method_member_ordinal(),
            );
            if !seen.insert(key) {
                return Err(DeclaredQueryBodySourceIssueV1::DuplicateSelectedBodyRow {
                    box_statement_ordinal: key.0,
                    member_ordinal: key.1,
                });
            }
            rows.push(VerifiedDeclaredQueryBodySourceRowRefV1 {
                body,
                contract: *contract,
            });
        }

        for contract in &selected {
            let declaration = contract.declaration();
            let key = (
                declaration.box_statement_ordinal(),
                declaration.method_member_ordinal(),
            );
            if !seen.contains(&key) {
                return Err(DeclaredQueryBodySourceIssueV1::MissingSelectedBodyRow {
                    box_statement_ordinal: key.0,
                    member_ordinal: key.1,
                });
            }
        }

        Ok(VerifiedDeclaredQueryBodySourceCatalogV1 {
            resolver_brand: body_catalog.resolver_brand(),
            parser_provenance: body_catalog.parser_provenance(),
            rows: rows.into_boxed_slice(),
        })
    }
}

fn same_declaration_identity(
    contract: &DeclaredInstanceMethodContractRefV1<'_>,
    body: &VerifiedInstanceMethodBodySourceRowV1,
    resolver_brand: ResolverCatalogBrandV1,
) -> bool {
    let declaration = contract.declaration();
    declaration.resolver_brand() == resolver_brand
        && body.resolver_brand() == resolver_brand
        && declaration.nominal_box_type() == body.nominal_box_type()
        && declaration.box_statement_ordinal() == body.box_statement_ordinal()
        && declaration.method_member_ordinal() == body.method_member_ordinal()
}

#[cfg(test)]
#[path = "declared_query_body_source_tests.rs"]
mod tests;
