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
#[path = "ordinary_new_borrowed_formal_source_target_static.rs"]
mod target_static;
#[path = "ordinary_new_borrowed_formal_value_domain.rs"]
mod value_domain;

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
    /// Original AppMain co-seal identity; never inferred from a function name.
    pub(super) app_main_owner: Option<FunctionOwnerIdV1>,
    pub(super) app_main_terminal_sites: BTreeSet<OwnedExprSiteV1>,
    // Disjoint excluded owners from the same original draft issuance.
    pub(super) source_only_definitions: BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    pub(super) target_static:
        BTreeMap<FunctionOwnerIdV1, target_static::TargetStaticSourceCohortV1>,
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
    original_formal_seeds: source_seeds::OriginalFormalSeedProductV1,
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
    let inventory = prepare_original_incoming_v1(
        batch, selected, &BTreeSet::new(), contracts, &calls, &drafts.0, &drafts.1,
        None, &BTreeMap::new(), None, None, instance_constructors,
        entry_home_loans, local_candidates, callable_result_classes, &drafts.3,
    );
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
        BTreeMap::new(),
        inventory,
    )
}

#[path = "ordinary_new_borrowed_formal_ingress_finish.rs"]
mod ingress_finish;
pub(super) use ingress_finish::{prepare_original_incoming_v1, finish_ingress_from_drafts_v1};

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

/// Original source classification retains a forward's declaration constraint
/// without deciding transport membership. The same row can be projected later.
#[derive(Debug)]
enum OriginalFormalActualSeedV1 {
    Integer,
    /// Exact usize declaration: Integer lane, without signed result permission.
    SourceInteger,
    Class(Box<str>),
    Null,
    Forward {
        origin: BindingRefV1,
        declared_constraint: Option<Box<str>>,
    },
    Conflict,
}
impl OriginalFormalActualSeedV1 {
    fn project_source_integer_v1(&self, current_i64: bool) -> FormalActualSeedV1 {
        match self {
            Self::Forward { origin, declared_constraint: Some(_) } => FormalActualSeedV1::Forward(*origin),
            Self::Forward { origin, declared_constraint: None } => FormalActualSeedV1::ForwardIntegerOnly(*origin),
            Self::Integer => FormalActualSeedV1::Integer,
            Self::SourceInteger => if current_i64 { FormalActualSeedV1::Conflict } else { FormalActualSeedV1::Integer },
            Self::Class(class) => FormalActualSeedV1::Class(class.clone()),
            Self::Null => FormalActualSeedV1::Null,
            Self::Conflict => FormalActualSeedV1::Conflict,
        }
    }

    fn project_selected_v1(
        &self,
        caller: FunctionOwnerIdV1,
        transport_owners: Option<&BTreeSet<FunctionOwnerIdV1>>,
    ) -> FormalActualSeedV1 {
        match self {
            Self::Integer | Self::SourceInteger => FormalActualSeedV1::Integer,
            Self::Class(class) => FormalActualSeedV1::Class(class.clone()),
            Self::Null => FormalActualSeedV1::Null,
            Self::Conflict => FormalActualSeedV1::Conflict,
            Self::Forward { origin, declared_constraint } => {
                let outside = transport_owners.is_some_and(|owners| !owners.contains(&caller));
                match (outside, declared_constraint) {
                    (false, Some(class)) => FormalActualSeedV1::Class(class.clone()),
                    (true, None) => FormalActualSeedV1::ForwardIntegerOnly(*origin),
                    _ => FormalActualSeedV1::Forward(*origin),
                }
            }
        }
    }
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
    classify_original_actual_seed_in_scope(
        input, batch, selected, callable_result_classes, caller, candidates,
        receiver, definitions, contracts, site, guarded,
    ).project_selected_v1(caller, transport_owners)
}

#[allow(clippy::too_many_arguments)]
fn classify_original_actual_seed_in_scope(
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
    guarded: Option<(&BorrowedGuardedActualV1, &OwnedExprSiteV1, u32)>,
) -> OriginalFormalActualSeedV1 {
    if let Some((fact, call, ordinal)) = guarded {
        return if fact.corroborates(input, call, ordinal, site) {
            OriginalFormalActualSeedV1::Integer
        } else {
            OriginalFormalActualSeedV1::Conflict
        };
    }
    let function = input.function();
    match function.expression_source().literal(site) {
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null) => {
            return OriginalFormalActualSeedV1::Null;
        }
        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_)) => {
            return OriginalFormalActualSeedV1::Integer;
        }
        Some(_) => return OriginalFormalActualSeedV1::Conflict,
        None => {}
    }
    if function
        .expression_source()
        .negative_integer_immediate(site)
        .is_some()
    {
        return OriginalFormalActualSeedV1::Integer;
    }
    let Some(mut binding) = arg_site_binding(input, site) else {
        return OriginalFormalActualSeedV1::Conflict;
    };
    if value_domain::declared_integer_seed_v1(input, contracts, binding) {
        return if contracts.iter().filter(|row| row.owner == input.owner()).flat_map(|row| &row.parameters)
            .any(|row| row.binding == binding && matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
        { OriginalFormalActualSeedV1::Integer } else { OriginalFormalActualSeedV1::SourceInteger };
    }
    let mut visited = BTreeSet::new();
    loop {
        if binding.owner() != caller || !visited.insert(binding) {
            return OriginalFormalActualSeedV1::Conflict;
        }
        if let Some(candidate) = candidates
            .iter()
            .find(|candidate| candidate.destination == binding)
        {
            return if candidate.construction.is_ok() {
                OriginalFormalActualSeedV1::Class(candidate.class.clone())
            } else {
                OriginalFormalActualSeedV1::Conflict
            };
        }
        if let Some((root, row)) = receiver {
            if root == binding {
                return OriginalFormalActualSeedV1::Class(row.name().into());
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
            return OriginalFormalActualSeedV1::Class(class);
        }
        if let Some(formal) = definitions
            .get(&caller)
            .and_then(|draft| draft.origins.get(&binding))
        {
            let declared_constraint = contracts
                .iter()
                .find(|row| row.owner == caller)
                .and_then(|row| row.parameters.iter().find(|row| row.binding == *formal))
                .and_then(|row| match &row.kind {
                    CallableParameterContractKindV1::DeclaredObject(class) => Some(class.clone()),
                    _ => None,
                });
            return OriginalFormalActualSeedV1::Forward {
                origin: *formal,
                declared_constraint,
            };
        }
        match alias_source_binding(input, binding) {
            Some(next) => binding = next,
            None => return OriginalFormalActualSeedV1::Conflict,
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
                BorrowedIncomingSourceV1::Instance(original) | BorrowedIncomingSourceV1::SelectedStored { target: original, .. } => {
                    let source = *calls
                        .get(&incoming.call)
                        .ok_or_else(|| freeze("borrowed-formal/final-source-missing"))?;
                    incoming.source.bind_selected_v1(source)
                        .map_err(|_| freeze("borrowed-formal/final-incoming-drift"))?;
                    if source != original {
                        return Err(freeze("borrowed-formal/final-incoming-drift"));
                    }
                }
                BorrowedIncomingSourceV1::PendingStored(_) => {
                    return Err(freeze("borrowed-formal/incoming-stored-pending"));
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
            let mut originals = self.source_incoming.exact_rows()
                .filter(|row| row.call == incoming.call);
            let original = originals.next()
                .ok_or_else(|| freeze("borrowed-formal/final-original-incoming-missing"))?;
            if originals.next().is_some() || !incoming.source.same_source(&original.source) {
                return Err(freeze("borrowed-formal/final-original-incoming-drift"));
            }
            if original.callee != incoming.callee || original.arguments != incoming.arguments {
                return Err(freeze("borrowed-formal/final-incoming-drift"));
            }
            let source = incoming.source.as_loan()?;
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
