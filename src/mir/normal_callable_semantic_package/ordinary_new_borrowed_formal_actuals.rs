//! Source/domain/liveness join of the existing borrowed ingress preparation.
//! Pending actuals do not arm LocalCallObservation or a physical signature.
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualCandidateV1, BorrowedCallActualValueV1, LocalCallArgumentV1, SourceScalarKind,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedFormalActualSourceV1 {
    Integer(i64),
    Bool(bool),
    /// The exact `null` literal — tag `0` on the wire, never a zero
    /// payload borrowed from an integer or bool producer.
    Null,
    /// A caller-owned received nullable (`NullableObject(C)` call
    /// result): the ABI owner selects tag `0` or tag `3` at runtime;
    /// the sealed claim class is the sole class authority.
    ReceivedNullable {
        binding: BindingRefV1,
        class: Box<str>,
    },
    Scalar {
        binding: BindingRefV1,
        kind: SourceScalarKind,
    },
    TypedHome {
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: OwnedExprSiteV1,
        class: Box<str>,
    },
    EntryReceiver {
        binding: BindingRefV1,
        root: BindingRefV1,
        class: Box<str>,
    },
    /// The caller's own `DeclaredObject` formal passed through as an
    /// opaque actual — the caller's sealed parameter contract is the
    /// sole class authority; ordinary-borrowed `origins`/`forwards`
    /// drafts never cover declared formals.
    DeclaredFormal {
        binding: BindingRefV1,
        root: BindingRefV1,
        class: Box<str>,
    },
    Forwarded {
        binding: BindingRefV1,
        formal: BindingRefV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedBorrowedFormalActualV1 {
    pub(crate) ordinal: u32,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) formal: BindingRefV1,
    pub(crate) source: BorrowedFormalActualSourceV1,
}

/// The same pending call owns both its opaque proofs and the full ordered
/// argument projection. No second site inventory or domain authority is minted.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BorrowedCallActualEvidencePhaseV1 {
    Executable,
    SourceStatic(static_source::StaticSourceActualIdentityV1),
    SourceObject(object_source::ObjectSourceActualIdentityV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct PreparedBorrowedCallActualsV1 {
    phase: BorrowedCallActualEvidencePhaseV1,
    pub(super) opaque_actuals: Box<[PreparedBorrowedFormalActualV1]>,
    pub(super) ordered_arguments: Box<[LocalCallArgumentV1]>,
}

impl PreparedBorrowedCallActualsV1 {
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_for_test(
        ingress: &PreparedBorrowedFormalIngressV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1,
        candidates: &[BorrowedCallActualCandidateV1],
    ) -> Self {
        object_source::prepare_object_source_actuals_v1(ingress, contracts, call, candidates)
            .unwrap()
            .unwrap()
    }

    pub(super) fn require_executable_v1(&self) -> Result<(), String> {
        match self.phase {
            BorrowedCallActualEvidencePhaseV1::Executable => Ok(()),
            BorrowedCallActualEvidencePhaseV1::SourceObject(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-object-actuals",
            )),
            BorrowedCallActualEvidencePhaseV1::SourceStatic(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-static-actuals",
            )),
        }
    }

    pub(super) fn ordered_arguments_for_v1(
        &self,
        call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> Result<&[LocalCallArgumentV1], String> {
        self.require_executable_v1()?;
        if self.ordered_arguments.len() != call.source.argument_sites().len() {
            return Err(freeze("borrowed-entry/ordered-arguments-cardinality"));
        }
        for (ordinal, argument) in self.ordered_arguments.iter().enumerate() {
            let opaque = call
                .arguments
                .iter()
                .find(|(index, _, _)| *index as usize == ordinal);
            match (opaque, argument) {
                    (Some((index, site, _)), crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: actual, site: observed })
                        if actual == index && observed == site => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(_)) => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Scalar(binding))
                        if binding.owner() == call.call.owner() => {}
                    _ => return Err(freeze("borrowed-entry/ordered-arguments-identity")),
                }
        }
        if self.opaque_actuals.len() != call.arguments.len() {
            return Err(freeze("borrowed-entry/actuals-cardinality"));
        }
        for (actual, (ordinal, site, formal)) in
            self.opaque_actuals.iter().zip(call.arguments.iter())
        {
            if actual.ordinal != *ordinal
                || &actual.site != site
                || actual.formal != *formal
                || formal.owner() != call.callee
            {
                return Err(freeze("borrowed-entry/actuals-identity"));
            }
        }
        Ok(&self.ordered_arguments)
    }
}

pub(in crate::mir::normal_callable_semantic_package) type PendingBorrowedFormalActualsV1 =
    BTreeMap<OwnedExprSiteV1, Result<PreparedBorrowedCallActualsV1, String>>;

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
    let mut incoming = prepared.incoming.iter().filter(|row| &row.call == call);
    let Some(incoming_row) = incoming.next() else {
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
    if needs_original_object_forward_source_v1(prepared, incoming_row, call, actuals)? {
        return object_source::prepare_object_source_actuals_v1(prepared, contracts, call, actuals);
    }
    construct_borrowed_call_actuals_v1(
        prepared,
        incoming_row,
        contracts,
        call,
        actuals,
        candidates,
        receiver,
        nullable_class,
    )
}

/// Select before execution construction when an exact source forward has no
/// final caller definition. Entry receivers and declared objects keep their law.
fn needs_original_object_forward_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    incoming: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
) -> Result<bool, String> {
    let Some(target) = incoming.source.instance() else {
        return Ok(false);
    };
    if !target.has_object_source_requirement() || prepared.definitions.contains_key(&call.owner()) {
        return Ok(false);
    }
    let Some(forwards) = target.object_source_forwards() else {
        return Ok(false);
    };
    let mut source_only = false;
    for actual in actuals {
        let BorrowedCallActualValueV1::SelfRooted { binding, root } = actual.value else {
            continue;
        };
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

/// Construct from an original incoming row after the caller has selected it.
/// Source selection and all later entry/Completion permissions stay separate.
#[allow(clippy::too_many_arguments)]
fn construct_borrowed_call_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    incoming_row: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    candidates: &[super::super::candidate::OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
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
    if incoming_row.source.as_loan().declaration_mode() != contract.mode {
        return Err(freeze("borrowed-actual/source-mode"));
    }
    if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
        &incoming_row.source
    {
        original.require_qualified()?;
        let retained = prepared
            .source_incoming
            .static_observations()
            .get(call)
            .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
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
            BorrowedCallActualValueV1::ReceivedNullable(binding)
                if binding.owner() == call.owner() =>
            {
                // The sealed `NullableObject` claim is the sole class
                // authority — a fake, foreign, or unclaimed producer can
                // never mint the nullable typed-object actual.
                let class = nullable_class(*binding)
                    .ok_or_else(|| freeze("borrowed-actual/nullable-class-unavailable"))?;
                BorrowedFormalActualSourceV1::ReceivedNullable {
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
                let mut matching = candidates
                    .iter()
                    .filter(|row| &row.site == acquisition && row.destination == *root);
                let candidate = matching
                    .next()
                    .ok_or_else(|| freeze("borrowed-actual/home-source"))?;
                if matching.next().is_some()
                    || binding.owner() != call.owner()
                    || root.owner() != call.owner()
                    || acquisition.owner() != call.owner()
                    || !candidate.construction.is_ok()
                {
                    return Err(freeze("borrowed-actual/home-domain"));
                }
                BorrowedFormalActualSourceV1::TypedHome {
                    binding: *binding,
                    root: *root,
                    acquisition: acquisition.clone(),
                    class: candidate.class.clone(),
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
                } else {
                    // A declared-object formal is an entry-stable source
                    // too — the caller's own sealed parameter contract
                    // names its class; `origins`/`forwards` cover only
                    // ordinary-borrowed formals.
                    if let Some(class) = contracts
                        .iter()
                        .find(|row| row.owner == call.owner())
                        .and_then(|row| {
                            row.parameters
                                .iter()
                                .find(|formal| formal.binding == *binding)
                        })
                        .and_then(|formal| match &formal.kind {
                            CallableParameterContractKindV1::DeclaredObject(class) => {
                                Some(class.clone())
                            }
                            _ => None,
                        })
                    {
                        if binding.owner() != call.owner() || root.owner() != call.owner() {
                            return Err(freeze("borrowed-actual/entry-source"));
                        }
                        BorrowedFormalActualSourceV1::DeclaredFormal {
                            binding: *binding,
                            root: *root,
                            class: class.as_ref().into(),
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
            }
            _ => return Err(freeze("borrowed-actual/unsupported-or-unavailable")),
        };
        if let CallableParameterContractKindV1::DeclaredObject(expected) =
            &contract.parameters[*ordinal as usize].kind
        {
            let class = match &source {
                BorrowedFormalActualSourceV1::Null => None,
                BorrowedFormalActualSourceV1::TypedHome { class, .. }
                | BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                | BorrowedFormalActualSourceV1::ReceivedNullable { class, .. } => {
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
        phase: BorrowedCallActualEvidencePhaseV1::Executable,
        opaque_actuals: rows.into_boxed_slice(),
        ordered_arguments: ordered_arguments.into_boxed_slice(),
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

/// Demand by exact selected site. Source failure cannot become an empty result.
/// Borrow the original incoming target and exact ordered executable arguments.
/// Result corroboration belongs to the requesting result lane, not this lender.
pub(super) fn lend_pending_borrowed_arguments_v1<'a>(
    source: &'a Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &'a PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
) -> Result<
    Option<(
        &'a super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
        &'a [LocalCallArgumentV1],
    )>,
    String,
> {
    let source = source.as_ref().map_err(Clone::clone)?;
    let mut incoming = source.incoming.iter().filter(|call| &call.call == site);
    let Some(call) = incoming.next() else {
        return Ok(None);
    };
    if incoming.next().is_some()
        || call.source.call_site() != site
        || call.source.callee_owner() != call.callee
        || !source.definitions.contains_key(&call.callee)
    {
        return Err(freeze("borrowed-call/source-identity"));
    }
    let actuals = actuals
        .get(site)
        .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    let arguments = actuals.ordered_arguments_for_v1(call)?;
    Ok(Some((call, arguments)))
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_i64_result_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    results: &BTreeMap<
        FunctionOwnerIdV1,
        Result<super::borrowed_formal_result::BorrowedI64ResultSourceV1, String>,
    >,
    site: &OwnedExprSiteV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let arguments = project_pending_borrowed_i64_arguments_v1(source, actuals, results, site)?;
    let Some(arguments) = arguments else {
        return Ok(None);
    };
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let call = ingress
        .incoming
        .iter()
        .find(|row| &row.call == site)
        .ok_or_else(|| freeze("borrowed-call/source-identity"))?;
    let proof = results
        .get(&call.callee)
        .ok_or_else(|| freeze("borrowed-call/result-source-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if proof.class != super::borrowed_formal_result::BorrowedResultClassV1::I64 {
        return Ok(None);
    }
    Ok(Some(arguments))
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_borrowed_i64_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    results: &BTreeMap<
        FunctionOwnerIdV1,
        Result<super::borrowed_formal_result::BorrowedI64ResultSourceV1, String>,
    >,
    site: &OwnedExprSiteV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let Some((call, arguments)) = lend_pending_borrowed_arguments_v1(source, actuals, site)? else {
        return Ok(None);
    };
    let proof = results
        .get(&call.callee)
        .ok_or_else(|| freeze("borrowed-call/result-source-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    proof.require_source_sealed_v1()?;
    if proof.returns.is_empty() || proof.returns.iter().any(|site| site.owner() != call.callee) {
        return Err(freeze("borrowed-call/result-source-identity"));
    }
    // Final corroboration follows Completion issuance; demanding it now cycles.
    Ok(Some(arguments.to_vec().into_boxed_slice()))
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
        if matches!(row, Ok(actuals) if matches!(actuals.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(_) | BorrowedCallActualEvidencePhaseV1::SourceObject(_)))
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
pub(in crate::mir::normal_callable_semantic_package) use static_source::project_pending_static_source_arguments_v1;

#[cfg(test)]
#[path = "ordinary_new_borrowed_actual_source_atom_tests.rs"]
mod source_atom_tests;

#[path = "ordinary_new_borrowed_object_arguments.rs"]
mod object_arguments;
pub(in crate::mir::normal_callable_semantic_package) use object_arguments::{
    corroborate_received_object_receiver_v1, project_pending_object_arguments_v1,
};

#[path = "ordinary_new_borrowed_object_source_actuals.rs"]
mod object_source;

#[path = "ordinary_new_borrowed_object_input_finish.rs"]
mod object_input_finish;

#[path = "ordinary_new_received_producer_arguments.rs"]
mod received_producer;
pub(in crate::mir::normal_callable_semantic_package) use received_producer::received_producer_arguments_v1;
