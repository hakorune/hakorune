//! Request-local projection of verified Brand constructor relations.
//!
//! Resolved semantics remains the sole issuer. This projection only retains
//! exact owner/site coverage for later lowering consumers; it owns no AST,
//! route selection, or physical value.

use std::collections::{BTreeMap, BTreeSet};

use crate::analysis::brand_program_declaration_catalog::BrandDeclarationSourceIdV1;
use crate::mir::resolved_semantics::{
    BrandCallSourceRelationKindV1, FunctionOwnerIdV1, SourceExprSiteV1, SourceNodeSiteV1,
    VerifiedBrandCallSourceRelationV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProjectedBrandConstructorV1 {
    owner: FunctionOwnerIdV1,
    declaration: BrandDeclarationSourceIdV1,
    name: Box<str>,
    underlying_type: Box<str>,
    call_site: SourceNodeSiteV1,
    operand_site: SourceNodeSiteV1,
}

impl ProjectedBrandConstructorV1 {
    fn from_verified(row: &VerifiedBrandCallSourceRelationV1) -> Self {
        Self {
            owner: row.owner(),
            declaration: row.declaration(),
            name: row.name().into(),
            underlying_type: row.underlying_type().into(),
            call_site: row.call_site().node().clone(),
            operand_site: row.operand_site().node().clone(),
        }
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn declaration(&self) -> BrandDeclarationSourceIdV1 {
        self.declaration
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn underlying_type(&self) -> &str {
        &self.underlying_type
    }

    pub(super) fn call_site(&self) -> &SourceNodeSiteV1 {
        &self.call_site
    }

    pub(super) fn operand_site(&self) -> &SourceNodeSiteV1 {
        &self.operand_site
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BrandConstructorDispositionRefV1<'row> {
    Constructor(&'row ProjectedBrandConstructorV1),
    NonBrand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BrandConstructorProjectionErrorV1 {
    MissingExpressionSite(SourceNodeSiteV1),
    ForeignOwner(SourceNodeSiteV1),
    RelationOutsideExpressionInventory(SourceNodeSiteV1),
    DuplicateConstructorSite(SourceNodeSiteV1),
}

#[derive(Debug)]
pub(super) struct BrandConstructorLoweringProjectionV1 {
    owner: FunctionOwnerIdV1,
    expression_sites: BTreeSet<SourceNodeSiteV1>,
    constructors: BTreeMap<SourceNodeSiteV1, ProjectedBrandConstructorV1>,
}

impl BrandConstructorLoweringProjectionV1 {
    pub(super) fn from_verified_owner<'source>(
        owner: FunctionOwnerIdV1,
        expression_sites: impl Iterator<Item = &'source SourceExprSiteV1>,
        relations: impl Iterator<
            Item = (
                &'source SourceExprSiteV1,
                &'source VerifiedBrandCallSourceRelationV1,
            ),
        >,
    ) -> Result<Self, BrandConstructorProjectionErrorV1> {
        let expression_sites = expression_sites
            .map(|site| site.node().clone())
            .collect::<BTreeSet<_>>();
        let constructors = relations
            .filter(|(_, row)| row.kind() == BrandCallSourceRelationKindV1::Constructor)
            .map(|(_, row)| ProjectedBrandConstructorV1::from_verified(row))
            .collect();
        Self::seal(owner, expression_sites, constructors)
    }

    fn seal(
        owner: FunctionOwnerIdV1,
        expression_sites: BTreeSet<SourceNodeSiteV1>,
        rows: Vec<ProjectedBrandConstructorV1>,
    ) -> Result<Self, BrandConstructorProjectionErrorV1> {
        let mut constructors = BTreeMap::new();
        for row in rows {
            let site = row.call_site.clone();
            if row.owner != owner {
                return Err(BrandConstructorProjectionErrorV1::ForeignOwner(site));
            }
            if !expression_sites.contains(&site) || !expression_sites.contains(&row.operand_site) {
                return Err(
                    BrandConstructorProjectionErrorV1::RelationOutsideExpressionInventory(site),
                );
            }
            if constructors.insert(site.clone(), row).is_some() {
                return Err(BrandConstructorProjectionErrorV1::DuplicateConstructorSite(
                    site,
                ));
            }
        }
        Ok(Self {
            owner,
            expression_sites,
            constructors,
        })
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn disposition(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<BrandConstructorDispositionRefV1<'_>, BrandConstructorProjectionErrorV1> {
        if !self.expression_sites.contains(site) {
            return Err(BrandConstructorProjectionErrorV1::MissingExpressionSite(
                site.clone(),
            ));
        }
        Ok(match self.constructors.get(site) {
            Some(row) => BrandConstructorDispositionRefV1::Constructor(row),
            None => BrandConstructorDispositionRefV1::NonBrand,
        })
    }

    pub(super) fn constructor_count(&self) -> usize {
        self.constructors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1,
    };

    fn owner() -> FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap()
    }

    fn site(index: u32) -> SourceNodeSiteV1 {
        SourcePathV1::function_body()
            .child(SourcePathSegmentV1::Body(index))
            .node()
    }

    fn row(owner: FunctionOwnerIdV1, call: u32, operand: u32) -> ProjectedBrandConstructorV1 {
        ProjectedBrandConstructorV1 {
            owner,
            declaration: BrandDeclarationSourceIdV1::from_program_item_ordinal(3).unwrap(),
            name: "BlockId".into(),
            underlying_type: "i64".into(),
            call_site: site(call),
            operand_site: site(operand),
        }
    }

    #[test]
    fn exact_sites_distinguish_constructor_and_non_brand() {
        let owner = owner();
        let projection = BrandConstructorLoweringProjectionV1::seal(
            owner,
            [site(1), site(2), site(3)].into_iter().collect(),
            vec![row(owner, 1, 2)],
        )
        .unwrap();

        let BrandConstructorDispositionRefV1::Constructor(constructor) =
            projection.disposition(&site(1)).unwrap()
        else {
            panic!("exact constructor site must retain its row")
        };
        assert_eq!(constructor.owner(), owner);
        assert_eq!(constructor.declaration().program_item_ordinal(), 3);
        assert_eq!(constructor.name(), "BlockId");
        assert_eq!(constructor.underlying_type(), "i64");
        assert_eq!(constructor.call_site(), &site(1));
        assert_eq!(constructor.operand_site(), &site(2));
        assert_eq!(
            projection.disposition(&site(3)).unwrap(),
            BrandConstructorDispositionRefV1::NonBrand
        );
    }

    #[test]
    fn missing_foreign_and_outside_sites_reject() {
        let owner = owner();
        let covered = [site(1), site(2)].into_iter().collect::<BTreeSet<_>>();
        let projection = BrandConstructorLoweringProjectionV1::seal(
            owner,
            covered.clone(),
            vec![row(owner, 1, 2)],
        )
        .unwrap();
        assert!(matches!(
            projection.disposition(&site(8)),
            Err(BrandConstructorProjectionErrorV1::MissingExpressionSite(_))
        ));
        assert!(matches!(
            BrandConstructorLoweringProjectionV1::seal(
                owner,
                covered.clone(),
                vec![row(self::owner(), 1, 2)]
            ),
            Err(BrandConstructorProjectionErrorV1::ForeignOwner(_))
        ));
        assert!(matches!(
            BrandConstructorLoweringProjectionV1::seal(owner, covered, vec![row(owner, 1, 7)]),
            Err(BrandConstructorProjectionErrorV1::RelationOutsideExpressionInventory(_))
        ));
    }
}
