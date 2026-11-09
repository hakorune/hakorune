//! Static source and executable projections share the original incoming inventory.
//! SourceStatic itself owns no opaque actual and never activates an entry or ABI.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;
use super::call_result_source::ordinary_argument_call_source_v1;

#[path = "ordinary_new_borrowed_static_argument_source.rs"]
mod argument_source;
pub(crate) use argument_source::ArgumentExpressionSourceLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use argument_source::{
    bind_argument_sources_v1, lend_argument_expression_source_v1, lend_expression_core_source_v1,
    lend_field_array_text_entry_read_v1, lend_text_entry_read_v1, observed_argument_source_v1,
    observed_carrier_array_append_argument_source_v1,
};
pub(in crate::mir::normal_callable_semantic_package) use argument_source::{
    complete_argument_expression_v1, stage_observed_call_actuals_v1,
};
pub(super) use argument_source::{
    validate_qualified_argument_sources_v1, validate_static_source_actuals_v1,
};

#[path = "ordinary_new_borrowed_initializer_text_source.rs"]
mod initializer_source;
pub(crate) use initializer_source::DynamicEntryReadV1;
pub(crate) use initializer_source::FiniteEntryReadV1;
pub(in crate::mir::normal_callable_semantic_package) use initializer_source::{
    lend_dynamic_entry_read_v1, lend_finite_entry_read_v1,
};

#[derive(Debug, Clone)]
pub(super) struct StaticSourceActualIdentityV1 {
    pub(super) source: Rc<StaticIncomingSourceV1>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) integer_evidence: Box<[bool]>,
    pub(super) argument_sources: BTreeMap<u32, ArgumentExpressionV1>,
    pub(super) core_arguments: BTreeMap<u32, Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>,
}

impl PartialEq for StaticSourceActualIdentityV1 {
    fn eq(&self, other: &Self) -> bool {
        self.same_original_source_v1(other) && self.argument_sources == other.argument_sources
    }
}
impl StaticSourceActualIdentityV1 {
    fn same_original_source_v1(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && self.candidates == other.candidates
            && self.integer_evidence == other.integer_evidence
            && self.argument_sources.len() == other.argument_sources.len()
            && self.argument_sources.iter().all(|(ordinal, plan)| {
                other
                    .argument_sources
                    .get(ordinal)
                    .is_some_and(|other| Rc::ptr_eq(plan, other))
            })
            && self.core_arguments.len() == other.core_arguments.len()
            && self.core_arguments.iter().all(|(ordinal, receipt)| {
                other
                    .core_arguments
                    .get(ordinal)
                    .is_some_and(|other| Rc::ptr_eq(receipt, other))
            })
    }
}
impl Eq for StaticSourceActualIdentityV1 {}

/// Passive argument plans borrow the complete original source census. They must
/// not insert omitted callers into the legacy static-observation projection.
pub(in crate::mir::normal_callable_semantic_package) fn original_static_source_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    call: &OwnedExprSiteV1,
) -> Result<Option<&'a Rc<StaticIncomingSourceV1>>, String> {
    let mut rows = prepared
        .source_incoming
        .source_rows_v1()
        .filter(|row| &row.call == call);
    let Some(row) = rows.next() else {
        return Ok(None);
    };
    if rows.next().is_some() {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source) = &row.source
    else {
        return Ok(None);
    };
    if let Some(legacy) = prepared.source_incoming.static_observations().get(call) {
        if !Rc::ptr_eq(legacy.as_ref().map_err(Clone::clone)?, source) {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
    }
    Ok(Some(source))
}

impl PreparedBorrowedFormalIngressV1 {
    /// Source projection uses the same original destination purpose as admission.
    pub(in crate::mir::normal_callable_semantic_package) fn has_finite_forward_source_call_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<bool, String> {
        Ok(
            original_static_source_v1(self, site)?.is_some_and(|source| {
                self.original_finite_forward_input_request_v1(source.callee_owner())
            }),
        )
    }
}

/// The SAME original ordinary Text call source; classification only, never an
/// executable result/ABI grant. A genuine method is selected before Array/Core.
pub(super) fn ordinary_text_argument_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
) -> Result<Option<Rc<StaticIncomingSourceV1>>, String> {
    let Some(source) = original_static_source_v1(prepared, site)? else {
        return Ok(None);
    };
    if !matches!(source.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString) {
        return Ok(None);
    }
    if method.owner() != site.owner() || method.site() != site.site()
        || !std::sync::Arc::ptr_eq(source.method_source(), method)
        || source.call_site() != site
        || source.call_result_evidence().is_some_and(|row| row.static_target_key() != Some(source.target())
            || row.result_representation() != crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactString)
    { return Err(freeze("borrowed-actual/text-call-source-identity")); }
    Ok(Some(Rc::clone(source)))
}

/// Shared original Core argument identity; source admission grants no frame,
/// input or executable result. Both Static and Instance bind that same frame later.
pub(super) fn core_argument_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1, site: &OwnedExprSiteV1,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    formal: &CallableParameterContractKindV1,
) -> Result<Option<Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>, String>{
    let Some(receipt) = prepared.ordinary_core_source_v1(site) else {
        return Ok(None);
    };
    if !receipt.matches_original_argument_v1(site, method, formal) {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(Some(Rc::clone(receipt)))
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
            let (caller, site, target, literals) = provider
                .original_call_v1()
                .ok_or_else(|| freeze("provider-static/call-source-missing"))?;
            let Some(prepared) = self.borrowed_formal_source.as_ref() else {
                continue;
            };
            let source = prepared.as_ref().map_err(Clone::clone)?;
            // Eligibility selects only the original whole-input family. An
            // unrelated provider keeps its existing source owner and refusal.
            if source.scalar_qualified_source_call_v1(&site)?.is_none() {
                continue;
            }
            let original = source
                .source_incoming
                .static_observations()
                .get(&site)
                .ok_or_else(|| freeze("provider-static/incoming-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if original.caller() != caller
                || original.call_site() != &site
                || original.target() != target
                || original.qualified_scalar_result_v1().is_none()
                || !original.catalog_brand().is_same(claims.catalog_brand())
                || original.argument_sites().len() != literals.len()
                || literals.len() != target.arity() as usize
            {
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
                prepared,
                contracts,
                &site,
                &candidates,
                &[],
                None,
                &mut |_| None,
            )?;
            if rows.is_none() {
                return Err(freeze("provider-static/actuals-missing"));
            }
            stage_borrowed_call_actuals_v1(&mut self.borrowed_formal_actuals, &site, Ok(rows));
            self.borrowed_formal_actuals
                .get(&site)
                .ok_or_else(|| freeze("provider-static/actuals-unstaged"))?
                .as_ref()
                .map_err(Clone::clone)?;
        }
        Ok(())
    }
}

/// Select passive expression inputs before the primitive executable issuer.
/// Original source membership and Core identity, never a failed-issuer retry.
pub(super) fn selects_static_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<bool, String> {
    let original = original_static_source_v1(prepared, call)?;
    let mut core_source = false;
    for actual in actuals {
        let BorrowedCallActualValueV1::MethodCall(method) = &actual.value else {
            continue;
        };
        let site = OwnedExprSiteV1::new(call.owner(), actual.site.clone());
        core_source |= prepared
            .ordinary_core_source_v1(&site)
            .is_some_and(|receipt| std::sync::Arc::ptr_eq(method, receipt.source().source()))
            || ordinary_argument_call_source_v1(prepared, &site, method)?.is_some_and(|source|
                original.is_some_and(|outer| outer.parameters()
                    .get(actual.ordinal as usize).is_some_and(|formal|
                        source.accepts_argument_formal_v1(&formal.kind))));
    }
    let original_text_formal = |root| {
        prepared.source_value_sum_v1(root).is_some_and(|sum| {
            sum.alternatives().len() == 1
                && sum
                    .alternatives()
                    .contains(&super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)
        })
    };
    let source_expression = actuals.iter().any(|actual| match actual.value {
        BorrowedCallActualValueV1::Expression
        | BorrowedCallActualValueV1::String(_)
        | BorrowedCallActualValueV1::Binding(_) => true,
        BorrowedCallActualValueV1::SelfRooted { root, .. } => original_text_formal(root),
        _ => false,
    });
    let finite_cohort = original.is_some_and(|source|
        prepared.original_finite_forward_input_request_v1(source.callee_owner())
            || source.parameters().iter().any(|formal|
        matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle) && prepared.source_value_sum_v1(formal.binding)
            .is_some_and(|sum| sum.alternatives().contains(&super::super::borrowed_formal_source::SourceValueAlternativeV1::Text))));
    let sibling_source = finite_cohort
        && actuals.iter().all(|actual| {
            !matches!(actual.value, BorrowedCallActualValueV1::MethodCall(_))
                || prepared
                    .ordinary_core_source_v1(&OwnedExprSiteV1::new(
                        call.owner(),
                        actual.site.clone(),
                    ))
                    .is_some()
        });
    // Input observation retains the original target and input demand only.
    // Result meaning is issued separately from the same completed Return owner.
    let ordinary_input_source = original.is_some_and(|source|
        source.qualified_scalar_result_v1().is_some()
            && crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1::owning_ordinary_source_input_cohort_v1(source.parameters()));
    Ok(original.is_some() && (core_source || source_expression || sibling_source || ordinary_input_source))
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
    let source_expression = selects_static_expression_source_v1(prepared, call, actuals)?;
    let original_text_formal = |root| {
        prepared.source_value_sum_v1(root).is_some_and(|sum| {
            sum.alternatives().len() == 1
                && sum
                    .alternatives()
                    .contains(&super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)
        })
    };
    let original = original_static_source_v1(prepared, call)?;
    let original_source = original
        .is_some_and(|source| {
            source.is_zeroarg_i64_v1()
                || (source.qualified_scalar_result_v1().is_some() && crate::mir::resolved_value_profile::expression_position::CallExecutorV1::at(call.site()).is_argument_frame())
                || source.is_current_owner_i64_source_v1()
                || source.is_current_owner_bool_source_v1()
                || prepared.is_static_terminal_source_v1(source)
                || source_expression
                || matches!(source.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
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
            || formal != original
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
        let unavailable = || {
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[static-actual/source] call={call:?} ordinal={} formal={:?} value={:?}",
                    actual.ordinal, formal.kind, actual.value
                );
            }
            freeze("borrowed-static/source-actual-unavailable")
        };
        let integer = match &actual.value {
            BorrowedCallActualValueV1::Integer(_) => true,
            BorrowedCallActualValueV1::Home { binding, root, acquisition, moved: Some(moved) }
                if formal.demand == crate::mir::resolved_semantics::HomeDemandV1::Home
                    && binding.owner() == call.owner() && root.owner() == call.owner()
                    && acquisition.owner() == call.owner() && moved.site() == &actual.site && moved.is_move() =>
            {
                // Original owned source only. The common actual constructor
                // still proves its acquisition/class before entry adoption.
                false
            }
            BorrowedCallActualValueV1::Expression
                if matches!(&formal.kind,
                CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) =>
            {
                false
            }
            BorrowedCallActualValueV1::Expression
                if source_expression
                    && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle) =>
            {
                false
            }
            BorrowedCallActualValueV1::Bool(_) | BorrowedCallActualValueV1::Null => false,
            BorrowedCallActualValueV1::String(_) if formal.kind.is_ordinary_borrowed_handle() => {
                false
            }
            BorrowedCallActualValueV1::Binding(binding)
                if binding.owner() == call.owner() && formal.kind.is_ordinary_borrowed_handle() =>
            {
                false
            }
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                *kind == SourceScalarKind::Integer
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                if prepared
                    .static_arguments
                    .contains_key(&(call.clone(), actual.ordinal))
                {
                    prepared.candidate_integer_agreement(*root)
                } else if binding.owner() == call.owner()
                    && root.owner() == call.owner()
                    && formal.kind.is_ordinary_borrowed_handle()
                    && original_text_formal(*root)
                {
                    // Full-census Text source stays passive. The binder checks
                    // exact Entry provenance; the covered purpose checks current
                    // roots. No legacy forward fact or Integer proof is minted.
                    false
                } else {
                    return Err(freeze("borrowed-static/forward-source-missing"));
                }
            }
            BorrowedCallActualValueV1::MethodCall(method) => {
                let site = OwnedExprSiteV1::new(call.owner(), actual.site.clone());
                if let Some(original) = ordinary_argument_call_source_v1(prepared, &site, method)? {
                    if !original.accepts_argument_formal_v1(&formal.kind) {
                        return Err(freeze("borrowed-static/text-formal-domain"));
                    }
                    if matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) {
                        arguments.push(LocalCallArgumentV1::Expression { ordinal: actual.ordinal, site: actual.site.clone() });
                    } else {
                        arguments.push(LocalCallArgumentV1::BorrowedActual { ordinal: actual.ordinal, site: actual.site.clone() });
                    }
                    integer_evidence.push(false);
                    continue;
                }
                let receipt = core_argument_source_v1(prepared, &site, method, &formal.kind)?
                    .ok_or_else(unavailable)?;
                core_arguments.insert(actual.ordinal, receipt);
                false
            }
            _ => return Err(unavailable()),
        };
        let argument = match &formal.kind {
            kind if kind.is_ordinary_borrowed_handle()
                && matches!(actual.value, BorrowedCallActualValueV1::Expression)
                && source_expression
                && prepared
                    .source_value_sum_v1(formal.binding)
                    .is_some_and(|sum| sum.signed_only()) =>
            {
                LocalCallArgumentV1::Expression {
                    ordinal: actual.ordinal,
                    site: actual.site.clone(),
                }
            }
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
                    BorrowedCallActualValueV1::Expression
                    | BorrowedCallActualValueV1::MethodCall(_) => LocalCallArgumentV1::Expression {
                        ordinal: actual.ordinal,
                        site: actual.site.clone(),
                    },
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
        dynamic_invocation_arguments: None,
        phase: BorrowedCallActualEvidencePhaseV1::SourceStatic(StaticSourceActualIdentityV1 {
            source: Rc::clone(source),
            candidates: actuals.to_vec().into_boxed_slice(),
            integer_evidence: integer_evidence.into_boxed_slice(),
            core_arguments,
            argument_sources: BTreeMap::new(),
        }),
    }))
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

pub(in crate::mir::normal_callable_semantic_package) use super::static_argument_projection::{
    project_pending_static_source_arguments_v1, project_pending_current_owner_static_source_arguments_v1,
    project_current_owner_loop_scalar_source_v1, project_pending_static_source_arguments_for_route_v1,
};
pub(super) use super::static_argument_projection::checked_executable_qualified_arguments_v1;
