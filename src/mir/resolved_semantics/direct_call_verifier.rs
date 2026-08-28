//! Verification local to function-relative direct-call target rows.

use super::direct_call::ResolvedDirectCallVerificationErrorV1;
use super::product::ResolvedFunctionDataV1;

pub(super) fn verify_direct_call_targets(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedDirectCallVerificationErrorV1> {
    let target_sites = data
        .direct_call_targets
        .keys()
        .collect::<std::collections::BTreeSet<_>>();
    let observation_sites = data
        .direct_call_observations
        .keys()
        .collect::<std::collections::BTreeSet<_>>();
    if !target_sites.is_empty()
        && !observation_sites.is_empty()
        && target_sites != observation_sites
    {
        return Err(ResolvedDirectCallVerificationErrorV1::SiteCoverageMismatch);
    }
    for target in data.direct_call_targets.values().copied() {
        let target_owner = target.callable().owner();
        if target_owner.compilation_brand() != data.owner.compilation_brand() {
            return Err(ResolvedDirectCallVerificationErrorV1::ForeignCompilation {
                function: data.owner,
                target: target_owner,
            });
        }
    }
    for (site, observation) in &data.direct_call_observations {
        if observation.argument_sites().len() != observation.arity() as usize {
            return Err(
                ResolvedDirectCallVerificationErrorV1::ArgumentSiteCardinalityMismatch {
                    site: site.clone(),
                },
            );
        }
        let mut argument_sites = std::collections::BTreeSet::new();
        for argument in observation.argument_sites() {
            if !argument_sites.insert(argument) {
                return Err(
                    ResolvedDirectCallVerificationErrorV1::DuplicateArgumentSite {
                        site: argument.clone(),
                    },
                );
            }
        }
    }
    Ok(())
}
