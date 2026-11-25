//! Mode and coverage retained by a single-observation Generic G0 demand.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopFamilyAdmissionCoverageV1 {
    Complete,
    Incomplete,
}
