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
    birth: OriginalNewBirthV1,
    seeds: Box<[OriginalFormalActualSeedV1]>,
    fields: Box<[Option<OriginalNewFieldActualV1>]>,
    array_reads: Box<[Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>]>,
}

impl OriginalNewCallSeedsV1 {
    fn corroborates_birth_v1(&self, constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1) -> bool {
        let OriginalNewBirthV1::Birth { source, owner, formals } = &self.birth else { return false; };
        let mut matching = constructors.rows().iter().filter(|ctor| ctor.source_id() == source);
        let ctor = matching.next();
        matching.next().is_none() && ctor.is_some_and(|ctor|
            ctor.kind() == ConstructorSourceKindV1::Birth && ctor.box_name() == self.original.class()
                && ctor.forest().roots() == [*owner]
                && ctor.formal_contracts().iter().map(|formal| formal.binding()).eq(formals.iter().copied()))
            && self.seeds.len() == self.original.arguments().len() && self.seeds.len() == formals.len()
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub(super) struct OriginalNewSeedsV1 {
    rows: Vec<OriginalNewCallSeedsV1>,
    incomplete: bool,
}
impl OriginalNewSeedsV1 {
    /// Field-base evidence must retain all original construction obligations.
    /// Override typing is not supplied by the constructor-argument projection.
    pub(super) fn field_bases_available(&self, class: &str) -> bool {
        !self.incomplete && self.rows.iter().filter(|row| row.original.class() == class).all(|row|
            matches!(row.birth, OriginalNewBirthV1::Birth { .. } | OriginalNewBirthV1::NoBirth)
                && row.original.field_initializers().is_empty())
    }

    pub(super) fn missing_owner(&mut self) { self.incomplete = true; }

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
        if input.body_shape().is_none() { self.incomplete = true; }
        let mut original_array_locals = BTreeMap::new();
        let caller = batch.declarations().find(|row| row.owner() == input.owner())
            .and_then(|row| match selected.key_for_batch_slot(row.batch_slot()) {
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key),
                _ => None,
            });
        for original in input.function().expression_source().constructions() {
            let site = OwnedExprSiteV1::new(input.owner(), original.site().clone());
            if self.rows.iter().any(|row| row.site == site) { self.incomplete = true; }
            let birth = match batch.ordinary_box_coverage().row_for(original.class()) {
                Ok(Some(source)) => match constructors.birth_for(source, original.arguments().len()) {
                    Ok(Some(row)) => {
                        match row.forest().roots() {
                            [owner] => {
                                let formals = row.formal_contracts();
                                let function = row.forest().owner(*owner);
                                if formals.len() != original.arguments().len()
                                    || !formals.iter().enumerate().all(|(index, formal)| {
                                        formal.ordinal() as usize == index && formal.binding().owner() == *owner
                                            && function.is_some_and(|function| function.declaration_binding(
                                                &SourceBindingSiteV1::Parameter { index: formal.ordinal() }) == Some(formal.binding()))
                                    })
                                { OriginalNewBirthV1::Unavailable }
                                else { OriginalNewBirthV1::Birth {
                                    source: row.source_id().clone(), owner: *owner,
                                    formals: formals.iter().map(|formal| formal.binding()).collect(),
                                } }
                            }
                            _ => OriginalNewBirthV1::Unavailable,
                        }
                    }
                    Ok(None) => OriginalNewBirthV1::NoBirth,
                    Err(_) => OriginalNewBirthV1::Unavailable,
                },
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
                } else { None };
                array_reads.push(read);
            }
            let mut seeds = Vec::with_capacity(original.arguments().len());
            for (ordinal, argument) in original.arguments().iter().enumerate() {
                let retained = claims.map(|claims| constructors.provider_static_argument_source_v1(
                    input, original, ordinal, claims, selected)).transpose()?.flatten();
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
                site, original: original.clone(), birth, seeds: seeds.into_boxed_slice(), fields,
                array_reads: array_reads.into_boxed_slice(),
            });
        }
        Ok(())
    }

    pub(super) fn corroborate_provider_source_v1(
        &self,
        provider: &std::rc::Rc<crate::mir::normal_callable_semantic_package::ProviderStaticArgumentSourceV1>,
    ) -> Result<(), String> {
        let mut matching = self.rows.iter().flat_map(|row| row.seeds.iter().enumerate()
            .filter_map(move |(ordinal, seed)| match seed {
                OriginalFormalActualSeedV1::ProviderStatic(held)
                    if std::rc::Rc::ptr_eq(held, provider) => Some((row, ordinal)),
                _ => None,
            }));
        let Some((row, ordinal)) = matching.next() else {
            return Err(freeze("provider-static/original-new-source-missing"));
        };
        if matching.next().is_some() || !provider.matches_original(&row.site, &row.original, ordinal) {
            return Err(freeze("provider-static/original-new-source-drift"));
        }
        Ok(())
    }

    pub(super) fn retained_provider_sources_match(&self) -> bool {
        self.rows.iter().all(|row| row.seeds.len() == row.original.arguments().len()
            && row.seeds.iter().enumerate().all(|(ordinal, seed)| match seed {
                OriginalFormalActualSeedV1::ProviderStatic(loan) => loan.matches_original(&row.site, &row.original, ordinal),
                _ => true,
            }))
    }

    pub(super) fn current_provider_sources_match(&self, constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1) -> bool {
        self.retained_provider_sources_match() && self.rows.iter().flat_map(|row| row.seeds.iter()).all(|seed| match seed {
            OriginalFormalActualSeedV1::ProviderStatic(loan) => loan.matches_constructor(constructors),
            _ => true,
        })
    }

    /// Lend SAME unprojected rows. Missing/invalid/empty participants are
    /// explicit None obligations; no consumer may silently zip them away.
    pub(super) fn visit_raw_source_seeds_v1(
        &self, constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        mut visit: impl FnMut(BindingRefV1, Option<&OriginalFormalActualSeedV1>),
    ) {
        let mut observed = BTreeSet::new();
        let mut vetoed_classes = BTreeSet::new();
        for row in &self.rows {
            match &row.birth {
                OriginalNewBirthV1::Birth { formals, .. } if row.corroborates_birth_v1(constructors) => {
                    for (formal, seed) in formals.iter().zip(&row.seeds) {
                        observed.insert(*formal);
                        visit(*formal, Some(seed));
                    }
                }
                OriginalNewBirthV1::Birth { .. } | OriginalNewBirthV1::Unavailable => { vetoed_classes.insert(row.original.class()); }
                OriginalNewBirthV1::NoBirth | OriginalNewBirthV1::OutsideOrdinary => {}
            }
        }
        for ctor in constructors.rows().iter().filter(|row| row.kind() == ConstructorSourceKindV1::Birth) {
            for formal in ctor.formal_contracts() {
                if self.incomplete || vetoed_classes.contains(ctor.box_name()) || !observed.contains(&formal.binding()) {
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
        let mut incoming: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = constructors.rows().iter()
            .filter(|row| row.kind() == ConstructorSourceKindV1::Birth)
            .flat_map(|row| row.formal_contracts().iter().map(|formal| (formal.binding(), Vec::new())))
            .collect();
        let mut vetoes = BTreeSet::new();
        for row in &self.rows {
            match &row.birth {
                OriginalNewBirthV1::Birth { formals, .. } => {
                    if !row.corroborates_birth_v1(constructors) { vetoes.insert(row.original.class()); continue; }
                    for (ordinal, (formal, seed)) in formals.iter().zip(row.seeds.iter()).enumerate() {
                        let seed = match seed {
                            OriginalFormalActualSeedV1::ProviderStatic(loan)
                                if loan.matches_original(&row.site, &row.original, ordinal)
                                    && loan.matches_constructor(constructors) => FormalActualSeedV1::Integer,
                            _ if objects => seed.project_source_object_v1(),
                            _ => seed.project_source_integer_v1(current_i64),
                        };
                        incoming.entry(*formal).or_default().push(seed);
                    }
                }
                OriginalNewBirthV1::Unavailable => { vetoes.insert(row.original.class()); }
                OriginalNewBirthV1::NoBirth | OriginalNewBirthV1::OutsideOrdinary => {}
            }
        }
        for ctor in constructors.rows().iter().filter(|row| row.kind() == ConstructorSourceKindV1::Birth) {
            for formal in ctor.formal_contracts() {
                let rows = incoming.get_mut(&formal.binding()).expect("original Birth formal");
                if self.incomplete || vetoes.contains(ctor.box_name()) || rows.is_empty() {
                    rows.push(FormalActualSeedV1::Conflict);
                }
            }
        }
        for (formal, rows) in incoming { seeds.entry(formal).or_default().extend(rows); }
    }
}

/// Constructor formals must be grounded by original New actuals, never their
/// annotation. Stable alias hops retain the exact formal binding as Forward.
pub(super) fn stable_constructor_formal_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> Option<BindingRefV1> {
    let formals = constructors.rows().iter().find(|row|
        row.kind() == ConstructorSourceKindV1::Birth && row.forest().roots() == [input.owner()])?.formal_contracts();
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
        if formals.iter().any(|formal| formal.binding() == binding) { return Some(binding); }
        binding = alias_source_binding(input, binding)?;
    }
}

#[cfg(test)]
impl OriginalNewSeedsV1 {
    pub(super) fn corrupted_provider_evidence_for_test(&self, erase_rows: bool) -> Self {
        let mut changed = self.clone();
        if erase_rows { changed.rows.clear(); }
        else {
            for seed in changed.rows.iter_mut().flat_map(|row| row.seeds.iter_mut()) {
                if matches!(seed, OriginalFormalActualSeedV1::ProviderStatic(_)) {
                    *seed = OriginalFormalActualSeedV1::Conflict;
                }
            }
        }
        changed
    }
}
