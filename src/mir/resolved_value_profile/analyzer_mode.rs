//! Exhaustive policy quadrants for the trivial canonical analyzer.

use crate::mir::compiler::normal_source_plan::VerifiedNormalMainRoleV1;

/// Existing analyzer policy combinations, expressed as one internal mode.
/// This value selects policy only; it does not issue a semantic receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrivialCanonicalAnalysisModeV1 {
    OrdinaryClosed,
    OrdinaryFiniteDirectCalls,
    NormalMainClosed { role: VerifiedNormalMainRoleV1 },
    NormalMainFiniteDirectCalls { role: VerifiedNormalMainRoleV1 },
}
