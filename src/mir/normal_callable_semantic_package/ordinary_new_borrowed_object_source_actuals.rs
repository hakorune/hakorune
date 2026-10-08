//! Original Instance source arguments remain on the existing pending actual row.
//! No source phase owns opaque proofs or grants entry/ABI/Normal permission.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObjectSourceActualIdentityV1 {
    pub(super) source: LexicalInstanceCallSourceTargetV1,
    pub(super) incoming_arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
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
        || original.source.as_loan().declaration_mode() != contract.mode
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
        if let BorrowedCallActualValueV1::SelfRooted { binding, root } = actual.value {
            let forwards = target
                .object_source_forwards()
                .ok_or_else(|| freeze("borrowed-object/forward-source-unavailable"))?;
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
        }),
        opaque_actuals: Box::new([]),
        ordered_arguments: arguments.into_boxed_slice(),
    }))
}
