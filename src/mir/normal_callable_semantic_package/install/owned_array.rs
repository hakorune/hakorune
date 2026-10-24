//! A scoped borrow of the installed owner's existing Array recipe authority.
//! No result-cohort clone, source inventory or executable receipt is issued.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

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
    authority: ArrayConstructionAuthorityV1,
    relation: crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
}
#[derive(Debug)]
enum ArrayConstructionAuthorityV1 {
    Typed(Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>),
    DynamicCarrier(super::super::NamedArrayHolderSourceLoanV1),
}
impl PreparedOwnedArrayConstructionV1 {
    pub(crate) fn site(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            self.relation.binding().owner(),
            self.relation
                .initializer_site()
                .expect("prepared Array initializer")
                .clone(),
        )
    }
    pub(crate) fn descriptor(&self) -> crate::mir::instruction::OwnedArraySlotDescriptorV1 {
        match &self.authority {
            ArrayConstructionAuthorityV1::Typed(recipe) => recipe.slot_descriptor_v1(),
            ArrayConstructionAuthorityV1::DynamicCarrier(_) => {
                unreachable!("opaque carrier Array has no object descriptor")
            }
        }
    }
    pub(crate) fn relation(
        &self,
    ) -> &crate::mir::resolved_semantics::ResolvedInitializerRelationV1 {
        &self.relation
    }
    pub(crate) fn has_typed_recipe(&self) -> bool {
        matches!(&self.authority, ArrayConstructionAuthorityV1::Typed(_))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn typed_recipe(
        &self,
    ) -> Option<&Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>> {
        match &self.authority {
            ArrayConstructionAuthorityV1::Typed(recipe) => Some(recipe),
            ArrayConstructionAuthorityV1::DynamicCarrier(_) => None,
        }
    }
    pub(crate) fn dynamic_carrier_source(
        &self,
    ) -> Option<&super::super::NamedArrayHolderSourceLoanV1> {
        match &self.authority {
            ArrayConstructionAuthorityV1::Typed(_) => None,
            ArrayConstructionAuthorityV1::DynamicCarrier(source) => Some(source),
        }
    }
    pub(crate) fn for_dynamic_carrier_v1(
        source: super::super::NamedArrayHolderSourceLoanV1,
        relation: crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
    ) -> Result<Self, String> {
        if !source.is_dynamic_carrier_append_only_v1()
            || source.binding() != relation.binding()
            || relation.initializer_site() != Some(source.construction_site())
            || !matches!(
                relation.declaration_site(),
                crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
            )
        {
            return Err("[freeze:contract][carrier-array/construction-source-drift]".into());
        }
        Ok(Self {
            authority: ArrayConstructionAuthorityV1::DynamicCarrier(source),
            relation,
        })
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
        Ok(found.map(|recipe| PreparedOwnedArrayConstructionV1 {
            authority: ArrayConstructionAuthorityV1::Typed(recipe),
            relation,
        }))
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
        if recipe.original_read_v1(&site).is_none() {
            return Err("[freeze:contract][owned-array/original-read-missing]".into());
        }
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

#[derive(Debug, Clone)]
enum OwnedArrayAppendAuthorityV1 {
    Typed(Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>),
    DynamicCarrier {
        observation: crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1,
        receiver: crate::mir::resolved_semantics::SourceExprSiteV1,
    },
}

/// One original discarded Append, owned by its typed recipe or exact RootHome observation.
#[derive(Debug, Clone)]
pub(crate) struct OwnedArrayAppendSourceLoanV1 {
    authority: OwnedArrayAppendAuthorityV1,
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
}
impl OwnedArrayAppendSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn typed_recipe(
        &self,
    ) -> Option<&Rc<super::super::ordinary_new_coseal::ArrayHolderLifecycleRecipeV1>> {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => Some(recipe),
            OwnedArrayAppendAuthorityV1::DynamicCarrier { .. } => None,
        }
    }
    pub(crate) fn dynamic_carrier_observation(
        &self,
    ) -> Option<&crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1> {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(_) => None,
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => Some(observation),
        }
    }
    pub(crate) fn dynamic_carrier_source(
        &self,
    ) -> Option<super::super::NamedArrayHolderSourceLoanV1> {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => {
                let source = recipe.holder_source_loan_v1();
                source.is_dynamic_carrier_append_only_v1().then_some(source)
            }
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    source, ..
                } = observation else { return None; };
                Some(source.clone())
            }
        }
    }
    pub(crate) fn matches_original_v1(
        &self,
        holder: &super::super::NamedArrayHolderSourceLoanV1,
        candidate: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> bool {
        let holder_matches = match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => {
                *holder == recipe.holder_source_loan_v1()
            }
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                matches!(observation,
                    crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                        source, ..
                    } if source == holder)
            }
        };
        holder_matches && *candidate == self.candidate()
    }
    pub(crate) fn site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn holder(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => recipe.holder(),
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    source, ..
                } = observation else { unreachable!("Dynamic carrier Append observation") };
                source.binding()
            }
        }
    }
    pub(crate) fn descriptor(
        &self,
    ) -> Option<crate::mir::instruction::OwnedArraySlotDescriptorV1> {
        self.typed_recipe().map(|recipe| recipe.slot_descriptor_v1())
    }
    pub(crate) fn receiver(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => recipe
                .original_append_v1(&self.site)
                .expect("original Append loan")
                .0,
            OwnedArrayAppendAuthorityV1::DynamicCarrier { receiver, .. } => receiver,
        }
    }
    pub(crate) fn candidate(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => {
                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    self.site.owner(),
                    recipe
                        .original_append_v1(&self.site)
                        .expect("original Append loan")
                        .1
                        .clone(),
                )
            }
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    candidate, ..
                } = observation else { unreachable!("Dynamic carrier Append observation") };
                candidate.clone()
            }
        }
    }
    pub(crate) fn null_candidate(&self) -> bool {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => recipe
                .original_append_v1(&self.site)
                .expect("original Append loan")
                .2,
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    source, call, ..
                } = observation else { unreachable!("Dynamic carrier Append observation") };
                source.append_candidate_is_null_v1(call) == Some(true)
            }
        }
    }
    pub(crate) fn prior_homes(&self) -> Result<&[LiveHomeV1], String> {
        match &self.authority {
            OwnedArrayAppendAuthorityV1::Typed(recipe) => recipe.append_prior_homes_v1(&self.site),
            OwnedArrayAppendAuthorityV1::DynamicCarrier { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    prior_homes, ..
                } = observation else { return Err("owned-array/carrier-append-purpose".into()); };
                Ok(prior_homes)
            }
        }
    }
    pub(crate) fn check_candidate_v1(
        &self,
        row: &super::super::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        match self.typed_recipe() {
            Some(recipe) => recipe.check_append_candidate_v1(&self.site, row),
            None => Err("owned-array/dynamic-carrier-has-no-lexical-candidate".into()),
        }
    }
}
impl OwnedArrayLifecycleSourceLoanV1<'_> {
    pub(crate) fn prepare_append_v1(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<Option<OwnedArrayAppendSourceLoanV1>, String> {
        let mut found = None;
        for recipe in self.recipes_v1()? {
            if recipe.original_append_v1(site).is_none() {
                continue;
            }
            if found.is_some() {
                return Err("[freeze:contract][owned-array/append-source-duplicate]".into());
            }
            recipe.append_prior_homes_v1(site)?;
            found = Some(OwnedArrayAppendSourceLoanV1 {
                authority: OwnedArrayAppendAuthorityV1::Typed(recipe),
                site: site.clone(),
            });
        }
        let dynamic = self.ledger.named_dynamic_carrier_array_append_v1(site)?;
        if found.is_some() && dynamic.is_some() {
            return Err("[freeze:contract][owned-array/append-source-duplicate]".into());
        }
        if let Some(observation) = dynamic {
            let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                source, call, candidate, availability, ..
            } = &observation else {
                return Err("[freeze:contract][owned-array/carrier-append-purpose]".into());
            };
            let receiver = source
                .append_receiver_v1(call)
                .ok_or("[freeze:contract][owned-array/carrier-append-receiver-missing]")?;
            if call != site
                || source.binding().owner() != site.owner()
                || source.append_candidate(call).as_ref() != Some(candidate)
                || !source.is_dynamic_carrier_append_only_v1()
            {
                return Err("[freeze:contract][owned-array/carrier-append-source-drift]".into());
            }
            availability
                .as_ref()
                .map_err(|_| "[freeze:contract][owned-array/carrier-append-unavailable]")?;
            found = Some(OwnedArrayAppendSourceLoanV1 {
                authority: OwnedArrayAppendAuthorityV1::DynamicCarrier {
                    observation,
                    receiver,
                },
                site: site.clone(),
            });
        }
        Ok(found)
    }
}
