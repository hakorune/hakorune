//! Source/domain/liveness join of the existing borrowed ingress preparation.
//! Pending actuals do not arm LocalCallObservation or a physical signature.
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualCandidateV1, BorrowedCallActualValueV1, LocalCallArgumentV1, SourceScalarKind,
};
use std::collections::BTreeMap;
use std::rc::Rc;

#[path = "ordinary_new_borrowed_formal_actual_model.rs"]
mod model;
pub(super) use instance_source::{
    original_core_argument_input_request_v1, revalidate_passive_instance_incoming_v1,
    revalidate_passive_instance_incoming_with_array_context_v1,
};
pub(in crate::mir::normal_callable_semantic_package) use instance_source::{
    recheck_pending_array_source_actuals_v1, source_instance_scalar_reads_v1,
};
pub(in crate::mir::normal_callable_semantic_package) use model::PreparedBorrowedCallActualsV1;
use model::{ArgumentExpressionV1, BorrowedCallActualEvidencePhaseV1};
pub(crate) use model::{BorrowedFormalActualSourceV1, PreparedBorrowedFormalActualV1};
pub(in crate::mir::normal_callable_semantic_package) use static_source::project_pending_static_source_arguments_for_route_v1;

#[path = "ordinary_new_borrowed_static_bool_entry_finish.rs"]
mod static_bool_entry_finish;
#[path = "ordinary_new_borrowed_static_mixed_input_finish.rs"]
mod static_mixed_input_finish;
#[path = "ordinary_new_borrowed_static_scalar_input_finish.rs"]
mod static_scalar_input_finish;
#[path = "ordinary_new_borrowed_static_selected_actual.rs"]
mod static_selected_actual;
#[path = "ordinary_new_borrowed_static_tagged_input_finish.rs"]
mod static_tagged_input_finish;
pub(in crate::mir::normal_callable_semantic_package) use static_selected_actual::{
    issue_original_static_forwarded_actual_v1, VerifiedStaticForwardedActualV1,
};

/// Revalidate a passive Object input without lending executable arguments.
pub(super) fn revalidate_passive_object_incoming_v1(
    ingress: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<bool, String> {
    if call.source.instance().is_none()
        && !matches!(
            rows.phase,
            BorrowedCallActualEvidencePhaseV1::SourceObject(_)
        )
    {
        return Ok(false);
    }
    let mut originals = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| row.call == call.call);
    let original = originals
        .next()
        .ok_or_else(|| freeze("borrowed-object/source-row-missing"))?;
    if let Some(target) = original.source.instance() {
        if object_source::selected_original_object_phase_v1(ingress, target.call_site())?
            && !matches!(
                rows.phase,
                BorrowedCallActualEvidencePhaseV1::SourceObject(_)
                    | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)
            )
        {
            return Err(freeze("borrowed-object/source-phase-drift"));
        }
    }
    if originals.next().is_some()
        || original.call != call.call
        || original.callee != call.callee
        || original.arguments != call.arguments
        || original.source.require_instance()? != call.source.require_instance()?
    {
        return Err(freeze("borrowed-object/source-actual-identity"));
    }
    if let BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) = &rows.phase {
        object_source::recheck_object_source_actuals_v1(
            ingress, contracts, original, rows, identity, ledger,
        )?;
        return Ok(false);
    }
    let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &rows.phase else {
        if original.callee != call.callee
            || original.arguments != call.arguments
            || original.source.require_instance()? != call.source.require_instance()?
        {
            return Err(freeze("borrowed-object/source-actual-identity"));
        }
        return Ok(false);
    };
    if original.call != call.call
        || original.callee != call.callee
        || original.arguments != call.arguments
        || original.source.require_instance()? != call.source.require_instance()?
        || identity.source != *call.source.require_instance()?
        || identity.incoming_arguments != call.arguments
        || !rows.opaque_actuals.is_empty()
    {
        return Err(freeze("borrowed-object/source-actual-identity"));
    }
    if call
        .arguments
        .iter()
        .any(|(_, _, formal)| ingress.formal_object_view(*formal).is_some())
    {
        return Err(freeze("borrowed-entry/object-view-drift"));
    }
    let mut rechecked = object_source::prepare_object_source_actuals_v1(
        ingress,
        contracts,
        &call.call,
        &identity.candidates,
    )?
    .ok_or_else(|| freeze("borrowed-object/source-phase-drift"))?;
    model::retain_object_argument_sources_v1(&mut rechecked, identity)?;
    if rows != &rechecked {
        return Err(freeze("borrowed-object/source-snapshot-drift"));
    }
    Ok(true)
}

pub(in crate::mir::normal_callable_semantic_package) type PendingBorrowedFormalActualsV1 =
    BTreeMap<OwnedExprSiteV1, Result<PreparedBorrowedCallActualsV1, String>>;

/// Retain the exact Dynamic target's argument expression plans in its one
/// pending actual row. Dynamic invocation has no callee formal contract; the
/// source-bound operation supplies the I64 argument contract instead.
pub(in crate::mir::normal_callable_semantic_package) fn prepare_dynamic_invocation_argument_actuals_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    target: &crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1,
    actuals: &[BorrowedCallActualCandidateV1],
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
) -> Result<PreparedBorrowedCallActualsV1, String> {
    use crate::mir::resolved_value_profile::expression_plan::{
        call_argument_expression_source_plan_v1, ExpressionPlanRootV1,
    };
    let reject = || freeze("dynamic-argument/source-identity");
    if input.owner() != target.owner()
        || target.result_site() != target.call_site()
        || target.receiver_binding().owner() != input.owner()
        || target.dynamic_origin().owner() != input.owner()
        || target.arguments().len() != actuals.len()
        || target.dispatch().arity() as usize != actuals.len()
    {
        return Err(reject());
    }
    let method = input
        .function()
        .method_call_source_v1(target.call_site())
        .ok_or_else(reject)?;
    if method.owner() != input.owner()
        || method.receiver_site() != target.receiver_site()
        || method.arity() != target.dispatch().arity()
        || method.arguments().len() != target.arguments().len()
    {
        return Err(reject());
    }
    for (index, (candidate, original)) in actuals.iter().zip(target.arguments()).enumerate() {
        if candidate.ordinal as usize != index
            || candidate.ordinal != original.ordinal()
            || &candidate.site != original.site()
            || method.arguments().get(index).is_none_or(|argument| {
                argument.ordinal() != candidate.ordinal || argument.site() != &candidate.site
            })
        {
            return Err(reject());
        }
    }

    let source = crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1::from_admitted_target_v1(target);
    let previous = previous
        .map(|row| row.as_ref().map_err(|_| reject()))
        .transpose()?;
    let old = previous.and_then(|row| row.dynamic_invocation_arguments.as_ref());
    if let Some(old) = old {
        if old.source != source || old.candidates.as_ref() != actuals {
            return Err(reject());
        }
    } else if previous.is_some() {
        return Err(reject());
    }

    let mut arguments = BTreeMap::new();
    let mut ordered_arguments = Vec::with_capacity(actuals.len());
    for candidate in actuals {
        let plan = if let Some(slot) = old.and_then(|old| old.arguments.get(&candidate.ordinal)) {
            let plan = match slot {
                ArgumentExpressionV1::Source(plan) => Rc::clone(plan),
                ArgumentExpressionV1::Qualified(frame) => Rc::clone(frame.source()),
            };
            let ExpressionPlanRootV1::CallArgument {
                call: original,
                ordinal,
                ..
            } = plan.root()
            else {
                return Err(reject());
            };
            if !std::sync::Arc::ptr_eq(original, &method)
                || *ordinal != candidate.ordinal
                || plan
                    .nodes()
                    .first()
                    .is_none_or(|node| node.site() != &candidate.site)
            {
                return Err(reject());
            }
            slot.clone()
        } else {
            call_argument_expression_source_plan_v1(input, &method, candidate.ordinal)
                .map_err(|_| reject())?
                .into()
        };
        if arguments.insert(candidate.ordinal, plan).is_some() {
            return Err(reject());
        }
        ordered_arguments.push(LocalCallArgumentV1::Expression {
            ordinal: candidate.ordinal,
            site: candidate.site.clone(),
        });
    }
    if old.is_some_and(|old| old.arguments.len() != arguments.len()) {
        return Err(reject());
    }
    Ok(PreparedBorrowedCallActualsV1 {
        phase: BorrowedCallActualEvidencePhaseV1::Executable,
        opaque_actuals: Box::new([]),
        ordered_arguments: ordered_arguments.into_boxed_slice(),
        dynamic_invocation_arguments: Some(model::DynamicInvocationArgumentSourcesV1 {
            source,
            candidates: actuals.to_vec().into_boxed_slice(),
            arguments,
        }),
    })
}

pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_call_actuals_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    candidates: &[super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    let prepared = prepared.as_ref().map_err(Clone::clone)?;
    if object_source::selected_returned_field_input_v1(prepared, call)? {
        return object_source::prepare_object_source_actuals_v1(prepared, contracts, call, actuals);
    }
    if let Some(rows) =
        instance_source::prepare_instance_source_actuals_v1(prepared, contracts, call, actuals)?
    {
        return Ok(Some(rows));
    }
    // Select the original expression owner before either primitive issuer.
    // SourceStatic remains passive until whole-input completion.
    if static_source::selects_static_expression_source_v1(prepared, call, actuals)? {
        return static_source::prepare_static_source_actuals_v1(prepared, contracts, call, actuals);
    }
    let mut incoming = prepared.incoming.iter().filter(|row| &row.call == call);
    let Some(incoming_row) = incoming.next() else {
        if let Some(original) = prepared.scalar_qualified_source_call_v1(call)? {
            // The same source row and constructor issue only the argument
            // proof. Whole-cohort entry/Completion/packet checks stay later.
            return construct_borrowed_call_actuals_v1(
                prepared,
                original,
                contracts,
                call,
                actuals,
                object_source::ObjectActualHomeSourceV1::Candidates(candidates),
                receiver,
                nullable_class,
                None,
                None,
                &mut |_, _, _| Ok(None),
            );
        }
        if let Some(actuals) =
            static_source::prepare_static_source_actuals_v1(prepared, contracts, call, actuals)?
        {
            return Ok(Some(actuals));
        }
        return object_source::prepare_object_source_actuals_v1(prepared, contracts, call, actuals);
    };
    if incoming.next().is_some() {
        return Err(freeze("borrowed-actual/duplicate-incoming"));
    }
    if matches!(&incoming_row.source,
        super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
            if original.is_current_owner_i64_source_v1())
    {
        // Final-callee membership is a candidate only. Keep the original
        // CurrentOwner actual source-only until signature and Completion close
        // its whole selected incoming cohort.
        return static_source::prepare_static_source_actuals_v1(prepared, contracts, call, actuals);
    }
    if needs_original_object_forward_source_v1(prepared, incoming_row, call, actuals)? {
        return object_source::prepare_object_source_actuals_v1(prepared, contracts, call, actuals);
    }
    construct_borrowed_call_actuals_v1(
        prepared,
        incoming_row,
        contracts,
        call,
        actuals,
        object_source::ObjectActualHomeSourceV1::Candidates(candidates),
        receiver,
        nullable_class,
        None,
        None,
        &mut |_, _, _| Ok(None),
    )
}

/// Construct from an original incoming row after the caller has selected it.
/// Source selection and all later entry/Completion permissions stay separate.
#[allow(clippy::too_many_arguments)]
fn construct_borrowed_call_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    incoming_row: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    homes: object_source::ObjectActualHomeSourceV1<'_>,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
    static_zero: Option<static_input_finish::StaticInputFinishV1>,
    guarded_definition: Option<&super::borrowed_formal_uses::BorrowedFormalUsesDraftV1>,
    source_actual: &mut dyn FnMut(
        u32,
        &SourceExprSiteV1,
        BindingRefV1,
    ) -> Result<Option<BorrowedFormalActualSourceV1>, String>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    let mut callee_contracts = contracts
        .iter()
        .filter(|row| row.owner == incoming_row.callee);
    let contract = callee_contracts
        .next()
        .ok_or_else(|| freeze("borrowed-actual/formal-missing"))?;
    if callee_contracts.next().is_some() || actuals.len() != contract.parameters.len() {
        return Err(freeze("borrowed-actual/arity"));
    }
    if incoming_row.source.call_site() != call
        || incoming_row.source.callee_owner() != contract.owner
        || incoming_row.source.target_batch_slot() != contract.batch_slot
        || incoming_row.source.argument_sites().len() != contract.parameters.len()
    {
        return Err(freeze("borrowed-actual/source-identity"));
    }
    if incoming_row.source.as_loan()?.declaration_mode() != contract.mode {
        return Err(freeze("borrowed-actual/source-mode"));
    }
    if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
        &incoming_row.source
    {
        if let Some(finish) = &static_zero {
            if !finish.corroborates_source_v1(original) {
                return Err(freeze("static-zero/input-finish-source-drift"));
            }
        } else {
            original.require_qualified_i64_source_v1()?;
        }
        let retained = if static_zero.is_some() {
            static_input_finish::original_zero_input_source_v1(prepared, original)?
                .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        } else {
            prepared
                .source_incoming
                .static_observations()
                .get(call)
                .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
                .as_ref()
                .map_err(Clone::clone)?
        };
        if !std::rc::Rc::ptr_eq(original, retained)
            || original.parameters().len() != contract.parameters.len()
            || original
                .parameters()
                .iter()
                .zip(&contract.parameters)
                .any(|(source, formal)| {
                    source.ordinal != formal.ordinal
                        || source.binding != formal.binding
                        || source.kind != formal.kind
                })
        {
            return Err(freeze("borrowed-static/executable-source-identity"));
        }
    }
    // The opaque subset cannot prove the unchanged scalar arguments beside
    // it. Cover every source ordinal before lending any successful actuals
    // to entry adoption or a continuation of this selected definition.
    for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
        if formal.ordinal as usize != index
            || formal.binding.owner() != contract.owner
            || actual.ordinal != formal.ordinal
            || actual.site != incoming_row.source.argument_sites()[index]
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        match &formal.kind {
            kind if kind.is_ordinary_borrowed_handle() => {}
            CallableParameterContractKindV1::ExactTrivial(_) => match &actual.value {
                BorrowedCallActualValueV1::Integer(_) => {}
                BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer)
                    if binding.owner() == call.owner() => {}
                _ => return Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
            },
            // Existing Map/Text/declared-handle contracts need their own
            // source consumer proof; an opaque neighbour cannot provide it.
            _ => return Err(freeze("borrowed-actual/nonopaque-contract-unproved")),
        }
    }
    let mut rows = Vec::new();
    for (ordinal, site, formal) in &incoming_row.arguments {
        let actual = actuals
            .get(*ordinal as usize)
            .ok_or_else(|| freeze("borrowed-actual/ordinal"))?;
        if actual.ordinal != *ordinal
            || actual.site != *site
            || formal.owner() != incoming_row.callee
            || contract
                .parameters
                .get(*ordinal as usize)
                .is_none_or(|row| {
                    row.ordinal != *ordinal
                        || row.binding != *formal
                        || !row.kind.is_ordinary_borrowed_handle()
                })
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        let source = match &actual.value {
            BorrowedCallActualValueV1::Integer(value) => {
                BorrowedFormalActualSourceV1::Integer(*value)
            }
            BorrowedCallActualValueV1::Bool(value) => BorrowedFormalActualSourceV1::Bool(*value),
            BorrowedCallActualValueV1::Null => BorrowedFormalActualSourceV1::Null,
            BorrowedCallActualValueV1::String(_)
            | BorrowedCallActualValueV1::Expression
            | BorrowedCallActualValueV1::Binding(_)
            | BorrowedCallActualValueV1::MethodCall(_)
            | BorrowedCallActualValueV1::ReceivedField { .. }
            | BorrowedCallActualValueV1::Unknown => {
                let source = source_actual(*ordinal, site, *formal)?
                    .ok_or_else(|| freeze("borrowed-actual/unsupported-or-unavailable"))?;
                model::validate_computed_actual_source_v1(
                    &source,
                    actual,
                    call,
                    &contract.parameters[*ordinal as usize],
                )?;
                source
            }
            BorrowedCallActualValueV1::ReceivedNullable(binding)
                if binding.owner() == call.owner() =>
            {
                // The sealed `NullableObject` claim is the sole class
                // authority — a fake, foreign, or unclaimed producer can
                // never mint the nullable typed-object actual.
                let class = match homes {
                    object_source::ObjectActualHomeSourceV1::Ledger(ledger) => {
                        let Some(class) = ledger.borrowed_nullable_actual_class_v1(*binding)?
                        else {
                            return Ok(None);
                        };
                        class
                    }
                    _ => nullable_class(*binding)
                        .ok_or_else(|| freeze("borrowed-actual/nullable-class-unavailable"))?,
                };
                BorrowedFormalActualSourceV1::ReceivedNullable {
                    binding: *binding,
                    class,
                }
            }
            BorrowedCallActualValueV1::ReceivedMixed(binding)
                if binding.owner() == call.owner() =>
            {
                let original = prepared
                    .mixed_received_actual_source_v1(*binding)?
                    .ok_or_else(|| freeze("borrowed-actual/mixed-source-unavailable"))?;
                let class: Box<str> = match homes {
                    object_source::ObjectActualHomeSourceV1::Ledger(ledger) => {
                        let Some(class) = ledger.borrowed_mixed_actual_class_v1(*binding)? else {
                            return Ok(None);
                        };
                        class
                    }
                    _ => original.fresh_class().into(),
                };
                if class.as_ref() != original.fresh_class() {
                    return Err(freeze("borrowed-actual/mixed-class-drift"));
                }
                BorrowedFormalActualSourceV1::ReceivedMixed {
                    binding: *binding,
                    class,
                }
            }
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                BorrowedFormalActualSourceV1::Scalar {
                    binding: *binding,
                    kind: *kind,
                }
            }
            BorrowedCallActualValueV1::Home {
                binding,
                root,
                acquisition,
            } => {
                let Some(class) = homes.home_class(call, *binding, *root, acquisition)? else {
                    return Ok(None);
                };
                BorrowedFormalActualSourceV1::TypedHome {
                    binding: *binding,
                    root: *root,
                    acquisition: acquisition.clone(),
                    class,
                }
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                if let Some(formal_origin) = prepared
                    .definitions
                    .get(&call.owner())
                    .and_then(|draft| draft.origins.get(binding))
                {
                    if root != formal_origin
                        || !prepared.forwards.iter().any(|row| {
                            &row.call == call
                                && row.ordinal == *ordinal
                                && row.site.site() == site
                                && row.binding == *binding
                                && row.source_formal == *formal_origin
                                && row.callee_formal == *formal
                        })
                    {
                        return Err(freeze("borrowed-actual/forward-identity"));
                    }
                    BorrowedFormalActualSourceV1::Forwarded {
                        binding: *binding,
                        formal: *formal_origin,
                    }
                } else if let Some(class) = instance_source::declared_source_actual_class_v1(
                    prepared, contracts, call, *binding, *root,
                )? {
                    BorrowedFormalActualSourceV1::DeclaredFormal {
                        binding: *binding,
                        root: *root,
                        class,
                    }
                } else if let Some(draft) =
                    guarded_definition.filter(|draft| draft.origins.get(binding) == Some(root))
                {
                    let selected = !prepared
                        .guarded_actuals
                        .contains_key(&(call.clone(), *ordinal))
                        && prepared
                            .original_mixed_tagged_forward_v1(
                                contracts, call, *ordinal, site, *binding, *root,
                            )?
                            .is_some_and(|original| std::ptr::eq(original, draft));
                    if !selected {
                        let fact = prepared
                            .guarded_actuals
                            .get(&(call.clone(), *ordinal))
                            .ok_or_else(|| freeze("borrowed-guarded/actual-guard-missing"))?;
                        let mut uses = draft.uses.iter().filter(|row| row.site.site() == site);
                        let row = uses
                            .next()
                            .ok_or_else(|| freeze("borrowed-guarded/actual-use-missing"))?;
                        if uses.next().is_some()
                            || row.binding != *binding
                            || row.formal != *root
                            || !fact.matches_draft_argument(row, call, *ordinal, draft)
                        {
                            return Err(freeze("borrowed-guarded/actual-use-identity"));
                        }
                    }
                    BorrowedFormalActualSourceV1::Forwarded {
                        binding: *binding,
                        formal: *root,
                    }
                } else {
                    let (entry, class) =
                        receiver.ok_or_else(|| freeze("borrowed-actual/entry-domain"))?;
                    if *root != entry
                        || binding.owner() != call.owner()
                        || entry.owner() != call.owner()
                    {
                        return Err(freeze("borrowed-actual/entry-source"));
                    }
                    BorrowedFormalActualSourceV1::EntryReceiver {
                        binding: *binding,
                        root: entry,
                        class: class.name().into(),
                    }
                }
            }
            _ => return Err(freeze("borrowed-actual/unsupported-or-unavailable")),
        };
        if let CallableParameterContractKindV1::DeclaredObject(expected) =
            &contract.parameters[*ordinal as usize].kind
        {
            let class = match &source {
                BorrowedFormalActualSourceV1::Null => None,
                BorrowedFormalActualSourceV1::Read(loan) => Some(loan.class()),
                BorrowedFormalActualSourceV1::TypedHome { class, .. }
                | BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                | BorrowedFormalActualSourceV1::LocalObjectBinding { class, .. }
                | BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                | BorrowedFormalActualSourceV1::ReceivedMixed { class, .. }
                | BorrowedFormalActualSourceV1::DeclaredFormal { class, .. } => {
                    Some(class.as_ref())
                }
                BorrowedFormalActualSourceV1::Forwarded { formal, .. } => Some(
                    prepared
                        .object_views
                        .get(formal)
                        .ok_or_else(|| freeze("borrowed-actual/declared-forward-class"))?
                        .class(),
                ),
                _ => return Err(freeze("borrowed-actual/declared-object-domain")),
            };
            if class.is_some_and(|actual| actual != expected.as_ref()) {
                return Err(freeze("borrowed-actual/declared-object-class"));
            }
        }
        rows.push(PreparedBorrowedFormalActualV1 {
            ordinal: *ordinal,
            site: site.clone(),
            formal: *formal,
            source,
        });
    }
    let ordered_arguments = actuals
        .iter()
        .map(|actual| {
            if rows.iter().any(|row| row.ordinal == actual.ordinal) {
                Ok(LocalCallArgumentV1::BorrowedActual {
                    ordinal: actual.ordinal,
                    site: actual.site.clone(),
                })
            } else {
                match actual.value {
                    BorrowedCallActualValueV1::Integer(value) => {
                        Ok(LocalCallArgumentV1::Integer(value))
                    }
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) => {
                        Ok(LocalCallArgumentV1::Scalar(binding))
                    }
                    _ => Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
                }
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Some(PreparedBorrowedCallActualsV1 {
        phase: match static_zero {
            Some(finish) => BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish),
            None => BorrowedCallActualEvidencePhaseV1::Executable,
        },
        opaque_actuals: rows.into_boxed_slice(),
        ordered_arguments: ordered_arguments.into_boxed_slice(),
        dynamic_invocation_arguments: None,
    }))
}

pub(in crate::mir::normal_callable_semantic_package) fn stage_borrowed_call_actuals_v1(
    staged: &mut PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    result: Result<Option<PreparedBorrowedCallActualsV1>, String>,
) {
    let result = match result {
        Ok(None) => return,
        Ok(Some(rows)) => Ok(rows),
        Err(error) => Err(error),
    };
    if let Some(previous) = staged.get(site) {
        if previous != &result {
            staged.insert(
                site.clone(),
                Err(freeze("borrowed-actual/repeated-walk-drift")),
            );
        }
    } else {
        staged.insert(site.clone(), result);
    }
}

/// A failed partial source walk cannot leave successful actual rows behind.
pub(in crate::mir::normal_callable_semantic_package) fn reject_borrowed_actuals_for_owner_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    owner: FunctionOwnerIdV1,
    staged: &mut PendingBorrowedFormalActualsV1,
    issue: String,
) {
    if let Ok(prepared) = prepared {
        for incoming in prepared
            .incoming
            .iter()
            .filter(|row| row.call.owner() == owner)
        {
            staged.insert(incoming.call.clone(), Err(issue.clone()));
        }
        for incoming in prepared.source_incoming.exact_rows().filter(|row| {
            row.call.owner() == owner
                && row.source.instance().is_some_and(|target| {
                    prepared.source_incoming.has_object_input_callee_v1(target)
                })
        }) {
            staged.insert(incoming.call.clone(), Err(issue.clone()));
        }
    }
    for (site, row) in staged.iter_mut().filter(|(site, _)| site.owner() == owner) {
        if matches!(row, Ok(actuals) if matches!(actuals.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(_) | BorrowedCallActualEvidencePhaseV1::SourceObject(_) | BorrowedCallActualEvidencePhaseV1::SourceInstance(_)))
        {
            *row = Err(format!("{issue} call={site:?}"));
        }
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn finish_borrowed_call_actuals_v1(
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    staged: &mut PendingBorrowedFormalActualsV1,
) {
    if let Ok(prepared) = prepared {
        for incoming in &prepared.incoming {
            staged
                .entry(incoming.call.clone())
                .or_insert_with(|| Err(freeze("borrowed-actual/selected-incoming-unobserved")));
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_actual_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_discard_tests.rs"]
mod discard_tests;

#[path = "ordinary_new_borrowed_static_source_actuals.rs"]
mod static_source;
pub(in crate::mir::normal_callable_semantic_package) use static_source::project_pending_current_owner_static_source_arguments_v1;
pub(in crate::mir::normal_callable_semantic_package) use static_source::{
    bind_argument_sources_v1, complete_argument_expression_v1, lend_argument_expression_source_v1,
    lend_dynamic_entry_read_v1, lend_expression_core_source_v1,
    lend_field_array_text_entry_read_v1, lend_finite_entry_read_v1, lend_text_entry_read_v1,
    observed_argument_source_v1, observed_carrier_array_append_argument_source_v1,
    original_static_source_v1, project_current_owner_loop_scalar_source_v1,
    stage_observed_call_actuals_v1,
};
#[path = "ordinary_new_borrowed_call_result_source.rs"]
mod call_result_source;
pub(in crate::mir::normal_callable_semantic_package) use call_result_source::source_ordinary_result_arguments_v1;
pub(crate) use call_result_source::OrdinaryResultSourceLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use static_source::project_pending_static_source_arguments_v1;

#[cfg(test)]
#[path = "ordinary_new_borrowed_actual_source_atom_tests.rs"]
mod source_atom_tests;

#[path = "ordinary_new_borrowed_object_arguments.rs"]
mod object_arguments;
pub(in crate::mir::normal_callable_semantic_package) use object_arguments::{
    corroborate_received_object_receiver_v1, project_pending_object_arguments_v1,
    project_pending_object_target_arguments_v1,
};

#[path = "ordinary_new_borrowed_object_source_actuals.rs"]
mod object_source;
use object_source::needs_original_object_forward_source_v1;
pub(super) use object_source::selected_source_only_object_forward_v1;

enum TypedInputClosureV1 {
    Pending,
    Refused(String),
    Ready(Vec<(OwnedExprSiteV1, PreparedBorrowedCallActualsV1)>),
}

#[path = "ordinary_new_borrowed_static_input_finish.rs"]
mod static_input_finish;
#[path = "ordinary_new_borrowed_static_one_input_finish.rs"]
mod static_one_input_finish;

#[path = "ordinary_new_borrowed_object_input_finish.rs"]
pub(super) mod object_input_finish;

#[path = "ordinary_new_received_producer_arguments.rs"]
mod received_producer;
pub(in crate::mir::normal_callable_semantic_package) use received_producer::{
    received_field_alias_producer_arguments_v1, received_producer_arguments_v1, original_owned_call_target_v1,
};

#[path = "ordinary_new_borrowed_instance_source_actuals.rs"]
mod instance_source;
#[path = "ordinary_new_borrowed_instance_source_validation.rs"]
mod instance_validation;
pub(in crate::mir::normal_callable_semantic_package) use instance_source::project_pending_instance_source_arguments_v1;
pub(super) use instance_source::project_pending_instance_source_arguments_with_array_context_v1;

#[path = "ordinary_new_borrowed_formal_argument_projection.rs"]
mod argument_projection;
pub(super) use argument_projection::lend_pending_borrowed_arguments_v1;
pub(in crate::mir::normal_callable_semantic_package) use argument_projection::{
    project_pending_borrowed_i64_arguments_v1, project_pending_i64_result_arguments_v1,
    project_pending_instance_arguments_by_phase_v1,
};

/// Exact reconstructed guarded source edge for the original reverse dependency graph.
pub(super) fn revalidate_guarded_object_incoming_v1(
    source: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<bool, String> {
    if !matches!(
        rows.phase,
        BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)
    ) {
        return Ok(false);
    }
    if call.source.instance().is_none() {
        return Err(freeze("borrowed-guarded/noninstance-source"));
    }
    revalidate_passive_object_incoming_v1(source, contracts, call, rows, ledger)?;
    let BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) = &rows.phase else {
        unreachable!()
    };
    Ok(object_source::contains_guarded_object_forward_v1(
        source, call, identity,
    ))
}

#[path = "ordinary_new_borrowed_instance_input_finish.rs"]
mod instance_input_finish;
pub(super) use instance_input_finish::{
    recheck_finished_instance_input_v1, revalidate_guarded_instance_incoming_v1,
};

pub(crate) use static_source::{
    ArgumentExpressionSourceLoanV1, DynamicEntryReadV1, FiniteEntryReadV1,
};
