//! Ordinary borrowed transport selection over the prepared lexical source rows.
//! These rows precede live-actual proof and never install a physical carrier.
use super::borrowed_formal_uses::*;
use super::*;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use std::collections::{BTreeMap, BTreeSet};

#[path = "ordinary_new_borrowed_formal_source_drafts.rs"]
mod source_drafts;

pub(super) use source_drafts::collect_borrowed_source_drafts_v1;

/// The sealed class view one borrowed formal may carry on a dominated
/// `formal.field` read: minted only when every incoming actual names one
/// agreed ordinary class (the exact `null` literal is always admissible —
/// it proves `Void`, never a class). Mixed classes, unsupported actual
/// shapes, or unresolvable forward chains mint no row and the read stays
/// fail-closed. `object` is the canonical definition the physical param
/// row corroborates, resolved through the same ordinary-box coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedFormalObjectViewV1 {
    class: Box<str>,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    /// Original DeclaredObject constraint, not inferred from incoming class.
    declared: bool,
}

impl BorrowedFormalObjectViewV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str {
        &self.class
    }

    pub(in crate::mir::normal_callable_semantic_package) fn is_declared(&self) -> bool {
        self.declared
    }

    pub(in crate::mir::normal_callable_semantic_package) fn object(
        &self,
    ) -> hakorune_mir_defs::CanonicalObjectIdV1 {
        self.object
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct PreparedBorrowedFormalIngressV1 {
    pub(super) definitions: BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    /// Dominated-view value-use sites across every classified owner —
    /// `ArrayElementValue`, `AddOperand`, or `NewArgument` rows recorded
    /// at draft classification time, before borrowed-transport selection.
    /// Owners whose only incoming edges are non-lexical (for example an
    /// `me.<field>` receiver call) are never part of the borrowed-entry
    /// profile, yet their sealed drafts still admit these value uses —
    /// the dominated view answers a source-classification question, not
    /// a transport-selection one.
    pub(super) dominated_view_sites: BTreeSet<OwnedExprSiteV1>,
    pub(super) forwards: Box<[BorrowedForwardUseDraftRowV1]>,
    pub(super) incoming: Box<[BorrowedIncomingCallDraftV1]>,
    /// Raw source facts survive transport pruning; these grant no execution.
    pub(in crate::mir::normal_callable_semantic_package) static_arguments: BTreeMap<(OwnedExprSiteV1, u32), super::borrowed_static_argument::QualifiedStaticArgumentSourceV1>,
    pub(in crate::mir::normal_callable_semantic_package) static_observations: BTreeMap<OwnedExprSiteV1, Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::QualifiedStaticIncomingSourceV1>, String>>,
    /// `formal -> sealed class view`, complete across the co-sealed
    /// incoming call set. A formal absent from the map has no agreed
    /// class — its guarded field read stays fail-closed.
    pub(super) object_views: BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
}

/// One incoming argument's contribution to a callee formal's class view.
pub(super) enum FormalActualSeedV1 {
    /// A class-carrying actual: claim-local `new` local, the entry
    /// receiver, a sealed received nullable, or a resolved forward.
    Class(Box<str>),
    /// The exact `null` literal — proves `Void`, never a class.
    Null,
    /// The caller's own opaque formal (or its copy) forwarded onward —
    /// the class resolves to that formal's view, possibly pending.
    Forward(BindingRefV1),
    /// Scalar actual, unproven/foreign binding, or inline construction —
    /// the formal's view mints no row.
    Conflict,
}

/// Only complete transport-use profiles are candidates. An ordinary source
/// use outside this profile is not an attempted borrowed ABI followed by a
/// fallback. Source identity corruption is never classified as profile-outside.
// Test adapter for direct ingress corruption; production uses the unified profile.
#[cfg(test)]
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_formal_ingress_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    prepared: &PreparedLexicalInstanceCallSourceTargetsV1,
    app_main_slot: Option<u32>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::candidate::OrdinaryNewCandidate>,
            super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let mut calls = BTreeMap::new();
    for row in prepared.as_ref().map_err(Clone::clone)? {
        if let Some(row) = row.as_ref().map_err(Clone::clone)? {
            if calls.insert(row.call_site().clone(), row).is_some() {
                return Err(freeze("borrowed-formal/duplicate-source-call"));
            }
        }
    }
    let drafts = collect_borrowed_source_drafts_v1(
        batch,
        selected,
        contracts,
        app_main_slot,
        dynamic_slot,
        entry_home_loans,
        instance_constructors,
    )?;
    finish_ingress_from_drafts_v1(
        batch,
        selected,
        contracts,
        entry_home_loans,
        instance_constructors,
        local_candidates,
        callable_result_classes,
        calls,
        None,
        None,
        drafts,
    )
}

/// Consume the original drafts once; never reconstruct them after profile selection.
#[allow(clippy::too_many_arguments)]
pub(super) fn finish_ingress_from_drafts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::candidate::OrdinaryNewCandidate>,
            super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    calls: BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
    drafts: (
        BTreeSet<FunctionOwnerIdV1>,
        BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        BTreeSet<OwnedExprSiteV1>,
    ),
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let (ordinary_callers, mut definitions, dominated_view_sites) = drafts;
    let static_arguments = super::borrowed_static_argument::collect_static_argument_sources_v1(
        batch, selected, contracts, &definitions, static_claims, app_main,
    )?;
    let static_context = static_claims.map(|claims| StaticIncomingContextV1 {
        claims, arguments: &static_arguments, main: app_main,
    });
    // Close the finite graph before selection. Removing one outside-profile
    // destination invalidates every source that forwards an opaque value to it.
    // Repetition terminates because every nonfinal pass removes an owner.
    loop {
        let mut outside = BTreeSet::new();
        for (owner, draft) in &definitions {
            if !calls.values().any(|call| call.callee_owner() == *owner) {
                outside.insert(*owner);
            }
            for row in &draft.uses {
                let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &row.kind
                else {
                    continue;
                };
                let Some(target) = calls.get(call) else {
                    outside.insert(*owner);
                    continue;
                };
                let contract = contracts
                    .iter()
                    .find(|contract| contract.owner == target.callee_owner());
                let Some(contract) = contract else {
                    return Err(freeze("borrowed-formal/missing-source-contract"));
                };
                if contract.batch_slot != target.target_batch_slot()
                    || row.site.site()
                        != target
                            .argument_sites()
                            .get(*ordinal as usize)
                            .ok_or_else(|| freeze("borrowed-formal/argument-ordinal"))?
                {
                    return Err(freeze("borrowed-formal/forward-source-identity"));
                }
                if !definitions.contains_key(&contract.owner)
                    || contract
                        .parameters
                        .get(*ordinal as usize)
                        .is_none_or(|formal| !formal.kind.is_ordinary_borrowed_handle())
                {
                    outside.insert(*owner);
                }
            }
        }
        if outside.is_empty() {
            break;
        }
        for owner in outside {
            definitions.remove(&owner);
        }
    }
    let forwards = join_borrowed_forward_uses_v1(&definitions, contracts, &calls)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/forward-coverage")))?;
    // After profile selection, unresolved or outside-scope incoming calls are
    // named terminals. Never remove the selected definition to regain old ABI.
    let inventory = inventory_borrowed_incoming_calls_v1(
        batch,
        selected,
        &definitions,
        contracts,
        &calls,
        &ordinary_callers,
        static_context.as_ref(),
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    let incoming = inventory.incoming;
    let static_observations = inventory.static_observations;
    let object_views = prepare_borrowed_formal_object_views_v1(
        batch,
        selected,
        entry_home_loans,
        instance_constructors,
        callable_result_classes,
        local_candidates,
        contracts,
        &definitions,
        &incoming,
    )?;
    Ok(PreparedBorrowedFormalIngressV1 {
        definitions,
        dominated_view_sites,
        forwards,
        incoming,
        static_arguments,
        static_observations,
        object_views,
    })
}

/// Co-seal every borrowed formal's object view before the declaration
/// walk: classify each incoming argument's exact source expression — never
/// MIR types or runtime layout — then agree the classes per callee formal
/// across all calls, resolving forwarded chains to a fixed point. This
/// mints admission evidence only; the physical param row still must carry
/// the canonical object id it claims.
#[allow(clippy::too_many_arguments)]
fn prepare_borrowed_formal_object_views_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::candidate::OrdinaryNewCandidate>,
            super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    incoming: &[BorrowedIncomingCallDraftV1],
) -> Result<BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>, String> {
    let slots: BTreeMap<FunctionOwnerIdV1, u32> = batch
        .declarations()
        .map(|row| (row.owner(), row.batch_slot()))
        .collect();
    let mut seeds: BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>> = BTreeMap::new();
    let mut declared = BTreeSet::new();
    for contract in contracts
        .iter()
        .filter(|row| definitions.contains_key(&row.owner))
    {
        for formal in &contract.parameters {
            if let CallableParameterContractKindV1::DeclaredObject(class) = &formal.kind {
                declared.insert(formal.binding);
                seeds
                    .entry(formal.binding)
                    .or_default()
                    .push(FormalActualSeedV1::Class(class.clone()));
            }
        }
    }
    for call in incoming {
        let caller = call.call.owner();
        let caller_slot = slots
            .get(&caller)
            .ok_or_else(|| freeze("borrowed-view/caller-slot"))?;
        let candidates = local_candidates
            .get(caller_slot)
            .and_then(|rows| rows.as_ref().ok())
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let receiver =
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                selected,
                batch,
                entry_home_loans.for_batch_slot(*caller_slot),
                *caller_slot,
            );
        let arguments: Vec<_> = call.arguments.to_vec();
        batch
            .with_lowering_input(*caller_slot, |input| {
                for (_, site, formal) in &arguments {
                    seeds.entry(*formal).or_default().push(classify_actual_seed(
                        input,
                        batch,
                        selected,
                        callable_result_classes,
                        caller,
                        candidates,
                        receiver,
                        definitions,
                        contracts,
                        site,
                    ));
                }
            })
            .map_err(|_| freeze("borrowed-view/caller-loan"))?;
    }
    // `Some(view)` proves agreement; `None` records a proven refusal so
    // dependent forwards decline instead of waiting forever.
    let mut resolved: BTreeMap<BindingRefV1, Option<BorrowedFormalObjectViewV1>> = BTreeMap::new();
    loop {
        let mut progressed = false;
        for (formal, rows) in &seeds {
            if resolved.contains_key(formal) {
                continue;
            }
            let mut class: Option<Box<str>> = None;
            let mut blocked = false;
            let mut declined = false;
            for seed in rows {
                match seed {
                    FormalActualSeedV1::Null => {}
                    FormalActualSeedV1::Class(name) => {
                        declined |= class.as_ref().is_some_and(|prev| prev != name);
                        class = class.or_else(|| Some(name.clone()));
                    }
                    FormalActualSeedV1::Forward(origin) => match resolved.get(origin) {
                        None => blocked = true,
                        Some(None) => declined = true,
                        Some(Some(view)) => {
                            declined |= class.as_ref().is_some_and(|prev| *prev != view.class);
                            class = class.or_else(|| Some(view.class.clone()));
                        }
                    },
                    FormalActualSeedV1::Conflict => declined = true,
                }
            }
            if declined {
                resolved.insert(*formal, None);
                progressed = true;
            } else if !blocked {
                let view =
                    class.and_then(|name| object_view_for(batch, instance_constructors, &name));
                resolved.insert(*formal, view);
                progressed = true;
            }
        }
        if !progressed {
            // Remaining rows are cycles of unresolved forwards — they carry
            // no seed of their own and decline like an all-null formal.
            for formal in seeds.keys() {
                resolved.entry(*formal).or_insert(None);
            }
            break;
        }
    }
    for formal in &declared {
        let view = resolved
            .get_mut(formal)
            .and_then(Option::as_mut)
            .ok_or_else(|| freeze("borrowed-view/declared-object-class"))?;
        view.declared = true;
    }
    Ok(resolved
        .into_iter()
        .filter_map(|(formal, view)| view.map(|view| (formal, view)))
        .collect())
}

/// One incoming argument's exact source class evidence. The walk mirrors
/// the caller's own stored-local provenance: a `new` destination names the
/// claim-local candidate class, the entry receiver names the owner box, a
/// received nullable names its sealed `NullableObject` claim, and a
/// formal-rooted binding forwards to the caller's own view. Anything else
/// conflicts — a scalar, foreign, or unproven actual can never carry a
/// borrowed object class.
#[allow(clippy::too_many_arguments)]
pub(super) fn classify_actual_seed(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    caller: FunctionOwnerIdV1,
    candidates: &[super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &SourceExprSiteV1,
) -> FormalActualSeedV1 {
    let function = input.function();
    match function.expression_source().literal(site) {
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null) => {
            return FormalActualSeedV1::Null;
        }
        Some(_) => return FormalActualSeedV1::Conflict,
        None => {}
    }
    let Some(mut binding) = arg_site_binding(input, site) else {
        return FormalActualSeedV1::Conflict;
    };
    let mut visited = BTreeSet::new();
    loop {
        if binding.owner() != caller || !visited.insert(binding) {
            return FormalActualSeedV1::Conflict;
        }
        if let Some(candidate) = candidates
            .iter()
            .find(|candidate| candidate.destination == binding)
        {
            return if candidate.construction.is_ok() {
                FormalActualSeedV1::Class(candidate.class.clone())
            } else {
                FormalActualSeedV1::Conflict
            };
        }
        if let Some((root, row)) = receiver {
            if root == binding {
                return FormalActualSeedV1::Class(row.name().into());
            }
        }
        if let Some(class) =
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue::nullable_received_result_class(
                selected,
                batch,
                callable_result_classes,
                candidates,
                input,
                binding,
            )
        {
            return FormalActualSeedV1::Class(class);
        }
        if let Some(formal) = definitions
            .get(&caller)
            .and_then(|draft| draft.origins.get(&binding))
        {
            if let Some(class) = contracts
                .iter()
                .find(|row| row.owner == caller)
                .and_then(|row| row.parameters.iter().find(|row| row.binding == *formal))
                .and_then(|row| match &row.kind {
                    CallableParameterContractKindV1::DeclaredObject(class) => Some(class),
                    _ => None,
                })
            {
                return FormalActualSeedV1::Class(class.clone());
            }
            return FormalActualSeedV1::Forward(*formal);
        }
        match alias_source_binding(input, binding) {
            Some(next) => binding = next,
            None => return FormalActualSeedV1::Conflict,
        }
    }
}

/// The binding an argument site names: a local variable ref first, then a
/// `me` shape row's lexical receiver.
pub(super) fn arg_site_binding(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Option<BindingRefV1> {
    if let Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) =
        input.function().variable_ref(site)
    {
        return Some(binding);
    }
    match input
        .body_shape()
        .and_then(|shape| shape.expression_shape(site))
    {
        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
            receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding),
            ..
        }) => Some(*binding),
        _ => None,
    }
}

/// One alias hop for `local q = <binding>`: the initializer declaring
/// `binding` names its own source binding. Foreign, double-declared or
/// non-variable initializers answer `None` and the walk conflicts.
fn alias_source_binding(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> Option<BindingRefV1> {
    let function = input.function();
    let mut matching = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let initializer = matching.next()?;
    if matching.next().is_some() {
        return None;
    }
    arg_site_binding(input, initializer.initializer_site()?)
}

/// `class` -> canonical object definition, resolved through the same
/// ordinary-box coverage the field issuer consults — a name outside
/// coverage or without a source object definition mints no view.
fn object_view_for(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    class: &str,
) -> Option<BorrowedFormalObjectViewV1> {
    let row = batch
        .ordinary_box_coverage()
        .row_for(class)
        .ok()
        .flatten()?;
    instance_constructors
        .with_source_object_definition(row, |object, _| object)
        .ok()
        .map(|object| BorrowedFormalObjectViewV1 {
            class: class.into(),
            object,
            declared: false,
        })
}

impl PreparedBorrowedFormalIngressV1 {
    /// The co-sealed object view for one callee formal: `Some` only when
    /// every incoming actual agreed on one ordinary class.
    pub(in crate::mir::normal_callable_semantic_package) fn formal_object_view(
        &self,
        formal: BindingRefV1,
    ) -> Option<&BorrowedFormalObjectViewV1> {
        self.object_views.get(&formal)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn incoming_calls_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.incoming.iter().any(|row| row.call.owner() == owner)
    }

    /// `true` when this owner's sealed use draft admits a dominated
    /// `formal.field` read whose co-sealed object view was actually
    /// minted — such a callee needs the verified walk so the guarded read
    /// is issued by the same completion authority that proves its non-null
    /// successor. A callee whose incoming actuals minted no view (all-null
    /// or conflicting evidence) stays on the bounded sibling: the read is
    /// truthfully unclaimed there and the caller-side/physical stops keep
    /// their named fail-closed ownership. Draft membership already implies
    /// an incoming call targets the owner.
    pub(in crate::mir::normal_callable_semantic_package) fn formal_field_read_target(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.definitions.get(&owner).is_some_and(|draft| {
            draft.uses.iter().any(|row| {
                matches!(
                    row.kind,
                    BorrowedFormalUseDraftKindV1::FieldReadOperand { .. }
                ) && self.object_views.contains_key(&row.formal)
            })
        })
    }

    /// `true` when `owner`'s sealed use draft admitted a dominated-view
    /// value use at this exact leaf site — `ArrayElementValue`,
    /// `AddOperand`, or `NewArgument`. The consult reads classification
    /// output, not borrowed-transport membership: an owner invoked only
    /// through `me.<field>` receivers never enters the borrowed-entry
    /// profile, yet its draft still proves these uses. This is a
    /// coverage consult only — the draft stays the sole admission
    /// authority and the physical `borrowed_call_uses` whitelist still
    /// proves each routed operand.
    pub(in crate::mir::normal_callable_semantic_package) fn dominated_view_use_at(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
    ) -> bool {
        site.owner() == owner && self.dominated_view_sites.contains(site)
    }

    /// Corroborate the final issuer consumes precisely the source rows used by
    /// selection. This does not authorize actual liveness, result or tagged ABI.
    pub(super) fn corroborate_source_targets(
        &self,
        prepared: &[Result<Option<LexicalInstanceCallSourceTargetV1>, String>],
    ) -> Result<(), String> {
        let calls: BTreeMap<_, _> = prepared
            .iter()
            .filter_map(|row| {
                row.as_ref()
                    .ok()
                    .and_then(Option::as_ref)
                    .map(|row| (row.call_site(), row))
            })
            .collect();
        for incoming in &self.incoming {
            let source = calls
                .get(&incoming.call)
                .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
            let draft = self
                .definitions
                .get(&incoming.callee)
                .ok_or_else(|| freeze("borrowed-formal/final-definition-missing"))?;
            if *source != &incoming.source
                || source.callee_owner() != incoming.callee
                || incoming.arguments.iter().any(|(ordinal, site, formal)| {
                    source.argument_sites().get(*ordinal as usize) != Some(site)
                        || draft.origins.get(formal) != Some(formal)
                })
            {
                return Err(freeze("borrowed-formal/final-incoming-drift"));
            }
        }
        for forward in &self.forwards {
            let source = calls
                .get(&forward.call)
                .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
            if source.target() != &forward.target
                || source.callee_owner() != forward.callee_formal.owner()
                || source.argument_sites().get(forward.ordinal as usize)
                    != Some(forward.site.site())
                || self
                    .definitions
                    .get(&forward.site.owner())
                    .and_then(|draft| draft.origins.get(&forward.binding))
                    != Some(&forward.source_formal)
            {
                return Err(freeze("borrowed-formal/final-forward-drift"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_source_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_declared_borrow_source_tests.rs"]
mod declared_tests;
