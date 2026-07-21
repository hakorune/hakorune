//! HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-P0: disconnected route matrix.
//!
//! This is a passive proof product.  It records the route families and their
//! failure laws before any production capture/commit call is rewired.  It
//! owns no Builder, module, draft, fact, or retry authority.

use super::module_draft_collector::DraftPublicationPolicyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationRootFamilyV1 {
    Raw,
    CanonicalAPlus,
    BindingSsaTrivial,
    BindingSsaAcyclic,
    BindingSsaRecursive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationEntryV1 {
    MainRoot,
    RawStaticChild,
    RawInstanceConstructorChild,
    SyntheticConditionFn,
    CanonicalRoot,
    CanonicalChild,
    CallableModuleBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationIdentityV1 {
    Main,
    LegacySymbol,
    SyntheticConditionFn,
    CanonicalResolvedOwner,
    CanonicalCallable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationFailureStageV1 {
    Primary,
    Cleanup,
    Admission,
    Panic,
    FinalPreflight,
    Drain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationFailureLawV1 {
    stages: &'static [InvocationFailureStageV1],
    collector_prefix_unchanged: bool,
    parent_restored_once: bool,
    invocation_dropped_without_publish: bool,
    retry: bool,
}

impl InvocationFailureLawV1 {
    pub(in crate::mir::builder) const fn stages(self) -> &'static [InvocationFailureStageV1] {
        self.stages
    }

    pub(in crate::mir::builder) const fn collector_prefix_unchanged(self) -> bool {
        self.collector_prefix_unchanged
    }

    pub(in crate::mir::builder) const fn parent_restored_once(self) -> bool {
        self.parent_restored_once
    }

    pub(in crate::mir::builder) const fn invocation_dropped_without_publish(self) -> bool {
        self.invocation_dropped_without_publish
    }

    pub(in crate::mir::builder) const fn retry(self) -> bool {
        self.retry
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationRouteMatrixRowV1 {
    name: &'static str,
    family: InvocationRootFamilyV1,
    entry: InvocationEntryV1,
    identity: InvocationIdentityV1,
    publication: DraftPublicationPolicyV1,
    failure: InvocationFailureLawV1,
}

impl InvocationRouteMatrixRowV1 {
    pub(in crate::mir::builder) const fn name(self) -> &'static str {
        self.name
    }

    pub(in crate::mir::builder) const fn family(self) -> InvocationRootFamilyV1 {
        self.family
    }

    pub(in crate::mir::builder) const fn entry(self) -> InvocationEntryV1 {
        self.entry
    }

    pub(in crate::mir::builder) const fn identity(self) -> InvocationIdentityV1 {
        self.identity
    }

    pub(in crate::mir::builder) const fn publication(self) -> DraftPublicationPolicyV1 {
        self.publication
    }

    pub(in crate::mir::builder) const fn failure(self) -> InvocationFailureLawV1 {
        self.failure
    }
}

const RAW_CHILD_FAILURE: InvocationFailureLawV1 = InvocationFailureLawV1 {
    stages: &[
        InvocationFailureStageV1::Primary,
        InvocationFailureStageV1::Cleanup,
        InvocationFailureStageV1::Admission,
        InvocationFailureStageV1::Panic,
    ],
    collector_prefix_unchanged: true,
    parent_restored_once: true,
    invocation_dropped_without_publish: false,
    retry: false,
};

const ROOT_FAILURE: InvocationFailureLawV1 = InvocationFailureLawV1 {
    stages: &[
        InvocationFailureStageV1::Primary,
        InvocationFailureStageV1::Cleanup,
        InvocationFailureStageV1::Admission,
        InvocationFailureStageV1::Panic,
        InvocationFailureStageV1::FinalPreflight,
        InvocationFailureStageV1::Drain,
    ],
    collector_prefix_unchanged: true,
    parent_restored_once: false,
    invocation_dropped_without_publish: true,
    retry: false,
};

const CANONICAL_FAILURE: InvocationFailureLawV1 = InvocationFailureLawV1 {
    stages: &[
        InvocationFailureStageV1::Primary,
        InvocationFailureStageV1::Admission,
        InvocationFailureStageV1::FinalPreflight,
        InvocationFailureStageV1::Drain,
    ],
    collector_prefix_unchanged: true,
    parent_restored_once: false,
    invocation_dropped_without_publish: true,
    retry: false,
};

/// The complete P0 route matrix.  Every row converges on one final collector
/// drain; the matrix itself cannot perform that drain.
pub(in crate::mir::builder) struct InvocationRouteMatrixV1;

impl InvocationRouteMatrixV1 {
    pub(in crate::mir::builder) const fn rows() -> &'static [InvocationRouteMatrixRowV1] {
        &[
            InvocationRouteMatrixRowV1 {
                name: "raw_main_root",
                family: InvocationRootFamilyV1::Raw,
                entry: InvocationEntryV1::MainRoot,
                identity: InvocationIdentityV1::Main,
                publication: DraftPublicationPolicyV1::LegacyReplaceWholePair,
                failure: ROOT_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "raw_static_child",
                family: InvocationRootFamilyV1::Raw,
                entry: InvocationEntryV1::RawStaticChild,
                identity: InvocationIdentityV1::LegacySymbol,
                publication: DraftPublicationPolicyV1::LegacyReplaceWholePair,
                failure: RAW_CHILD_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "raw_instance_constructor_child",
                family: InvocationRootFamilyV1::Raw,
                entry: InvocationEntryV1::RawInstanceConstructorChild,
                identity: InvocationIdentityV1::LegacySymbol,
                publication: DraftPublicationPolicyV1::LegacyReplaceWholePair,
                failure: RAW_CHILD_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "synthetic_condition_fn",
                family: InvocationRootFamilyV1::Raw,
                entry: InvocationEntryV1::SyntheticConditionFn,
                identity: InvocationIdentityV1::SyntheticConditionFn,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: ROOT_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "canonical_a_plus_root",
                family: InvocationRootFamilyV1::CanonicalAPlus,
                entry: InvocationEntryV1::CanonicalRoot,
                identity: InvocationIdentityV1::CanonicalResolvedOwner,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: CANONICAL_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "canonical_a_plus_child",
                family: InvocationRootFamilyV1::CanonicalAPlus,
                entry: InvocationEntryV1::CanonicalChild,
                identity: InvocationIdentityV1::CanonicalResolvedOwner,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: CANONICAL_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "binding_ssa_trivial_root",
                family: InvocationRootFamilyV1::BindingSsaTrivial,
                entry: InvocationEntryV1::CanonicalRoot,
                identity: InvocationIdentityV1::CanonicalResolvedOwner,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: CANONICAL_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "binding_ssa_acyclic_module",
                family: InvocationRootFamilyV1::BindingSsaAcyclic,
                entry: InvocationEntryV1::CallableModuleBatch,
                identity: InvocationIdentityV1::CanonicalCallable,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: CANONICAL_FAILURE,
            },
            InvocationRouteMatrixRowV1 {
                name: "binding_ssa_recursive_module",
                family: InvocationRootFamilyV1::BindingSsaRecursive,
                entry: InvocationEntryV1::CallableModuleBatch,
                identity: InvocationIdentityV1::CanonicalCallable,
                publication: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                failure: CANONICAL_FAILURE,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn p0_route_matrix_covers_every_root_and_child_family() {
        let rows = InvocationRouteMatrixV1::rows();
        assert_eq!(rows.len(), 9);
        let names = rows.iter().map(|row| row.name()).collect::<BTreeSet<_>>();
        assert_eq!(names.len(), rows.len());
        for family in [
            InvocationRootFamilyV1::Raw,
            InvocationRootFamilyV1::CanonicalAPlus,
            InvocationRootFamilyV1::BindingSsaTrivial,
            InvocationRootFamilyV1::BindingSsaAcyclic,
            InvocationRootFamilyV1::BindingSsaRecursive,
        ] {
            assert!(rows.iter().any(|row| row.family() == family));
        }
        assert!(rows
            .iter()
            .any(|row| row.entry() == InvocationEntryV1::MainRoot));
        assert!(rows
            .iter()
            .any(|row| row.entry() == InvocationEntryV1::SyntheticConditionFn));
        assert!(rows
            .iter()
            .any(|row| row.identity() == InvocationIdentityV1::CanonicalCallable));
    }

    #[test]
    fn p0_failure_matrix_forbids_retry_and_partial_publication() {
        for row in InvocationRouteMatrixV1::rows() {
            let failure = row.failure();
            assert!(failure.collector_prefix_unchanged());
            assert!(!failure.retry());
            if row.family() == InvocationRootFamilyV1::Raw
                && row.entry() != InvocationEntryV1::RawStaticChild
                && row.entry() != InvocationEntryV1::RawInstanceConstructorChild
            {
                assert!(failure.invocation_dropped_without_publish());
            }
        }
    }
}
