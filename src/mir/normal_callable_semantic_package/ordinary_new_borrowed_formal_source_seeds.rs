//! Sole original actual classifier product; projections never revisit source.
//! Source Integer evidence and selected transport share the same immutable rows.
use super::*;
#[path = "ordinary_new_borrowed_formal_source_seed_preparation.rs"]
mod preparation;
#[path = "ordinary_new_borrowed_formal_source_sum.rs"]
mod source_sum;
pub(super) use preparation::prepare_original_formal_seeds_v1;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use source_sum::OriginalIntegerBasisV1;
pub(in crate::mir) use source_sum::{SourceValueSumV1, SourceValueAlternativeV1};
pub(in crate::mir) use source_sum::SourceValueClassFactsV1;
pub(in crate::mir::normal_callable_semantic_package) use source_sum::ClosedOriginalCoreSourceV1;
#[path = "ordinary_new_borrowed_formal_new_seeds.rs"]
mod new_seeds;
#[path = "ordinary_new_borrowed_object_producer_source.rs"]
mod object_producer;

use std::rc::Rc;
#[path = "ordinary_new_borrowed_formal_source_object.rs"]
mod source_object;
pub(in crate::mir::normal_callable_semantic_package) use source_object::OriginalFormalObjectBasisV1;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use new_seeds::OriginalNewI64InputsLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use new_seeds::OriginalNewSourceLoanV1;

/// The cached fold shares its exact immutable original New evidence. A
/// detached, erased or substituted row set cannot reuse the cached domain.
#[derive(Debug)]
struct SourceIntegerDomainV1 {
    members: BTreeSet<BindingRefV1>,
    new_calls: Rc<new_seeds::OriginalNewSeedsV1>,
}

#[derive(Debug)]
struct SourceObjectDomainV1 {
    views: BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
    new_calls: Rc<new_seeds::OriginalNewSeedsV1>,
}

#[derive(Debug)]
struct OriginalCallSeedsV1 {
    incoming: BorrowedIncomingCallDraftV1,
    legacy: bool,
    seeds: Box<[OriginalFormalActualSeedV1]>,
}

#[derive(Debug)]
pub(super) struct OriginalFormalSeedProductV1 {
    pub(super) original_uses: Option<BTreeMap<FunctionOwnerIdV1, Rc<OriginalFormalUseSourceV1>>>,
    original_use_owners: BTreeSet<FunctionOwnerIdV1>,
    rows: Box<[OriginalCallSeedsV1]>,
    owners: BTreeSet<FunctionOwnerIdV1>,
    source_owners: BTreeSet<FunctionOwnerIdV1>,
    sum_vetoes: BTreeSet<FunctionOwnerIdV1>,
    source_refusals: SourceIncomingRefusalsV1,
    source_vetoes: BTreeSet<FunctionOwnerIdV1>,
    definition_owners: BTreeSet<FunctionOwnerIdV1>,
    source_integers: Option<SourceIntegerDomainV1>,
    source_i64s: Option<SourceIntegerDomainV1>,
    source_objects: Option<Rc<SourceObjectDomainV1>>,
    source_sums: Option<source_sum::OriginalSourceSumsV1>,
    ordinary_core: BTreeMap<OwnedExprSiteV1, Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>,
    new_calls: Rc<new_seeds::OriginalNewSeedsV1>,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalBorrowedFormalSourceV1
{
    pub(super) inventory: BorrowedIncomingInventoryV1,
    pub(super) array_sources: super::array_source::OriginalArraySourcesV1,
    pub(super) seeds: OriginalFormalSeedProductV1,
}

impl OriginalBorrowedFormalSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn field_bases_available(
        &self,
        class: &str,
    ) -> bool {
        self.seeds.corroborate_original_v1(&self.inventory)
            && self.seeds.new_calls.field_bases_available(class)
    }
    /// Keep the original all-New census separate from the cached Integer domain.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn constructor_i64_inputs_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        constructor: &crate::parser::ConstructorSourceIdV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> Option<OriginalNewI64InputsLoanV1> {
        if !self.seeds.corroborate_original_v1(&self.inventory)
            || new_seeds::stable_constructor_formal_v1(input, site, constructors) != Some(binding)
        {
            return None;
        }
        self.seeds.new_calls.lend_i64_inputs_v1(
            constructor,
            binding,
            constructors,
            &self.seeds.source_i64s.as_ref()?.members,
        )
    }
}

#[derive(Clone, Copy)]
enum FormalDomainProjectionV1<'a> {
    SourceInteger,
    SourceI64,
    SourceObject,
    SelectedTransport {
        owners: &'a BTreeSet<FunctionOwnerIdV1>,
        arrays: &'a super::array_source::ArraySourceRequirementsV1,
    },
}

impl OriginalFormalSeedProductV1 {
    /// Binding selected Stored targets changes no original row/ordered actual.
    /// Equal but newly issued source Rcs, loss and additional rows stay refused.
    pub(super) fn corroborate_original_v1(&self, inventory: &BorrowedIncomingInventoryV1) -> bool {
        if let Some(sums) = &self.source_sums {
            if self.ordinary_core.len() != sums.core_calls.len()
                || self.ordinary_core.iter().any(|(site, call)| {
                    sums.core_calls.get(site).is_none_or(|original| {
                        !Rc::ptr_eq(original, call.operands())
                            || !std::sync::Arc::ptr_eq(original.method(), call.source().source())
                    })
                })
            {
                return false;
            }
        } else if !self.ordinary_core.is_empty() {
            return false;
        }
        if self
            .source_integers
            .iter()
            .chain(self.source_i64s.iter())
            .any(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || self
                .source_objects
                .as_ref()
                .is_some_and(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || self
                .source_sums
                .as_ref()
                .is_some_and(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || !self.new_calls.retained_provider_sources_match()
            || inventory.registered_owners_v1() != &self.owners
            || inventory.source_vetoed_owners_v1() != self.source_vetoes
            || inventory.source_rows_v1().count() != self.rows.len()
            || inventory.source_registered_owners_v1() != &self.source_owners
            || inventory.sum_vetoed_owners_v1() != self.sum_vetoes
            || !inventory.corroborates_refusals_v1(&self.source_refusals)
        {
            return false;
        }
        let mut current = BTreeMap::new();
        for (call, legacy) in inventory.source_rows_with_legacy_v1() {
            if current.insert(call.call.clone(), (call, legacy)).is_some() {
                return false;
            }
        }
        if let Some(sums) = &self.source_sums {
            if sums.static_sources.iter().any(|(site, source)| current.get(site).is_none_or(|(row, _)|
                row.callee != source.callee_owner() || !matches!(&row.source,
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(same) if Rc::ptr_eq(same, source))))
                || sums.return_sites.iter().any(|(owner, sites)| sites.is_empty()
                    || sites.iter().any(|site| site.owner() != *owner)
                    || sites.iter().collect::<BTreeSet<_>>().len() != sites.len()
                    || sums.flows.get(owner).is_none_or(|flow| flow.owner() != *owner || !flow.is_complete())) { return false; }
        }
        self.rows.iter().all(|row| {
            let Some((call, legacy)) = current.get(&row.incoming.call) else {
                return false;
            };
            *legacy == row.legacy
                && call.callee == row.incoming.callee
                && call.arguments == row.incoming.arguments
                && call.source.same_original_source_v1(&row.incoming.source)
                && row.seeds.len() == call.arguments.len()
        })
    }

    pub(super) fn source_value_sum_v1(
        &self,
        inventory: &BorrowedIncomingInventoryV1,
        formal: BindingRefV1,
    ) -> Option<&SourceValueSumV1> {
        self.corroborate_original_v1(inventory).then_some(())?;
        self.source_sums
            .as_ref()?
            .values
            .get(&formal)
            .map(Rc::as_ref)
    }
    fn closed_core_source_v1(
        &self,
        inventory: &BorrowedIncomingInventoryV1,
        site: &OwnedExprSiteV1,
    ) -> Option<&Rc<ClosedOriginalCoreSourceV1>> {
        self.corroborate_original_v1(inventory).then_some(())?;
        self.source_sums.as_ref()?.core_calls.get(site)
    }

    pub(super) fn candidate_integer_agreement_v1(
        &self,
        inventory: &BorrowedIncomingInventoryV1,
        formal: BindingRefV1,
    ) -> bool {
        self.corroborate_original_v1(inventory)
            && self
                .source_integers
                .as_ref()
                .is_some_and(|domain| domain.members.contains(&formal))
    }

    pub(super) fn candidate_i64_source_agreement_v1(
        &self,
        inventory: &BorrowedIncomingInventoryV1,
        formal: BindingRefV1,
    ) -> bool {
        self.corroborate_original_v1(inventory)
            && self
                .source_i64s
                .as_ref()
                .is_some_and(|domain| domain.members.contains(&formal))
    }

    pub(super) fn selected_views_v1(
        &self,
        arrays: &super::array_source::ArraySourceRequirementsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        owners: &BTreeSet<FunctionOwnerIdV1>,
        inventory: &BorrowedIncomingInventoryV1,
    ) -> Result<
        (
            BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
            BTreeSet<BindingRefV1>,
            BTreeSet<BindingRefV1>,
        ),
        String,
    > {
        self.project_v1(
            batch,
            constructors,
            contracts,
            inventory,
            FormalDomainProjectionV1::SelectedTransport { owners, arrays },
        )
    }

    fn project_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        inventory: &BorrowedIncomingInventoryV1,
        projection: FormalDomainProjectionV1<'_>,
    ) -> Result<
        (
            BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
            BTreeSet<BindingRefV1>,
            BTreeSet<BindingRefV1>,
        ),
        String,
    > {
        if !self.corroborate_original_v1(inventory) {
            return Err(freeze("borrowed-view/original-seed-identity"));
        }
        let vetoes = match projection {
            FormalDomainProjectionV1::SourceInteger
            | FormalDomainProjectionV1::SourceI64
            | FormalDomainProjectionV1::SourceObject => inventory.source_vetoed_owners_v1(),
            FormalDomainProjectionV1::SelectedTransport { .. } => inventory.vetoed_owners(),
        };
        let mut seeds: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = BTreeMap::new();
        let mut declared = BTreeSet::new();
        for contract in contracts
            .iter()
            .filter(|row| self.definition_owners.contains(&row.owner))
        {
            let clean_declaration = inventory
                .project_declaration_v1(contract.owner)
                .is_ok_and(|rows| rows.is_empty());
            for formal in &contract.parameters {
                let declared_constraint = clean_declaration
                    && matches!(
                        formal.kind,
                        CallableParameterContractKindV1::DeclaredObject(_)
                    );
                if formal.kind.is_ordinary_borrowed_handle()
                    && vetoes.contains(&contract.owner)
                    && !declared_constraint
                {
                    seeds
                        .entry(formal.binding)
                        .or_default()
                        .push(FormalActualSeedV1::Conflict);
                }
                if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
                    declared.insert(formal.binding);
                    seeds
                        .entry(formal.binding)
                        .or_default()
                        .push(FormalActualSeedV1::Class(class.clone()));
                }
            }
        }
        for row in self.rows.iter().filter(|row| row.legacy) {
            let caller = row.incoming.call.owner();
            for ((_, site, formal), raw) in row.incoming.arguments.iter().zip(row.seeds.iter()) {
                let seed = match projection {
                    FormalDomainProjectionV1::SourceInteger
                    | FormalDomainProjectionV1::SourceI64 => raw.project_source_integer_v1(
                        matches!(projection, FormalDomainProjectionV1::SourceI64),
                    ),
                    FormalDomainProjectionV1::SourceObject => raw.project_source_object_v1(),
                    FormalDomainProjectionV1::SelectedTransport { owners, arrays } => {
                        match arrays.get(&OwnedExprSiteV1::new(caller, site.clone())) {
                            Some(source) => match &source.requirement {
                                Ok(source) => FormalActualSeedV1::Class(source.class().into()),
                                Err(_) => FormalActualSeedV1::Conflict,
                            },
                            None => raw.project_selected_v1(caller, Some(owners)),
                        }
                    }
                };
                seeds.entry(*formal).or_default().push(seed);
            }
        }
        if matches!(
            projection,
            FormalDomainProjectionV1::SourceInteger
                | FormalDomainProjectionV1::SourceI64
                | FormalDomainProjectionV1::SourceObject
        ) {
            self.new_calls.append_source_seeds(
                constructors,
                &mut seeds,
                matches!(projection, FormalDomainProjectionV1::SourceI64),
                matches!(projection, FormalDomainProjectionV1::SourceObject),
            );
        }
        let (mut objects, integers) =
            value_domain::resolve_formal_domains_v1(&seeds, batch, constructors);
        let mut mismatches = BTreeSet::new();
        for formal in &declared {
            if let Some(view) = objects.get_mut(formal) {
                view.declared = true;
            } else {
                mismatches.insert(*formal);
            }
        }
        // The early projection lends Integer only; no Object source permission.
        if matches!(
            projection,
            FormalDomainProjectionV1::SourceInteger | FormalDomainProjectionV1::SourceI64
        ) {
            objects.clear();
        }
        Ok((objects, integers, mismatches))
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// One original request selects the callee's existing parameter/Home walk;
    /// it lends no entry values, completed inputs or result permission.
    pub(in crate::mir::normal_callable_semantic_package) fn original_returned_field_input_request_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<OwnedExprSiteV1>, String> {
        let mut rows = self.original_formal_seeds.rows.iter().filter(|row| {
            row.legacy
                && row.incoming.callee == owner
                && row
                    .seeds
                    .iter()
                    .any(|seed| matches!(seed, OriginalFormalActualSeedV1::ReturnedField(_)))
        });
        let Some(first) = rows.next() else {
            return Ok(None);
        };
        if !self
            .original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
        {
            return Err(freeze("returned-field/original-incoming-census-drift"));
        }
        for row in std::iter::once(first).chain(rows) {
            let target = row.incoming.source.require_instance()?;
            if target.callee_owner() != owner || target.call_site() != &row.incoming.call
                || row.incoming.source.as_loan()?.declaration_mode()
                    != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
                return Err(freeze("returned-field/input-request-identity"));
            }
            for ((ordinal, site, formal), seed) in
                row.incoming.arguments.iter().zip(row.seeds.iter())
            {
                if let OriginalFormalActualSeedV1::ReturnedField(source) = seed {
                    if formal.owner() != owner
                        || target.argument_sites().get(*ordinal as usize) != Some(site)
                        || source.read().owner() != row.incoming.call.owner()
                        || source.read().site() != site
                    {
                        return Err(freeze("returned-field/input-request-argument"));
                    }
                }
            }
        }
        Ok(Some(first.incoming.call.clone()))
    }
}

#[cfg(test)]
impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_replaced_original_new_evidence_for_test<
        R,
    >(
        &mut self,
        erase_rows: bool,
        execute: impl FnOnce(&Self) -> R,
    ) -> R {
        let original = Rc::clone(&self.original_formal_seeds.new_calls);
        self.original_formal_seeds.new_calls =
            Rc::new(original.corrupted_provider_evidence_for_test(erase_rows));
        let result = execute(self);
        self.original_formal_seeds.new_calls = original;
        result
    }
}

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_original_provider_static_source_v1(
        &self,
        provider: &Rc<crate::mir::normal_callable_semantic_package::ProviderStaticArgumentSourceV1>,
    ) -> Result<(), String> {
        if !self
            .original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
        {
            return Err(freeze("provider-static/original-input-product-drift"));
        }
        self.original_formal_seeds
            .new_calls
            .corroborate_provider_source_v1(provider)
    }
}

/// The original inventory lends a conditional result and its exact Forward
/// actual together. Source domain agreement is not runtime liveness or entry.
#[allow(clippy::too_many_arguments)]
pub(super) fn classify_mixed_actual_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    results: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    caller: FunctionOwnerIdV1,
    candidates: &[super::super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    binding: BindingRefV1,
    inventory: &BorrowedIncomingInventoryV1,
    visited: &BTreeSet<BindingRefV1>,
    source_context: Option<&OriginalActualSourceContextV1<'_>>,
) -> Option<OriginalFormalActualSeedV1> {
    let mut rows = inventory.exact_rows().filter_map(|row| {
        row.source
            .instance()
            .and_then(|source| source.mixed_local_source())
            .filter(|source| source.destination() == binding)
    });
    let source = rows.next()?;
    let mut initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let exact = initializers.next().is_some_and(|row| {
        row.initializer_site() == Some(source.call().site())
            && row.declaration_site() == source.declaration()
            && input.function().declaration_binding(row.declaration_site()) == Some(binding)
    });
    if rows.next().is_some() || initializers.next().is_some() || !exact
        || input.owner() != caller || binding.owner() != caller || source.call().owner() != caller
        || input.function().assignment_targets().any(|(_, target)| matches!(target,
            crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding))
        || input.forest().semantic_owners().any(|(_, function)|
            function.assignment_targets().any(|(_, target)| matches!(target,
                crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::UpvarRebind(upvar)
                    if upvar.source() == binding)))
        || results.recheck_mixed_local_roots_v1(source).is_err() {
        return Some(OriginalFormalActualSeedV1::Conflict);
    }
    let Some(argument) = input
        .function()
        .method_call(source.call().site())
        .and_then(|call| call.arguments().get(source.callee_ordinal() as usize))
    else {
        return Some(OriginalFormalActualSeedV1::Conflict);
    };
    if argument.ordinal() != source.callee_ordinal()
        || arg_site_binding(input, argument.site()) != Some(source.support())
    {
        return Some(OriginalFormalActualSeedV1::Conflict);
    }
    let anchor = classify_original_actual_seed_with_seen_v1(
        input,
        batch,
        selected,
        results,
        caller,
        candidates,
        receiver,
        definitions,
        contracts,
        argument.site(),
        None,
        Some(inventory),
        source_context,
        visited,
    );
    Some(OriginalFormalActualSeedV1::Mixed {
        source: Box::new(source.clone()),
        anchor: Box::new(anchor),
    })
}

impl OriginalBorrowedFormalSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_source_v1(
        &self,
    ) -> Result<OriginalNewSourceLoanV1, String> {
        if !self.seeds.corroborate_original_v1(&self.inventory) {
            return Err(freeze("array-read/original-seed-drift"));
        }
        Ok(self.seeds.new_calls.source_loan_v1())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_array_get_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
        String,
    > {
        if !self.seeds.corroborate_original_v1(&self.inventory) {
            return Err(freeze("array-read/original-seed-drift"));
        }
        self.seeds.new_calls.array_get_at_v1(call)
    }
}
impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_argument_array_read_v1(
        &self,
        new_site: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<
        Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>,
        String,
    > {
        if !self
            .original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
        {
            return Err(freeze("array-read/original-seed-drift"));
        }
        self.original_formal_seeds
            .new_calls
            .argument_array_read_v1(new_site, ordinal, site, binding)
    }
}

/// Join only the existing original incoming census, never scan another program.
fn classify_returned_field_seed_v1(
    results: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    inventory: &BorrowedIncomingInventoryV1,
    read: &OwnedExprSiteV1,
) -> Result<Option<OriginalFormalActualSeedV1>, String> {
    let source = results
        .returned_field_source_v1(batch, selected, constructors, read, &mut |site| {
            let mut rows = inventory.exact_rows().filter(|row| &row.call == site);
            let Some(row) = rows.next() else {
                return Ok(None);
            };
            if rows.next().is_some() {
                return Err("returned-field/original-target-duplicate");
            }
            let target = row
                .source
                .require_instance()
                .map_err(|_| "returned-field/original-target-kind")?;
            if target.call_site() != site || target.callee_owner() != row.callee {
                return Err("returned-field/original-target-identity");
            }
            Ok(Some(target))
        })
        .map_err(freeze)?;
    Ok(source.map(|source| OriginalFormalActualSeedV1::ReturnedField(Box::new(source))))
}

impl PreparedBorrowedFormalIngressV1 {
    /// Selection only; retained source validation remains with the lender.
    pub(in crate::mir::normal_callable_semantic_package) fn returned_field_argument_selected_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> bool {
        self.original_formal_seeds
            .rows
            .iter()
            .filter(|row| row.legacy)
            .any(|row| {
                &row.incoming.call == call
                    && row
                        .seeds
                        .iter()
                        .any(|seed| matches!(seed, OriginalFormalActualSeedV1::ReturnedField(_)))
            })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn original_returned_field_actual_source_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
    ) -> Result<Option<&super::super::super::result_class_claim::ReturnedFieldSourceLoanV1>, String>
    {
        if !self
            .original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
        {
            return Err(freeze("returned-field/original-incoming-census-drift"));
        }
        let mut rows = self
            .original_formal_seeds
            .rows
            .iter()
            .filter(|row| row.legacy && &row.incoming.call == call);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() {
            return Err(freeze("returned-field/original-call-duplicate"));
        }
        for ((actual_ordinal, site, _), seed) in row.incoming.arguments.iter().zip(row.seeds.iter())
        {
            if *actual_ordinal != ordinal {
                continue;
            }
            let OriginalFormalActualSeedV1::ReturnedField(source) = seed else {
                return Ok(None);
            };
            if source.read().owner() != call.owner() || source.read().site() != site {
                return Err(freeze("returned-field/original-actual-drift"));
            }
            return Ok(Some(source));
        }
        Ok(None)
    }
}

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_formal_uses_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<&Rc<OriginalFormalUseSourceV1>, String> {
        if !self
            .original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
        {
            return Err(freeze("borrowed-use/original-census-drift"));
        }
        let originals = self
            .original_formal_seeds
            .original_uses
            .as_ref()
            .ok_or_else(|| freeze("borrowed-use/original-source-unjoined"))?;
        if originals.keys().copied().collect::<BTreeSet<_>>()
            != self.original_formal_seeds.original_use_owners
            || !self
                .source_incoming
                .source_registered_owners_v1()
                .contains(&owner)
        {
            return Err(freeze("borrowed-use/original-owner-coverage"));
        }
        originals
            .get(&owner)
            .ok_or_else(|| freeze("borrowed-use/original-owner-missing"))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_formal_read_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> Result<OriginalFormalReadLoanV1, String> {
        self.original_formal_uses_v1(owner)?
            .lend_read(site, binding, formal)
            .ok_or_else(|| freeze("borrowed-use/original-read-unproved"))
    }
}
