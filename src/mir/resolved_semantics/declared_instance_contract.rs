//! Resolver-only co-seal for declared Query behavior and Home ABI.
//!
//! The Home catalog owns the declaration catalog. This module adds no semantic
//! axis; it only seals exact same-declaration coverage for the selected Query
//! subset.

use std::collections::BTreeSet;

use super::{
    ResolverCatalogBrandV1, VerifiedDeclaredInstanceMethodHomeCatalogV1,
    VerifiedDeclaredQueryBehaviorCatalogV1, VerifiedDeclaredQueryBehaviorV1, VerifiedHomeAbiV1,
    VerifiedInstanceMethodDeclarationV1,
};
use crate::parser::ResolverSourceInvocationProvenanceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredInstanceMethodContractIssueV1 {
    HomeCatalogEmpty,
    QueryCatalogEmpty,
    ResolverBrandMismatch,
    HomeRowCountMismatch,
    HomeDeclarationIdentityMismatch { index: usize },
    QueryDeclarationNotFound { query_index: usize },
    DuplicateQueryDeclaration { query_index: usize },
    QueryOrderViolation { query_index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SelectedDeclarationPairV1 {
    declaration_index: usize,
    home_index: usize,
    query_index: usize,
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceMethodContractCatalogV1 {
    home_catalog: VerifiedDeclaredInstanceMethodHomeCatalogV1,
    query_catalog: VerifiedDeclaredQueryBehaviorCatalogV1,
    selected_pairs: Box<[SelectedDeclarationPairV1]>,
}

impl VerifiedDeclaredInstanceMethodContractCatalogV1 {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.home_catalog.resolver_brand()
    }

    pub(crate) fn declarations(&self) -> &[VerifiedInstanceMethodDeclarationV1] {
        self.home_catalog.declarations()
    }

    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        self.home_catalog.parser_provenance()
    }

    pub(crate) fn home_abis(&self) -> &[VerifiedHomeAbiV1] {
        self.home_catalog.home_abis()
    }

    pub(crate) fn query_behaviors(&self) -> &[VerifiedDeclaredQueryBehaviorV1] {
        self.query_catalog.rows()
    }

    pub(crate) fn selected_pair_count(&self) -> usize {
        self.selected_pairs.len()
    }
}

pub(crate) struct DeclaredInstanceMethodContractIssuerV1;

impl DeclaredInstanceMethodContractIssuerV1 {
    pub(crate) fn issue(
        home_catalog: VerifiedDeclaredInstanceMethodHomeCatalogV1,
        query_catalog: VerifiedDeclaredQueryBehaviorCatalogV1,
    ) -> Result<
        VerifiedDeclaredInstanceMethodContractCatalogV1,
        DeclaredInstanceMethodContractIssueV1,
    > {
        if home_catalog.declarations().is_empty() {
            return Err(DeclaredInstanceMethodContractIssueV1::HomeCatalogEmpty);
        }
        if query_catalog.rows().is_empty() {
            return Err(DeclaredInstanceMethodContractIssueV1::QueryCatalogEmpty);
        }

        let resolver_brand = home_catalog.resolver_brand();
        if resolver_brand != query_catalog.resolver_brand() {
            return Err(DeclaredInstanceMethodContractIssueV1::ResolverBrandMismatch);
        }

        let declarations = home_catalog.declarations();
        let home_abis = home_catalog.home_abis();
        if declarations.len() != home_abis.len() {
            return Err(DeclaredInstanceMethodContractIssueV1::HomeRowCountMismatch);
        }
        for (index, (declaration, home_abi)) in declarations.iter().zip(home_abis).enumerate() {
            if !same_declaration_identity(declaration, home_abi) {
                return Err(
                    DeclaredInstanceMethodContractIssueV1::HomeDeclarationIdentityMismatch {
                        index,
                    },
                );
            }
        }

        let mut seen_declarations = BTreeSet::new();
        let mut selected_pairs = Vec::with_capacity(query_catalog.rows().len());
        let mut previous_index = None;
        for (query_index, query) in query_catalog.rows().iter().enumerate() {
            let Some(declaration_index) = declarations
                .iter()
                .position(|declaration| same_query_identity(declaration, query, resolver_brand))
            else {
                return Err(
                    DeclaredInstanceMethodContractIssueV1::QueryDeclarationNotFound { query_index },
                );
            };
            if !seen_declarations.insert(declaration_index) {
                return Err(
                    DeclaredInstanceMethodContractIssueV1::DuplicateQueryDeclaration {
                        query_index,
                    },
                );
            }
            if previous_index.is_some_and(|previous| declaration_index <= previous) {
                return Err(DeclaredInstanceMethodContractIssueV1::QueryOrderViolation {
                    query_index,
                });
            }
            previous_index = Some(declaration_index);
            selected_pairs.push(SelectedDeclarationPairV1 {
                declaration_index,
                home_index: declaration_index,
                query_index,
            });
        }

        Ok(VerifiedDeclaredInstanceMethodContractCatalogV1 {
            home_catalog,
            query_catalog,
            selected_pairs: selected_pairs.into_boxed_slice(),
        })
    }
}

fn same_declaration_identity(
    declaration: &VerifiedInstanceMethodDeclarationV1,
    home_abi: &VerifiedHomeAbiV1,
) -> bool {
    declaration.resolver_brand() == home_abi.resolver_brand()
        && declaration.nominal_box_type() == home_abi.nominal_box_type()
        && declaration.box_statement_ordinal() == home_abi.box_statement_ordinal()
        && declaration.method_member_ordinal() == home_abi.method_member_ordinal()
}

fn same_query_identity(
    declaration: &VerifiedInstanceMethodDeclarationV1,
    query: &VerifiedDeclaredQueryBehaviorV1,
    resolver_brand: ResolverCatalogBrandV1,
) -> bool {
    query.resolver_brand() == resolver_brand
        && query.nominal_box_type() == declaration.nominal_box_type()
        && query.box_statement_ordinal() == declaration.box_statement_ordinal()
        && query.method_member_ordinal() == declaration.method_member_ordinal()
}
