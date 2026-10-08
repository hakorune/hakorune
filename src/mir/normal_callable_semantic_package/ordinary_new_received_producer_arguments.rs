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
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    actuals: &PendingBorrowedFormalActualsV1,
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    candidates: &[OrdinaryNewCandidate],
    classes: &OrdinaryNewResultClassClaimsV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
    requested: LocalCallResultClassV1,
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
    for row in targets.as_ref().map_err(Clone::clone)? {
        row.as_ref().map_err(Clone::clone)?;
    }
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
        source, actuals, target,
    )?;
    if matches!(arguments, ObjectCallSourceSupportV1::Unavailable) {
        return Err(freeze("arguments-unavailable"));
    }
    Ok(Some(BorrowedCallArgumentsV1::SourceObject {
        result,
        arguments,
    }))
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][received-producer/{reason}]")
}
