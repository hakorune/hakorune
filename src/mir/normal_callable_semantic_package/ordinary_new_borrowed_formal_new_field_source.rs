//! A numeric dependency loan from the sole original New census. This never
//! promotes Conflict in the cached formal domain or supplies executable input.
use super::*;
use std::rc::Rc;

type FieldKeyV1 = (Box<str>, Box<str>);

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalNewI64InputsLoanV1
{
    original: Rc<OriginalNewSeedsV1>,
    constructor: ConstructorSourceIdV1,
    formal: BindingRefV1,
    ordinal: usize,
    rows: Box<[usize]>,
    field_rows: Box<[usize]>,
    array_rows: Box<[usize]>,
    origins: Box<[BindingRefV1]>,
}
impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn lend_i64_inputs_v1(
        self: &Rc<Self>,
        constructor: &ConstructorSourceIdV1,
        formal: BindingRefV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        current_i64: &BTreeSet<BindingRefV1>,
    ) -> Option<OriginalNewI64InputsLoanV1> {
        if self.incomplete || !self.current_provider_sources_match(constructors) {
            return None;
        }
        let mut targets = constructors
            .rows()
            .iter()
            .filter(|row| row.source_id() == constructor);
        let target = targets.next()?;
        if targets.next().is_some()
            || target.kind() != ConstructorSourceKindV1::Birth
            || target.forest().roots() != [formal.owner()]
        {
            return None;
        }
        let ordinal = target
            .formal_contracts()
            .iter()
            .position(|row| row.binding() == formal)?;
        if target.formal_contracts()[ordinal].ordinal() as usize != ordinal {
            return None;
        }
        let mut rows = Vec::new();
        let mut origins = Vec::new();
        let mut field_rows = Vec::new();
        let mut array_rows = Vec::new();
        // Filter only by source class first: an unavailable/other constructor
        // of this class is a veto, never an omitted incoming construction.
        for (index, row) in self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.original.class() == target.box_name())
        {
            let OriginalNewBirthV1::Birth {
                source,
                owner,
                formals,
            } = &row.birth
            else {
                return None;
            };
            if source != constructor
                || *owner != formal.owner()
                || row.original.site() != row.site.site()
                || !row.original.field_initializers().is_empty()
                || formals
                    .iter()
                    .copied()
                    .ne(target.formal_contracts().iter().map(|row| row.binding()))
                || formals.get(ordinal) != Some(&formal)
                || row.seeds.len() != formals.len()
                || row.fields.len() != formals.len()
                || row.array_reads.len() != formals.len()
                || row.original.arguments().len() != formals.len()
            {
                return None;
            }
            match &row.seeds[ordinal] {
                OriginalFormalActualSeedV1::ProviderStatic(loan)
                    if loan.matches_original(&row.site, &row.original, ordinal)
                        && loan.matches_constructor(constructors) => {}
                seed => match seed.project_source_integer_v1(true) {
                    FormalActualSeedV1::Integer => {}
                    FormalActualSeedV1::Forward(origin)
                    | FormalActualSeedV1::ForwardIntegerOnly(origin)
                        if current_i64.contains(&origin) =>
                    {
                        origins.push(origin)
                    }
                    FormalActualSeedV1::Conflict => {
                        match (&row.fields[ordinal], &row.array_reads[ordinal]) {
                            (Some(field), None) => {
                                if row.original.arguments().get(ordinal) != Some(&field.site)
                                    || field.receiver.owner() != row.site.owner()
                                {
                                    return None;
                                }
                                field_rows.push(index);
                            }
                            (None, Some(read))
                                if read.binding().owner() == row.site.owner()
                                    && read.read().call().owner() == row.site.owner() =>
                            {
                                array_rows.push(index)
                            }
                            _ => return None,
                        }
                    }
                    _ => return None,
                },
            }
            rows.push(index);
        }
        if rows.is_empty() {
            return None;
        }
        Some(OriginalNewI64InputsLoanV1 {
            original: Rc::clone(self),
            constructor: constructor.clone(),
            formal,
            ordinal,
            rows: rows.into_boxed_slice(),
            field_rows: field_rows.into_boxed_slice(),
            array_rows: array_rows.into_boxed_slice(),
            origins: origins.into_boxed_slice(),
        })
    }
}
impl OriginalNewI64InputsLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn fields(
        &self,
    ) -> impl Iterator<Item = FieldKeyV1> + '_ {
        self.field_rows
            .iter()
            .filter_map(|index| self.original.rows[*index].fields[self.ordinal].as_ref())
            .map(|field| (field.class.clone(), field.field.clone()))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ready(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
    ) -> bool {
        let seeds = &source.original_formal_seeds;
        Rc::ptr_eq(&self.original, &seeds.new_calls)
            && source.corroborated_original_v1()
            && self.origins.iter().all(|origin| source.candidate_i64_source_agreement(*origin))
            && self.array_rows.iter().all(|index| self.original.rows[*index].array_reads[self.ordinal].as_ref()
                .is_some_and(|read| read.binding().owner() == self.original.rows[*index].site.owner()
                    && read.read().call().owner() == read.binding().owner()))
            && self.rows.iter().all(|index| matches!(&self.original.rows[*index].birth,
                OriginalNewBirthV1::Birth { source, formals, .. }
                    if *source == self.constructor && formals.get(self.ordinal) == Some(&self.formal)))
    }
}

impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn array_get_at_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
        String,
    > {
        let mut original = None;
        for read in self
            .rows
            .iter()
            .flat_map(|row| row.array_reads.iter())
            .flatten()
            .filter(|read| read.read().call() == call)
        {
            if let Some(held) = &original {
                if !Rc::ptr_eq(held, read.read()) {
                    return Err(freeze("array-read/original-new-source-conflict"));
                }
            } else {
                original = Some(Rc::clone(read.read()));
            }
        }
        Ok(original)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn argument_array_read_v1(
        &self,
        new_site: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<
        Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>,
        String,
    > {
        let mut rows = self.rows.iter().filter(|row| &row.site == new_site);
        let row = rows
            .next()
            .ok_or_else(|| freeze("array-read/original-new-missing"))?;
        let ordinal = ordinal as usize;
        if rows.next().is_some()
            || row.original.arguments().get(ordinal) != Some(site)
            || row.array_reads.len() != row.original.arguments().len()
            || binding.owner() != new_site.owner()
        {
            return Err(freeze("array-read/original-new-argument-drift"));
        }
        let read = row.array_reads[ordinal].as_ref();
        if read.is_some_and(|read| {
            read.binding() != binding || read.read().call().owner() != new_site.owner()
        }) {
            return Err(freeze("array-read/original-new-binding-drift"));
        }
        Ok(read)
    }
}

/// Passive original source lender, issued before transport selection. A failed
/// transport or result seal cannot erase the original New/operand identities.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct OriginalNewSourceLoanV1(
    Rc<OriginalNewSeedsV1>,
);
impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_loan_v1(
        self: &Rc<Self>,
    ) -> OriginalNewSourceLoanV1 {
        OriginalNewSourceLoanV1(Rc::clone(self))
    }
}
impl OriginalNewSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn argument_array_read_v1(
        &self,
        new_site: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<
        Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>,
        String,
    > {
        self.0
            .argument_array_read_v1(new_site, ordinal, site, binding)
    }
}

/// Capture explicit source move hops with the existing all-owner New reads.
/// This grants no transfer: Home planning and final SSA still verify moves.
pub(super) fn observe_local_uses_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    initial: &crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
    reads: &BTreeMap<BindingRefV1, BTreeSet<SourceExprSiteV1>>,
    counts: &BTreeMap<BindingRefV1, usize>,
    rebindings: &BTreeSet<BindingRefV1>, captures: &BTreeSet<BindingRefV1>,
) -> Option<OriginalNewLocalUsesV1> {
    use crate::mir::resolved_semantics::BindingKindV1;
    let function = input.function();
    let mut result = OriginalNewLocalUsesV1 { initializer: initial.clone(), reads: BTreeSet::new(),
        bindings: BTreeSet::new(), moves: BTreeSet::new() };
    let mut current = initial;
    loop {
        let binding = current.binding();
        if binding.owner() != input.owner() || counts.get(&binding) != Some(&1)
            || !matches!(function.binding(binding).map(|record| record.kind()), Some(BindingKindV1::Local { .. }))
            || function.declaration_binding(current.declaration_site()) != Some(binding)
            || rebindings.contains(&binding) || captures.contains(&binding)
            || !result.bindings.insert(binding) { return None; }
        result.reads.extend(reads.get(&binding).into_iter().flatten().cloned());
        let mut next = function.expression_source().initializers().filter_map(|init| {
            let unary = function.expression_source().moved(init.initializer_site()?)?;
            (arg_site_binding(input, unary.operand()) == Some(binding)).then_some((init, unary))
        });
        let Some((target, unary)) = next.next() else { return Some(result); };
        if next.next().is_some() || !result.moves.insert(unary.operand().clone()) { return None; }
        current = target;
    }
}

impl OriginalNewSeedsV1 {
    /// Close original New/owned-result local reads and owned Return roots
    /// against the same child relations and incoming methods. This source filter
    /// does not establish Home transfer order or physical call correspondence.
    fn class_local_uses_v1(
        &self, class: &str, sites: &BTreeSet<OwnedExprSiteV1>,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        callee: FunctionOwnerIdV1, inventory: &BorrowedIncomingInventoryV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{ResultWitnessStepV1, ResultValueOriginV1};
        if self.class_sites_v1(class, constructors).as_ref() != Some(sites)
            || !inventory.source_registered_owners_v1().contains(&callee)
            || inventory.sum_vetoed_owners_v1().contains(&callee) { return false; }
        // Validate every retained target row, including owners that create no
        // object locally. An unresolved or detached entry cannot disappear.
        let mut unmatched = BTreeSet::new();
        for (incoming, method) in inventory.source_rows_with_method_v1().filter(|(row, _)| row.callee == callee) {
            let (Some(method), Some(reference)) = (method, incoming.source.original_instance_reference_v1()) else { return false; };
            if reference.callee_owner != callee || reference.target.owner() != class
                || reference.call_site != incoming.call || !unmatched.insert(incoming.call.clone()) { return false; }
            let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                batch, constructors, incoming.call.owner(), |_, input|
                    reference.matches_original_call_v1(input, method)
                        && input.function().method_call_source_v1(method.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, method)));
            if checked != Ok(true) { return false; }
            if let Some(stored) = incoming.source.original_stored_receiver_v1() {
                use super::super::source::StoredReceiverSourceOriginV1;
                let StoredReceiverSourceOriginV1::ProvisionalDeclared(pair) = &stored.origin else { return false; };
                if incoming.call.owner() != callee || stored.parent_new.is_some()
                    || stored.parent_class.as_ref() != class || stored.child_class.as_ref() != class
                    || pair.child() != object || !pair.recheck_v1(facts, &stored.field_name) { return false; }
                let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                    batch, constructors, incoming.call.owner(), |_, input| {
                        use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};
                        let Some(shape) = input.body_shape() else { return false; };
                        // A child call cannot invalidate the caller's paired
                        // residence through an unknown effect. Close recursive
                        // calls on the same original incoming inventory.
                        use crate::mir::resolved_semantics::{BodyEffectKindV1 as Effect, BodyStatementShapeV1};
                        if shape.statements().iter().any(|row| matches!(row, BodyStatementShapeV1::Print { .. }))
                            || !shape.effects().iter().all(|effect| match effect.kind {
                                Effect::Call => inventory.source_rows_with_method_v1().any(|(row, original)|
                                    row.call.owner() == callee && row.callee == callee && row.call.site() == &effect.site
                                        && original.is_some_and(|original| input.function().method_call_source_v1(&effect.site)
                                            .is_some_and(|current| std::sync::Arc::ptr_eq(current, original)))),
                                Effect::NonLocalControl => shape.statements().iter().any(|row|
                                    matches!(row, BodyStatementShapeV1::Return { site, .. } if site.node() == effect.site.node())),
                                _ => false,
                            }) { return false; }
                        matches!(shape.expression_shape(method.receiver_site()), Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
                            if object == &stored.parent_site && field == &stored.field_name)
                            && matches!(shape.expression_shape(&stored.parent_site), Some(BodyExpressionShapeV1::Me {
                                receiver: BodyMeReceiverV1::Lexical(binding), .. }) if *binding == stored.parent_binding)
                            && stored.parent_binding.owner() == callee
                            && crate::mir::normal_callable_semantic_package::instance_constructor_non_escape::verify_receiver_non_escape_v1(
                                input.function(), shape, input.forest()).is_ok()
                            && pair.local_child_uses_v1(input, stored.parent_binding)
                            && if pair.nullable() {
                                stored.guard.is_some() && crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue::source_claims::stored_child_nonnull_guard_v1(
                                    batch, constructors, input, method, stored.parent_binding, &stored.parent_site, class, pair)
                                    .is_ok_and(|guard| guard == stored.guard)
                            } else { stored.guard.is_none() }
                    });
                if checked != Ok(true) || !unmatched.remove(&incoming.call) { return false; }
            }
        }
        let mut roots = BTreeSet::new();
        let mut child_to_parent = BTreeMap::new();
        let mut returned = BTreeSet::new();
        for (key, exits) in facts.source_rows() {
            if !exits.iter().flat_map(|row| row.witnesses()).any(|root|
                matches!(root.step(), ResultWitnessStepV1::FreshConstruction)
                    && matches!(root.origin(), ResultValueOriginV1::Fresh(name) if name.as_ref() == class)
                    && sites.contains(root.site())) { continue; }
            let Some(owner) = exits.first().map(|row| row.site().owner()) else { continue; };
            let Ok(original) = facts.checked_original_source_roots_v1(key, owner) else { return false; };
            for root in original {
                if matches!(root.step(), ResultWitnessStepV1::FreshConstruction)
                    && matches!(root.origin(), ResultValueOriginV1::Fresh(name) if name.as_ref() == class)
                    && sites.contains(root.site()) { returned.insert(root.site().clone()); }
            }
        }
        let mut locals = Vec::new();
        for row in self.rows.iter().filter(|row| row.original.class() == class) {
            match &row.local_uses {
                Some(original) => locals.push((&row.site, original, Some(row.original.arguments().len()), None)),
                None if returned.contains(&row.site) => { roots.insert(row.site.clone()); }
                None => return false,
            }
        }
        for (site, observed) in &self.call_locals {
            let mut incoming = inventory.source_rows_with_method_v1().filter(|(row, _)| &row.call == site);
            let Some((row, Some(method))) = incoming.next() else { continue; };
            if incoming.next().is_some() { return false; }
            let Some(reference) = row.source.original_instance_reference_v1() else { continue; };
            let Ok(callees) = facts.checked_original_source_roots_v1(&reference.target, row.callee) else { continue; };
            if callees.is_empty() || !callees.iter().all(|root|
                matches!(root.origin(), ResultValueOriginV1::Fresh(name) if name.as_ref() == class)
                    && matches!(root.step(), ResultWitnessStepV1::FreshConstruction) && returned.contains(root.site())) { continue; }
            let Some(original) = observed else { return false; };
            let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                batch, constructors, site.owner(), |_, input| reference.call_site == *site
                    && reference.callee_owner == row.callee && reference.matches_original_call_v1(input, method)
                    && input.function().method_call_source_v1(site.site()).is_some_and(|held| std::sync::Arc::ptr_eq(held, method)));
            if checked != Ok(true) { return false; }
            locals.push((site, original, None, Some((reference.target, callees.into_iter().cloned().collect::<Vec<_>>()))));
        }
        for (site, original, arity, call_roots) in locals {
            let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                batch, constructors, site.owner(), |_, input| {
                    let function = input.function();
                    let binding = original.initializer.binding();
                    if original.initializer.initializer_site() != Some(site.site())
                        || binding.owner() != input.owner() { return false; }
                    let mut allowed = original.moves.clone();
                    let mut transfers = 0usize;
                    for relation in facts.construction_child_relation_loans() {
                        if relation.field().object() != object || relation.child() != object
                            || relation.outer().owner() != relation.actual().owner() || !sites.contains(relation.outer())
                            || relation.actual().owner() != input.owner()
                            || !original.reads.contains(relation.actual().site()) { continue; }
                        let witnesses = relation.witnesses();
                        let matches_source = match &call_roots {
                            None => matches!(witnesses, [witness] if witness.site() == relation.actual()
                                && matches!(witness.step(), ResultWitnessStepV1::LocalConstruction(source)
                                    if source.matches_local_new_v1(binding, site, class, arity.expect("New arity")))),
                            Some((key, roots)) => witnesses.len() == roots.len() && witnesses.iter().zip(roots).all(|(witness, root)|
                                witness.site() == relation.actual() && matches!(witness.step(), ResultWitnessStepV1::Call {
                                    site: original, key: actual_key, callee, substitution: None,
                                } if original == site && actual_key == key && Rc::ptr_eq(callee, root))),
                        };
                        if !matches_source || relation.provided_field_source_v1(class).is_err()
                            || !allowed.insert(relation.actual().site().clone()) { return false; }
                        if child_to_parent.insert(site.clone(), relation.outer().clone()).is_some() { return false; }
                        transfers += 1;
                    }
                    if transfers > 1 { return false; }
                    for (incoming, method) in inventory.source_rows_with_method_v1().filter(|(incoming, _)| incoming.callee == callee && incoming.call.owner() == input.owner()) {
                        let Some(method) = method else { return false; };
                        let Some(reference) = incoming.source.original_instance_reference_v1() else { return false; };
                        if reference.call_site != incoming.call || reference.callee_owner != callee
                            || reference.target.owner() != class || reference.receiver_site != *method.receiver_site()
                            || method.site() != incoming.call.site()
                            || !function.method_call_source_v1(method.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, method)) { return false; }
                        if original.reads.contains(method.receiver_site()) {
                            let BorrowedIncomingSourceV1::Instance(target) = &incoming.source else { return false; };
                            if target.is_self_receiver() || target.receiver_binding().ok().is_none_or(|binding| !original.bindings.contains(&binding))
                                || !target.matches_receiver(input, method) || !unmatched.remove(&incoming.call) { return false; }
                            roots.insert(site.clone());
                            if !allowed.insert(method.receiver_site().clone()) { return false; }
                        }
                    }
                    if original.reads != allowed && crate::config::env::builder_debug_enabled() {
                        eprintln!("[stored-child] class={class} source={:?} unaccounted={:?} foreign={:?}", site,
                            original.reads.difference(&allowed).collect::<Vec<_>>(), allowed.difference(&original.reads).collect::<Vec<_>>());
                    }
                    !original.reads.is_empty() && original.reads == allowed
                });
            if checked != Ok(true) { return false; }
        }
        if roots.is_empty() || !unmatched.is_empty() { return false; }
        for site in sites {
            let mut current = site;
            let mut visiting = BTreeSet::new();
            while !roots.contains(current) {
                if !visiting.insert(current.clone()) { return false; }
                let Some(parent) = child_to_parent.get(current) else { return false; };
                current = parent;
            }
        }
        true
    }

}

impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
    fn declared_child_uses_v1(&self,
        reference: &super::super::super::source::CallTargetReferenceV1,
        receiver: &super::super::super::source::StoredReceiverSourceV1,
        inventory: &BorrowedIncomingInventoryV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        use super::super::super::source::StoredReceiverSourceOriginV1;
        let StoredReceiverSourceOriginV1::ProvisionalDeclared(pair) = &receiver.origin else { return false; };
        if !pair.recheck_v1(facts, &receiver.field_name) || pair.child_class() != receiver.child_class.as_ref()
            || reference.target.owner() != receiver.child_class.as_ref()
            || receiver.parent_new.is_some() || receiver.parent_binding.owner() != reference.call_site.owner()
            || !inventory.source_rows_v1().any(|row|
                row.source.original_instance_reference_v1().as_ref() == Some(reference)
                    && row.source.original_stored_receiver_v1() == Some(receiver)) { return false; }
        let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, reference.call_site.owner(), |_, input| {
                use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};
                let Some(shape) = input.body_shape() else { return false; };
                let Some(method) = input.function().method_call_source_v1(reference.call_site.site()) else { return false; };
                reference.matches_original_call_v1(input, method)
                    && matches!(shape.expression_shape(method.receiver_site()), Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
                        if object == &receiver.parent_site && field == &receiver.field_name)
                    && matches!(shape.expression_shape(&receiver.parent_site), Some(BodyExpressionShapeV1::Me {
                        receiver: BodyMeReceiverV1::Lexical(binding), .. }) if *binding == receiver.parent_binding)
                    && crate::mir::normal_callable_semantic_package::instance_constructor_non_escape::verify_receiver_non_escape_v1(
                        input.function(), shape, input.forest()).is_ok()
                    && pair.local_child_uses_v1(input, receiver.parent_binding)
                    && if pair.nullable() {
                        receiver.guard.is_some() && crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue::source_claims::stored_child_nonnull_guard_v1(
                            batch, constructors, input, method, receiver.parent_binding, &receiver.parent_site, &receiver.parent_class, pair)
                            .is_ok_and(|guard| guard == receiver.guard)
                    } else { receiver.guard.is_none() }
            });
        let original = checked == Ok(true);
        let readonly = original && readonly_call_closure_v1(reference.callee_owner, inventory, batch, Some(constructors), None);
        let parent_uses = readonly && facts.child_construction_sites_v1(pair.field().object()).is_some_and(|sites|
            self.class_local_uses_v1(&receiver.parent_class, sites, pair.field().object(),
                reference.call_site.owner(), inventory, facts, batch, constructors));
        if !parent_uses && crate::config::env::builder_debug_enabled() {
            eprintln!("[stored-child] caller={:?} callee={:?} original={original} readonly={readonly} parent-uses={parent_uses}",
                reference.call_site.owner(), reference.callee_owner);
        }
        parent_uses
    }
}
