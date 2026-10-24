//! Exact argument projection for selected borrowed calls and scalar result lanes.
use super::*;

/// Demand by exact selected site. Source failure cannot become an empty result.
/// Borrow the original incoming target and exact ordered executable arguments.
/// Result corroboration belongs to the requesting result lane, not this lender.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn lend_pending_borrowed_arguments_v1<
    'a,
>(
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

/// Same original argument row, selected by its retained phase, never by retry.
/// This lends only source argument observation; result/entry authority is separate.
pub(in crate::mir::normal_callable_semantic_package) fn project_pending_instance_arguments_by_phase_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    target: &LexicalInstanceCallSourceTargetV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Box<[LocalCallArgumentV1]>, String> {
    let site = target.call_site();
    let row = actuals
        .get(site)
        .ok_or_else(|| freeze("borrowed-instance/source-actuals-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    use BorrowedCallActualEvidencePhaseV1 as Phase;
    match &row.phase {
        Phase::SourceInstance(_)
        | Phase::ExecutableSourceInstance(_)
        | Phase::SourceObject(_)
        | Phase::ExecutableSourceObject(_) => {
            super::project_pending_instance_source_arguments_with_array_context_v1(
                source,
                actuals,
                contracts,
                site,
                array_ledger,
            )?
            .ok_or_else(|| freeze("borrowed-instance/source-actuals-missing"))
        }
        Phase::Executable => {
            let (incoming, arguments) = lend_pending_borrowed_arguments_v1(source, actuals, site)?
                .ok_or_else(|| freeze("borrowed-instance/source-actuals-missing"))?;
            if incoming.source.require_instance()? != target {
                return Err(freeze("borrowed-instance/source-identity"));
            }
            Ok(arguments.into())
        }
        _ => Err(freeze("borrowed-instance/source-phase-drift")),
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_i64_result_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    results: &BorrowedResultCohortV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let source_instance = matches!(
        actuals.get(site),
        Some(Ok(rows))
            if matches!(&rows.phase,
                BorrowedCallActualEvidencePhaseV1::SourceInstance(_)
                    | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_))
    );
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let mut originals = ingress
        .source_incoming
        .source_rows_v1()
        .filter(|row| &row.call == site);
    let call = if let Some(call) = originals.next() {
        if originals.next().is_some() {
            return Err(freeze("borrowed-call/source-identity"));
        }
        call
    } else {
        let mut calls = ingress.incoming.iter().filter(|row| &row.call == site);
        let call = calls
            .next()
            .ok_or_else(|| freeze("borrowed-call/source-identity"))?;
        if calls.next().is_some() {
            return Err(freeze("borrowed-call/source-identity"));
        }
        call
    };
    if call.source.call_site() != site || call.source.callee_owner() != call.callee {
        return Err(freeze("borrowed-call/source-identity"));
    }
    if !ingress.definitions.contains_key(&call.callee)
        && !ingress.source_only_definitions.contains_key(&call.callee)
    {
        return Ok(None);
    }
    let original_result = results.original_return_source_v1(&call.callee)?;
    if !original_result.is_some_and(|source| source.scalar_class_v1().is_some()) {
        return Ok(None);
    }
    let arguments = if source_instance {
        super::project_pending_instance_source_arguments_v1(source, actuals, contracts, site)?
    } else {
        project_pending_borrowed_i64_arguments_v1(source, actuals, results, site)?
    };
    let Some(arguments) = arguments else {
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
    if proof.class != super::borrowed_formal_result::BorrowedResultClassV1::I64 {
        return Ok(None);
    }
    Ok(Some(arguments))
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_borrowed_i64_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    results: &BorrowedResultCohortV1,
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
