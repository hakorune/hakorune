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
    // Indexed rows are co-sealed in both directions.  ObserveOnly rows have
    // no target inventory by design, so an empty target set with observations
    // is valid; target-only and partial rows remain invalid.
    if !target_sites.is_empty() && target_sites != observation_sites {
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

#[cfg(test)]
mod tests {
    use super::super::direct_call::{ResolvedDirectCallObservationV1, ResolvedDirectCallTargetV1};
    use super::super::ids::FunctionOwnerIdV1;
    use super::super::source_site::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
    use super::super::{ResolvedCallableRefV1, ResolvedDirectCallVerificationErrorV1};
    use super::verify_direct_call_targets;
    use hakorune_mir_core::BindingId;

    fn site(index: u32) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
            SourcePathSegmentV1::Value,
        ]))
    }

    fn owner() -> FunctionOwnerIdV1 {
        super::super::FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap()
    }

    #[test]
    fn direct_call_verifier_rejects_target_only_inventory() {
        let owner = owner();
        let mut data = super::super::tests::sample_data(owner, BindingId::new(0));
        data.direct_call_targets.insert(
            site(4),
            ResolvedDirectCallTargetV1::from_resolved(ResolvedCallableRefV1::for_test(owner)),
        );

        assert_eq!(
            verify_direct_call_targets(&data),
            Err(ResolvedDirectCallVerificationErrorV1::SiteCoverageMismatch)
        );
    }

    #[test]
    fn direct_call_verifier_rejects_partial_site_inventory() {
        let owner = owner();
        let mut data = super::super::tests::sample_data(owner, BindingId::new(0));
        data.direct_call_targets.insert(
            site(4),
            ResolvedDirectCallTargetV1::from_resolved(ResolvedCallableRefV1::for_test(owner)),
        );
        data.direct_call_observations.insert(
            site(5),
            ResolvedDirectCallObservationV1::from_parts(
                "helper".into(),
                0,
                Vec::new().into_boxed_slice(),
            ),
        );

        assert_eq!(
            verify_direct_call_targets(&data),
            Err(ResolvedDirectCallVerificationErrorV1::SiteCoverageMismatch)
        );
    }
}
