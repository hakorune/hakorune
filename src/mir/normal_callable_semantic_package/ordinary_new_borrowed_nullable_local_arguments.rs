//! Exact retained LOCAL borrowed source loan and original Instance argument rows.
//! No result classification, owning acquisition or execution permission here.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1;

pub(in crate::mir::normal_callable_semantic_package) fn arguments_v1(
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    facts: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let mut prepared = Vec::new();
    for row in targets.as_ref().map_err(Clone::clone)? {
        if let Some(target) = row.as_ref().map_err(Clone::clone)? {
            if target.call_site() == site {
                prepared.push(target);
            }
        }
    }
    let prepared_demand = prepared
        .iter()
        .any(|row| row.borrowed_nullable_local_source().is_some());
    let ingress = match source {
        Ok(ingress) => ingress,
        Err(_) if !prepared_demand => return Ok(None),
        Err(issue) => return Err(issue.clone()),
    };
    let mut originals = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| &row.call == site);
    let original = originals.next();
    if originals.next().is_some() {
        return Err(freeze("original-not-unique"));
    }
    let original_target = original.and_then(|row| row.source.instance());
    let demanded = prepared_demand
        || original_target.is_some_and(|row| row.borrowed_nullable_local_source().is_some());
    if !demanded {
        return Ok(None);
    }
    let [target] = prepared.as_slice() else {
        return Err(freeze("prepared-target-missing-or-duplicate"));
    };
    let original = original.ok_or_else(|| freeze("original-missing"))?;
    let original_target = original.source.require_instance()?;
    let loan = original_target
        .borrowed_nullable_local_source()
        .ok_or_else(|| freeze("original-loan-missing"))?;
    if *target != original_target
        || original.callee != loan.callee_owner()
        || loan.call() != site
        || loan.destination() != destination
        || loan.key() != target.target()
        || loan.receiver() != target.receiver_site()
        || loan.support_root().owner() != site.owner()
        || target.has_object_source_requirement()
        || (!loan.is_map_slot() && target.stored_receiver().is_none())
    {
        return Err(freeze("source-identity"));
    }
    let roots = facts
        .checked_original_source_roots_v1(loan.key(), loan.callee_owner())
        .map_err(freeze)?;
    facts
        .check_borrowed_nullable_roots_v1(loan, &roots)
        .map_err(freeze)?;
    let arguments = match project_pending_object_target_arguments_v1(
        source, contracts, pending, target, None,
    )? {
        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::SourceOnly(arguments)
        | crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::Observed(arguments) => arguments,
        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::Unavailable =>
            return Err(freeze("source-arguments-missing")),
    };
    Ok(Some(BorrowedCallArgumentsV1::BorrowedNullableLocalSource {
        loan: loan.clone(),
        arguments,
    }))
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][borrowed-nullable-local/{reason}]")
}

pub(in crate::mir::normal_callable_semantic_package) fn mixed_arguments_v1(
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    facts: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let mut prepared = Vec::new();
    for row in targets.as_ref().map_err(Clone::clone)? {
        if let Some(target) = row.as_ref().map_err(Clone::clone)? {
            if target.call_site() == site { prepared.push(target); }
        }
    }
    if !prepared.iter().any(|target| target.mixed_local_source().is_some()) {
        return Ok(None);
    }
    let [target] = prepared.as_slice() else { return Err(freeze("mixed/prepared-not-unique")); };
    let loan = target.mixed_local_source().ok_or_else(|| freeze("mixed/loan-missing"))?;
    let ingress = source.as_ref().map_err(Clone::clone)?;
    let mut originals = ingress.source_incoming.exact_rows().filter(|row| &row.call == site);
    let original = originals.next().ok_or_else(|| freeze("mixed/original-missing"))?;
    if originals.next().is_some() || original.source.require_instance()? != *target
        || original.callee != loan.callee_owner() || loan.call() != site
        || loan.destination() != destination || loan.key() != target.target()
        || loan.receiver() != target.receiver_site() || loan.support().owner() != site.owner()
        || target.has_object_source_requirement()
    { return Err(freeze("mixed/source-identity")); }
    facts.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
    let arguments = match project_pending_object_target_arguments_v1(
        source, contracts, pending, target, None,
    )? {
        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::SourceOnly(arguments)
        | crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::Observed(arguments) => arguments,
        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::Unavailable =>
            return Err(freeze("mixed/arguments-missing")),
    };
    Ok(Some(BorrowedCallArgumentsV1::MixedLocalSource { loan: loan.clone(), arguments }))
}
