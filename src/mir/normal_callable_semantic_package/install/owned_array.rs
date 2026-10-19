//! A scoped borrow of the installed owner's existing Array recipe authority.
//! No result-cohort clone, source inventory or executable receipt is issued.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;

#[derive(Clone, Copy)]
pub(crate) struct OwnedArrayLifecycleSourceLoanV1<'package> {
    ledger: &'package OrdinaryNewClaimLedgerV1,
    results: &'package VerifiedCallableResultContractCohortV1,
}
impl<'package> NormalCallableSemanticPackagePortV1<'package> {
    pub(crate) fn owned_array_lifecycle_source_v1(
        &self,
    ) -> OwnedArrayLifecycleSourceLoanV1<'package> {
        OwnedArrayLifecycleSourceLoanV1 {
            ledger: self.installed.ordinary_new_claim_ledger.as_ref(),
            results: &self.installed.result_contracts,
        }
    }
}
impl OwnedArrayLifecycleSourceLoanV1<'_> {
    pub(crate) fn require_same_ledger_v1(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        if !std::ptr::eq(self.ledger, ledger) {
            return Err("[freeze:contract][owned-array/foreign-source-scope]".into());
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recipes_v1(
        &self,
    ) -> Result<Vec<Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>>, String>
    {
        self.ledger.owned_array_lifecycle_recipes_v1(self.results)
    }
}

/// The same retained holder recipe paired with its original initializer relation.
/// This prepares construction only; it does not certify Append/Get/publication.
#[derive(Debug)]
pub(crate) struct PreparedOwnedArrayConstructionV1 {
    pub(in crate::mir::normal_callable_semantic_package) recipe:
        Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>,
    pub(in crate::mir::normal_callable_semantic_package) relation:
        crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
}
impl PreparedOwnedArrayConstructionV1 {
    pub(crate) fn site(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            self.recipe.owner(),
            self.recipe.construction().clone(),
        )
    }
    pub(crate) fn descriptor(&self) -> crate::mir::instruction::OwnedArraySlotDescriptorV1 {
        self.recipe.slot_descriptor_v1()
    }
}
impl OwnedArrayLifecycleSourceLoanV1<'_> {
    pub(crate) fn prepare_construction_v1(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        relation: crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
    ) -> Result<Option<PreparedOwnedArrayConstructionV1>, String> {
        let mut found = None;
        for recipe in self.recipes_v1()? {
            if recipe.owner() != site.owner() || recipe.construction() != site.site() {
                continue;
            }
            if found.is_some()
                || recipe.holder() != relation.binding()
                || relation.initializer_site() != Some(site.site())
                || !matches!(
                    relation.declaration_site(),
                    crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
                )
            {
                return Err("[freeze:contract][owned-array/construction-source-drift]".into());
            }
            recipe.initializer_prefix_v1()?;
            found = Some(recipe);
        }
        Ok(found.map(|recipe| PreparedOwnedArrayConstructionV1 { recipe, relation }))
    }
}

/// Exact original Get obligation over the SAME source-owned lifecycle recipe.
/// It grants no live slot, received Home or invented lexical binding.
#[derive(Debug, Clone)]
pub(crate) struct OwnedArrayReadSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) recipe:
        Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>,
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
}
impl PartialEq for OwnedArrayReadSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.recipe, &other.recipe) && self.site == other.site
    }
}
impl Eq for OwnedArrayReadSourceLoanV1 {}
impl OwnedArrayReadSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_original_recipe_v1(
        recipe: Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>,
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<Self, String> {
        if recipe.original_read_v1(&site).is_none() { return Err("[freeze:contract][owned-array/original-read-missing]".into()); }
        recipe.original_read_integer_v1(&site)?;
        Ok(Self { recipe, site })
    }
    pub(crate) fn site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn holder(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.recipe.holder()
    }
    pub(crate) fn class(&self) -> &str {
        self.recipe.element_class_v1()
    }
    pub(crate) fn descriptor(&self) -> crate::mir::instruction::OwnedArraySlotDescriptorV1 {
        self.recipe.slot_descriptor_v1()
    }
    pub(crate) fn index(&self) -> Result<i64, String> {
        self.recipe.original_read_integer_v1(&self.site)
    }
    pub(crate) fn receiver(
        &self,
    ) -> Result<&crate::mir::resolved_semantics::SourceExprSiteV1, String> {
        self.recipe.original_read_receiver_v1(&self.site)
    }
}
impl OwnedArrayLifecycleSourceLoanV1<'_> {
    pub(crate) fn prepare_read_v1(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<Option<OwnedArrayReadSourceLoanV1>, String> {
        let mut found = None;
        for recipe in self.recipes_v1()? {
            if recipe.original_read_v1(site).is_none() {
                continue;
            }
            if found.is_some() || recipe.owner() != site.owner() {
                return Err("[freeze:contract][owned-array/read-source-drift]".into());
            }
            recipe.original_read_integer_v1(site)?;
            found = Some(OwnedArrayReadSourceLoanV1 {
                recipe,
                site: site.clone(),
            });
        }
        Ok(found)
    }
}

/// One original discarded Append; the same holder recipe owns all operations.
#[derive(Debug, Clone)]
pub(crate) struct OwnedArrayAppendSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) recipe:
        Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>,
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
}
impl OwnedArrayAppendSourceLoanV1 {
    pub(crate) fn matches_original_v1(
        &self, holder: &super::super::NamedArrayHolderSourceLoanV1,
        candidate: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> bool {
        *holder == self.recipe.holder_source_loan_v1() && *candidate == self.candidate()
    }
    pub(crate) fn site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 { &self.site }
    pub(crate) fn holder(&self) -> crate::mir::resolved_semantics::BindingRefV1 { self.recipe.holder() }
    pub(crate) fn descriptor(&self) -> crate::mir::instruction::OwnedArraySlotDescriptorV1 { self.recipe.slot_descriptor_v1() }
    pub(crate) fn receiver(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.recipe.original_append_v1(&self.site).expect("original Append loan").0
    }
    pub(crate) fn candidate(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(self.site.owner(),
            self.recipe.original_append_v1(&self.site).expect("original Append loan").1.clone())
    }
    pub(crate) fn null_candidate(&self) -> bool {
        self.recipe.original_append_v1(&self.site).expect("original Append loan").2
    }
    pub(crate) fn prior_homes(&self) -> Result<&[LiveHomeV1], String> {
        self.recipe.append_prior_homes_v1(&self.site)
    }
    pub(crate) fn check_candidate_v1(&self, row: &super::super::LexicalInstanceCallDispositionRowV1) -> Result<(), String> {
        self.recipe.check_append_candidate_v1(&self.site, row)
    }
}
impl OwnedArrayLifecycleSourceLoanV1<'_> {
    pub(crate) fn prepare_append_v1(
        &self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<Option<OwnedArrayAppendSourceLoanV1>, String> {
        let mut found = None;
        for recipe in self.recipes_v1()? {
            if recipe.original_append_v1(site).is_none() { continue; }
            if found.is_some() { return Err("[freeze:contract][owned-array/append-source-duplicate]".into()); }
            recipe.append_prior_homes_v1(site)?;
            found = Some(OwnedArrayAppendSourceLoanV1 { recipe, site: site.clone() });
        }
        Ok(found)
    }
}
