//! Resolver-owned declared Query behavior.
//!
//! This module consumes typed `CallableContractSyntaxV1::Query` carriage from
//! an already-issued declaration catalog. It owns only the behavioral
//! obligation; Home demands/results, body conformance, and physical effects
//! belong to other owners.

use std::collections::BTreeSet;

use crate::parser::CallableContractSyntaxV1;

use super::{
    ResolverCatalogBrandV1, ResolverNominalBoxTypeIdV1, VerifiedInstanceMethodDeclarationCatalogV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclaredQueryBehaviorV1 {
    ReceiverDirectReadNoEffects,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QueryBehaviorIssueV1 {
    NoQueryDeclaration,
    ResolverBrandMismatch,
    NominalBoxBrandMismatch,
    DuplicateDeclarationSite {
        box_statement_ordinal: u32,
        method_member_ordinal: u32,
    },
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredQueryBehaviorV1 {
    resolver_brand: ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
    behavior: DeclaredQueryBehaviorV1,
    rune_ordinal: u32,
}

impl VerifiedDeclaredQueryBehaviorV1 {
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

    pub(crate) const fn behavior(&self) -> DeclaredQueryBehaviorV1 {
        self.behavior
    }

    pub(crate) const fn rune_ordinal(&self) -> u32 {
        self.rune_ordinal
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredQueryBehaviorCatalogV1 {
    resolver_brand: ResolverCatalogBrandV1,
    rows: Box<[VerifiedDeclaredQueryBehaviorV1]>,
}

impl VerifiedDeclaredQueryBehaviorCatalogV1 {
    pub(crate) fn rows(&self) -> &[VerifiedDeclaredQueryBehaviorV1] {
        &self.rows
    }

    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }
}

pub(crate) struct DeclaredQueryBehaviorIssuerV1;

impl DeclaredQueryBehaviorIssuerV1 {
    pub(crate) fn issue(
        catalog: &VerifiedInstanceMethodDeclarationCatalogV1,
    ) -> Result<VerifiedDeclaredQueryBehaviorCatalogV1, QueryBehaviorIssueV1> {
        let resolver_brand = catalog.resolver_brand();
        let mut seen_sites = BTreeSet::new();
        let mut rows = Vec::new();

        for declaration in catalog.declarations() {
            let Some(contract) = declaration.callable_contract() else {
                continue;
            };

            let CallableContractSyntaxV1::Query { .. } = contract;
            if declaration.resolver_brand() != resolver_brand {
                return Err(QueryBehaviorIssueV1::ResolverBrandMismatch);
            }
            if declaration.nominal_box_type().brand() != resolver_brand {
                return Err(QueryBehaviorIssueV1::NominalBoxBrandMismatch);
            }

            let site = (
                declaration.box_statement_ordinal(),
                declaration.method_member_ordinal(),
            );
            if !seen_sites.insert(site) {
                return Err(QueryBehaviorIssueV1::DuplicateDeclarationSite {
                    box_statement_ordinal: site.0,
                    method_member_ordinal: site.1,
                });
            }

            rows.push(VerifiedDeclaredQueryBehaviorV1 {
                resolver_brand,
                nominal_box_type: declaration.nominal_box_type(),
                box_statement_ordinal: site.0,
                method_member_ordinal: site.1,
                behavior: DeclaredQueryBehaviorV1::ReceiverDirectReadNoEffects,
                rune_ordinal: contract.source_site().rune_ordinal(),
            });
        }

        if rows.is_empty() {
            return Err(QueryBehaviorIssueV1::NoQueryDeclaration);
        }

        Ok(VerifiedDeclaredQueryBehaviorCatalogV1 {
            resolver_brand,
            rows: rows.into_boxed_slice(),
        })
    }
}
