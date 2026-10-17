//! Local acquisition observes the original forwarded producer's source only.
//! Original incoming target and Facts are authority; input phase stays separate.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::candidate::OrdinaryNewCandidate;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{
    OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallArgumentsV1, LocalCallResultClassV1, ObjectCallSourceSupportV1,
};

pub(in crate::mir::normal_callable_semantic_package) fn received_producer_arguments_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    actuals: &PendingBorrowedFormalActualsV1,
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    candidates: &[OrdinaryNewCandidate],
    classes: &OrdinaryNewResultClassClaimsV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
    requested: LocalCallResultClassV1,
    fields: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let mut originals = source
        .as_ref()
        .ok()
        .into_iter()
        .flat_map(|ingress| ingress.source_incoming.exact_rows())
        .filter(|row| &row.call == site);
    let original = originals.next();
    if originals.next().is_some() {
        return Err(freeze("original-not-unique"));
    }
    let prepared: Vec<_> = targets
        .as_ref()
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|row| row.as_ref().ok()?.as_ref())
        .filter(|target| target.call_site() == site)
        .collect();
    let original_target = original.and_then(|row| row.source.instance());
    let anchor = original_target.or_else(|| prepared.first().copied());
    let Some(anchor) = anchor else {
        // An error lacks a site: absence is not evidence of non-selection.
        source.as_ref().map_err(Clone::clone)?;
        for row in targets.as_ref().map_err(Clone::clone)? {
            row.as_ref().map_err(Clone::clone)?;
        }
        return Ok(None);
    };
    if anchor.owning_object_local_input_v1().is_some() {
        require_prepared_targets(targets)?;
        let ingress = source.as_ref().map_err(Clone::clone)?;
        let original = original.ok_or_else(|| freeze("original-missing"))?;
        let target = original.source.require_instance()?;
        let loan = ingress.original_owning_object_input_source_v1(target)?
            .ok_or_else(|| freeze("nullable-input-missing"))?;
        classes.recheck_owning_object_local_input_roots_v1(loan).map_err(freeze)?;
        if prepared.len() != 1 || prepared[0] != target || original.callee != target.callee_owner()
            || loan.destination() != destination || !loan.matches_target(target) {
            return Err(freeze("nullable-input-identity"));
        }
        let result = if loan.nullable() { LocalCallResultClassV1::Nullable } else { LocalCallResultClassV1::Handle };
        if requested != result { return Ok(None); }
        let arguments = super::object_arguments::project_pending_object_target_arguments_v1(
            source, contracts, actuals, target, None)?;
        if matches!(arguments, ObjectCallSourceSupportV1::Unavailable) {
            return Err(freeze("nullable-input-arguments-missing"));
        }
        return Ok(Some(BorrowedCallArgumentsV1::SourceObject {
            result,
            arguments,
        }));
    }
    if let Ok(ingress) = source {
        if let Some((loan, binding)) = ingress.recheck_field_producer_source_v1(anchor, classes, fields)? {
            require_prepared_targets(targets)?;
            let original = original.ok_or_else(|| freeze("field-producer/original-missing"))?;
            let target = original.source.require_instance()?;
            if prepared.len() != 1 || prepared[0] != target || loan.source() != target
                || original.callee != target.callee_owner() || binding != destination
                || !matches!(loan.result_class(), OrdinaryNewResultClassV1::Object(_)) {
                return Err(freeze("field-producer/source-identity"));
            }
            if requested != LocalCallResultClassV1::Handle { return Ok(None); }
            let arguments = super::object_arguments::project_pending_object_target_arguments_v1(
                source, contracts, actuals, target, None)?;
            if matches!(arguments, ObjectCallSourceSupportV1::Unavailable) {
                return Err(freeze("field-producer/arguments-missing"));
            }
            return Ok(Some(BorrowedCallArgumentsV1::SourceObject { result: requested, arguments }));
        }
    }
    if let Ok(ingress) = source {
        if ingress.recheck_field_producer_callee_v1(anchor, classes, fields)? {
            require_prepared_targets(targets)?;
            let original = original.ok_or_else(|| freeze("field-local/original-missing"))?;
            let target = original.source.require_instance()?;
            if prepared.len() != 1 || prepared[0] != target || original.callee != target.callee_owner() {
                return Err(freeze("field-local/source-identity"));
            }
            classes.check_owning_local_object_source_v1(batch, selected, target, destination).map_err(freeze)?;
            if requested != LocalCallResultClassV1::Handle { return Ok(None); }
            let arguments = super::object_arguments::project_pending_object_target_arguments_v1(
                source, contracts, actuals, target, None)?;
            if matches!(arguments, ObjectCallSourceSupportV1::Unavailable) {
                return Err(freeze("field-local/arguments-missing"));
            }
            return Ok(Some(BorrowedCallArgumentsV1::SourceObject { result: requested, arguments }));
        }
    }
    // Existing qualified/leaf/scalar source rows remain on their own protocol.
    if anchor.object_producer_dependencies().is_none() {
        if anchor.object_return_sources().is_none()
            && classes
                .object_return_dependencies(anchor.target(), anchor.callee_owner())
                .is_some()
        {
            return Err(freeze("producer-dependency-missing"));
        }
        return Ok(None);
    }
    source.as_ref().map_err(Clone::clone)?;
    let original = original.ok_or_else(|| freeze("original-missing"))?;
    let target = original.source.require_instance()?;
    require_prepared_targets(targets)?;
    if prepared.len() != 1 || prepared[0] != target || original.callee != target.callee_owner() {
        return Err(freeze("prepared-target-identity"));
    }
    let dependencies = classes
        .object_return_dependencies(target.target(), target.callee_owner())
        .ok_or_else(|| freeze("producer-dependency-missing"))?;
    if dependencies.is_empty()
        || target.object_producer_dependencies() != Some(dependencies.as_ref())
    {
        return Err(freeze("producer-dependency-identity"));
    }
    let result = match classes.get(target.target()) {
        Some(OrdinaryNewResultClassV1::Object(_)) => LocalCallResultClassV1::Handle,
        Some(OrdinaryNewResultClassV1::NullableObject(_)) => LocalCallResultClassV1::Nullable,
        _ => return Err(freeze("producer-result-missing")),
    };
    let receiver = target.receiver_binding()?;
    let mut matching = candidates.iter().filter(|row| row.destination == receiver);
    let candidate = matching
        .next()
        .ok_or_else(|| freeze("claim-local-missing"))?;
    if matching.next().is_some()
        || target.is_self_receiver()
        || destination.owner() != site.owner()
        || receiver.owner() != site.owner()
        || candidate.site.owner() != site.owner()
        || candidate.class.as_ref() != target.target().owner()
        || target.call_site() != site
        || target.argument_sites().len() != target.target().arity() as usize
    {
        return Err(freeze("claim-local-identity"));
    }
    if result != requested {
        return Ok(None);
    }
    let arguments = super::object_arguments::project_pending_object_target_arguments_v1(
        source, contracts, actuals, target,
        None,
    )?;
    if matches!(arguments, ObjectCallSourceSupportV1::Unavailable) {
        return Err(freeze("arguments-unavailable"));
    }
    Ok(Some(BorrowedCallArgumentsV1::SourceObject {
        result,
        arguments,
    }))
}
fn require_prepared_targets(targets: &PreparedLexicalInstanceCallSourceTargetsV1) -> Result<(), String> {
    for row in targets.as_ref().map_err(Clone::clone)? { row.as_ref().map_err(Clone::clone)?; }
    Ok(())
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][received-producer/{reason}]")
}
