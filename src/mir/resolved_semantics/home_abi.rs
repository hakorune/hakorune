//! Resolver-owned semantic Home ABI for the bounded instance-method cohort.
//!
//! The issuer consumes a declaration catalog and a same-catalog capability
//! environment in one shot.  It has no Home Flow, Query behavior, target,
//! Recipe, MIR, or physical-ABI authority.

use super::{
    HomeDemandV1, HomeRelationBrandIssuerV1, HomeRelationBrandV1, HomeRelationRejectV1,
    HomeResultRelationV1, ResolverNominalBoxTypeIdV1, ResolverSemanticValueTypeV1,
    VerifiedInstanceMethodDeclarationCatalogV1, VerifiedInstanceMethodDeclarationV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HomeCapabilitySchemaV1 {
    I64UnitTrivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HomeCapabilityDeclarationAnchorV1 {
    resolver_brand: super::ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
}

impl HomeCapabilityDeclarationAnchorV1 {
    fn from_declaration(declaration: &VerifiedInstanceMethodDeclarationV1) -> Self {
        Self {
            resolver_brand: declaration.resolver_brand(),
            nominal_box_type: declaration.nominal_box_type(),
            box_statement_ordinal: declaration.box_statement_ordinal(),
            method_member_ordinal: declaration.method_member_ordinal(),
        }
    }
}

/// Resolver-owned capability evidence bound to one declaration catalog.
///
/// The relation brand is batch provenance only.  It is intentionally a
/// different type from `ResolverCatalogBrandV1` and never becomes nominal
/// type or source declaration identity.
#[derive(Debug)]
pub(crate) struct ResolverHomeCapabilityEnvironmentV1 {
    resolver_brand: super::ResolverCatalogBrandV1,
    schema: HomeCapabilitySchemaV1,
    relation_batch: HomeRelationBrandIssuerV1,
    declaration_anchors: Box<[HomeCapabilityDeclarationAnchorV1]>,
}

impl ResolverHomeCapabilityEnvironmentV1 {
    pub(crate) fn issue(
        catalog: &VerifiedInstanceMethodDeclarationCatalogV1,
    ) -> Result<Self, HomeAbiIssueV1> {
        if catalog.declarations().is_empty() {
            return Err(HomeAbiIssueV1::DeclarationCatalogEmpty);
        }
        let relation_batch =
            HomeRelationBrandIssuerV1::issue().map_err(HomeAbiIssueV1::RelationBatchIssue)?;
        let declaration_anchors = catalog
            .declarations()
            .iter()
            .map(HomeCapabilityDeclarationAnchorV1::from_declaration)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self {
            resolver_brand: catalog.resolver_brand(),
            schema: HomeCapabilitySchemaV1::I64UnitTrivial,
            relation_batch,
            declaration_anchors,
        })
    }

    fn relation_batch_brand(&self) -> HomeRelationBrandV1 {
        self.relation_batch.brand()
    }

    fn matches_catalog(&self, catalog: &VerifiedInstanceMethodDeclarationCatalogV1) -> bool {
        self.resolver_brand == catalog.resolver_brand()
            && self.declaration_anchors.len() == catalog.declarations().len()
            && self
                .declaration_anchors
                .iter()
                .zip(catalog.declarations())
                .all(|(anchor, declaration)| {
                    *anchor == HomeCapabilityDeclarationAnchorV1::from_declaration(declaration)
                })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomeAbiIssueV1 {
    DeclarationCatalogEmpty,
    ResolverBrandMismatch,
    DeclarationAnchorMismatch { index: usize },
    RelationBatchIssue(HomeRelationRejectV1),
}

/// One complete semantic Home ABI row.  The fields are private so no caller
/// can forge a standalone receiver/parameter/result receipt.
#[derive(Debug)]
pub(crate) struct VerifiedHomeAbiV1 {
    resolver_brand: super::ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
    relation_batch_brand: HomeRelationBrandV1,
    receiver: HomeDemandV1,
    parameters: Box<[HomeDemandV1]>,
    result: HomeResultRelationV1,
}

impl VerifiedHomeAbiV1 {
    pub(crate) const fn resolver_brand(&self) -> super::ResolverCatalogBrandV1 {
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

    pub(crate) const fn relation_batch_brand(&self) -> HomeRelationBrandV1 {
        self.relation_batch_brand
    }

    pub(crate) const fn receiver(&self) -> HomeDemandV1 {
        self.receiver
    }

    pub(crate) fn parameters(&self) -> &[HomeDemandV1] {
        &self.parameters
    }

    pub(crate) const fn result(&self) -> HomeResultRelationV1 {
        self.result
    }
}

/// One-shot Home ABI catalog paired with the declaration catalog it consumed.
#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceMethodHomeCatalogV1 {
    declarations: VerifiedInstanceMethodDeclarationCatalogV1,
    relation_batch_brand: HomeRelationBrandV1,
    home_abis: Box<[VerifiedHomeAbiV1]>,
}

impl VerifiedDeclaredInstanceMethodHomeCatalogV1 {
    pub(crate) fn declarations(&self) -> &[VerifiedInstanceMethodDeclarationV1] {
        self.declarations.declarations()
    }

    pub(crate) fn home_abis(&self) -> &[VerifiedHomeAbiV1] {
        &self.home_abis
    }

    pub(crate) const fn relation_batch_brand(&self) -> HomeRelationBrandV1 {
        self.relation_batch_brand
    }
}

pub(crate) struct CallableHomeAbiIssuerV1;

impl CallableHomeAbiIssuerV1 {
    pub(crate) fn issue(
        catalog: VerifiedInstanceMethodDeclarationCatalogV1,
        environment: ResolverHomeCapabilityEnvironmentV1,
    ) -> Result<VerifiedDeclaredInstanceMethodHomeCatalogV1, HomeAbiIssueV1> {
        if catalog.declarations().is_empty() {
            return Err(HomeAbiIssueV1::DeclarationCatalogEmpty);
        }
        if catalog.resolver_brand() != environment.resolver_brand {
            return Err(HomeAbiIssueV1::ResolverBrandMismatch);
        }
        if !environment.matches_catalog(&catalog) {
            let index = environment
                .declaration_anchors
                .iter()
                .zip(catalog.declarations())
                .position(|(anchor, declaration)| {
                    *anchor != HomeCapabilityDeclarationAnchorV1::from_declaration(declaration)
                })
                .unwrap_or(environment.declaration_anchors.len());
            return Err(HomeAbiIssueV1::DeclarationAnchorMismatch { index });
        }

        let relation_batch_brand = environment.relation_batch_brand();
        let schema = environment.schema;
        let home_abis = catalog
            .declarations()
            .iter()
            .map(|declaration| issue_home_abi(declaration, schema, relation_batch_brand))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok(VerifiedDeclaredInstanceMethodHomeCatalogV1 {
            declarations: catalog,
            relation_batch_brand,
            home_abis,
        })
    }
}

fn issue_home_abi(
    declaration: &VerifiedInstanceMethodDeclarationV1,
    schema: HomeCapabilitySchemaV1,
    relation_batch_brand: HomeRelationBrandV1,
) -> Result<VerifiedHomeAbiV1, HomeAbiIssueV1> {
    let parameters = declaration
        .signature()
        .parameters()
        .iter()
        .map(|value_type| classify_parameter(schema, *value_type))
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let result = classify_result(schema, declaration.signature().result())?;
    Ok(VerifiedHomeAbiV1 {
        resolver_brand: declaration.resolver_brand(),
        nominal_box_type: declaration.nominal_box_type(),
        box_statement_ordinal: declaration.box_statement_ordinal(),
        method_member_ordinal: declaration.method_member_ordinal(),
        relation_batch_brand,
        receiver: HomeDemandV1::Handle,
        parameters,
        result,
    })
}

fn classify_parameter(
    schema: HomeCapabilitySchemaV1,
    value_type: ResolverSemanticValueTypeV1,
) -> Result<HomeDemandV1, HomeAbiIssueV1> {
    match schema {
        HomeCapabilitySchemaV1::I64UnitTrivial => match value_type {
            ResolverSemanticValueTypeV1::I64 | ResolverSemanticValueTypeV1::Unit => {
                Ok(HomeDemandV1::Trivial)
            }
        },
    }
}

fn classify_result(
    schema: HomeCapabilitySchemaV1,
    value_type: ResolverSemanticValueTypeV1,
) -> Result<HomeResultRelationV1, HomeAbiIssueV1> {
    match schema {
        HomeCapabilitySchemaV1::I64UnitTrivial => match value_type {
            ResolverSemanticValueTypeV1::I64 => Ok(HomeResultRelationV1::Trivial),
            ResolverSemanticValueTypeV1::Unit => Ok(HomeResultRelationV1::Unit),
        },
    }
}

#[cfg(test)]
#[path = "home_abi_tests.rs"]
mod tests;
