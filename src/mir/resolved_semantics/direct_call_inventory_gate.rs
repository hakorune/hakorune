//! Shared forest-wide gate for unissued direct-call observations.
//!
//! A callable forest may contain ordinary FunctionCall observations in a
//! nested lambda even when its root owner has none.  Keep that completeness
//! rule in one small helper so package and source admission cannot drift.

use super::VerifiedSemanticOwnerForestV1;

pub(crate) fn forest_has_unissued_direct_call_observation_v1(
    forest: &VerifiedSemanticOwnerForestV1,
) -> bool {
    forest
        .owners()
        .any(|(_, function)| function.direct_call_observations().next().is_some())
}
