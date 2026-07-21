//! HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0: failure ownership matrix.
//!
//! This is a passive, disconnected matrix for the future module-finalization
//! transaction.  It records which unpublished boundary survives each failure
//! stage; it does not execute repair, drain, publication, or retry.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleFinalizationFailureStageV1 {
    ChildPrimary,
    ChildCleanup,
    Admission,
    RootCompletion,
    DrainPreflight,
    PostDrainVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleFinalizationCandidateDispositionV1 {
    CollectorPrefixPreserved,
    InvocationCandidateDiscarded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct ModuleFinalizationFailureRowV1 {
    stage: ModuleFinalizationFailureStageV1,
    candidate: ModuleFinalizationCandidateDispositionV1,
    external_publication_unchanged: bool,
    parent_restored_once: bool,
    retry_forbidden: bool,
}

const FAILURE_ROWS: [ModuleFinalizationFailureRowV1; 6] = [
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::ChildPrimary,
        candidate: ModuleFinalizationCandidateDispositionV1::CollectorPrefixPreserved,
        external_publication_unchanged: true,
        parent_restored_once: true,
        retry_forbidden: true,
    },
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::ChildCleanup,
        candidate: ModuleFinalizationCandidateDispositionV1::CollectorPrefixPreserved,
        external_publication_unchanged: true,
        parent_restored_once: true,
        retry_forbidden: true,
    },
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::Admission,
        candidate: ModuleFinalizationCandidateDispositionV1::CollectorPrefixPreserved,
        external_publication_unchanged: true,
        parent_restored_once: true,
        retry_forbidden: true,
    },
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::RootCompletion,
        candidate: ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded,
        external_publication_unchanged: true,
        parent_restored_once: false,
        retry_forbidden: true,
    },
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::DrainPreflight,
        candidate: ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded,
        external_publication_unchanged: true,
        parent_restored_once: false,
        retry_forbidden: true,
    },
    ModuleFinalizationFailureRowV1 {
        stage: ModuleFinalizationFailureStageV1::PostDrainVerification,
        candidate: ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded,
        external_publication_unchanged: true,
        parent_restored_once: false,
        retry_forbidden: true,
    },
];

impl ModuleFinalizationFailureRowV1 {
    pub(in crate::mir::builder) fn stage(&self) -> ModuleFinalizationFailureStageV1 {
        self.stage
    }

    pub(in crate::mir::builder) fn candidate(&self) -> ModuleFinalizationCandidateDispositionV1 {
        self.candidate
    }

    pub(in crate::mir::builder) fn external_publication_unchanged(&self) -> bool {
        self.external_publication_unchanged
    }

    pub(in crate::mir::builder) fn parent_restored_once(&self) -> bool {
        self.parent_restored_once
    }

    pub(in crate::mir::builder) fn retry_forbidden(&self) -> bool {
        self.retry_forbidden
    }
}

pub(in crate::mir::builder) struct ModuleFinalizationFailureMatrixV1;

impl ModuleFinalizationFailureMatrixV1 {
    pub(in crate::mir::builder) fn rows() -> &'static [ModuleFinalizationFailureRowV1] {
        &FAILURE_ROWS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_failures_preserve_prefix_and_restore_parent() {
        let rows = ModuleFinalizationFailureMatrixV1::rows();
        for row in rows.iter().take(3) {
            assert_eq!(
                row.candidate(),
                ModuleFinalizationCandidateDispositionV1::CollectorPrefixPreserved
            );
            assert!(row.parent_restored_once());
            assert!(row.external_publication_unchanged());
            assert!(row.retry_forbidden());
        }
    }

    #[test]
    fn root_and_drain_failures_discard_unpublished_invocation() {
        let rows = ModuleFinalizationFailureMatrixV1::rows();
        for row in rows.iter().skip(3) {
            assert_eq!(
                row.candidate(),
                ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded
            );
            assert!(!row.parent_restored_once());
            assert!(row.external_publication_unchanged());
            assert!(row.retry_forbidden());
        }
    }

    #[test]
    fn post_drain_verification_has_no_fallback_route() {
        let row = ModuleFinalizationFailureMatrixV1::rows()
            .iter()
            .find(|row| row.stage() == ModuleFinalizationFailureStageV1::PostDrainVerification)
            .unwrap();
        assert_eq!(
            row.candidate(),
            ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded
        );
        assert!(row.external_publication_unchanged());
        assert!(row.retry_forbidden());
    }

    #[test]
    fn matrix_has_one_row_per_failure_owner() {
        assert_eq!(ModuleFinalizationFailureMatrixV1::rows().len(), 6);
    }
}
