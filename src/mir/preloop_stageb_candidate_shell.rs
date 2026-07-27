//! Read-only readiness vocabulary for one prepared pre-loop Stage-B shell.
//!
//! This module owns no Builder opener, source policy, catalog install, alias
//! install, lowering, retry, or fallback authority. `MirBuilder` issues the
//! receipt only after observing its already-open candidate shell.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopStageBCandidateShellReadinessErrorV1 {
    CandidateModuleMissing,
    PhysicalMainMissing,
    PhysicalMainNameMismatch,
    PhysicalMainArityMismatch,
    CurrentBlockMissing,
    CurrentBlockIsNotPhysicalMainEntry,
    CallableCatalogLaneOccupied,
    ImportAliasLaneConflict,
}

#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBCandidateShellReadinessV1 {
    _seal: VerifiedPreloopStageBCandidateShellReadinessSealV1,
}

#[derive(Debug)]
struct VerifiedPreloopStageBCandidateShellReadinessSealV1(());

impl VerifiedPreloopStageBCandidateShellReadinessV1 {
    pub(in crate::mir) const fn new() -> Self {
        Self {
            _seal: VerifiedPreloopStageBCandidateShellReadinessSealV1(()),
        }
    }
}
