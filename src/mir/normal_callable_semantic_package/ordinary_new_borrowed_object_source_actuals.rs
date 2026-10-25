//! Original Instance source arguments remain on the existing pending actual row.
//! No source phase owns opaque proofs or grants entry/ABI/Normal permission.
use super::*;

/// Where the common actual constructor borrows the original local New domain.
/// This chooses an issuer context; a failed source lookup never retries another.
#[derive(Clone, Copy)]
pub(super) enum ObjectActualHomeSourceV1<'a> {
    Candidates(&'a [super::super::super::candidate::OrdinaryNewCandidate]),
    Ledger(&'a crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1),
    SourceOnly,
}
impl ObjectActualHomeSourceV1<'_> {
    pub(super) fn home_class(
        self,
        call: &OwnedExprSiteV1,
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) -> Result<Option<Box<str>>, String> {
        if binding.owner() != call.owner()
            || root.owner() != call.owner()
            || acquisition.owner() != call.owner()
        {
            return Err(freeze("borrowed-actual/home-domain"));
        }
        match self {
            Self::Candidates(candidates) => {
                let mut matching = candidates
                    .iter()
                    .filter(|row| &row.site == acquisition && row.destination == root);
                let candidate = matching
                    .next()
                    .ok_or_else(|| freeze("borrowed-actual/home-source"))?;
                if matching.next().is_some() || candidate.construction.is_err() {
                    return Err(freeze("borrowed-actual/home-domain"));
                }
                Ok(Some(candidate.class.clone()))
            }
            Self::Ledger(ledger) => ledger.borrowed_local_new_actual_class_v1(acquisition, root),
            Self::SourceOnly => Err(freeze("borrowed-actual/home-source-unconnected")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObjectSourceActualIdentityV1 {
    pub(super) source: LexicalInstanceCallSourceTargetV1,
    pub(super) incoming_arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) argument_sources: BTreeMap<u32, ArgumentExpressionV1>,
}
impl ObjectSourceActualIdentityV1 {
    pub(super) fn same_original_source_v1(&self, other: &Self) -> bool {
        self.source == other.source
            && self.incoming_arguments == other.incoming_arguments
            && self.candidates == other.candidates
    }
}

/// The existing Object input owner keeps its phase for Field arguments too.
pub(super) fn selected_returned_field_input_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &OwnedExprSiteV1,
) -> Result<bool, String> {
    if prepared
        .source_incoming
        .static_observations()
        .contains_key(call)
    {
        return Ok(false);
    }
    Ok(prepared.returned_field_argument_selected_v1(call)
        && prepared
            .object_source_target_at_v1(call)?
            .is_some_and(|target| prepared.source_incoming.has_object_input_callee_v1(target)))
}

/// Issuer and both projection ports require the same original Object phase.
/// This selects input evidence only, without result or executable permission.
pub(super) fn selected_original_object_phase_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &OwnedExprSiteV1,
) -> Result<bool, String> {
    // Static observations keep their own input owner, including its errors.
    if prepared
        .source_incoming
        .static_observations()
        .contains_key(call)
    {
        return Ok(false);
    }
    if selected_returned_field_input_v1(prepared, call)? {
        return Ok(true);
    }
    let Some(target) = prepared.object_source_target_at_v1(call)? else {
        return Ok(false);
    };
    if prepared.original_signed_expression_argument_request_v1(call)
        && prepared.source_incoming.has_object_input_callee_v1(target)
    {
        return Ok(true);
    }
    Ok(selected_source_only_object_forward_v1(prepared, target))
}

pub(super) fn prepare_object_source_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    let mut rows = prepared
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == call);
    let Some(original) = rows.next() else {
        return Ok(None);
    };
    if rows.next().is_some() {
        return Err(freeze("borrowed-object/duplicate-source"));
    }
    let Some(target) = original.source.instance() else {
        return Ok(None);
    };
    let qualifications = target.object_return_sources();
    if qualifications.is_some_and(|rows| {
        rows.is_empty()
            || rows
                .iter()
                .any(|loan| loan.call() != call || loan.key() != target.target())
    }) {
        return Err(freeze("borrowed-object/source-actual-identity"));
    }
    if !prepared.source_incoming.has_object_input_callee_v1(target) {
        return Ok(None);
    }
    let mut matching = contracts.iter().filter(|row| row.owner == original.callee);
    let contract = matching
        .next()
        .ok_or_else(|| freeze("borrowed-object/source-contract"))?;
    if matching.next().is_some()
        || original.callee != target.callee_owner()
        || target.call_site() != call
        || contract.batch_slot != target.target_batch_slot()
        || target.argument_sites().len() != target.target().arity() as usize
        || contract.parameters.len() != actuals.len()
        || actuals.len() != target.argument_sites().len()
        || original.source.as_loan()?.declaration_mode() != contract.mode
    {
        return Err(freeze("borrowed-object/source-actual-identity"));
    }
    let opaque: Vec<_> = contract
        .parameters
        .iter()
        .filter(|formal| formal.kind.is_ordinary_borrowed_handle())
        .collect();
    if opaque.len() != original.arguments.len()
        || opaque
            .iter()
            .zip(&original.arguments)
            .any(|(formal, (ordinal, site, binding))| {
                *ordinal != formal.ordinal
                    || *binding != formal.binding
                    || target.argument_sites().get(*ordinal as usize) != Some(site)
            })
    {
        return Err(freeze("borrowed-object/source-opaque-identity"));
    }
    let mut arguments = Vec::with_capacity(actuals.len());
    for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
        if formal.ordinal as usize != index
            || actual.ordinal != formal.ordinal
            || formal.binding.owner() != original.callee
            || target.argument_sites().get(index) != Some(&actual.site)
        {
            return Err(freeze("borrowed-object/source-actual-identity"));
        }
        if let BorrowedCallActualValueV1::ReceivedField {
            parent,
            acquisition,
        } = &actual.value
        {
            let source = prepared
                .original_returned_field_actual_source_v1(call, actual.ordinal)?
                .ok_or_else(|| freeze("borrowed-object/field-original-source-missing"))?;
            if source.read().site() != &actual.site
                || source.parent() != *parent
                || source.acquisition() != acquisition
            {
                return Err(freeze("borrowed-object/field-original-source-drift"));
            }
        }
        let scalar_forward = scalar_forward_v1(prepared, call, actual);
        if target
            .object_source_forwards()
            .is_some_and(|forwards| forwards.iter().any(|row| row.ordinal() == actual.ordinal))
            && !matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { .. })
            && scalar_forward.is_none()
        {
            return Err(freeze("borrowed-object/forward-source-identity"));
        }
        if target.object_source_forwards().is_none()
            && prepared.source_only_definitions.get(&call.owner()).is_some_and(|draft| {
                draft.uses.iter().any(|row| matches!(&row.kind,
                    super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument {
                        call: original, ordinal
                    } if original == call && *ordinal == actual.ordinal))
            })
        {
            validate_source_only_sibling_forward_v1(prepared, target, actual)?;
        }
        let forwarded = match actual.value {
            BorrowedCallActualValueV1::SelfRooted { binding, root } => Some((binding, root)),
            _ => scalar_forward,
        };
        if let Some((binding, root)) = forwarded {
            if let Some(forwards) = target.object_source_forwards() {
                let mut matching = forwards
                    .iter()
                    .filter(|forward| forward.ordinal() == actual.ordinal);
                let forward = matching
                    .next()
                    .ok_or_else(|| freeze("borrowed-object/forward-source-missing"))?;
                if matching.next().is_some()
                    || forward.call() != call
                    || forward.target() != target.target()
                    || forward.site().site() != &actual.site
                    || forward.binding() != binding
                    || forward.source_formal() != root
                    || forward.callee_formal() != formal.binding
                {
                    return Err(freeze("borrowed-object/forward-source-identity"));
                }
            } else if scalar_forward.is_some() {
                // The sibling proof covers a self-rooted forward only.
                return Err(freeze("borrowed-object/forward-source-identity"));
            } else {
                validate_source_only_sibling_forward_v1(prepared, target, actual)?;
            }
        }
        arguments.push(match &formal.kind {
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
                    _ => return Err(freeze("borrowed-object/nonopaque-integer-unproved")),
                }
            }
            _ => return Err(freeze("borrowed-object/input-contract-unsupported")),
        });
    }
    Ok(Some(PreparedBorrowedCallActualsV1 {
        phase: BorrowedCallActualEvidencePhaseV1::SourceObject(ObjectSourceActualIdentityV1 {
            source: target.clone(),
            incoming_arguments: original.arguments.clone(),
            candidates: actuals.to_vec().into_boxed_slice(),
            argument_sources: BTreeMap::new(),
        }),
        opaque_actuals: Box::new([]),
        ordered_arguments: arguments.into_boxed_slice(),
        dynamic_invocation_arguments: None,
    }))
}

/// A source-proven integer actual rides the same opaque forward position as a
/// self-rooted handle: the caller's own binding is both value and root, and its
/// closed source SUM must be signed-only. This lends no class or payload.
fn scalar_forward_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &OwnedExprSiteV1,
    actual: &BorrowedCallActualCandidateV1,
) -> Option<(BindingRefV1, BindingRefV1)> {
    let BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) = actual.value else {
        return None;
    };
    if binding.owner() != call.owner() {
        return None;
    }
    prepared
        .source_actual_value_sum_v1(call, actual.ordinal, &actual.site, Some(binding))
        .is_some_and(|sum| sum.signed_only())
        .then_some((binding, binding))
}

/// An unqualified sibling lends only its original input use. It cannot borrow
/// another caller's return qualification or the executable forwarding protocol.
pub(super) fn validate_source_only_sibling_forward_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    target: &LexicalInstanceCallSourceTargetV1,
    actual: &BorrowedCallActualCandidateV1,
) -> Result<(), String> {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
    let call = target.call_site();
    let draft = prepared
        .source_only_definitions
        .get(&call.owner())
        .filter(|_| !prepared.definitions.contains_key(&call.owner()))
        .ok_or_else(|| freeze("borrowed-object/sibling-source-draft"))?;
    let BorrowedCallActualValueV1::SelfRooted { binding, root } = actual.value else {
        return Err(freeze("borrowed-object/sibling-source-actual"));
    };
    let mut uses = draft
        .uses
        .iter()
        .filter(|row| row.site.site() == &actual.site);
    let row = uses
        .next()
        .ok_or_else(|| freeze("borrowed-object/sibling-source-use"))?;
    if uses.next().is_some()
        || binding.owner() != call.owner()
        || root.owner() != call.owner()
        || target.argument_sites().get(actual.ordinal as usize) != Some(&actual.site)
        || row.site.owner() != call.owner()
        || row.binding != binding
        || row.formal != root
        || draft.origins.get(&binding) != Some(&root)
        || !matches!(&row.kind, BorrowedFormalUseDraftKindV1::UnresolvedArgument { call: original, ordinal }
            if original == call && *ordinal == actual.ordinal)
    {
        return Err(freeze("borrowed-object/sibling-source-identity"));
    }
    Ok(())
}

/// Demand uses original source selection even if a pending phase was lost.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn selected_source_only_object_forward_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    target: &LexicalInstanceCallSourceTargetV1,
) -> bool {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1;
    let call = target.call_site();
    if prepared.definitions.contains_key(&call.owner())
        || !prepared.source_incoming.has_object_input_callee_v1(target)
    {
        return false;
    }
    if let Some(forwards) = target.object_source_forwards() {
        return !forwards.is_empty();
    }
    prepared.source_only_definitions.get(&call.owner()).is_some_and(|draft| {
        draft.uses.iter().any(|row| row.site.owner() == call.owner()
            && draft.origins.get(&row.binding) == Some(&row.formal)
            && matches!(&row.kind, BorrowedFormalUseDraftKindV1::UnresolvedArgument { call: original, .. }
                if original == call))
    })
}

/// Reconstruct the original input protocol; whole declaration/use closure stays separate.
/// Entry and final physical Normal/use closure remain independent obligations.
pub(super) fn reconstruct_object_source_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    identity: &ObjectSourceActualIdentityV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    if identity.source != *call.source.require_instance()?
        || identity.incoming_arguments != call.arguments
    {
        return Err(freeze("borrowed-guarded/original-input"));
    }
    let mut original =
        prepare_object_source_actuals_v1(prepared, contracts, &call.call, &identity.candidates)?
            .ok_or_else(|| freeze("borrowed-guarded/source-unavailable"))?;
    model::retain_object_argument_sources_v1(&mut original, identity)?;
    if original.phase != BorrowedCallActualEvidencePhaseV1::SourceObject(identity.clone()) {
        return Err(freeze("borrowed-guarded/source-identity"));
    }
    let Some(mut regenerated) = reconstruct_original_argument_protocol_v1(
        prepared,
        contracts,
        call,
        &identity.candidates,
        ledger,
    )?
    else {
        return Ok(None);
    };
    if regenerated.ordered_arguments != original.ordered_arguments {
        return Err(freeze("borrowed-guarded/ordered-input"));
    }
    regenerated.phase = BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity.clone());
    Ok(Some(regenerated))
}

/// One original per-argument constructor context, independent of result law.
pub(super) fn reconstruct_original_argument_protocol_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    candidates: &[BorrowedCallActualCandidateV1],
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    reconstruct_original_argument_protocol_with_sources_v1(
        prepared,
        contracts,
        call,
        candidates,
        &mut |ordinal, site, formal| {
            let Some(candidate) = candidates
                .iter()
                .find(|actual| actual.ordinal == ordinal && &actual.site == site)
            else {
                return Err(freeze("borrowed-object/field-candidate-missing"));
            };
            let core_integer = if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
                contracts.iter().find(|row| row.owner == call.callee)
                    .and_then(|row| row.parameters.iter().find(|row| row.binding == formal))
                    .filter(|row| matches!(row.kind, CallableParameterContractKindV1::OpaqueHandle))
                    .map(|row| static_source::core_argument_source_v1(prepared,
                        &OwnedExprSiteV1::new(call.call.owner(), site.clone()), method, &row.kind))
                    .transpose()?.flatten().is_some_and(|core| core.source().target().result()
                        == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller)
            } else { false };
            if core_integer || matches!(
                candidate.value,
                BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::Unknown
            ) {
                return ledger
                    .map(|ledger| {
                        ledger
                            .borrowed_expression_actual_source_v1(&call.call, ordinal, site)
                            .map(|actual| Some(actual.source))
                    })
                    .unwrap_or(Ok(None));
            }
            let BorrowedCallActualValueV1::ReceivedField {
                parent,
                acquisition,
            } = &candidate.value
            else {
                return Ok(None);
            };
            let Some(ledger) = ledger else {
                return Ok(None);
            };
            ledger.retained_returned_field_read_source_v1(&call.call, ordinal, *parent, acquisition)?
            .map(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_field_v1).transpose()
            .map(|row| row.map(BorrowedFormalActualSourceV1::Read))
        },
        ledger,
    )
}

pub(super) fn reconstruct_original_argument_protocol_with_sources_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    candidates: &[BorrowedCallActualCandidateV1],
    source_actual: &mut dyn FnMut(
        u32,
        &SourceExprSiteV1,
        BindingRefV1,
    ) -> Result<Option<BorrowedFormalActualSourceV1>, String>,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    if let Some(ledger) = ledger {
        let retained = ledger
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-object/source-ledger-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if !std::ptr::eq(retained, prepared) {
            return Err(freeze("borrowed-object/source-ledger-drift"));
        }
    }
    let ObjectInputActualContextV1::Prepared { guarded_definition } =
        original_object_input_context_v1(prepared, contracts, call, candidates, ledger)?
    else {
        return Ok(None);
    };
    construct_borrowed_call_actuals_v1(
        prepared,
        call,
        contracts,
        &call.call,
        candidates,
        ledger
            .map(ObjectActualHomeSourceV1::Ledger)
            .unwrap_or(ObjectActualHomeSourceV1::SourceOnly),
        None,
        &mut |_| None,
        None,
        guarded_definition,
        source_actual,
    )
}

/// A borrowed constructor context, not an entry or execution permission.
enum ObjectInputActualContextV1<'a> {
    Pending,
    Prepared {
        guarded_definition:
            Option<&'a super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1>,
    },
}

fn original_object_input_context_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    candidates: &[BorrowedCallActualCandidateV1],
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<ObjectInputActualContextV1<'a>, String> {
    use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
    use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
    let mut guarded_definition = None;
    for actual in candidates {
        match &actual.value {
            BorrowedCallActualValueV1::Home { .. }
            | BorrowedCallActualValueV1::ReceivedField { .. }
            | BorrowedCallActualValueV1::ReceivedNullable(_)
            | BorrowedCallActualValueV1::ReceivedMixed(_) => {
                // Reconstruction borrows the exact source domain from the
                // same ledger; it never fabricates an initial New candidate.
                if ledger.is_none() {
                    return Ok(ObjectInputActualContextV1::Pending);
                }
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root }
                if prepared
                    .definitions
                    .get(&call.call.owner())
                    .and_then(|draft| draft.origins.get(binding))
                    != Some(root) =>
            {
                let mut originals = contracts
                    .iter()
                    .filter(|row| row.owner == call.call.owner());
                let contract = originals
                    .next()
                    .ok_or_else(|| freeze("borrowed-guarded/caller-contract-missing"))?;
                if originals.next().is_some() || contract.mode != Mode::InstanceBoxMethod {
                    return Err(freeze("borrowed-guarded/caller-contract-identity"));
                }
                let mut formals = contract
                    .parameters
                    .iter()
                    .filter(|formal| formal.binding == *root);
                let Some(formal) = formals.next() else {
                    // Entry receiver proof is not retained in this passive input.
                    return Ok(ObjectInputActualContextV1::Pending);
                };
                if formals.next().is_some()
                    || formal.binding.owner() != call.call.owner()
                    || contract
                        .parameters
                        .get(formal.ordinal as usize)
                        .map(|row| row.binding)
                        != Some(*root)
                {
                    return Err(freeze("borrowed-guarded/caller-formal-identity"));
                }
                if matches!(
                    formal.kind,
                    CallableParameterContractKindV1::DeclaredObject(_)
                ) {
                    continue;
                }
                if formal.kind != CallableParameterContractKindV1::OpaqueHandle {
                    return Ok(ObjectInputActualContextV1::Pending);
                }
                let Some(draft) = prepared.source_only_definitions.get(&call.call.owner()) else {
                    return Ok(ObjectInputActualContextV1::Pending);
                };
                let mut uses = draft
                    .uses
                    .iter()
                    .filter(|row| row.site.site() == &actual.site);
                let row = uses
                    .next()
                    .ok_or_else(|| freeze("borrowed-guarded/actual-use-missing"))?;
                if uses.next().is_some()
                    || row.site.owner() != call.call.owner()
                    || row.binding != *binding
                    || row.formal != *root
                    || draft.origins.get(binding) != Some(root)
                    || !matches!(&row.kind, Use::UnresolvedArgument { call: original, ordinal }
                        if original == &call.call && *ordinal == actual.ordinal)
                {
                    return Err(freeze("borrowed-guarded/actual-use-identity"));
                }
                if !prepared
                    .guarded_actuals
                    .contains_key(&(call.call.clone(), actual.ordinal))
                {
                    if let Some(selected) = prepared.original_mixed_tagged_forward_v1(
                        contracts,
                        &call.call,
                        actual.ordinal,
                        &actual.site,
                        *binding,
                        *root,
                    )? {
                        if !std::ptr::eq(selected, draft) {
                            return Err(freeze("mixed-forward/draft-drift"));
                        }
                        guarded_definition = Some(draft);
                        continue;
                    }
                }
                let Some(fact) = prepared
                    .guarded_actuals
                    .get(&(call.call.clone(), actual.ordinal))
                else {
                    return Ok(ObjectInputActualContextV1::Pending);
                };
                if !fact.matches_draft_argument(row, &call.call, actual.ordinal, draft) {
                    return Err(freeze("borrowed-guarded/actual-guard-identity"));
                }
                let incoming = prepared
                    .source_incoming
                    .project_declaration_v1(call.call.owner())
                    .map_err(|issue| {
                        format!("{}: {issue:?}", freeze("borrowed-guarded/caller-incoming"))
                    })?;
                let modes = incoming
                    .iter()
                    .map(|row| row.source.as_loan().map(|loan| loan.declaration_mode()))
                    .collect::<Result<Vec<_>, String>>()?;
                if modes.iter().any(|mode| *mode != contract.mode) {
                    return Err(freeze("borrowed-guarded/caller-incoming-mode"));
                }
                guarded_definition = Some(draft);
            }
            _ => {}
        }
    }
    Ok(ObjectInputActualContextV1::Prepared { guarded_definition })
}

/// A special phase alone is not a guarded reverse dependency edge.
pub(super) fn contains_guarded_object_forward_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    identity: &ObjectSourceActualIdentityV1,
) -> bool {
    identity.candidates.iter().any(|actual| match actual.value {
        BorrowedCallActualValueV1::SelfRooted { binding, root } => {
            !prepared
                .definitions
                .get(&call.call.owner())
                .is_some_and(|draft| draft.origins.get(&binding) == Some(&root))
                && prepared
                    .source_only_definitions
                    .get(&call.call.owner())
                    .is_some_and(|draft| draft.origins.get(&binding) == Some(&root))
                && prepared
                    .guarded_actuals
                    .contains_key(&(call.call.clone(), actual.ordinal))
        }
        _ => false,
    })
}

pub(super) fn recheck_object_source_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
    identity: &ObjectSourceActualIdentityV1,
    ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<(), String> {
    let regenerated =
        reconstruct_object_source_actuals_v1(prepared, contracts, call, identity, ledger)?
            .ok_or_else(|| freeze("borrowed-guarded/reconstruction-unavailable"))?;
    if rows != &regenerated {
        return Err(freeze("borrowed-guarded/reconstructed-input-drift"));
    }
    Ok(())
}

/// Select the original passive input before execution construction.
pub(super) fn needs_original_object_forward_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    incoming: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<bool, String> {
    let Some(target) = incoming.source.instance() else {
        return Ok(false);
    };
    if !prepared.source_incoming.has_object_input_callee_v1(target)
        || prepared.definitions.contains_key(&call.owner())
    {
        return Ok(false);
    }
    let Some(forwards) = target.object_source_forwards() else {
        // A sibling can share the callee's original Object input demand without
        // owning an Object-return qualification. Keep its input nonexecuting.
        return Ok(prepared
            .source_only_definitions
            .get(&call.owner())
            .is_some_and(|draft| {
                actuals.iter().any(|actual| {
                    matches!(actual.value,
                BorrowedCallActualValueV1::SelfRooted { binding, .. }
                    if draft.origins.contains_key(&binding))
                })
            }));
    };
    let mut source_only = false;
    for actual in actuals {
        let matching: Vec<_> = forwards
            .iter()
            .filter(|row| row.ordinal() == actual.ordinal)
            .collect();
        let [forward] = matching.as_slice() else {
            if matching.is_empty() {
                continue;
            }
            return Err(freeze("borrowed-object/forward-source-identity"));
        };
        let (binding, root) = match actual.value {
            BorrowedCallActualValueV1::SelfRooted { binding, root } => (binding, root),
            _ => scalar_forward_v1(prepared, call, actual)
                .ok_or_else(|| freeze("borrowed-object/forward-source-identity"))?,
        };
        if forward.call() != call
            || forward.target() != target.target()
            || forward.site().site() != &actual.site
            || forward.binding() != binding
            || forward.source_formal() != root
            || !incoming.arguments.iter().any(|(ordinal, site, formal)| {
                *ordinal == actual.ordinal
                    && *site == actual.site
                    && *formal == forward.callee_formal()
            })
        {
            return Err(freeze("borrowed-object/forward-source-identity"));
        }
        source_only = true;
    }
    if source_only {
        let mut originals = prepared
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == call);
        let original = originals
            .next()
            .ok_or_else(|| freeze("borrowed-object/source-row-missing"))?;
        if originals.next().is_some()
            || original.source.require_instance()? != target
            || original.callee != incoming.callee
            || original.arguments != incoming.arguments
            || incoming.call != *call
            || target.call_site() != call
        {
            return Err(freeze("borrowed-object/source-actual-identity"));
        }
    }
    Ok(source_only)
}
