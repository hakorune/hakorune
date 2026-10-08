//! One qualified Received Handle source demand, using the original target/loans.
//! No target resolution, phase promotion or executable permission is issued here.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallSourceTargetV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{
    ObjectReturnCallQualificationV1, OrdinaryNewResultClassV1,
};
use crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1;

pub(super) fn received_handle_arguments_v1(
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    candidates: &[OrdinaryNewCandidate],
    classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
    project: &mut impl FnMut(
        &LexicalInstanceCallSourceTargetV1,
        &ObjectReturnCallQualificationV1,
    ) -> Result<ObjectCallSourceSupportV1, String>,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let loans = classes.qualifications_at_call(site);
    // An accessible contradictory Object target cannot hide behind the old lane.
    // Unselected sites do not demand unrelated global/per-target errors.
    if let Ok(rows) = targets {
        for target in rows.iter().filter_map(|row| row.as_ref().ok()?.as_ref()) {
            if target.call_site() == site
                && target.object_return_sources().is_some_and(|required| {
                    required
                        .iter()
                        .any(|loan| matches!(loan.class(), OrdinaryNewResultClassV1::Object(_)))
                        && required != loans.as_ref()
                })
            {
                return Err(freeze("received-handle/target-qualification"));
            }
        }
    }
    if !loans
        .iter()
        .any(|loan| matches!(loan.class(), OrdinaryNewResultClassV1::Object(_)))
    {
        return Ok(None);
    }
    let first = &loans[0];
    if destination.owner() != site.owner()
        || loans.iter().any(|loan| {
            loan.call() != site
                || loan.value().owner() != site.owner()
                || loan.key() != first.key()
                || loan.class() != first.class()
                || !matches!(loan.class(), OrdinaryNewResultClassV1::Object(_))
        })
    {
        return Err(freeze("received-handle/qualification-identity"));
    }
    let mut exact = Vec::new();
    for row in targets.as_ref().map_err(Clone::clone)? {
        if let Some(target) = row.as_ref().map_err(Clone::clone)? {
            if target.call_site() == site {
                exact.push(target);
            }
        }
    }
    let target = match exact.as_slice() {
        [target] => *target,
        [] => return Err(freeze("received-handle/target-missing")),
        _ => return Err(freeze("received-handle/target-not-unique")),
    };
    let receiver = target.receiver_binding()?;
    let mut matching = candidates.iter().filter(|row| row.destination == receiver);
    let candidate = matching
        .next()
        .ok_or_else(|| freeze("received-handle/claim-local-missing"))?;
    if matching.next().is_some()
        || target.is_self_receiver()
        || receiver.owner() != site.owner()
        || candidate.site.owner() != site.owner()
        || candidate.class.as_ref() != first.key().owner()
        || target.target() != first.key()
        || target.object_return_sources() != Some(loans.as_ref())
    {
        return Err(freeze("received-handle/claim-local-identity"));
    }
    let support = corroborate_all_arguments_v1(&loans, &mut |loan| project(target, loan))?;
    let arguments = match support {
        ObjectCallSourceSupportV1::Observed(arguments)
        | ObjectCallSourceSupportV1::SourceOnly(arguments) => arguments,
        ObjectCallSourceSupportV1::Unavailable => unreachable!("unavailable refused before return"),
    };
    Ok(Some(BorrowedCallArgumentsV1::HandleSource(arguments)))
}

/// Preserve each original phase and ordered row until every return corroborates.
fn corroborate_all_arguments_v1(
    loans: &[ObjectReturnCallQualificationV1],
    project: &mut impl FnMut(
        &ObjectReturnCallQualificationV1,
    ) -> Result<ObjectCallSourceSupportV1, String>,
) -> Result<ObjectCallSourceSupportV1, String> {
    let mut agreed = None;
    for loan in loans {
        let support = project(loan)?;
        if matches!(support, ObjectCallSourceSupportV1::Unavailable) {
            return Err(freeze("received-handle/arguments-unavailable"));
        }
        if agreed.as_ref().is_some_and(|original| original != &support) {
            return Err(freeze("received-handle/arguments-disagree"));
        }
        agreed = Some(support);
    }
    agreed.ok_or_else(|| freeze("received-handle/qualification-missing"))
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_received_handle_argument_tests.rs"]
mod tests;
