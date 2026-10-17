//! Static source and executable projections share the original incoming inventory.
//! SourceStatic itself owns no opaque actual and never activates an entry or ABI.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1;
use crate::mir::resolved_semantics::home_new_prefix::StaticI64CallClaimV1;
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
use std::rc::Rc;

#[path = "ordinary_new_borrowed_static_argument_source.rs"]
mod argument_source;
pub(in crate::mir::normal_callable_semantic_package) use argument_source::{bind_static_argument_sources_v1, observed_argument_source_v1, lend_text_entry_read_v1, lend_argument_expression_source_v1};
pub(crate) use argument_source::ArgumentExpressionSourceLoanV1;

#[path = "ordinary_new_borrowed_initializer_text_source.rs"]
mod initializer_source;
pub(crate) use initializer_source::InitializerTextEntryReadV1;
pub(in crate::mir::normal_callable_semantic_package) use initializer_source::lend_initializer_text_entry_read_v1;

#[derive(Debug, Clone)]
pub(super) struct StaticSourceActualIdentityV1 {
    pub(super) source: Rc<StaticIncomingSourceV1>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) integer_evidence: Box<[bool]>,
    pub(super) argument_sources: BTreeMap<u32, Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>,
    pub(super) core_arguments: BTreeMap<u32, Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>,
}

impl PartialEq for StaticSourceActualIdentityV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && self.candidates == other.candidates
            && self.integer_evidence == other.integer_evidence
            && self.argument_sources.len() == other.argument_sources.len()
            && self.argument_sources.iter().all(|(ordinal, plan)| other.argument_sources.get(ordinal).is_some_and(|other| Rc::ptr_eq(plan, other)))
            && self.core_arguments.len() == other.core_arguments.len()
            && self.core_arguments.iter().all(|(ordinal, receipt)| other.core_arguments.get(ordinal).is_some_and(|other| Rc::ptr_eq(receipt, other)))
    }
}
impl Eq for StaticSourceActualIdentityV1 {}

/// Passive argument plans borrow the complete original source census. They must
/// not insert omitted callers into the legacy static-observation projection.
fn original_static_source_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1, call: &OwnedExprSiteV1,
) -> Result<Option<&'a Rc<StaticIncomingSourceV1>>, String> {
    let mut rows = prepared.source_incoming.source_rows_v1().filter(|row| &row.call == call);
    let Some(row) = rows.next() else { return Ok(None); };
    if rows.next().is_some() { return Err(freeze("borrowed-static/core-source-identity")); }
    let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source) = &row.source
        else { return Ok(None); };
    if let Some(legacy) = prepared.source_incoming.static_observations().get(call) {
        if !Rc::ptr_eq(legacy.as_ref().map_err(Clone::clone)?, source) {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
    }
    Ok(Some(source))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Original constructor provider arguments use the same ordinary actual
    /// issuer. No Home observation, terminal membership or input finish is made.
    pub(in crate::mir::normal_callable_semantic_package) fn stage_provider_static_actuals_v1(
        &mut self,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        claims: &crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), String> {
        let sources = constructors.provider_static_sources_v1(claims, selected)?;
        for provider in sources {
            let (caller, site, target, literals) = provider.original_call_v1()
                .ok_or_else(|| freeze("provider-static/call-source-missing"))?;
            let Some(prepared) = self.borrowed_formal_source.as_ref() else { continue };
            let source = prepared.as_ref().map_err(Clone::clone)?;
            // Eligibility selects only the original whole-input family. An
            // unrelated provider keeps its existing source owner and refusal.
            if source.scalar_qualified_source_call_v1(&site)?.is_none() { continue; }
            let original = source.source_incoming.static_observations().get(&site)
                .ok_or_else(|| freeze("provider-static/incoming-missing"))?
                .as_ref().map_err(Clone::clone)?;
            if original.caller() != caller || original.call_site() != &site
                || original.target() != target || !original.is_qualified_i64_source_v1()
                || !original.catalog_brand().is_same(claims.catalog_brand())
                || original.argument_sites().len() != literals.len()
                || literals.len() != target.arity() as usize {
                return Err(freeze("provider-static/incoming-source-drift"));
            }
            let candidates: Vec<_> = original.argument_sites().iter().zip(literals).enumerate()
                .map(|(ordinal, (site, literal))| BorrowedCallActualCandidateV1 {
                    ordinal: ordinal as u32,
                    site: site.clone(),
                    value: match literal {
                        crate::mir::normal_callable_semantic_package::QualifiedStaticCallArgumentKindV1::Integer(value) => BorrowedCallActualValueV1::Integer(*value),
                        crate::mir::normal_callable_semantic_package::QualifiedStaticCallArgumentKindV1::Bool(value) => BorrowedCallActualValueV1::Bool(*value),
                    },
                }).collect();
            let rows = prepare_borrowed_call_actuals_v1(
                prepared, contracts, &site, &candidates,
                &[], None, &mut |_| None,
            )?;
            if rows.is_none() { return Err(freeze("provider-static/actuals-missing")); }
            stage_borrowed_call_actuals_v1(&mut self.borrowed_formal_actuals, &site, Ok(rows));
            self.borrowed_formal_actuals.get(&site)
                .ok_or_else(|| freeze("provider-static/actuals-unstaged"))?
                .as_ref().map_err(Clone::clone)?;
        }
        Ok(())
    }
}

pub(super) fn prepare_static_source_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    // A zero-input CurrentOwner call retains the SAME original source without
    // inventing an opaque-argument fact. It is still SourceStatic, not Ready.
    let opaque_source = prepared
        .static_arguments
        .keys()
        .any(|(site, _)| site == call);
    let core_source = actuals.iter().any(|actual| {
        let BorrowedCallActualValueV1::MethodCall(method) = &actual.value else { return false; };
        prepared.ordinary_core_source_v1(&OwnedExprSiteV1::new(call.owner(), actual.site.clone()))
            .is_some_and(|receipt| std::sync::Arc::ptr_eq(method, receipt.source().source()))
    });
    let original_text_formal = |root| prepared.source_value_sum_v1(root).is_some_and(|sum|
        sum.alternatives().len() == 1 && sum.alternatives().contains(&super::super::borrowed_formal_source::SourceValueAlternativeV1::Text));
    let source_expression = actuals.iter().any(|actual| match actual.value {
        BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_) => true,
        BorrowedCallActualValueV1::SelfRooted { root, .. } => original_text_formal(root),
        _ => false,
    });
    let original = original_static_source_v1(prepared, call)?;
    let original_source = original
        .is_some_and(|source| {
            source.is_zeroarg_i64_v1()
                || source.is_current_owner_i64_source_v1()
                || source.is_current_owner_bool_source_v1()
                || prepared.is_static_terminal_source_v1(source)
                || (source.is_qualified_i64_source_v1() && (core_source || source_expression))
        });
    if !opaque_source && !original_source {
        return Ok(None);
    }
    let source = original.ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
    let mut matching = contracts
        .iter()
        .filter(|row| row.owner == source.callee_owner());
    let contract = matching
        .next()
        .ok_or_else(|| freeze("borrowed-static/target-contract"))?;
    if matching.next().is_some()
        || source.call_site() != call
        || contract.batch_slot != source.target_batch_slot()
        || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
        || contract.parameters.len() != source.parameters().len()
        || actuals.len() != source.argument_sites().len()
        || actuals.len() != contract.parameters.len()
    {
        return Err(freeze("borrowed-static/source-actual-identity"));
    }
    let mut arguments = Vec::with_capacity(actuals.len());
    let mut integer_evidence = Vec::with_capacity(actuals.len());
    let mut core_arguments = BTreeMap::new();
    for (index, ((formal, original), actual)) in contract
        .parameters
        .iter()
        .zip(source.parameters())
        .zip(actuals)
        .enumerate()
    {
        if formal.ordinal as usize != index
            || formal.ordinal != original.ordinal
            || formal.binding != original.binding
            || formal.kind != original.kind
            || formal.binding.owner() != contract.owner
            || actual.ordinal != formal.ordinal
            || source.argument_sites().get(index) != Some(&actual.site)
        {
            return Err(freeze("borrowed-static/source-actual-identity"));
        }
        if let Some(fact) = prepared
            .static_arguments
            .get(&(call.clone(), actual.ordinal))
        {
            let same_binding = match &actual.value {
                BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                    *binding == fact.binding() && *root == fact.formal()
                }
                BorrowedCallActualValueV1::Scalar(binding, _) => *binding == fact.binding(),
                _ => false,
            };
            if !Rc::ptr_eq(fact.retained_call_source(), source)
                || fact.call() != call
                || fact.ordinal() != actual.ordinal
                || fact.use_site().site() != &actual.site
                || fact.target_formal() != formal.binding
                || !same_binding
            {
                return Err(freeze("borrowed-static/forward-source-identity"));
            }
        }
        let integer = match &actual.value {
            BorrowedCallActualValueV1::Integer(_) => true,
            BorrowedCallActualValueV1::Bool(_) | BorrowedCallActualValueV1::Null => false,
            BorrowedCallActualValueV1::String(_) if formal.kind.is_ordinary_borrowed_handle() => false,
            BorrowedCallActualValueV1::Binding(binding)
                if binding.owner() == call.owner() && formal.kind.is_ordinary_borrowed_handle() => false,
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                *kind == SourceScalarKind::Integer
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                if prepared.static_arguments.contains_key(&(call.clone(), actual.ordinal)) {
                    prepared.candidate_integer_agreement(*root)
                } else if binding.owner() == call.owner() && root.owner() == call.owner()
                    && formal.kind.is_ordinary_borrowed_handle() && original_text_formal(*root) {
                    // Full-census Text source stays passive. The binder checks
                    // exact Entry provenance; the covered purpose checks current
                    // roots. No legacy forward fact or Integer proof is minted.
                    false
                } else { return Err(freeze("borrowed-static/forward-source-missing")); }
            }
            BorrowedCallActualValueV1::MethodCall(method) => {
                let site = OwnedExprSiteV1::new(call.owner(), actual.site.clone());
                let receipt = prepared.ordinary_core_source_v1(&site)
                    .ok_or_else(|| freeze("borrowed-static/source-actual-unavailable"))?;
                if method.owner() != call.owner() || method.site() != &actual.site
                    || !std::sync::Arc::ptr_eq(method, receipt.source().source())
                    || receipt.source().target().schema() != crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                    || receipt.source().target().result() != crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
                    || !formal.kind.is_ordinary_borrowed_handle()
                { return Err(freeze("borrowed-static/core-source-identity")); }
                core_arguments.insert(actual.ordinal, Rc::clone(receipt));
                false
            }
            _ => return Err(freeze("borrowed-static/source-actual-unavailable")),
        };
        let argument = match &formal.kind {
            kind if kind.is_ordinary_borrowed_handle() => LocalCallArgumentV1::BorrowedActual {
                ordinal: actual.ordinal,
                site: actual.site.clone(),
            },
            CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64() => {
                match actual.value {
                    BorrowedCallActualValueV1::Integer(value) => {
                        LocalCallArgumentV1::Integer(value)
                    }
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer)
                        if binding.owner() == call.owner() =>
                    {
                        LocalCallArgumentV1::Scalar(binding)
                    }
                    _ if source.is_current_owner_i64_source_v1()
                        && !matches!(
                            &actual.value,
                            BorrowedCallActualValueV1::Bool(_)
                                | BorrowedCallActualValueV1::Null
                                | BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Bool)
                        ) =>
                    {
                        // No I64 source proof yet. Keep the original incoming
                        // inventory passive without staging a selected call.
                        return Ok(None);
                    }
                    _ => return Err(freeze("borrowed-static/nonopaque-integer-unproved")),
                }
            }
            _ => return Err(freeze("borrowed-static/input-contract-unsupported")),
        };
        arguments.push(argument);
        integer_evidence.push(integer);
    }
    Ok(Some(PreparedBorrowedCallActualsV1 {
        opaque_actuals: Box::new([]),
        ordered_arguments: arguments.into_boxed_slice(),
        phase: BorrowedCallActualEvidencePhaseV1::SourceStatic(StaticSourceActualIdentityV1 {
            source: Rc::clone(source),
            candidates: actuals.to_vec().into_boxed_slice(),
            integer_evidence: integer_evidence.into_boxed_slice(),
            core_arguments,
            argument_sources: BTreeMap::new(),
        }),
    }))
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &QualifiedStaticCallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    project_pending_static_source_arguments_for_route_v1(
        source.as_ref().map_err(Clone::clone)?,
        pending,
        site,
        claim.required_i64_arguments(),
        false,
    )
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_current_owner_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &StaticI64CallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let required = claim
        .current_owner_source_required_i64_arguments()
        .ok_or_else(|| freeze("borrowed-static/current-owner-claim-required"))?;
    let prepared = source.as_ref().map_err(Clone::clone)?;
    let original = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !claim.corroborates_source(
        site,
        ResolvedMethodCallReceiverSourceV1::CurrentOwner,
        original.argument_sites().len() as u32,
    ) {
        return Err(freeze("borrowed-static/current-owner-claim-identity"));
    }
    if !pending.contains_key(site) {
        return Ok(None);
    }
    project_pending_static_source_arguments_for_route_v1(prepared, pending, site, required, true)
}

/// The original source candidate for one selected loop-body scalar actual.
/// This only proves binding/class/site, never the per-iteration payload or ABI.
pub(in crate::mir::normal_callable_semantic_package) fn project_current_owner_loop_scalar_source_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &StaticI64CallClaimV1,
) -> Result<Option<(Rc<StaticIncomingSourceV1>, BindingRefV1, SourceExprSiteV1)>, String> {
    let reject = || freeze("borrowed-static/loop-scalar-source-drift");
    let Some(arguments) =
        project_pending_current_owner_static_source_arguments_v1(source, pending, site, claim)?
    else {
        return Ok(None);
    };
    let [LocalCallArgumentV1::BorrowedActual {
        ordinal: 0,
        site: argument_site,
    }] = arguments.as_ref()
    else {
        return Err(reject());
    };
    let row = pending
        .get(site)
        .ok_or_else(reject)?
        .as_ref()
        .map_err(Clone::clone)?;
    let (original, binding, original_site) =
        super::static_scalar_input_finish::original_scalar_source_v1(row)?.ok_or_else(reject)?;
    let prepared = source.as_ref().map_err(Clone::clone)?;
    let retained = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(reject)?
        .as_ref()
        .map_err(Clone::clone)?;
    if !Rc::ptr_eq(original, retained) || original_site != argument_site {
        return Err(reject());
    }
    Ok(Some((Rc::clone(original), binding, original_site.clone())))
}

/// Shared argument proof for one already-issued Qualified row. Membership is
/// lent separately by the unchanged transport or the whole original cohort.
pub(super) fn checked_executable_qualified_arguments_v1<'a>(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    required_i64_arguments: &[u32],
) -> Result<&'a [LocalCallArgumentV1], String> {
    if !matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        return Err(freeze("borrowed-static/executable-phase-required"));
    }
    let site = &call.call;
    let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
        &call.source
    else {
        return Err(freeze("borrowed-static/executable-source-kind"));
    };
    original.require_qualified_i64_source_v1()?;
    let retained = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !Rc::ptr_eq(original, retained)
        || original.call_site() != site
        || original.callee_owner() != call.callee
        || required_i64_arguments != original.required_i64_arguments()
    {
        return Err(freeze("borrowed-static/executable-source-identity"));
    }
    let arguments = rows.ordered_arguments_for_v1(call)?;
    for ordinal in required_i64_arguments {
        let integer = match arguments.get(*ordinal as usize) {
            Some(LocalCallArgumentV1::Integer(_) | LocalCallArgumentV1::Scalar(_)) => true,
            Some(LocalCallArgumentV1::BorrowedActual { ordinal, .. }) => rows
                .opaque_actuals
                .iter()
                .find(|actual| actual.ordinal == *ordinal)
                .is_some_and(|actual| match &actual.source {
                    BorrowedFormalActualSourceV1::Integer(_) => true,
                    BorrowedFormalActualSourceV1::Scalar {
                        kind: SourceScalarKind::Integer,
                        ..
                    } => true,
                    BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                        prepared.candidate_integer_agreement(*formal)
                    }
                    _ => false,
                }),
            _ => false,
        };
        if !integer {
            return Err(freeze("borrowed-static/required-integer-source-unproved"));
        }
    }
    Ok(arguments)
}

pub(super) fn project_pending_static_source_arguments_for_route_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
    current_owner: bool,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let source_qualified = if current_owner {
        None
    } else {
        prepared.scalar_qualified_source_call_v1(site)?
    };
    let has_source_fact = prepared
        .static_arguments
        .keys()
        .any(|(call, _)| call == site);
    let has_final_static = prepared.incoming.iter().any(|row| {
        &row.call == site
            && matches!(
                row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(ref original) if original.is_qualified_i64_source_v1()
            )
    });
    let has_expression_source = pending.get(site).is_some_and(|rows| matches!(rows,
        Ok(rows) if matches!(&rows.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(identity)
            if !identity.core_arguments.is_empty() || identity.candidates.iter().any(|actual| matches!(&actual.value, BorrowedCallActualValueV1::MethodCall(_) | BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))))));
    let retained = original_static_source_v1(prepared, site)?;
    let has_current_owner_source = retained.is_some_and(|row| row.is_current_owner_i64_source_v1());
    let has_static_terminal_source = !current_owner
        && retained.is_some_and(|row| prepared.is_static_terminal_source_v1(row));
    if !has_source_fact
        && !has_final_static
        && !has_static_terminal_source
        && source_qualified.is_none()
        && !has_expression_source
        && !(current_owner && has_current_owner_source)
    {
        return Ok(None);
    }
    if retained.is_none_or(|row| {
        if current_owner {
            !row.is_current_owner_i64_source_v1()
        } else {
            !row.is_qualified_i64_source_v1()
        }
    }) {
        return Err(freeze("borrowed-static/source-route-identity"));
    }
    let rows = pending
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        if current_owner {
            return Err(freeze(
                "borrowed-static/current-owner-source-phase-required",
            ));
        }
        let mut incoming = prepared.incoming.iter().filter(|row| &row.call == site);
        let call = incoming
            .next()
            .or(source_qualified)
            .ok_or_else(|| freeze("borrowed-static/executable-incoming-missing"))?;
        if incoming.next().is_some() {
            return Err(freeze("borrowed-static/executable-source-identity"));
        }
        return checked_executable_qualified_arguments_v1(
            prepared,
            call,
            rows,
            required_i64_arguments,
        )
        .map(|arguments| Some(arguments.into()));
    }
    if !has_source_fact && !current_owner && !has_static_terminal_source && source_qualified.is_none() && !has_expression_source {
        return Err(freeze("borrowed-static/source-fact-required"));
    }
    if let BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(finish) = &rows.phase {
        if !current_owner {
            return Err(freeze("borrowed-static/executable-current-owner-required"));
        }
        let original =
            retained.ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
        if !finish.corroborates_source_v1(original)
            || required_i64_arguments != original.required_i64_arguments()
        {
            return Err(freeze("borrowed-static/executable-source-drift"));
        }
        let cohort = prepared.static_incoming_cohort_v1(original)?;
        if cohort.len() == 2 {
            let target = prepared
                .target_static
                .get(&original.callee_owner())
                .ok_or_else(|| freeze("borrowed-static/executable-cohort-missing"))?;
            if target.incoming.len() != 2 {
                return Err(freeze("borrowed-static/executable-cohort-drift"));
            }
            for sibling in &cohort {
                let ready = pending
                    .get(sibling.call_site())
                    .ok_or_else(|| freeze("borrowed-static/executable-sibling-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                let BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(proof) = &ready.phase
                else {
                    return Err(freeze("borrowed-static/executable-sibling-unfinished"));
                };
                if !proof.corroborates_source_v1(sibling) {
                    return Err(freeze("borrowed-static/executable-sibling-drift"));
                }
            }
        } else if cohort.len() != 1 {
            return Err(freeze("borrowed-static/executable-whole-cohort"));
        }
        let incoming_rows = prepared
            .target_static
            .get(&original.callee_owner())
            .map_or(prepared.incoming.as_ref(), |row| row.incoming.as_ref());
        let mut matching = incoming_rows.iter().filter(|row| &row.call == site);
        let call = matching
            .next()
            .ok_or_else(|| freeze("borrowed-static/executable-incoming-missing"))?;
        if matching.next().is_some() {
            return Err(freeze("borrowed-static/executable-incoming-duplicate"));
        }
        return rows
            .ordered_arguments_for_v1(call)
            .map(|rows| Some(rows.into()));
    }
    if let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &rows.phase {
        if !current_owner && !has_static_terminal_source && source_qualified.is_none() {
            return Err(freeze("borrowed-static/scalar-source-required"));
        }
        let original =
            retained.ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
        if required_i64_arguments != original.required_i64_arguments()
            || !(required_i64_arguments == [0] || required_i64_arguments.is_empty()) {
            return Err(freeze("borrowed-static/scalar-required-integer-drift"));
        }
        super::static_scalar_input_finish::checked_ready_scalar_rows_v1(
            prepared,
            pending,
            original,
            &proof.completion,
        )?
        .ok_or_else(|| freeze("borrowed-static/scalar-sibling-unfinished"))?;
        let mut matching = prepared
            .source_incoming
            .exact_rows()
            .filter(|call| &call.call == site && call.callee == original.callee_owner());
        let call = matching
            .next()
            .ok_or_else(|| freeze("borrowed-static/scalar-incoming-missing"))?;
        if matching.next().is_some() {
            return Err(freeze("borrowed-static/scalar-incoming-duplicate"));
        }
        return rows
            .ordered_arguments_for_v1(call)
            .map(|rows| Some(rows.into()));
    }
    if matches!(
        rows.phase,
        BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(_)
    ) {
        if !current_owner || !required_i64_arguments.is_empty() {
            return Err(freeze("borrowed-static/mixed-route-identity"));
        }
        let original = retained.ok_or_else(|| freeze("borrowed-static/mixed-source-missing"))?;
        let cohort =
            super::static_mixed_input_finish::checked_mixed_rows_v1(prepared, pending, original)?
                .ok_or_else(|| freeze("borrowed-static/mixed-cohort-unfinished"))?;
        let call = cohort
            .iter()
            .find(|row| row.call == *site)
            .ok_or_else(|| freeze("borrowed-static/mixed-incoming-missing"))?;
        return rows
            .ordered_arguments_for_v1(call)
            .map(|args| Some(args.into()));
    }
    let identity = argument_source::validate_static_source_actuals_v1(prepared, rows, site, required_i64_arguments)?;
    // Result/source closure does not evaluate the expression. Keep this phase
    // passive even for a callee with no required-I64 arguments or unused input.
    if !identity.core_arguments.is_empty() {
        return Err(freeze("borrowed-static/core-expression-execution-pending"));
    }
    if identity.candidates.iter().any(|actual| matches!(actual.value,
        BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))
        || matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { .. })
            && !prepared.static_arguments.contains_key(&(site.clone(), actual.ordinal))) {
        return Err(freeze("borrowed-static/text-expression-execution-pending"));
    }
    if required_i64_arguments.iter().any(|ordinal| {
        if identity.integer_evidence.get(*ordinal as usize) == Some(&true) {
            return false;
        }
        let index = *ordinal as usize;
        let checked_forward = current_owner
            && matches!(
                identity.candidates.get(index).map(|row| &row.value),
                Some(BorrowedCallActualValueV1::SelfRooted { .. })
            )
            && identity
                .source
                .parameters()
                .get(index)
                .is_some_and(|formal| {
                    formal.kind.is_ordinary_borrowed_handle()
                        && prepared.checked_static_input(formal.binding)
                });
        !checked_forward
    }) {
        if current_owner {
            return Ok(None);
        }
        return Err(freeze("borrowed-static/required-integer-source-unproved"));
    }
    Ok(Some(rows.ordered_arguments.clone()))
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_source_actual_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_selected_actual_tests.rs"]
mod selected_actual_tests;

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_local_issuer_tests.rs"]
mod local_issuer_tests;
