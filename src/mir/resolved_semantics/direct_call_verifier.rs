//! Verification local to function-relative direct-call target rows.

use super::direct_call::ResolvedDirectCallVerificationErrorV1;
use super::product::ResolvedFunctionDataV1;

pub(super) fn verify_direct_call_targets(
    data: &ResolvedFunctionDataV1,
) -> Result<(), ResolvedDirectCallVerificationErrorV1> {
    for target in data.direct_call_targets.values().copied() {
        let target_owner = target.callable().owner();
        if target_owner.compilation_brand() != data.owner.compilation_brand() {
            return Err(ResolvedDirectCallVerificationErrorV1::ForeignCompilation {
                function: data.owner,
                target: target_owner,
            });
        }
    }
    Ok(())
}
