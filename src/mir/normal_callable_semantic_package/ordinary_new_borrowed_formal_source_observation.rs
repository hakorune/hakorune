//! Original Static observations retained beside borrowed-formal source facts.
use super::*;

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn stored_local_new_root_v1(
        &self, target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<(u32, &std::rc::Rc<super::super::super::LocalNewRootSourceV1>)>, String> {
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("stored-child/original-incoming-census-drift"));
        }
        let mut originals = self.source_incoming.exact_rows().filter(|row| &row.call == target.call_site());
        let original = originals.next().ok_or_else(|| freeze("stored-child/original-incoming-missing"))?;
        if originals.next().is_some() || original.source.require_instance()? != target {
            return Err(freeze("stored-child/original-incoming-drift"));
        }
        original.source.local_new_root_source_v1(target)
    }
    /// Exact retained LOCAL input demand, including source-only/zero-actual rows.
    pub(in crate::mir::normal_callable_semantic_package) fn original_owning_object_input_source_v1<'a>(
        &self, target: &'a LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&'a super::super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>, String> {
        let loan = self.original_owning_object_input_target_v1(target)?;
        if loan.is_some() && !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("nullable-input/original-source-drift"));
        }
        Ok(loan)
    }

    // Exact target checks remain per call. The callee-wide operation below
    // corroborates the immutable whole inventory once before visiting its rows.
    fn original_owning_object_input_target_v1<'a>(
        &self, target: &'a LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&'a super::super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>, String> {
        let mut rows = self.source_incoming.exact_rows().filter(|row| &row.call == target.call_site());
        let Some(row) = rows.next() else {
            return if target.owning_object_local_input_v1().is_some() { Err(freeze("nullable-input/original-missing")) } else { Ok(None) };
        };
        if rows.next().is_some() || row.source.require_instance()? != target || row.callee != target.callee_owner() {
            return Err(freeze("nullable-input/original-target-drift"));
        }
        let Some(loan) = target.owning_object_local_input_v1() else { return Ok(None); };
        if !loan.matches_target(target) {
            return Err(freeze("nullable-input/original-source-drift"));
        }
        Ok(Some(loan))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn original_owning_object_input_callee_v1(
        &self, target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("nullable-input/original-incoming-census-drift"));
        }
        self.original_owning_object_input_target_v1(target)?;
        let mut selected = false;
        for row in self.source_incoming.exact_rows().filter(|row| row.callee == target.callee_owner()) {
            let original = row.source.require_instance()?;
            if original.target() != target.target() || original.target_batch_slot() != target.target_batch_slot() {
                return Err(freeze("nullable-input/callee-identity"));
            }
            selected |= self.original_owning_object_input_target_v1(original)?.is_some();
        }
        Ok(selected)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn erase_array_read_for_test(&mut self, site: &OwnedExprSiteV1) {
        self.array_sources.remove(site);
    }

    /// Original structural membership survives class refusal; this lends no
    /// element type, Normal acquisition, slot, payload or execution permission.
    pub(in crate::mir::normal_callable_semantic_package) fn array_holder_structure_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&std::rc::Rc<crate::mir::resolved_semantics::NamedArrayAppendOnlySourceV1>>, String> {
        let Some(row) = self.array_sources.get(site) else { return Ok(None) };
        let holder = row.holder.as_ref().map_err(Clone::clone)?;
        let current = holder.construction();
        let identity = &row.construction;
        if identity.owner() != current.owner()
            || identity.binding() != current.binding()
            || identity.construction() != current.construction()
            || current.owner() != site.owner()
            || holder.reads().iter().filter(|read| read.call() == site.site()).count() != 1 {
            return Err(freeze("array-source/read-identity"));
        }
        Ok(Some(holder))
    }

    /// Conditional source requirement only, never a live slot or actual carrier.
    pub(in crate::mir::normal_callable_semantic_package) fn array_element_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&std::rc::Rc<super::array_source::ArrayElementSourceRequirementV1>>, String> {
        let Some(row) = self.array_sources.get(site) else { return Ok(None) };
        let holder = self.array_holder_structure_source_v1(site)?
            .ok_or_else(|| freeze("array-source/holder-missing"))?;
        let source = row.requirement.as_ref().map_err(Clone::clone)?;
        if !std::rc::Rc::ptr_eq(holder, source.holder_source_v1()) {
            return Err(freeze("array-source/holder-identity"));
        }
        Ok(Some(source))
    }

    /// Pure original Append membership. Select the exact candidate before
    /// demanding qualification, so an unrelated holder cannot supply or erase
    /// this producer. No input/Normal/recipe/emission query occurs here.
    pub(in crate::mir::normal_callable_semantic_package) fn original_array_producer_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&super::super::super::result_class_claim::ObjectCallSourceLoanV1>, String> {
        let mut found: Option<&super::super::super::result_class_claim::ObjectCallSourceLoanV1> = None;
        for (read, row) in self.array_sources.iter().filter(|(read, _)| read.owner() == target.call_site().owner()) {
            // A failed structural issuer issued no candidate identity. The
            // selected original Get still retains that refusal independently.
            let structural_candidate = row.holder.as_ref().ok().is_some_and(|holder|
                holder.appends().iter().any(|append| append.candidate() == target.call_site().site()));
            let retained_producer = row.requirement.as_ref().ok().is_some_and(|requirement|
                requirement.producers().iter().any(|(site, _)| site == target.call_site()));
            if !structural_candidate && !retained_producer { continue; }
            let holder = self.array_holder_structure_source_v1(read)?
                .ok_or_else(|| freeze("array-producer/original-holder-missing"))?;
            if !structural_candidate { return Err(freeze("array-producer/append-membership-lost")); }
            let requirement = self.array_element_source_v1(read)?
                .ok_or_else(|| freeze("array-producer/original-requirement-missing"))?;
            // Every original Get must retain this same normalized source.
            // A healthy sibling read must not hide a dropped/refused row.
            for original in holder.reads() {
                let site = OwnedExprSiteV1::new(read.owner(), original.call().clone());
                let current = self.array_element_source_v1(&site)?
                    .ok_or_else(|| freeze("array-producer/original-read-missing"))?;
                if !std::rc::Rc::ptr_eq(requirement, current) {
                    return Err(freeze("array-producer/original-read-drift"));
                }
            }
            let mut producers = requirement.producers().iter().filter(|(site, _)| site == target.call_site());
            let (_, producer) = producers.next().ok_or_else(|| freeze("array-producer/source-missing"))?;
            if producers.next().is_some() || producer.source() != target {
                return Err(freeze("array-producer/original-target-drift"));
            }
            if found.is_some_and(|previous| !std::ptr::eq(previous, producer)) {
                return Err(freeze("array-producer/original-source-drift"));
            }
            found = Some(producer);
        }
        if found.is_some() {
            let original = self.object_source_target_at_v1(target.call_site())?
                .ok_or_else(|| freeze("array-producer/original-target-missing"))?;
            if original != target { return Err(freeze("array-producer/original-target-drift")); }
        }
        Ok(found)
    }

    /// Exact original Array argument membership, independent of transport
    /// pruning. A retained refusal remains a refusal; no input/live-read grant.
    pub(in crate::mir::normal_callable_semantic_package) fn original_array_argument_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let mut selected = false;
        for argument in target.argument_sites() {
            selected |= self.array_element_source_v1(
                &OwnedExprSiteV1::new(target.call_site().owner(), argument.clone()),
            )?.is_some();
        }
        if selected {
            let current = self.object_source_target_at_v1(target.call_site())?
                .ok_or_else(|| freeze("array-argument/original-missing"))?;
            if current != target { return Err(freeze("array-argument/original-drift")); }
        }
        Ok(selected)
    }

    /// Original class agreement only; neither header nor field-use permission.
    pub(in crate::mir::normal_callable_semantic_package) fn source_object_view_v1(
        &self,
        formal: BindingRefV1,
    ) -> Result<Option<&BorrowedFormalObjectViewV1>, String> {
        let Some(draft) = self.source_definition_for(formal.owner()) else {
            return Ok(None);
        };
        if draft.origins.get(&formal) != Some(&formal) {
            return Err(freeze("borrowed-view/source-formal-identity"));
        }
        self.source_incoming
            .project_declaration_v1(formal.owner())
            .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-view/source-incoming")))?;
        if self.source_declared_mismatches.contains(&formal) {
            return Err(freeze("borrowed-view/declared-object-class"));
        }
        Ok(self.object_views.get(&formal))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn is_static_terminal_site_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> bool {
        self.source_incoming
            .static_observations()
            .get(site)
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|original| self.is_static_terminal_source_v1(original))
            })
    }

    /// Exact original terminal membership is source-only, not entry permission.
    pub(in crate::mir::normal_callable_semantic_package) fn is_static_terminal_source_v1(
        &self,
        original: &crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    ) -> bool {
        original.is_qualified_i64_source_v1()
            && self.static_terminal_sites.contains(original.call_site())
            && original.argument_sites().len() == 1
            && original.parameters().len() == 1
            && original.parameters()[0].kind == CallableParameterContractKindV1::OpaqueHandle
    }

    /// Source lookup borrows the one original draft map's disjoint partition.
    /// This does not make an excluded owner eligible for executable transport.
    pub(in crate::mir::normal_callable_semantic_package) fn source_definition_for(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&BorrowedFormalUsesDraftV1> {
        self.definitions
            .get(&owner)
            .or_else(|| self.source_only_definitions.get(&owner))
    }

    /// Borrow one original raw Instance target after affine preparation is consumed.
    /// This lookup grants no incoming/domain or executable permission.
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_target_at_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&LexicalInstanceCallSourceTargetV1>, String> {
        let mut rows = self
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == site);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        let target = row.source.require_instance()?;
        if rows.next().is_some()
            || target.call_site() != site
            || target.callee_owner() != row.callee
        {
            return Err(freeze("object-source/target-identity"));
        }
        Ok(Some(target))
    }

    /// Source agreement only; an explicit physical projection is still required.
    pub(in crate::mir::normal_callable_semantic_package) fn formal_integer_agreement(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.definitions.contains_key(&formal.owner()) && self.integer_agreements.contains(&formal)
    }

    /// Complete input agreement independent of outgoing profile/transport permission.
    pub(in crate::mir::normal_callable_semantic_package) fn candidate_integer_agreement(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.original_formal_seeds.candidate_integer_agreement_v1(&self.source_incoming, formal)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn candidate_i64_source_agreement(&self, formal: BindingRefV1) -> bool {
        self.original_formal_seeds.candidate_i64_source_agreement_v1(&self.source_incoming, formal)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn contains_definition_for_test(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.definitions.contains_key(&owner)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn candidate_input_inventory_for_test(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> (usize, bool) {
        (
            self.source_incoming
                .exact_rows()
                .filter(|row| row.callee == owner)
                .count(),
            self.source_incoming.vetoed_owners().contains(&owner),
        )
    }

    /// Read the original source observation without selecting executable actuals.
    pub(in crate::mir::normal_callable_semantic_package) fn static_observation_for_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<
        &Result<
            std::rc::Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>,
            String,
        >,
    >{
        self.source_incoming.static_observations().get(site)
    }
}

/// Retain original ReturnValue sites, without granting a call or entry route.
pub(super) fn static_terminal_sites_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
) -> Result<BTreeSet<OwnedExprSiteV1>, String> {
    let mut sites = BTreeSet::new();
    let mut owners = BTreeSet::new();
    for contract in contracts {
        let is_main = app_main.is_some_and(|main| main.matches_contract(contract));
        let Some(_caller) = crate::mir::normal_callable_semantic_package::qualified_static_call_claim::caller_key_for_function(
            selected, contract.batch_slot, is_main, app_main.map(|main| main.catalog_key()),
        ) else { continue; };
        if !owners.insert(contract.owner) { return Err(freeze("borrowed-static/terminal-source-owner-duplicate")); }
        let original = batch.with_lowering_input(contract.batch_slot, |input| {
            if input.owner() != contract.owner { return Err(freeze("borrowed-static/terminal-source-owner")); }
            Ok(input.body_shape().into_iter().flat_map(|shape| shape.return_value_sites())
                .map(|site| OwnedExprSiteV1::new(input.owner(), site.clone())).collect::<Vec<_>>())
        }).map_err(|_| freeze("borrowed-static/terminal-source-loan"))??;
        sites.extend(original);
    }
    Ok(sites)
}
