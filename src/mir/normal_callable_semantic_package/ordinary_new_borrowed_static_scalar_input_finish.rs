//! Complete one original CurrentOwner Static scalar cohort in the existing ledger.
//! The later physical caller read remains an independent publication obligation.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticScalarInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    pub(super) completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    actual: StaticScalarActualV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticScalarActualV1 {
    CurrentOwner(BindingRefV1),
    QualifiedLiteral(static_source::StaticSourceActualIdentityV1),
    QualifiedExecutable(Box<PreparedBorrowedCallActualsV1>),
}

impl PartialEq for StaticScalarInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && self.actual == other.actual
    }
}
impl Eq for StaticScalarInputFinishV1 {}

impl StaticScalarInputFinishV1 {
    pub(super) fn corroborates(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    ) -> bool {
        Rc::ptr_eq(&self.source, original)
            && Rc::ptr_eq(&self.completion, completion)
            && match &self.actual {
                StaticScalarActualV1::CurrentOwner(binding) =>
                    original.is_current_owner_i64_source_v1()
                        && binding.owner() == original.call_site().owner(),
                StaticScalarActualV1::QualifiedLiteral(identity) =>
                    original.is_qualified_i64_source_v1() && Rc::ptr_eq(&identity.source, original),
                StaticScalarActualV1::QualifiedExecutable(_) => original.is_qualified_i64_source_v1(),
            }
            && (original.required_i64_arguments() == [0]
                || original.required_i64_arguments().is_empty())
            && original.parameters().len() == 1
            && original.argument_sites().len() == 1
            && completion.owner() == original.callee_owner()
    }
}

pub(super) fn check_exact_usize_scalar_caller_v1(
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    original: &Rc<StaticIncomingSourceV1>,
    binding: BindingRefV1,
) -> Result<(), String> {
    let mut callers = contracts
        .iter()
        .filter(|row| row.owner == original.call_site().owner());
    let caller = callers
        .next()
        .ok_or_else(|| freeze("static-scalar/caller-contract-missing"))?;
    let caller_signature = signatures
        .row(caller.batch_slot)
        .ok_or_else(|| freeze("static-scalar/caller-signature-missing"))?;
    if callers.next().is_some()
        || caller.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || caller.parameters.len() != 1
        || caller.parameters[0].ordinal != 0
        || caller.parameters[0].binding != binding
        || caller.parameters[0].kind
            != CallableParameterContractKindV1::ExactTrivial(ExactTrivialParameterAbiV1::USIZE)
        || !matches!(selected.key_for_batch_slot(caller.batch_slot),
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.caller())
        || caller_signature.owner() != caller.owner
        || caller_signature.mode() != caller.mode
        || caller_signature.source_logical_arity() != 1
        || caller_signature.lanes().len() != 1
        || caller_signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
        || caller_signature.lanes()[0].binding() != binding
    {
        return Err(freeze("static-scalar/caller-formal-drift"));
    }
    Ok(())
}


/// Keep qualified callers in their already-issued execution protocol. The
/// existing qualified projection checks original inventory, payload and required
/// Integer domains; this cannot recurse because only Executable is selected.
fn qualified_scalar_actual_v1(
    source: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
) -> Result<bool, String> {
    if !original.is_qualified_i64_source_v1() {
        return Ok(false);
    }
    let Some(actual) = pending.get(original.call_site()) else {
        return Ok(false);
    };
    let actual = actual.as_ref().map_err(Clone::clone)?;
    if let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &actual.phase {
        if let StaticScalarActualV1::QualifiedExecutable(snapshot) = &proof.actual {
            if !proof.corroborates(original, completion)
                || !source.is_compare_only_static_input_v1(original.callee_owner())
                || snapshot.opaque_actuals != actual.opaque_actuals
                || snapshot.ordered_arguments != actual.ordered_arguments {
                return Err(freeze("static-scalar/qualified-snapshot-drift"));
            }
            let call = source.scalar_qualified_source_call_v1(original.call_site())?
                .ok_or_else(|| freeze("static-scalar/qualified-source-missing"))?;
            super::static_source::checked_executable_qualified_arguments_v1(
                source, call, snapshot, original.required_i64_arguments(),
            )?;
            return Ok(true);
        }
        let StaticScalarActualV1::QualifiedLiteral(identity) = &proof.actual else {
            return Ok(false);
        };
        if !proof.corroborates(original, completion) {
            return Err(freeze("static-scalar/qualified-completion-drift"));
        }
        let Some(value) = super::static_tagged_input_finish::literal_identity_v1(
            source, original, identity, actual, original.parameters()[0].binding,
        )? else { return Ok(false) };
        if actual.opaque_actuals.as_ref() != [PreparedBorrowedFormalActualV1 {
            ordinal: 0,
            site: original.argument_sites()[0].clone(),
            formal: original.parameters()[0].binding,
            source: BorrowedFormalActualSourceV1::Integer(value),
        }] {
            return Err(freeze("static-scalar/qualified-payload-drift"));
        }
        return Ok(true);
    }
    if !matches!(actual.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        return Ok(false);
    }
    let mut originals = source
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == original.call_site() && row.callee == original.callee_owner());
    let call = originals
        .next()
        .ok_or_else(|| freeze("static-scalar/incoming-missing"))?;
    if originals.next().is_some() {
        return Err(freeze("static-scalar/incoming-identity"));
    }
    super::static_source::checked_executable_qualified_arguments_v1(
        source,
        call,
        actual,
        original.required_i64_arguments(),
    )?;
    Ok(true)
}

/// One whole original cohort, with each row's own issued argument protocol.
/// Qualified rows are borrowed, never reissued as current-owner Scalar rows.
pub(super) fn checked_ready_scalar_rows_v1<'a>(
    source: &'a PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    original: &Rc<StaticIncomingSourceV1>,
    completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
) -> Result<
    Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
    String,
> {
    let rows = source
        .source_incoming
        .project(&BTreeSet::from([original.callee_owner()]))
        .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/incoming-veto")))?;
    let cohort = source.static_incoming_cohort_v1(original)?;
    let owner = original.callee_owner();
    let compare_only = source.is_compare_only_static_input_v1(owner);
    if !cohort
        .iter()
        .any(|row| row.is_current_owner_i64_source_v1())
        && !cohort.iter().all(|row| {
            (source.is_static_terminal_source_v1(row) || compare_only)
                && source.checked_static_input(row.parameters()[0].binding)
                && matches!(pending.get(row.call_site()), Some(Ok(actual))
                    if matches!(&actual.phase,
                        BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof)
                            if matches!(proof.actual, StaticScalarActualV1::QualifiedLiteral(_)
                                | StaticScalarActualV1::QualifiedExecutable(_))))
        })
    {
        return Ok(None);
    }
    if cohort.len() != rows.len() {
        return Err(freeze("static-scalar/whole-cohort"));
    }
    let mut retained = Vec::with_capacity(rows.len());
    for original in &cohort {
        let mut incoming = source
            .source_incoming
            .exact_rows()
            .filter(|call| call.callee == owner && call.call == *original.call_site());
        let call = incoming
            .next()
            .ok_or_else(|| freeze("static-scalar/incoming-missing"))?;
        if incoming.next().is_some()
            || !matches!(&call.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                    if Rc::ptr_eq(source, original))
            || !(original.is_current_owner_i64_source_v1() || original.is_qualified_i64_source_v1())
            || !(original.required_i64_arguments() == [0]
                || original.required_i64_arguments().is_empty())
            || original.parameters().len() != 1
            || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            || call.arguments.as_ref()
                != [(
                    0,
                    original.argument_sites()[0].clone(),
                    original.parameters()[0].binding,
                )]
        {
            return Err(freeze("static-scalar/incoming-identity"));
        }
        let Some(actual) = pending.get(&call.call) else {
            return Ok(None);
        };
        let actual = actual.as_ref().map_err(Clone::clone)?;
        if original.is_qualified_i64_source_v1() {
            if !qualified_scalar_actual_v1(source, pending, original, completion)? {
                return Ok(None);
            }
            retained.push(call);
            continue;
        }
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) = &actual.phase else {
            return Ok(None);
        };
        let StaticScalarActualV1::CurrentOwner(proof_binding) = &proof.actual else {
            return Err(freeze("static-scalar/actual-source-kind"));
        };
        if !proof.corroborates(original, completion)
            || actual.ordered_arguments.as_ref()
                != [LocalCallArgumentV1::BorrowedActual {
                    ordinal: 0,
                    site: original.argument_sites()[0].clone(),
                }]
            || actual.opaque_actuals.len() != 1
            || actual.opaque_actuals[0].ordinal != 0
            || actual.opaque_actuals[0].site != original.argument_sites()[0]
            || actual.opaque_actuals[0].formal != original.parameters()[0].binding
            || !matches!(&actual.opaque_actuals[0].source,
                BorrowedFormalActualSourceV1::Scalar { binding, kind: SourceScalarKind::Integer }
                    if binding == proof_binding && binding.owner() == call.call.owner())
        {
            return Err(freeze("static-scalar/actual-identity"));
        }
        retained.push(call);
    }
    Ok(Some(retained.into_boxed_slice()))
}

/// Original scalar identity survives input finishing. This is only a source
/// loan; callers still demand the whole cohort before executable projection.
pub(super) fn original_scalar_source_v1(
    actual: &PreparedBorrowedCallActualsV1,
) -> Result<Option<(&Rc<StaticIncomingSourceV1>, BindingRefV1, &SourceExprSiteV1)>, String> {
    let (original, binding, site) = match &actual.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => {
            let [candidate] = identity.candidates.as_ref() else {
                return Ok(None);
            };
            let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) =
                candidate.value
            else {
                return Ok(None);
            };
            if candidate.ordinal != 0
                || identity.integer_evidence.as_ref() != [true]
                || !actual.opaque_actuals.is_empty()
            {
                return Err(freeze("borrowed-static/loop-scalar-source-drift"));
            }
            (&identity.source, binding, &candidate.site)
        }
        BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof) => {
            let StaticScalarActualV1::CurrentOwner(scalar_binding) = &proof.actual else {
                return Ok(None);
            };
            let [payload] = actual.opaque_actuals.as_ref() else {
                return Ok(None);
            };
            if !proof.corroborates(&proof.source, &proof.completion)
                || payload.ordinal != 0
                || proof.source.parameters().first().map(|p| p.binding) != Some(payload.formal)
                || !matches!(payload.source,
                    BorrowedFormalActualSourceV1::Scalar { binding, kind: SourceScalarKind::Integer }
                    if binding == *scalar_binding)
            {
                return Err(freeze("borrowed-static/loop-scalar-source-drift"));
            }
            (&proof.source, *scalar_binding, &payload.site)
        }
        _ => return Ok(None),
    };
    if original.call_site().owner() != binding.owner()
        || original.argument_sites() != [site.clone()]
        || actual.ordered_arguments.as_ref()
            != [LocalCallArgumentV1::BorrowedActual {
                ordinal: 0,
                site: site.clone(),
            }]
    {
        return Err(freeze("borrowed-static/loop-scalar-source-drift"));
    }
    Ok(Some((original, binding, site)))
}

impl OrdinaryNewClaimLedgerV1 {
    fn source_only_usize_scalar_cohort_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> bool {
        let Ok(rows) = source.source_incoming.project(&BTreeSet::from([owner])) else {
            return false;
        };
        !rows.is_empty()
            && rows.iter().all(|row| {
                let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                    &row.source
                else {
                    return false;
                };
                let Some(caller) = contracts
                    .iter()
                    .find(|contract| contract.owner == row.call.owner())
                else {
                    return false;
                };
                let Some(Ok(staged)) = self.borrowed_formal_actuals.get(&row.call) else {
                    return false;
                };
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(actual) = &staged.phase else {
                    return false;
                };
                if original.call_site() != &row.call
                    || original.callee_owner() != owner
                    || original.argument_sites().len() != 1
                    || original.parameters().len() != 1
                    || row.arguments.as_ref()
                        != [(
                            0,
                            original.argument_sites()[0].clone(),
                            original.parameters()[0].binding,
                        )]
                    || !Rc::ptr_eq(&actual.source, original)
                {
                    return false;
                }
                matches!((&caller.parameters[..], &actual.candidates[..]),
                ([formal], [BorrowedCallActualCandidateV1 {
                    value: BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer),
                    ..
                }]) if formal.ordinal == 0
                    && formal.binding == *binding
                    && formal.kind == CallableParameterContractKindV1::ExactTrivial(
                        ExactTrivialParameterAbiV1::USIZE))
                    && original.is_current_owner_i64_source_v1()
                    && original.required_i64_arguments().is_empty()
                    && original.parameters().len() == 1
                    && original.parameters()[0].kind.is_ordinary_borrowed_handle()
            })
    }

    /// Borrow only an entirely finished original scalar cohort; never infer
    /// entry permission from one successful caller or from a SourceStatic row.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_scalar_cohort_v1<
        'a,
    >(
        &self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<
        Option<Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>>,
        String,
    > {
        if !source.is_scalar_only_static_input_v1(owner)
            || source.definitions.contains_key(&owner)
            || source.target_static.contains_key(&owner)
        {
            return Ok(None);
        }
        // Source observations alone do not issue a completed entry cohort.
        // Apply this phase boundary to every scalar family; once a finish
        // receipt exists, project all original incoming rows below so a missing
        // sibling or veto remains an error rather than a partial permission.
        if !self.borrowed_formal_actuals.values().any(|row| matches!(row,
                Ok(actual) if matches!(&actual.phase,
                    BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(proof)
                        if proof.source.callee_owner() == owner))) {
            return Ok(None);
        }
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("static-scalar/original-catalog-missing"))?;
        let mut declarations = catalog.iter().filter(|row| row.owner == owner);
        let declaration = declarations
            .next()
            .ok_or_else(|| freeze("static-scalar/original-contract-missing"))?;
        if declarations.next().is_some() {
            return Err(freeze("static-scalar/original-contract-duplicate"));
        }
        if declaration.mode != CallableParameterDeclarationModeV1::StaticBoxMethod {
            return Ok(None);
        }
        let rows = source
            .source_incoming
            .project(&BTreeSet::from([owner]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/incoming-veto")))?;
        // A forwarding-only source does not become an entry route merely
        // because another scalar cohort was completed in this package.
        if !source.source_only_definitions[&owner].uses.iter().any(|row| matches!(
            row.kind,
            super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
        )) && !rows.iter().all(|row| matches!(
            self.borrowed_formal_actuals.get(&row.call),
            Some(Ok(actual)) if matches!(
                actual.phase,
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_)
            )
        )) {
            return Ok(None);
        }
        let Some(first) = rows.first() else {
            return Ok(None);
        };
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &first.source
        else {
            return Ok(None);
        };
        let completion = self
            .completion_index
            .get(&owner)
            .ok_or_else(|| freeze("static-scalar/completion-missing"))?
            .as_ref()
            .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/completion")))?;
        checked_ready_scalar_rows_v1(source, &self.borrowed_formal_actuals, original, completion)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_scalar_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let owners: Vec<_> = source.source_only_definitions.iter()
            .filter(|(owner, draft)| !source.target_static.contains_key(owner)
                && source.is_scalar_only_static_input_v1(**owner)
                && (draft.uses.iter().any(|row| matches!(row.kind,
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }))
                    || source.is_compare_only_static_input_v1(**owner)
                    || self.source_only_usize_scalar_cohort_v1(source, **owner, contracts)))
            .map(|(owner, _)| *owner).collect();
        for owner in owners {
            let compare_only = source.is_compare_only_static_input_v1(owner);
            let projected = match source.source_incoming.project(&BTreeSet::from([owner])) {
                Ok(rows) if !rows.is_empty() => rows,
                _ => continue, // A veto or absent caller cannot enable this entry.
            };
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(first) =
                &projected[0].source
            else {
                continue;
            };
            let cohort = match source.static_incoming_cohort_v1(first) {
                Ok(rows) if rows.len() == projected.len() => rows,
                _ => continue,
            };
            if !compare_only && !cohort.iter().any(|row| row.is_current_owner_i64_source_v1())
                && !cohort.iter().all(|row| source.is_static_terminal_source_v1(row)
                    && source.checked_static_input(row.parameters()[0].binding)) {
                continue;
            }
            let forward_only = !compare_only && !source.source_only_definitions[&owner]
                .uses
                .iter()
                .any(|row| {
                    matches!(
                row.kind,
                super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
            )
                });
            if cohort.iter().any(|original| {
                !(original.is_current_owner_i64_source_v1() || original.is_qualified_i64_source_v1())
                    || (if forward_only {
                        !original.required_i64_arguments().is_empty()
                    } else if compare_only {
                        !(original.required_i64_arguments().is_empty()
                            || original.required_i64_arguments() == [0])
                    } else {
                        original.required_i64_arguments() != [0]
                    })
                    || original.parameters().len() != 1
                    || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            }) {
                continue;
            }
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-scalar/contract-missing"))?;
            if matching.next().is_some()
                || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
                || contract.batch_slot != first.target_batch_slot()
                || contract.parameters.len() != 1
                || contract.parameters[0].ordinal != 0
                || contract.parameters[0].binding != first.parameters()[0].binding
                || contract.parameters[0].kind != first.parameters()[0].kind
                || !matches!(selected.key_for_batch_slot(contract.batch_slot),
                    Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == first.target())
            {
                return Err(freeze("static-scalar/contract-identity"));
            }
            let draft = &source.source_only_definitions[&owner];
            let scalar_formal_forward = !compare_only &&
                !draft.uses.iter().any(|row| {
                    matches!(
                row.kind,
                super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { .. }
            )
                });
            if scalar_formal_forward {
                let mut uses = draft.uses.iter().filter_map(|row| match &row.kind {
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument {
                        call, ordinal: 0,
                    } => Some((row, call)),
                    _ => None,
                });
                let (use_row, call) = uses
                    .next()
                    .ok_or_else(|| freeze("static-scalar/forward-use-missing"))?;
                let fact = source
                    .static_arguments
                    .get(&(call.clone(), 0))
                    .ok_or_else(|| freeze("static-scalar/forward-fact-missing"))?;
                if uses.next().is_some()
                    || !source.checked_static_input(contract.parameters[0].binding)
                    || fact.call() != call
                    || fact.ordinal() != 0
                    || fact.use_site() != &use_row.site
                    || fact.formal() != contract.parameters[0].binding
                    || draft.origins.get(&fact.binding()) != Some(&contract.parameters[0].binding)
                {
                    return Err(freeze("static-scalar/forward-chain-drift"));
                }
            }
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/selected-identity"))?;
            let signature = signatures
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/signature-missing"))?;
            if !signature.identity().same_as(identity)
                || signature.owner() != owner
                || signature.mode() != contract.mode
                || signature.source_logical_arity() != 1
                || signature.receiver_lane_count() != 0
                || signature.physical_formal_lane_count() != 1
                || signature.physical_callable_lane_count() != 1
                || signature.lanes().len() != 1
                || signature.lanes()[0].role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                || signature.lanes()[0].logical_ordinal() != Some(0)
                || signature.lanes()[0].binding() != contract.parameters[0].binding
            {
                return Err(freeze("static-scalar/signature-identity"));
            }
            let completion = self
                .completion_index
                .get(&owner)
                .ok_or_else(|| freeze("static-scalar/completion-missing"))?
                .as_ref()
                .map_err(|error| format!("{}: {error:?}", freeze("static-scalar/completion")))?;
            let result = results
                .row(contract.batch_slot)
                .ok_or_else(|| freeze("static-scalar/result-missing"))?
                .borrow();
            if result.owner() != owner
                || !result.identity().same_as(identity)
                || !result.declared_result_agrees_with_i64_source()
                || !completion.returns_value()
                || completion.owner() != owner
                || !std::ptr::eq(result.completion(), completion.as_ref())
            {
                return Err(freeze("static-scalar/result-completion-identity"));
            }
            let mut replacements = Vec::with_capacity(cohort.len());
            let mut checked_rows = 0;
            let has_current_owner = cohort.iter().any(|row| row.is_current_owner_i64_source_v1());
            for original in &cohort {
                let site = original.call_site();
                let Some(staged) = self.borrowed_formal_actuals.get(site) else {
                    replacements.clear();
                    break;
                };
                let staged = staged.as_ref().map_err(Clone::clone)?;
                if original.is_qualified_i64_source_v1() {
                    if qualified_scalar_actual_v1(source, &self.borrowed_formal_actuals, original, completion)? {
                        if compare_only && matches!(staged.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
                            let mut finished = staged.clone();
                            finished.phase = BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(
                                StaticScalarInputFinishV1 {
                                    source: Rc::clone(original), completion: Rc::clone(completion),
                                    actual: StaticScalarActualV1::QualifiedExecutable(Box::new(staged.clone())),
                                });
                            replacements.push((site.clone(), finished));
                        } else if !has_current_owner && !matches!(staged.phase,
                            BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_)) {
                            break;
                        }
                        checked_rows += 1;
                        continue;
                    }
                    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &staged.phase else { break };
                    let Some(value) = super::static_tagged_input_finish::literal_identity_v1(
                        source, original, identity, staged, contract.parameters[0].binding,
                    )? else { break };
                    if !staged.opaque_actuals.is_empty() { break; }
                    replacements.push((site.clone(), PreparedBorrowedCallActualsV1 {
                        phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(
                            StaticScalarInputFinishV1 {
                                source: Rc::clone(original),
                                completion: Rc::clone(completion),
                                actual: StaticScalarActualV1::QualifiedLiteral(identity.clone()),
                            }),
                        ordered_arguments: staged.ordered_arguments.clone(),
                        dynamic_invocation_arguments: None,
                        opaque_actuals: Box::new([PreparedBorrowedFormalActualV1 {
                            ordinal: 0, site: original.argument_sites()[0].clone(),
                            formal: contract.parameters[0].binding,
                            source: BorrowedFormalActualSourceV1::Integer(value),
                        }]),
                    }));
                    checked_rows += 1;
                    continue;
                }
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(source_actual) = &staged.phase
                else {
                    replacements.clear();
                    break;
                };
                let [candidate] = source_actual.candidates.as_ref() else {
                    replacements.clear();
                    break;
                };
                let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) =
                    candidate.value
                else {
                    replacements.clear();
                    break;
                };
                if scalar_formal_forward {
                    check_exact_usize_scalar_caller_v1(
                        selected, contracts, signatures, original, binding,
                    )?;
                }
                if !Rc::ptr_eq(&source_actual.source, original)
                    || source_actual.integer_evidence.as_ref() != [true]
                    || candidate.ordinal != 0
                    || candidate.site != original.argument_sites()[0]
                    || binding.owner() != site.owner()
                    || !staged.opaque_actuals.is_empty()
                    || staged.ordered_arguments.as_ref()
                        != [LocalCallArgumentV1::BorrowedActual {
                            ordinal: 0,
                            site: candidate.site.clone(),
                        }]
                    || projected.iter().filter(|row| row.call == *site).count() != 1
                {
                    replacements.clear();
                    break;
                }
                replacements.push((
                    site.clone(),
                    PreparedBorrowedCallActualsV1 {
                        phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(
                            StaticScalarInputFinishV1 {
                                source: Rc::clone(original),
                                completion: Rc::clone(completion),
                                actual: StaticScalarActualV1::CurrentOwner(binding),
                            },
                        ),
                        opaque_actuals: Box::new([PreparedBorrowedFormalActualV1 {
                            ordinal: 0,
                            site: candidate.site.clone(),
                            formal: original.parameters()[0].binding,
                            source: BorrowedFormalActualSourceV1::Scalar {
                                binding,
                                kind: SourceScalarKind::Integer,
                            },
                        }]),
                        ordered_arguments: staged.ordered_arguments.clone(),
                        dynamic_invocation_arguments: None,
                    },
                ));
                checked_rows += 1;
            }
            if checked_rows == cohort.len() {
                for (site, actuals) in replacements {
                    self.borrowed_formal_actuals.insert(site, Ok(actuals));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_scalar_input_finish_tests.rs"]
mod tests;
