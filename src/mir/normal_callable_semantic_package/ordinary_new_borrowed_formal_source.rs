//! Ordinary borrowed transport selection over the prepared lexical source rows.
//! These rows precede live-actual proof and never install a physical carrier.
use super::borrowed_formal_uses::*;
use super::*;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use std::collections::{BTreeMap, BTreeSet};

#[path = "ordinary_new_borrowed_integer_return_source.rs"]
mod integer_return_source;

#[path = "ordinary_new_borrowed_formal_source_drafts.rs"]
mod source_drafts;

pub(super) use source_drafts::collect_borrowed_source_drafts_v1;

#[path = "ordinary_new_borrowed_formal_source_checked_input.rs"]
mod checked_input;
#[path = "ordinary_new_borrowed_formal_source_observation.rs"]
mod observation;
#[path = "ordinary_new_borrowed_formal_source_seeds.rs"]
mod source_seeds;
#[path = "ordinary_new_borrowed_formal_source_static_cohort.rs"]
mod static_cohort;
#[path = "ordinary_new_borrowed_formal_value_domain.rs"]
mod value_domain;
use source_seeds::prepare_borrowed_formal_views_v1;

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
    // Disjoint excluded owners from the same original draft issuance.
    pub(super) source_only_definitions: BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    /// Dominated-view value-use sites across every classified owner —
    /// `ArrayElementValue`, `AddOperand`, `MulOperand`, or `NewArgument` rows recorded
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
    pub(in crate::mir::normal_callable_semantic_package) static_arguments:
        BTreeMap<(OwnedExprSiteV1, u32), super::borrowed_static_argument::StaticArgumentSourceV1>,
    pub(in crate::mir::normal_callable_semantic_package) source_incoming:
        BorrowedIncomingInventoryV1,
    /// `formal -> sealed class view`, complete across the co-sealed
    /// incoming call set. A formal absent from the map has no agreed
    /// class — its guarded field read stays fail-closed.
    pub(super) object_views: BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
    /// Complete original incoming agreement; never execution or payload permission.
    pub(super) integer_agreements: BTreeSet<BindingRefV1>,
    /// Source-only, least-fixed-point proof that the tagged formal is checked
    /// before numeric payload use, including exact Static forward chains.
    pub(super) checked_static_inputs: BTreeSet<BindingRefV1>,
    /// Exact original checked Normal comparison facts; no transport permission.
    pub(super) guarded_actuals: BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
}

/// One incoming argument's contribution to a callee formal's class view.
pub(super) enum FormalActualSeedV1 {
    /// Exact source integer, distinct from bool/null and carrier payload.
    Integer,
    /// A class-carrying actual: claim-local `new` local, the entry
    /// receiver, a sealed received nullable, or a resolved forward.
    Class(Box<str>),
    /// The exact `null` literal — proves `Void`, never a class.
    Null,
    /// The caller's own opaque formal (or its copy) forwarded onward —
    /// the class resolves to that formal's view, possibly pending.
    Forward(BindingRefV1),
    /// Outside-profile source forwards Integer only, preserving Object authority.
    ForwardIntegerOnly(BindingRefV1),
    /// Unsupported scalar actual, unproven/foreign binding, or inline construction —
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
        None,
        app_main_slot,
        dynamic_slot,
        entry_home_loans,
        instance_constructors,
    )?;
    finish_ingress_from_drafts_v1(
        batch,
        selected,
        &BTreeSet::new(),
        contracts,
        entry_home_loans,
        instance_constructors,
        local_candidates,
        callable_result_classes,
        calls,
        None,
        None,
        drafts,
        BTreeMap::new(),
        None,
    )
}

/// Consume the original drafts once; never reconstruct them after profile selection.
#[allow(clippy::too_many_arguments)]
pub(super) fn finish_ingress_from_drafts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeSet<FunctionOwnerIdV1>,
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
        BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
    ),
    static_arguments: BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
    stored_dispatch: Option<&super::source::PreparedSourceNeedsV1>,
) -> Result<PreparedBorrowedFormalIngressV1, String> {
    let (ordinary_callers, definitions, dominated_view_sites, guarded_actuals) = drafts;
    let mut transport_owners: BTreeSet<_> = contracts
        .iter()
        .filter(|row| {
            row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                && definitions.contains_key(&row.owner)
        })
        .map(|row| row.owner)
        .collect();
    let static_context = static_claims.map(|claims| StaticIncomingContextV1 {
        claims,
        arguments: &static_arguments,
        main: app_main,
    });
    let inventory = inventory_borrowed_incoming_with_stored_dispatch_v1(
        batch,
        selected,
        omitted_static_callers,
        &definitions,
        contracts,
        &calls,
        &ordinary_callers,
        static_context.as_ref(),
        stored_dispatch,
    )
    .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    let checked_static_inputs =
        checked_input::issue_checked_static_inputs_v1(contracts, &definitions, &static_arguments)?;
    source_drafts::seed_static_transport_owners_v1(
        selected,
        contracts,
        &definitions,
        &inventory,
        &static_arguments,
        &checked_static_inputs,
        &mut transport_owners,
    )?;
    let call_sources = borrow_call_sources_v1(
        &calls,
        inventory
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok().map(|source| source.as_ref())),
    )?;
    // Close the finite graph before selection. Removing one outside-profile
    // destination invalidates every source that forwards an opaque value to it.
    // Repetition terminates because every nonfinal pass removes an owner.
    loop {
        let mut outside = BTreeSet::new();
        for owner in &transport_owners {
            let draft = &definitions[owner];
            if !call_sources
                .values()
                .any(|call| call.callee_owner() == *owner)
            {
                outside.insert(*owner);
            }
            for row in &draft.uses {
                let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &row.kind
                else {
                    continue;
                };
                let Some(target) = call_sources.get(call) else {
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
                if !transport_owners.contains(&contract.owner)
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
            transport_owners.remove(&owner);
        }
    }
    let (mut object_views, integer_agreements, source_declared_mismatches) =
        prepare_borrowed_formal_views_v1(
            batch,
            selected,
            entry_home_loans,
            instance_constructors,
            callable_result_classes,
            local_candidates,
            contracts,
            &definitions,
            &transport_owners,
            &inventory,
            &guarded_actuals,
        )?;
    let (definitions, source_only_definitions): (BTreeMap<_, _>, BTreeMap<_, _>) = definitions
        .into_iter()
        .partition(|(owner, _)| transport_owners.contains(owner));
    let forwards = join_borrowed_forward_uses_v1(&definitions, contracts, &call_sources)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/forward-coverage")))?;
    // Project the SAME immutable inventory; retain it for candidate activation.
    let incoming = inventory
        .project(&transport_owners)
        .map_err(|error| format!("{}: {error:?}", freeze("borrowed-formal/incoming-coverage")))?;
    if source_declared_mismatches
        .iter()
        .any(|formal| transport_owners.contains(&formal.owner()))
    {
        return Err(freeze("borrowed-view/declared-object-class"));
    }
    object_views.retain(|formal, _| transport_owners.contains(&formal.owner()));
    Ok(PreparedBorrowedFormalIngressV1 {
        definitions,
        source_only_definitions,
        dominated_view_sites,
        forwards,
        incoming,
        source_incoming: inventory,
        object_views,
        integer_agreements,
        checked_static_inputs,
        static_arguments,
        guarded_actuals,
    })
}

/// One incoming argument's exact source class evidence. The walk mirrors
/// the caller's own stored-local provenance: a `new` destination names the
/// claim-local candidate class, the entry receiver names the owner box, a
/// received nullable names its sealed `NullableObject` claim, and a
/// formal-rooted binding forwards to the caller's own view. Anything else
/// conflicts — an Integer, foreign, or unproven actual can never carry a
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
    classify_actual_seed_in_scope(
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
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn classify_actual_seed_in_scope(
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
    transport_owners: Option<&BTreeSet<FunctionOwnerIdV1>>,
    guarded: Option<(&BorrowedGuardedActualV1, &OwnedExprSiteV1, u32)>,
) -> FormalActualSeedV1 {
    if let Some((fact, call, ordinal)) = guarded {
        return if fact.corroborates(input, call, ordinal, site) {
            FormalActualSeedV1::Integer
        } else {
            FormalActualSeedV1::Conflict
        };
    }
    let function = input.function();
    match function.expression_source().literal(site) {
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null) => {
            return FormalActualSeedV1::Null;
        }
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_)) => {
            return FormalActualSeedV1::Integer;
        }
        Some(_) => return FormalActualSeedV1::Conflict,
        None => {}
    }
    if function
        .expression_source()
        .negative_integer_immediate(site)
        .is_some()
    {
        return FormalActualSeedV1::Integer;
    }
    let Some(mut binding) = arg_site_binding(input, site) else {
        return FormalActualSeedV1::Conflict;
    };
    if value_domain::declared_integer_seed_v1(input, contracts, binding) {
        return FormalActualSeedV1::Integer;
    }
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
            let candidate_only = transport_owners.is_some_and(|owners| !owners.contains(&caller));
            if let Some(class) = contracts
                .iter()
                .find(|row| row.owner == caller)
                .and_then(|row| row.parameters.iter().find(|row| row.binding == *formal))
                .and_then(|row| match &row.kind {
                    CallableParameterContractKindV1::DeclaredObject(class) => Some(class),
                    _ => None,
                })
            {
                return if candidate_only {
                    FormalActualSeedV1::Conflict
                } else {
                    FormalActualSeedV1::Class(class.clone())
                };
            }
            return if candidate_only {
                FormalActualSeedV1::ForwardIntegerOnly(*formal)
            } else {
                FormalActualSeedV1::Forward(*formal)
            };
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
    /// Source lookup borrows the one original draft map's disjoint partition.
    /// This does not make an excluded owner eligible for executable transport.
    pub(super) fn source_definition_for(
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
        self.integer_agreements.contains(&formal)
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
            || self.source_incoming.exact_rows().any(|row| {
                row.call.owner() == owner
                    && row.source.instance().is_some_and(|target| {
                        self.source_incoming.has_object_input_callee_v1(target)
                    })
            })
            || self
                .static_arguments
                .keys()
                .any(|(call, _)| call.owner() == owner)
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
    /// `AddOperand`, `MulOperand`, or `NewArgument`. The consult reads classification
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
                    .map(|row| (row.call_site().clone(), row))
            })
            .collect();
        let call_sources = borrow_call_sources_v1(
            &calls,
            self.source_incoming
                .static_observations()
                .values()
                .filter_map(|row| row.as_ref().ok().map(|source| source.as_ref())),
        )?;
        for incoming in &self.incoming {
            match &incoming.source {
                BorrowedIncomingSourceV1::Instance(original) => {
                    let source = *calls
                        .get(&incoming.call)
                        .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
                    if source != original {
                        return Err(freeze("borrowed-formal/final-incoming-drift"));
                    }
                }
                BorrowedIncomingSourceV1::Static(original) => {
                    let observed = self
                        .source_incoming
                        .static_observations()
                        .get(&incoming.call)
                        .and_then(|row| row.as_ref().ok())
                        .ok_or_else(|| freeze("borrowed-formal/final-static-source-missing"))?;
                    if !std::rc::Rc::ptr_eq(original, observed) {
                        return Err(freeze("borrowed-formal/final-static-source-drift"));
                    }
                }
            }
            let source = incoming.source.as_loan();
            let draft = self
                .definitions
                .get(&incoming.callee)
                .ok_or_else(|| freeze("borrowed-formal/final-definition-missing"))?;
            if source.call_site() != &incoming.call
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
            let source = call_sources
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

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_candidate_veto_tests.rs"]
mod candidate_veto_tests;
