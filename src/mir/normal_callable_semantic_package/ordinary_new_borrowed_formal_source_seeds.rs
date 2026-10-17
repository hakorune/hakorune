//! Sole original actual classifier product; projections never revisit source.
//! Source Integer evidence and selected transport share the same immutable rows.
use super::*;
#[path = "ordinary_new_borrowed_formal_source_sum.rs"]
mod source_sum;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use source_sum::{SourceValueSumV1, SourceValueAlternativeV1};
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

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) struct OriginalBorrowedFormalSourceV1 {
    pub(super) inventory: BorrowedIncomingInventoryV1,
    pub(super) array_sources: super::array_source::OriginalArraySourcesV1,
    pub(super) seeds: OriginalFormalSeedProductV1,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) enum OriginalIntegerBasisV1 {
    /// Issued only after exact declaration, ordinal and whole-forest stability checks.
    Declared { binding: BindingRefV1, abi: crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1 },
    Incoming { binding: BindingRefV1, origin: BindingRefV1, current_i64: bool },
}
impl OriginalIntegerBasisV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn binding(self) -> BindingRefV1 {
        match self { Self::Declared { binding, .. } | Self::Incoming { binding, .. } => binding }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn i64_source(self) -> bool {
        match self { Self::Declared { abi, .. } => abi.is_i64(), Self::Incoming { current_i64, .. } => current_i64 }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ready(self, source: &PreparedBorrowedFormalIngressV1) -> bool {
        match self {
            Self::Declared { .. } => true,
            Self::Incoming { origin, current_i64, .. } => source.candidate_integer_agreement(origin)
                && (!current_i64 || source.candidate_i64_source_agreement(origin)),
        }
    }
}

impl OriginalBorrowedFormalSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn field_bases_available(&self, class: &str) -> bool {
        self.seeds.corroborate_original_v1(&self.inventory) && self.seeds.new_calls.field_bases_available(class)
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
            || new_seeds::stable_constructor_formal_v1(input, site, constructors) != Some(binding) { return None; }
        self.seeds.new_calls.lend_i64_inputs_v1(constructor, binding, constructors,
            &self.seeds.source_i64s.as_ref()?.members)
    }
    /// Source agreement plus the original stable use, not entry agreement alone.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn integer_origin_at_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Option<OriginalIntegerBasisV1> {
        if !self.seeds.new_calls.current_provider_sources_match(constructors) { return None; }
        let binding = arg_site_binding(input, site)?;
        if value_domain::declared_integer_seed_v1(input, contracts, binding) {
            let abi = contracts.iter().find(|row| row.owner == input.owner())?.parameters.iter()
                .find(|row| row.binding == binding).and_then(|row| match row.kind {
                    CallableParameterContractKindV1::ExactTrivial(abi) => Some(abi), _ => None,
                })?;
            return Some(OriginalIntegerBasisV1::Declared { binding, abi });
        }
        let origin = definitions.get(&input.owner()).and_then(|draft| draft.origins.get(&binding)).copied()
            .or_else(|| new_seeds::stable_constructor_formal_v1(input, site, constructors))?;
        self.seeds.candidate_integer_agreement_v1(&self.inventory, origin).then_some(OriginalIntegerBasisV1::Incoming { binding, origin, current_i64: self.seeds.candidate_i64_source_agreement_v1(&self.inventory, origin) })
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

/// Actual sites are classified once, before result/receiver eligibility.
/// PendingStored retains its complete original ordered input without a loan.
#[allow(clippy::too_many_arguments)]
pub(super) fn prepare_original_formal_seeds_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    results: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &BTreeMap<u32, Result<Vec<super::super::super::candidate::OrdinaryNewCandidate>, super::super::super::OrdinaryNewCoSealIssueV1>>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    inventory: &BorrowedIncomingInventoryV1,
    guarded: &BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    app_main: Option<AppMainSourceRefV1<'_>>,
    omitted: &BTreeMap<FunctionOwnerIdV1, CanonicalSameModuleCallableKeyV1>,
) -> Result<OriginalFormalSeedProductV1, String> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for (call, legacy) in inventory.source_rows_with_legacy_v1() {
        if !seen.insert(call.call.clone()) {
            return Err(freeze("borrowed-view/duplicate-original-call"));
        }
        let caller = call.call.owner();
        let seeds = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, caller, |context, input| -> Result<Vec<_>, String> {
                let original = input.function().method_call(call.call.site())
                    .ok_or_else(|| freeze("borrowed-view/original-call"))?;
                context.constructor_key_for_call(&call.call, original)?;
                if original.selector() != call.source.target().name()
                    || original.arity() != call.source.target().arity()
                    || original.arguments().iter().map(|argument| argument.site()).ne(call.source.argument_sites().iter())
                { return Err(freeze("borrowed-view/original-call-identity")); }
                let source_context = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                    static_claims, app_main, omitted)?;
                let slot = context.declaration_slot();
                let candidates = slot.and_then(|slot| candidates.get(&slot)).and_then(|rows| rows.as_ref().ok())
                    .map(Vec::as_slice).unwrap_or(&[]);
                let receiver = slot.and_then(|slot| contracts.iter().find(|row| row.owner == caller)
                    .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                    .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                        selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                call.arguments.iter().map(|(ordinal, site, _)| {
                    if let Some(source) = classify_returned_field_seed_v1(
                        results, batch, selected, constructors, inventory,
                        &OwnedExprSiteV1::new(caller, site.clone()),
                    )? { return Ok(source); }
                    if slot.is_none() {
                        if let Some(origin) = new_seeds::stable_constructor_formal_v1(input, site, constructors) {
                            return Ok(OriginalFormalActualSeedV1::Forward { origin, declared_constraint: None });
                        }
                    }
                    Ok(classify_original_actual_seed_in_scope(input, batch, selected, results, caller,
                        candidates, receiver, definitions, contracts, site,
                        guarded.get(&(call.call.clone(), *ordinal)).map(|fact| (fact, &call.call, *ordinal)), Some(inventory), source_context.as_ref()))
                }).collect::<Result<Vec<_>, String>>()
            })??;
        rows.push(OriginalCallSeedsV1 { incoming: call.clone(), legacy, seeds: seeds.into_boxed_slice() });
    }
    let mut new_calls = new_seeds::OriginalNewSeedsV1::default();
    for declaration in batch.declarations() {
        let slot = declaration.batch_slot();
        let root_owner = declaration.owner();
        let local_candidates = candidates.get(&slot).and_then(|rows| rows.as_ref().ok())
            .map(Vec::as_slice).unwrap_or(&[]);
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            selected, batch, entry_home_loans.for_batch_slot(slot), slot,
        );
        let observed = batch.with_lowering_input(slot, |root| {
            for (owner, _) in root.forest().semantic_owners() {
                match batch.with_owner_lowering_input(slot, owner, |input| {
                    let source_context = if owner == root_owner {
                        OriginalActualSourceContextV1::checked(
                            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Declaration { batch_slot: slot },
                            input, batch, selected, contracts, static_claims, app_main, omitted)?
                    } else { None };
                    new_calls.observe(input, batch, constructors, static_claims, selected, array_i64_fields, |site| {
                        classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                            if owner == root_owner { local_candidates } else { &[] },
                            if owner == root_owner { receiver } else { None },
                            definitions, contracts, site, None, Some(inventory), source_context.as_ref())
                    })
                }) {
                    Ok(result) => result?,
                    Err(_) => new_calls.missing_owner(),
                }
            }
            Ok::<(), String>(())
        });
        match observed { Ok(result) => result?, Err(_) => new_calls.missing_owner() }
    }
    let observed = batch.with_normal_program_source_loan(|loan| {
        for constructor in constructors.rows() {
            for (owner, _) in constructor.forest().semantic_owners() {
                match constructor.lowering_input_for_owner(loan.program(), owner) {
                    Ok(input) => {
                    let source_context = if constructor.kind() == crate::parser::ConstructorSourceKindV1::Birth
                        && constructor.forest().roots() == [owner] {
                        OriginalActualSourceContextV1::checked(
                            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Constructor { row: constructor, input },
                            input, batch, selected, contracts, static_claims, app_main, omitted)?
                    } else { None };
                    new_calls.observe(input, batch, constructors, static_claims, selected, array_i64_fields, |site| {
                        if let Some(origin) = new_seeds::stable_constructor_formal_v1(input, site, constructors) {
                            OriginalFormalActualSeedV1::Forward { origin, declared_constraint: None }
                        } else {
                            classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                                &[], None, definitions, contracts, site, None, Some(inventory), source_context.as_ref())
                        }
                    })?; },
                    Err(_) => new_calls.missing_owner(),
                }
            }
        }
        Ok::<(), String>(())
    });
    match observed { Ok(result) => result?, Err(_) => new_calls.missing_owner() }
    let mut product = OriginalFormalSeedProductV1 {
        rows: rows.into_boxed_slice(),
        owners: inventory.registered_owners_v1().clone(),
        source_owners: inventory.source_registered_owners_v1().clone(),
        sum_vetoes: inventory.sum_vetoed_owners_v1(),
        source_refusals: inventory.source_refusals_v1(),
        source_vetoes: inventory.source_vetoed_owners_v1(),
        definition_owners: definitions.keys().copied().collect(),
        source_integers: None,
        source_i64s: None,
        source_objects: None,
        source_sums: None,
        ordinary_core: BTreeMap::new(),
        new_calls: Rc::new(new_calls),
    };
    product.source_integers = Some(SourceIntegerDomainV1 {
        members: product.project_v1(batch, constructors, contracts, inventory,
            FormalDomainProjectionV1::SourceInteger)?.1,
        new_calls: Rc::clone(&product.new_calls),
    });
    product.source_i64s = Some(SourceIntegerDomainV1 {
        members: product.project_v1(batch, constructors, contracts, inventory,
            FormalDomainProjectionV1::SourceI64)?.1,
        new_calls: Rc::clone(&product.new_calls),
    });
    product.source_objects = Some(Rc::new(SourceObjectDomainV1 {
        views: product.project_v1(batch, constructors, contracts, inventory,
            FormalDomainProjectionV1::SourceObject)?.0,
        new_calls: Rc::clone(&product.new_calls),
    }));
    let mut expand = |owner, value| -> Result<source_sum::SourcePointExpansionV1, String> {
        use crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1 as Origin;
        use source_sum::SourcePointExpansionV1 as Expansion;
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, owner, |context, input| -> Result<Expansion, String> {
                let Some(origin) = input.function().expression_source().value_flow()
                    .filter(|flow| flow.owner() == owner).and_then(|flow| flow.origin(value)) else {
                    return Ok(Expansion::Unknown);
                };
                Ok(match origin {
                    Origin::Unknown => Expansion::Unknown,
                    Origin::Join { incoming, .. } => Expansion::Join(incoming.iter().map(|(_, value)| *value).collect()),
                    Origin::Entry(binding) => {
                        if value_domain::declared_integer_seed_v1(input, contracts, *binding) {
                            let signed = contracts.iter().filter(|row| row.owner == owner).flat_map(|row| &row.parameters)
                                .any(|row| row.binding == *binding && matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()));
                            Expansion::Seed(if signed { OriginalFormalActualSeedV1::Integer } else { OriginalFormalActualSeedV1::SourceInteger })
                        } else { Expansion::Entry(*binding) }
                    }
                    Origin::Expression(site) => {
                        let source_context = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                            static_claims, app_main, omitted)?.map(|context| context.for_point_premise());
                        let slot = context.declaration_slot();
                        let local_candidates = slot.and_then(|slot| candidates.get(&slot)).and_then(|rows| rows.as_ref().ok())
                            .map(Vec::as_slice).unwrap_or(&[]);
                        let receiver = slot.and_then(|slot| contracts.iter().find(|row| row.owner == owner)
                            .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                            .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                                selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                        Expansion::Seed(classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                            local_candidates, receiver, definitions, contracts, site, None, Some(inventory), source_context.as_ref()))
                    }
                })
            })?
    };
    let sums = source_sum::resolve_original_source_sums_v1(&product, contracts, constructors, &mut expand)?;
    for (site, operands) in &sums.core_calls {
        let receipt = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, site.owner(), |_, input|
                crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(input, operands))??;
        product.ordinary_core.insert(site.clone(), receipt);
    }
    product.source_sums = Some(sums);
    Ok(product)
}

impl OriginalFormalSeedProductV1 {
    /// Binding selected Stored targets changes no original row/ordered actual.
    /// Equal but newly issued source Rcs, loss and additional rows stay refused.
    pub(super) fn corroborate_original_v1(&self, inventory: &BorrowedIncomingInventoryV1) -> bool {
        if let Some(sums) = &self.source_sums {
            if self.ordinary_core.len() != sums.core_calls.len() || self.ordinary_core.iter().any(|(site, call)|
                sums.core_calls.get(site).is_none_or(|original| !Rc::ptr_eq(original, call.operands())
                    || !std::sync::Arc::ptr_eq(original.method(), call.source().source()))) { return false; }
        } else if !self.ordinary_core.is_empty() { return false; }
        if self.source_integers.iter().chain(self.source_i64s.iter())
                .any(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || self.source_objects.as_ref().is_some_and(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || self.source_sums.as_ref().is_some_and(|domain| !Rc::ptr_eq(&domain.new_calls, &self.new_calls))
            || !self.new_calls.retained_provider_sources_match()
            || inventory.registered_owners_v1() != &self.owners
            || inventory.source_vetoed_owners_v1() != self.source_vetoes
            || inventory.source_rows_v1().count() != self.rows.len()
            || inventory.source_registered_owners_v1() != &self.source_owners
            || inventory.sum_vetoed_owners_v1() != self.sum_vetoes
            || !inventory.corroborates_refusals_v1(&self.source_refusals)
        { return false; }
        let mut current = BTreeMap::new();
        for (call, legacy) in inventory.source_rows_with_legacy_v1() {
            if current.insert(call.call.clone(), (call, legacy)).is_some() { return false; }
        }
        self.rows.iter().all(|row| {
            let Some((call, legacy)) = current.get(&row.incoming.call) else { return false; };
            *legacy == row.legacy && call.callee == row.incoming.callee
                && call.arguments == row.incoming.arguments
                && call.source.same_original_source_v1(&row.incoming.source)
                && row.seeds.len() == call.arguments.len()
        })
    }

    pub(super) fn source_value_sum_v1(&self, inventory: &BorrowedIncomingInventoryV1, formal: BindingRefV1) -> Option<&SourceValueSumV1> {
        self.corroborate_original_v1(inventory).then_some(())?;
        self.source_sums.as_ref()?.values.get(&formal)
    }
    fn closed_core_source_v1(&self, inventory: &BorrowedIncomingInventoryV1, site: &OwnedExprSiteV1) -> Option<&Rc<ClosedOriginalCoreSourceV1>> {
        self.corroborate_original_v1(inventory).then_some(())?;
        self.source_sums.as_ref()?.core_calls.get(site)
    }

    pub(super) fn candidate_integer_agreement_v1(
        &self, inventory: &BorrowedIncomingInventoryV1, formal: BindingRefV1,
    ) -> bool {
        self.corroborate_original_v1(inventory) && self.source_integers.as_ref().is_some_and(|domain| domain.members.contains(&formal))
    }

    pub(super) fn candidate_i64_source_agreement_v1(&self, inventory: &BorrowedIncomingInventoryV1, formal: BindingRefV1) -> bool {
        self.corroborate_original_v1(inventory) && self.source_i64s.as_ref().is_some_and(|domain| domain.members.contains(&formal))
    }

    pub(super) fn selected_views_v1(
        &self,
        arrays: &super::array_source::ArraySourceRequirementsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        owners: &BTreeSet<FunctionOwnerIdV1>,
        inventory: &BorrowedIncomingInventoryV1,
    ) -> Result<(BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>, BTreeSet<BindingRefV1>, BTreeSet<BindingRefV1>), String> {
        self.project_v1(batch, constructors, contracts, inventory,
            FormalDomainProjectionV1::SelectedTransport { owners, arrays })
    }

    fn project_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        inventory: &BorrowedIncomingInventoryV1,
        projection: FormalDomainProjectionV1<'_>,
    ) -> Result<(BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>, BTreeSet<BindingRefV1>, BTreeSet<BindingRefV1>), String> {
        if !self.corroborate_original_v1(inventory) {
            return Err(freeze("borrowed-view/original-seed-identity"));
        }
        let vetoes = match projection {
            FormalDomainProjectionV1::SourceInteger | FormalDomainProjectionV1::SourceI64 | FormalDomainProjectionV1::SourceObject => inventory.source_vetoed_owners_v1(),
            FormalDomainProjectionV1::SelectedTransport { .. } => inventory.vetoed_owners(),
        };
        let mut seeds: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = BTreeMap::new();
        let mut declared = BTreeSet::new();
        for contract in contracts.iter().filter(|row| self.definition_owners.contains(&row.owner)) {
            let clean_declaration = inventory.project_declaration_v1(contract.owner)
                .is_ok_and(|rows| rows.is_empty());
            for formal in &contract.parameters {
                let declared_constraint = clean_declaration
                    && matches!(formal.kind, CallableParameterContractKindV1::DeclaredObject(_));
                if formal.kind.is_ordinary_borrowed_handle()
                    && vetoes.contains(&contract.owner) && !declared_constraint
                { seeds.entry(formal.binding).or_default().push(FormalActualSeedV1::Conflict); }
                if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
                    declared.insert(formal.binding);
                    seeds.entry(formal.binding).or_default().push(FormalActualSeedV1::Class(class.clone()));
                }
            }
        }
        for row in self.rows.iter().filter(|row| row.legacy) {
            let caller = row.incoming.call.owner();
            for ((_, site, formal), raw) in row.incoming.arguments.iter().zip(row.seeds.iter()) {
                let seed = match projection {
                    FormalDomainProjectionV1::SourceInteger | FormalDomainProjectionV1::SourceI64 => raw.project_source_integer_v1(matches!(projection, FormalDomainProjectionV1::SourceI64)),
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
        if matches!(projection, FormalDomainProjectionV1::SourceInteger | FormalDomainProjectionV1::SourceI64 | FormalDomainProjectionV1::SourceObject) {
            self.new_calls.append_source_seeds(constructors, &mut seeds, matches!(projection, FormalDomainProjectionV1::SourceI64),
                matches!(projection, FormalDomainProjectionV1::SourceObject));
        }
        let (mut objects, integers) = value_domain::resolve_formal_domains_v1(&seeds, batch, constructors);
        let mut mismatches = BTreeSet::new();
        for formal in &declared {
            if let Some(view) = objects.get_mut(formal) { view.declared = true; }
            else { mismatches.insert(*formal); }
        }
        // The early projection lends Integer only; no Object source permission.
        if matches!(projection, FormalDomainProjectionV1::SourceInteger | FormalDomainProjectionV1::SourceI64) { objects.clear(); }
        Ok((objects, integers, mismatches))
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// One original request selects the callee's existing parameter/Home walk;
    /// it lends no entry values, completed inputs or result permission.
    pub(in crate::mir::normal_callable_semantic_package) fn original_returned_field_input_request_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<OwnedExprSiteV1>, String> {
        let mut rows = self.original_formal_seeds.rows.iter().filter(|row| row.legacy && row.incoming.callee == owner
            && row.seeds.iter().any(|seed| matches!(seed, OriginalFormalActualSeedV1::ReturnedField(_))));
        let Some(first) = rows.next() else { return Ok(None); };
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("returned-field/original-incoming-census-drift"));
        }
        for row in std::iter::once(first).chain(rows) {
            let target = row.incoming.source.require_instance()?;
            if target.callee_owner() != owner || target.call_site() != &row.incoming.call
                || row.incoming.source.as_loan()?.declaration_mode()
                    != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
                return Err(freeze("returned-field/input-request-identity"));
            }
            for ((ordinal, site, formal), seed) in row.incoming.arguments.iter().zip(row.seeds.iter()) {
                if let OriginalFormalActualSeedV1::ReturnedField(source) = seed {
                    if formal.owner() != owner || target.argument_sites().get(*ordinal as usize) != Some(site)
                        || source.read().owner() != row.incoming.call.owner() || source.read().site() != site {
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
    pub(in crate::mir::normal_callable_semantic_package) fn with_replaced_original_new_evidence_for_test<R>(
        &mut self, erase_rows: bool, execute: impl FnOnce(&Self) -> R,
    ) -> R {
        let original = Rc::clone(&self.original_formal_seeds.new_calls);
        self.original_formal_seeds.new_calls = Rc::new(original.corrupted_provider_evidence_for_test(erase_rows));
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
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("provider-static/original-input-product-drift"));
        }
        self.original_formal_seeds.new_calls.corroborate_provider_source_v1(provider)
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
    let mut rows = inventory.exact_rows().filter_map(|row|
        row.source.instance().and_then(|source| source.mixed_local_source())
            .filter(|source| source.destination() == binding));
    let source = rows.next()?;
    let mut initializers = input.function().expression_source().initializers()
        .filter(|row| row.binding() == binding);
    let exact = initializers.next().is_some_and(|row|
        row.initializer_site() == Some(source.call().site())
            && row.declaration_site() == source.declaration()
            && input.function().declaration_binding(row.declaration_site()) == Some(binding));
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
    let Some(argument) = input.function().method_call(source.call().site())
        .and_then(|call| call.arguments().get(source.callee_ordinal() as usize)) else {
        return Some(OriginalFormalActualSeedV1::Conflict);
    };
    if argument.ordinal() != source.callee_ordinal()
        || arg_site_binding(input, argument.site()) != Some(source.support()) {
        return Some(OriginalFormalActualSeedV1::Conflict);
    }
    let anchor = classify_original_actual_seed_with_seen_v1(input, batch, selected, results,
        caller, candidates, receiver, definitions, contracts, argument.site(), None,
        Some(inventory), source_context, visited);
    Some(OriginalFormalActualSeedV1::Mixed { source: Box::new(source.clone()), anchor: Box::new(anchor) })
}

impl OriginalBorrowedFormalSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_source_v1(&self) -> Result<OriginalNewSourceLoanV1, String> {
        if !self.seeds.corroborate_original_v1(&self.inventory) { return Err(freeze("array-read/original-seed-drift")); }
        Ok(self.seeds.new_calls.source_loan_v1())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_array_get_v1(
        &self, call: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayGetSourceV1>>, String> {
        if !self.seeds.corroborate_original_v1(&self.inventory) { return Err(freeze("array-read/original-seed-drift")); }
        self.seeds.new_calls.array_get_at_v1(call)
    }
}
impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_argument_array_read_v1(
        &self, new_site: &OwnedExprSiteV1, ordinal: u32, site: &SourceExprSiteV1, binding: BindingRefV1,
    ) -> Result<Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>, String> {
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("array-read/original-seed-drift"));
        }
        self.original_formal_seeds.new_calls.argument_array_read_v1(new_site, ordinal, site, binding)
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
    let source = results.returned_field_source_v1(batch, selected, constructors, read, &mut |site| {
        let mut rows = inventory.exact_rows().filter(|row| &row.call == site);
        let Some(row) = rows.next() else { return Ok(None); };
        if rows.next().is_some() { return Err("returned-field/original-target-duplicate"); }
        let target = row.source.require_instance().map_err(|_| "returned-field/original-target-kind")?;
        if target.call_site() != site || target.callee_owner() != row.callee {
            return Err("returned-field/original-target-identity");
        }
        Ok(Some(target))
    }).map_err(freeze)?;
    Ok(source.map(|source| OriginalFormalActualSeedV1::ReturnedField(Box::new(source))))
}

impl PreparedBorrowedFormalIngressV1 {
    /// Selection only; retained source validation remains with the lender.
    pub(in crate::mir::normal_callable_semantic_package) fn returned_field_argument_selected_v1(
        &self, call: &OwnedExprSiteV1,
    ) -> bool {
        self.original_formal_seeds.rows.iter().filter(|row| row.legacy).any(|row| &row.incoming.call == call
            && row.seeds.iter().any(|seed| matches!(seed, OriginalFormalActualSeedV1::ReturnedField(_))))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn original_returned_field_actual_source_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32,
    ) -> Result<Option<&super::super::super::result_class_claim::ReturnedFieldSourceLoanV1>, String> {
        if !self.original_formal_seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("returned-field/original-incoming-census-drift"));
        }
        let mut rows = self.original_formal_seeds.rows.iter().filter(|row| row.legacy && &row.incoming.call == call);
        let Some(row) = rows.next() else { return Ok(None); };
        if rows.next().is_some() { return Err(freeze("returned-field/original-call-duplicate")); }
        for ((actual_ordinal, site, _), seed) in row.incoming.arguments.iter().zip(row.seeds.iter()) {
            if *actual_ordinal != ordinal { continue; }
            let OriginalFormalActualSeedV1::ReturnedField(source) = seed else { return Ok(None); };
            if source.read().owner() != call.owner() || source.read().site() != site {
                return Err(freeze("returned-field/original-actual-drift"));
            }
            return Ok(Some(source));
        }
        Ok(None)
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// Complete source alternatives on Normal, never entry or execution permission.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_value_sum_v1(
        &self, formal: BindingRefV1,
    ) -> Option<&SourceValueSumV1> {
        self.original_formal_seeds.source_value_sum_v1(&self.source_incoming, formal)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn closed_core_source_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<&Rc<ClosedOriginalCoreSourceV1>> {
        self.original_formal_seeds.closed_core_source_v1(&self.source_incoming, site)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ordinary_core_source_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>> {
        self.original_formal_seeds.corroborate_original_v1(&self.source_incoming).then_some(())?;
        self.original_formal_seeds.ordinary_core.get(site)
    }
}
