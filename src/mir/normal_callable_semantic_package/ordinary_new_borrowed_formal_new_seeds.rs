//! Original New inputs retained by the SAME early formal seed product.
//! Every resolver owner participates. This is source agreement only, not Birth
//! execution, argument ABI, construction readiness or a numeric field invariant.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{ResolvedNewExpressionSourceV1, SourceBindingSiteV1};
use crate::parser::{ConstructorSourceIdV1, ConstructorSourceKindV1};

#[path = "ordinary_new_borrowed_formal_new_field_source.rs"]
mod field_source;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use field_source::OriginalNewI64InputsLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use field_source::OriginalNewSourceLoanV1;

/// Passive source operand, captured at the sole original New census. Numeric
/// qualification joins this key to the existing field owner later.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalNewFieldActualV1 {
    site: SourceExprSiteV1,
    object: SourceExprSiteV1,
    receiver: BindingRefV1,
    class: Box<str>,
    field: Box<str>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
enum OriginalNewBirthV1 {
    Birth {
        source: ConstructorSourceIdV1,
        owner: FunctionOwnerIdV1,
        formals: Box<[BindingRefV1]>,
    },
    NoBirth,
    OutsideOrdinary,
    Unavailable,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
struct OriginalNewCallSeedsV1 {
    site: OwnedExprSiteV1,
    original: ResolvedNewExpressionSourceV1,
    local_uses: Option<OriginalNewLocalUsesV1>,
    birth: OriginalNewBirthV1,
    seeds: Box<[OriginalFormalActualSeedV1]>,
    fields: Box<[Option<OriginalNewFieldActualV1>]>,
    array_reads: Box<
        [Option<
            std::rc::Rc<
                crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1,
            >,
        >],
    >,
}

/// Original local binding and all of its reads, captured with the New census.
/// This retains source coordinates only, never a Home or a call permission.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalNewLocalUsesV1 {
    initializer: crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
    reads: BTreeSet<SourceExprSiteV1>,
}

impl OriginalNewCallSeedsV1 {
    fn corroborates_birth_v1(
        &self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        let OriginalNewBirthV1::Birth {
            source,
            owner,
            formals,
        } = &self.birth
        else {
            return false;
        };
        let mut matching = constructors
            .rows()
            .iter()
            .filter(|ctor| ctor.source_id() == source);
        let ctor = matching.next();
        matching.next().is_none()
            && ctor.is_some_and(|ctor| {
                ctor.kind() == ConstructorSourceKindV1::Birth
                    && ctor.box_name() == self.original.class()
                    && ctor.forest().roots() == [*owner]
                    && ctor
                        .formal_contracts()
                        .iter()
                        .map(|formal| formal.binding())
                        .eq(formals.iter().copied())
            })
            && self.seeds.len() == self.original.arguments().len()
            && self.seeds.len() == formals.len()
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub(super) struct OriginalNewSeedsV1 {
    rows: Vec<OriginalNewCallSeedsV1>,
    incomplete: bool,
}
impl OriginalNewSeedsV1 {
    /// Check every use of each original local New against the existing child
    /// relations and incoming method rows. This is a source-only filter: it
    /// does not establish Home transfer order or physical call correspondence.
    pub(super) fn class_local_uses_v1(
        &self, class: &str, sites: &BTreeSet<OwnedExprSiteV1>,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        callee: FunctionOwnerIdV1, inventory: &BorrowedIncomingInventoryV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ResultWitnessStepV1;
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
        for row in self.rows.iter().filter(|row| row.original.class() == class) {
            let checked = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                batch, constructors, row.site.owner(), |_, input| {
                    let function = input.function();
                    let Some(original) = &row.local_uses else { return false; };
                    let binding = original.initializer.binding();
                    if original.initializer.initializer_site() != Some(row.site.site())
                        || binding.owner() != input.owner() { return false; }
                    let mut allowed = BTreeSet::new();
                    let mut transfers = 0usize;
                    for relation in facts.construction_child_relation_loans() {
                        if relation.field().object() != object || relation.child() != object
                            || relation.outer().owner() != relation.actual().owner() || !sites.contains(relation.outer())
                            || relation.actual().owner() != input.owner()
                            || !original.reads.contains(relation.actual().site()) { continue; }
                        let [witness] = relation.witnesses() else { return false; };
                        if witness.site() != relation.actual() || relation.provided_field_source_v1(class).is_err()
                            || !matches!(witness.step(), ResultWitnessStepV1::LocalConstruction(source)
                                if source.matches_local_new_v1(binding, &row.site, class, row.original.arguments().len()))
                            || !allowed.insert(relation.actual().site().clone()) { return false; }
                        if child_to_parent.insert(row.site.clone(), relation.outer().clone()).is_some() { return false; }
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
                            if target.is_self_receiver() || target.receiver_binding().ok() != Some(binding)
                                || !target.matches_receiver(input, method) || !unmatched.remove(&incoming.call) { return false; }
                            roots.insert(row.site.clone());
                            if !allowed.insert(method.receiver_site().clone()) { return false; }
                        }
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
    /// Exact sites of this class from the existing all-owner New source scan.
    /// A missing owner, duplicate site, unknown Birth, or field override keeps
    /// this passive census unavailable; it grants no construction permission.
    pub(super) fn class_sites_v1(
        &self, class: &str,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> Option<BTreeSet<OwnedExprSiteV1>> {
        if self.incomplete { return None; }
        let mut sites = BTreeSet::new();
        for row in self.rows.iter().filter(|row| row.original.class() == class) {
            if row.original.site() != row.site.site() || !row.original.field_initializers().is_empty()
                || !row.corroborates_birth_v1(constructors) || !sites.insert(row.site.clone()) {
                return None;
            }
        }
        (!sites.is_empty()).then_some(sites)
    }
    /// Nominal source identity only; construction execution remains separate.
    pub(super) fn nominal_class_at_v1(&self, site: &OwnedExprSiteV1) -> Option<&str> {
        let mut rows = self.rows.iter().filter(|row| &row.site == site);
        let row = rows.next()?;
        if rows.next().is_some() || row.original.site() != site.site()
            || !matches!(row.birth, OriginalNewBirthV1::Birth { .. } | OriginalNewBirthV1::NoBirth)
        { return None; }
        Some(row.original.class())
    }

    /// Field-base evidence must retain all original construction obligations.
    /// Override typing is not supplied by the constructor-argument projection.
    pub(super) fn field_bases_available(&self, class: &str) -> bool {
        !self.incomplete
            && self
                .rows
                .iter()
                .filter(|row| row.original.class() == class)
                .all(|row| {
                    matches!(
                        row.birth,
                        OriginalNewBirthV1::Birth { .. } | OriginalNewBirthV1::NoBirth
                    ) && row.original.field_initializers().is_empty()
                })
    }

    pub(super) fn missing_owner(&mut self) {
        self.incomplete = true;
    }

    pub(super) fn observe(
        &mut self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
        selected: &VerifiedSelectedCallableBatchMapV1,
        array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
        mut classify: impl FnMut(&SourceExprSiteV1) -> OriginalFormalActualSeedV1,
    ) -> Result<(), String> {
        if input.body_shape().is_none() {
            self.incomplete = true;
        }
        if input.function().expression_source().constructions().next().is_none() {
            return Ok(());
        }
        // Capture local reads once per original owner rather than rediscovering
        // them at every consumer of each New. An ambiguous or captured local
        // has no local snapshot and does not affect unrelated New consumers.
        use crate::mir::resolved_semantics::{BindingKindV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1};
        let function = input.function();
        let mut local_reads: BTreeMap<BindingRefV1, BTreeSet<SourceExprSiteV1>> = BTreeMap::new();
        for (site, reference) in function.variable_refs() {
            if let ResolvedLexicalRefV1::Local(binding) = reference {
                local_reads.entry(*binding).or_default().insert(site.clone());
            }
        }
        let rebindings: BTreeSet<_> = function.assignment_targets().filter_map(|(_, target)| match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding) => Some(*binding), _ => None,
        }).collect();
        let captures: BTreeSet<_> = input.forest().upvars().iter().map(|upvar| upvar.source())
            .filter(|binding| binding.owner() == input.owner()).collect();
        let ledger_ok = input.forest().callable_source_ledger(input.owner()).is_ok();
        let mut initializers = BTreeMap::new();
        let mut counts = BTreeMap::<BindingRefV1, usize>::new();
        for init in function.expression_source().initializers() {
            *counts.entry(init.binding()).or_default() += 1;
            if let Some(site) = init.initializer_site() {
                if initializers.insert(site.clone(), Some(init)).is_some() {
                    initializers.insert(site.clone(), None);
                }
            }
        }
        let mut original_array_locals = BTreeMap::new();
        let caller = batch
            .declarations()
            .find(|row| row.owner() == input.owner())
            .and_then(|row| match selected.key_for_batch_slot(row.batch_slot()) {
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key),
                _ => None,
            });
        for original in input.function().expression_source().constructions() {
            let site = OwnedExprSiteV1::new(input.owner(), original.site().clone());
            if self.rows.iter().any(|row| row.site == site) {
                self.incomplete = true;
            }
            let birth = match batch.ordinary_box_coverage().row_for(original.class()) {
                Ok(Some(source)) => {
                    match constructors.birth_for(source, original.arguments().len()) {
                        Ok(Some(row)) => match row.forest().roots() {
                            [owner] => {
                                let formals = row.formal_contracts();
                                let function = row.forest().owner(*owner);
                                if formals.len() != original.arguments().len()
                                    || !formals.iter().enumerate().all(|(index, formal)| {
                                        formal.ordinal() as usize == index
                                            && formal.binding().owner() == *owner
                                            && function.is_some_and(|function| {
                                                function.declaration_binding(
                                                    &SourceBindingSiteV1::Parameter {
                                                        index: formal.ordinal(),
                                                    },
                                                ) == Some(formal.binding())
                                            })
                                    })
                                {
                                    OriginalNewBirthV1::Unavailable
                                } else {
                                    OriginalNewBirthV1::Birth {
                                        source: row.source_id().clone(),
                                        owner: *owner,
                                        formals: formals
                                            .iter()
                                            .map(|formal| formal.binding())
                                            .collect(),
                                    }
                                }
                            }
                            _ => OriginalNewBirthV1::Unavailable,
                        },
                        Ok(None) => OriginalNewBirthV1::NoBirth,
                        Err(_) => OriginalNewBirthV1::Unavailable,
                    }
                }
                Ok(None) => OriginalNewBirthV1::OutsideOrdinary,
                Err(_) => OriginalNewBirthV1::Unavailable,
            };
            let owner_class = batch.declarations().find(|row| row.owner() == input.owner())
                .and_then(|row| match selected.key_for_batch_slot(row.batch_slot()) {
                    Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                        if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => Some(key.owner()),
                    _ => None,
                }).or_else(|| constructors.rows().iter().find(|row|
                    row.kind() == ConstructorSourceKindV1::Birth && row.forest().roots() == [input.owner()])
                    .map(|row| row.box_name()));
            let fields = original.arguments().iter().map(|site| {
                use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1, BindingKindV1, ResolvedLexicalRefV1};
                let BodyExpressionShapeV1::FieldAccess { object, field, .. } = input.body_shape()?.expression_shape(site)? else { return None; };
                let Some(ResolvedLexicalRefV1::Local(receiver)) = input.function().variable_ref(object) else { return None; };
                if receiver.owner() != input.owner() || input.function().binding(receiver)?.kind() != BindingKindV1::Receiver
                    || !matches!(input.body_shape()?.expression_shape(object), Some(BodyExpressionShapeV1::Me {
                        receiver: BodyMeReceiverV1::Lexical(binding), .. }) if *binding == receiver)
                { return None; }
                Some(OriginalNewFieldActualV1 { site: site.clone(), object: object.clone(), receiver,
                    class: owner_class?.into(), field: field.clone() })
            }).collect();
            let mut array_reads = Vec::with_capacity(original.arguments().len());
            for site in original.arguments() {
                let binding = arg_site_binding(input, site);
                let read = if let (Some(binding), Some(caller)) = (binding, caller) {
                    if !original_array_locals.contains_key(&binding) {
                        let read = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_array_read_source::capture_local_get(
                            input, caller, binding, selected, batch, constructors, array_i64_fields)?;
                        original_array_locals.insert(binding, read);
                    }
                    original_array_locals.get(&binding).cloned().flatten()
                } else {
                    None
                };
                array_reads.push(read);
            }
            let mut seeds = Vec::with_capacity(original.arguments().len());
            for (ordinal, argument) in original.arguments().iter().enumerate() {
                let retained = claims
                    .map(|claims| {
                        constructors.provider_static_argument_source_v1(
                            input, original, ordinal, claims, selected,
                        )
                    })
                    .transpose()?
                    .flatten();
                seeds.push(match retained {
                    Some(loan) => {
                        if !loan.same_catalog(claims.expect("source lender requires index")) {
                            return Err(freeze("provider-static/source-catalog"));
                        }
                        OriginalFormalActualSeedV1::ProviderStatic(loan)
                    }
                    None => classify(argument),
                });
            }
            self.rows.push(OriginalNewCallSeedsV1 {
                local_uses: initializers.get(original.site()).copied().flatten().and_then(|init| {
                    let binding = init.binding();
                    (ledger_ok && binding.owner() == input.owner()
                        && matches!(function.binding(binding).map(|record| record.kind()), Some(BindingKindV1::Local { .. }))
                        && function.declaration_binding(init.declaration_site()) == Some(binding)
                        && counts.get(&binding) == Some(&1) && !rebindings.contains(&binding) && !captures.contains(&binding))
                        .then(|| OriginalNewLocalUsesV1 { initializer: init.clone(), reads: local_reads.get(&binding).cloned().unwrap_or_default() })
                }),
                site,
                original: original.clone(),
                birth,
                seeds: seeds.into_boxed_slice(),
                fields,
                array_reads: array_reads.into_boxed_slice(),
            });
        }
        Ok(())
    }

    pub(super) fn corroborate_provider_source_v1(
        &self,
        provider: &std::rc::Rc<
            crate::mir::normal_callable_semantic_package::ProviderStaticArgumentSourceV1,
        >,
    ) -> Result<(), String> {
        let mut matching = self.rows.iter().flat_map(|row| {
            row.seeds
                .iter()
                .enumerate()
                .filter_map(move |(ordinal, seed)| match seed {
                    OriginalFormalActualSeedV1::ProviderStatic(held)
                        if std::rc::Rc::ptr_eq(held, provider) =>
                    {
                        Some((row, ordinal))
                    }
                    _ => None,
                })
        });
        let Some((row, ordinal)) = matching.next() else {
            return Err(freeze("provider-static/original-new-source-missing"));
        };
        if matching.next().is_some()
            || !provider.matches_original(&row.site, &row.original, ordinal)
        {
            return Err(freeze("provider-static/original-new-source-drift"));
        }
        Ok(())
    }

    pub(super) fn retained_provider_sources_match(&self) -> bool {
        self.rows.iter().all(|row| {
            row.seeds.len() == row.original.arguments().len()
                && row
                    .seeds
                    .iter()
                    .enumerate()
                    .all(|(ordinal, seed)| match seed {
                        OriginalFormalActualSeedV1::ProviderStatic(loan) => {
                            loan.matches_original(&row.site, &row.original, ordinal)
                        }
                        _ => true,
                    })
        })
    }

    pub(super) fn current_provider_sources_match(
        &self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        self.retained_provider_sources_match()
            && self
                .rows
                .iter()
                .flat_map(|row| row.seeds.iter())
                .all(|seed| match seed {
                    OriginalFormalActualSeedV1::ProviderStatic(loan) => {
                        loan.matches_constructor(constructors)
                    }
                    _ => true,
                })
    }

    /// Lend SAME unprojected rows. Missing/invalid/empty participants are
    /// explicit None obligations; no consumer may silently zip them away.
    pub(super) fn visit_raw_source_seeds_v1(
        &self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        mut visit: impl FnMut(BindingRefV1, Option<&OriginalFormalActualSeedV1>),
    ) {
        let mut observed = BTreeSet::new();
        let mut vetoed_classes = BTreeSet::new();
        for row in &self.rows {
            match &row.birth {
                OriginalNewBirthV1::Birth { formals, .. }
                    if row.corroborates_birth_v1(constructors) =>
                {
                    for (formal, seed) in formals.iter().zip(&row.seeds) {
                        observed.insert(*formal);
                        visit(*formal, Some(seed));
                    }
                }
                OriginalNewBirthV1::Birth { .. } | OriginalNewBirthV1::Unavailable => {
                    vetoed_classes.insert(row.original.class());
                }
                OriginalNewBirthV1::NoBirth | OriginalNewBirthV1::OutsideOrdinary => {}
            }
        }
        for ctor in constructors
            .rows()
            .iter()
            .filter(|row| row.kind() == ConstructorSourceKindV1::Birth)
        {
            for formal in ctor.formal_contracts() {
                if self.incomplete
                    || vetoed_classes.contains(ctor.box_name())
                    || !observed.contains(&formal.binding())
                {
                    visit(formal.binding(), None);
                }
            }
        }
    }

    /// Join all original actuals, including failures, in the existing domain
    /// fold. No selected claim argument or birth_site_index is consulted.
    pub(super) fn append_source_seeds(
        &self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        seeds: &mut BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>>,
        current_i64: bool,
        objects: bool,
    ) {
        let mut incoming: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = constructors
            .rows()
            .iter()
            .filter(|row| row.kind() == ConstructorSourceKindV1::Birth)
            .flat_map(|row| {
                row.formal_contracts()
                    .iter()
                    .map(|formal| (formal.binding(), Vec::new()))
            })
            .collect();
        let mut vetoes = BTreeSet::new();
        for row in &self.rows {
            match &row.birth {
                OriginalNewBirthV1::Birth { formals, .. } => {
                    if !row.corroborates_birth_v1(constructors) {
                        vetoes.insert(row.original.class());
                        continue;
                    }
                    for (ordinal, (formal, seed)) in
                        formals.iter().zip(row.seeds.iter()).enumerate()
                    {
                        let seed = match seed {
                            OriginalFormalActualSeedV1::ProviderStatic(loan)
                                if loan.matches_original(&row.site, &row.original, ordinal)
                                    && loan.matches_constructor(constructors) =>
                            {
                                FormalActualSeedV1::Integer
                            }
                            _ if objects => seed.project_source_object_v1(),
                            _ => seed.project_source_integer_v1(current_i64),
                        };
                        incoming.entry(*formal).or_default().push(seed);
                    }
                }
                OriginalNewBirthV1::Unavailable => {
                    vetoes.insert(row.original.class());
                }
                OriginalNewBirthV1::NoBirth | OriginalNewBirthV1::OutsideOrdinary => {}
            }
        }
        for ctor in constructors
            .rows()
            .iter()
            .filter(|row| row.kind() == ConstructorSourceKindV1::Birth)
        {
            for formal in ctor.formal_contracts() {
                let rows = incoming
                    .get_mut(&formal.binding())
                    .expect("original Birth formal");
                if self.incomplete || vetoes.contains(ctor.box_name()) || rows.is_empty() {
                    rows.push(FormalActualSeedV1::Conflict);
                }
            }
        }
        for (formal, rows) in incoming {
            seeds.entry(formal).or_default().extend(rows);
        }
    }
}

/// Constructor formals must be grounded by original New actuals, never their
/// annotation. Stable alias hops retain the exact formal binding as Forward.
pub(super) fn stable_constructor_formal_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Option<BindingRefV1> {
    let formals = constructors
        .rows()
        .iter()
        .find(|row| {
            row.kind() == ConstructorSourceKindV1::Birth && row.forest().roots() == [input.owner()]
        })?
        .formal_contracts();
    let mut binding = arg_site_binding(input, site)?;
    let mut seen = BTreeSet::new();
    loop {
        if !seen.insert(binding) || binding.owner() != input.owner()
            || input.function().assignment_targets().any(|(_, target)|
                matches!(target, crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding))
            || input.forest().semantic_owners().any(|(_, product)|
                product.assignment_targets().any(|(_, target)|
                    matches!(target, crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == binding))
                || product.variable_refs().any(|(_, reference)|
                    matches!(reference, crate::mir::resolved_semantics::ResolvedLexicalRefV1::Upvar(upvar) if upvar.source() == binding)))
        { return None; }
        if formals.iter().any(|formal| formal.binding() == binding) {
            return Some(binding);
        }
        binding = alias_source_binding(input, binding)?;
    }
}

#[cfg(test)]
impl OriginalNewSeedsV1 {
    pub(super) fn corrupted_provider_evidence_for_test(&self, erase_rows: bool) -> Self {
        let mut changed = self.clone();
        if erase_rows {
            changed.rows.clear();
        } else {
            for seed in changed.rows.iter_mut().flat_map(|row| row.seeds.iter_mut()) {
                if matches!(seed, OriginalFormalActualSeedV1::ProviderStatic(_)) {
                    *seed = OriginalFormalActualSeedV1::Conflict;
                }
            }
        }
        changed
    }
}

// The original source owner filters its own candidate projections. The caller
// retains responsibility for rebuilding the inventory to a fixed point.
impl super::OriginalBorrowedFormalSourceV1 {
    /// Issuance borrows the original all-source product, not a saved admission bit.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn matches_declared_receiver_v1(
        &self, reference: &super::super::source::CallTargetReferenceV1,
        receiver: &super::super::source::StoredReceiverSourceV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        let super::super::source::StoredReceiverSourceOriginV1::ProvisionalDeclared(pair) = &receiver.origin else { return false; };
        pair.recheck_v1(facts, &receiver.field_name)
            && self.inventory.source_rows_v1().any(|row|
                row.source.original_instance_reference_v1().as_ref() == Some(reference)
                    && row.source.original_stored_receiver_v1() == Some(receiver))
            && facts.child_construction_sites_v1(pair.field().object()).is_some_and(|sites|
                self.child_sites_match_v1(&receiver.parent_class, sites, pair.field().object(),
                    reference.callee_owner, facts, batch, constructors))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn filter_declared_child_candidates_v1(
        &self,
        rows: &mut [Result<Option<super::super::source::PreparedSourceCallNeedV1>, String>],
        callable_result_classes: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        let mut source_candidates_changed = false;
        let mut checked_graphs = BTreeMap::new();
        for row in rows.iter_mut() {
            let keep = match row.as_ref().ok().and_then(Option::as_ref) {
                Some(super::super::source::PreparedSourceCallNeedV1::Stored { reference, receiver, .. }) => match &receiver.origin {
                    super::super::source::StoredReceiverSourceOriginV1::ProvisionalDeclared(pair) =>
                        pair.recheck_v1(callable_result_classes, &receiver.field_name)
                            && callable_result_classes.child_construction_sites_v1(pair.field().object())
                                .is_some_and(|sites| *checked_graphs.entry((receiver.parent_class.clone(), pair.field().object(), reference.callee_owner))
                                    .or_insert_with(|| self.child_sites_match_v1(&receiver.parent_class, sites,
                                        pair.field().object(), reference.callee_owner, callable_result_classes, batch, constructors))),
                    _ => true,
                },
                _ => true,
            };
            if !keep { *row = Ok(None); source_candidates_changed = true; }
        }
        source_candidates_changed
    }
}

impl super::OriginalFormalSeedProductV1 {
    /// Preflight against the same pending inventory before any selected-row
    /// mutation. No saved Boolean can substitute a different source product.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_stored_selection_v1(
        &self,
        inventory: &BorrowedIncomingInventoryV1,
        incoming: &BorrowedIncomingSourceV1,
        target: &super::super::LexicalInstanceCallSourceTargetV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        use super::super::source::StoredReceiverSourceOriginV1;
        let Some(receiver) = incoming.original_stored_receiver_v1() else { return false; };
        let StoredReceiverSourceOriginV1::ProvisionalDeclared(pair) = &receiver.origin else { return false; };
        if !self.corroborate_original_v1(inventory)
            || !inventory.source_rows_v1().any(|row| &row.call == target.call_site()
                && row.callee == target.callee_owner() && row.source.same_source(incoming))
            || !pair.recheck_v1(facts, &receiver.field_name)
            || target.stored_receiver().is_none_or(|(binding, site, field, child)|
                binding != receiver.parent_binding || site != &receiver.parent_site
                    || field != pair.field() || child != pair.child()) { return false; }
        let Some(sites) = facts.child_construction_sites_v1(pair.field().object()) else { return false; };
        self.new_calls.class_local_uses_v1(&receiver.parent_class, sites, pair.field().object(),
            target.callee_owner(), inventory, facts, batch, constructors)
    }
}
