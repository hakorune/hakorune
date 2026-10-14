//! Original New inputs retained by the SAME early formal seed product.
//! Every resolver owner participates. This is source agreement only, not Birth
//! execution, argument ABI, construction readiness or a numeric field invariant.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{ResolvedNewExpressionSourceV1, SourceBindingSiteV1};
use crate::parser::{ConstructorSourceIdV1, ConstructorSourceKindV1};

#[derive(Debug)]
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
struct OriginalNewCallSeedsV1 {
    site: OwnedExprSiteV1,
    original: ResolvedNewExpressionSourceV1,
    birth: OriginalNewBirthV1,
    seeds: Box<[OriginalFormalActualSeedV1]>,
}

#[derive(Debug, Default)]
pub(super) struct OriginalNewSeedsV1 {
    rows: Vec<OriginalNewCallSeedsV1>,
    incomplete: bool,
}
impl OriginalNewSeedsV1 {
    pub(super) fn missing_owner(&mut self) { self.incomplete = true; }

    pub(super) fn observe(
        &mut self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        mut classify: impl FnMut(&SourceExprSiteV1) -> OriginalFormalActualSeedV1,
    ) {
        if input.body_shape().is_none() { self.incomplete = true; }
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
            self.rows.push(OriginalNewCallSeedsV1 {
                site, original: original.clone(), birth,
                seeds: original.arguments().iter().map(&mut classify).collect(),
            });
        }
    }

    /// Join all original actuals, including failures, in the existing domain
    /// fold. No selected claim argument or birth_site_index is consulted.
    pub(super) fn append_source_seeds(
        &self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        seeds: &mut BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>>,
    ) {
        let mut incoming: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = constructors.rows().iter()
            .filter(|row| row.kind() == ConstructorSourceKindV1::Birth)
            .flat_map(|row| row.formal_contracts().iter().map(|formal| (formal.binding(), Vec::new())))
            .collect();
        let mut vetoes = BTreeSet::new();
        for row in &self.rows {
            match &row.birth {
                OriginalNewBirthV1::Birth { source, owner, formals } => {
                    let mut matching = constructors.rows().iter().filter(|ctor| ctor.source_id() == source);
                    let ctor = matching.next();
                    let exact = matching.next().is_none() && ctor.is_some_and(|ctor|
                        ctor.kind() == ConstructorSourceKindV1::Birth && ctor.box_name() == row.original.class()
                        && ctor.forest().roots() == [*owner]
                        && ctor.formal_contracts().iter().map(|formal| formal.binding()).eq(formals.iter().copied()));
                    if !exact || row.seeds.len() != row.original.arguments().len()
                        || row.seeds.len() != formals.len()
                    { vetoes.insert(row.original.class()); continue; }
                    for (formal, seed) in formals.iter().zip(row.seeds.iter()) {
                        incoming.entry(*formal).or_default().push(seed.project_source_integer_v1());
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
