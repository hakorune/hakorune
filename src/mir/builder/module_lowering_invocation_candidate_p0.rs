//! HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-P0: route/failure co-seal.
//!
//! This module consumes the existing nine-row invocation route matrix and the
//! disconnected Candidate0 abort proof.  It does not select routes, publish a
//! module, or provide a retry path.

use super::module_invocation_route_matrix::{InvocationRouteMatrixRowV1, InvocationRouteMatrixV1};
use super::module_lowering_invocation_candidate::{
    InvocationCandidateAbortProofV1, InvocationCandidatePublicationV1, InvocationCandidateRetryV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationCandidateRouteProofErrorV1 {
    DuplicateRoute { route: &'static str },
    BoundaryChanged { route: &'static str },
    PublicationChanged { route: &'static str },
    RetryEnabled { route: &'static str },
    UnexpectedRoute { route: &'static str },
    MissingRoute { route: &'static str },
    RouteMatrixDrift { route: &'static str },
    MatrixFailureLawDrift { route: &'static str },
}

impl std::fmt::Display for InvocationCandidateRouteProofErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][candidate0_p0] {self:?}")
    }
}

impl std::error::Error for InvocationCandidateRouteProofErrorV1 {}

/// One observed route row.  The route matrix remains the authority for
/// identity and publication policy; this row only pairs it with one abort
/// observation.
#[derive(Debug)]
pub(in crate::mir::builder) struct InvocationCandidateRouteProofRowV1 {
    route: InvocationRouteMatrixRowV1,
    abort: InvocationCandidateAbortProofV1,
}

impl InvocationCandidateRouteProofRowV1 {
    pub(in crate::mir::builder) fn route_name(&self) -> &'static str {
        self.route.name()
    }

    pub(in crate::mir::builder) fn route(&self) -> InvocationRouteMatrixRowV1 {
        self.route
    }

    pub(in crate::mir::builder) fn abort(&self) -> &InvocationCandidateAbortProofV1 {
        &self.abort
    }
}

/// Move-only route proof after all matrix rows have been observed exactly
/// once.  It has no Builder, shell, collector, module, or retry authority.
#[derive(Debug)]
pub(in crate::mir::builder) struct InvocationCandidateRouteProofV1 {
    rows: Box<[InvocationCandidateRouteProofRowV1]>,
    _seal: InvocationCandidateRouteProofSealV1,
}

#[derive(Debug)]
struct InvocationCandidateRouteProofSealV1;

/// Construction-only accumulator.  A production caller is intentionally not
/// provided; the future cutover must consume the sealed proof, not this
/// mutable observation helper.
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct InvocationCandidateRouteProofBuilderV1 {
    rows: Vec<InvocationCandidateRouteProofRowV1>,
}

impl InvocationCandidateRouteProofBuilderV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self::default()
    }

    pub(in crate::mir::builder) fn observe(
        &mut self,
        route: InvocationRouteMatrixRowV1,
        abort: InvocationCandidateAbortProofV1,
    ) -> Result<(), InvocationCandidateRouteProofErrorV1> {
        if self.rows.iter().any(|row| row.route_name() == route.name()) {
            return Err(InvocationCandidateRouteProofErrorV1::DuplicateRoute {
                route: route.name(),
            });
        }
        if !abort.boundary_unchanged() {
            return Err(InvocationCandidateRouteProofErrorV1::BoundaryChanged {
                route: route.name(),
            });
        }
        if abort.publication() != InvocationCandidatePublicationV1::Unchanged {
            return Err(InvocationCandidateRouteProofErrorV1::PublicationChanged {
                route: route.name(),
            });
        }
        if abort.retry_disposition() != InvocationCandidateRetryV1::Forbidden {
            return Err(InvocationCandidateRouteProofErrorV1::RetryEnabled {
                route: route.name(),
            });
        }
        self.rows
            .push(InvocationCandidateRouteProofRowV1 { route, abort });
        Ok(())
    }

    pub(in crate::mir::builder) fn seal(
        self,
    ) -> Result<InvocationCandidateRouteProofV1, InvocationCandidateRouteProofErrorV1> {
        let expected = InvocationRouteMatrixV1::rows();
        for actual in &self.rows {
            if !expected.iter().any(|row| row.name() == actual.route_name()) {
                return Err(InvocationCandidateRouteProofErrorV1::UnexpectedRoute {
                    route: actual.route_name(),
                });
            }
        }
        for expected_row in expected {
            let Some(actual) = self
                .rows
                .iter()
                .find(|row| row.route_name() == expected_row.name())
            else {
                return Err(InvocationCandidateRouteProofErrorV1::MissingRoute {
                    route: expected_row.name(),
                });
            };
            if !same_route(*expected_row, actual.route) {
                return Err(InvocationCandidateRouteProofErrorV1::RouteMatrixDrift {
                    route: expected_row.name(),
                });
            }
            let failure = expected_row.failure();
            if !failure.collector_prefix_unchanged() || failure.retry() {
                return Err(
                    InvocationCandidateRouteProofErrorV1::MatrixFailureLawDrift {
                        route: expected_row.name(),
                    },
                );
            }
        }
        Ok(InvocationCandidateRouteProofV1 {
            rows: self.rows.into_boxed_slice(),
            _seal: InvocationCandidateRouteProofSealV1,
        })
    }
}

impl InvocationCandidateRouteProofV1 {
    pub(in crate::mir::builder) fn rows(&self) -> &[InvocationCandidateRouteProofRowV1] {
        &self.rows
    }

    pub(in crate::mir::builder) fn route_count(&self) -> usize {
        self.rows.len()
    }

    pub(in crate::mir::builder) fn has_root_drop_law(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.route().failure().invocation_dropped_without_publish())
    }

    pub(in crate::mir::builder) fn has_child_restore_law(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.route().failure().parent_restored_once())
    }
}

fn same_route(left: InvocationRouteMatrixRowV1, right: InvocationRouteMatrixRowV1) -> bool {
    left.name() == right.name()
        && left.family() == right.family()
        && left.entry() == right.entry()
        && left.identity() == right.identity()
        && left.publication() == right.publication()
        && left.failure().stages() == right.failure().stages()
        && left.failure().collector_prefix_unchanged()
            == right.failure().collector_prefix_unchanged()
        && left.failure().parent_restored_once() == right.failure().parent_restored_once()
        && left.failure().invocation_dropped_without_publish()
            == right.failure().invocation_dropped_without_publish()
        && left.failure().retry() == right.failure().retry()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_lowering_invocation_candidate::{
        InvocationCandidateFailureStageV1, ModuleLoweringInvocationCandidateV1,
    };
    use crate::mir::builder::module_lowering_shell::ModuleLoweringShellV1;
    use crate::mir::MirModule;

    fn shell() -> ModuleLoweringShellV1 {
        ModuleLoweringShellV1::from_empty_module(MirModule::new("candidate-p0".into())).unwrap()
    }

    fn stage_for(row: InvocationRouteMatrixRowV1) -> InvocationCandidateFailureStageV1 {
        match row.entry() {
            super::super::module_invocation_route_matrix::InvocationEntryV1::RawStaticChild
            | super::super::module_invocation_route_matrix::InvocationEntryV1::RawInstanceConstructorChild => {
                InvocationCandidateFailureStageV1::ChildPrimary
            }
            _ => InvocationCandidateFailureStageV1::RootPreflight,
        }
    }

    #[test]
    fn candidate_abort_proof_co_seals_all_nine_route_rows() {
        let mut builder = InvocationCandidateRouteProofBuilderV1::new();
        for route in InvocationRouteMatrixV1::rows() {
            let candidate = ModuleLoweringInvocationCandidateV1::open(
                shell(),
                ModuleDraftCollectorV1::default(),
            );
            let proof = candidate.abort(stage_for(*route)).into_proof();
            builder.observe(*route, proof).unwrap();
        }
        let proof = builder.seal().unwrap();
        assert_eq!(proof.route_count(), 9);
        assert!(proof.has_root_drop_law());
        assert!(proof.has_child_restore_law());
        assert!(proof
            .rows()
            .iter()
            .all(|row| row.abort().boundary_unchanged()));
    }

    #[test]
    fn duplicate_route_is_rejected_before_seal() {
        let route = InvocationRouteMatrixV1::rows()[0];
        let mut builder = InvocationCandidateRouteProofBuilderV1::new();
        for _ in 0..2 {
            let candidate = ModuleLoweringInvocationCandidateV1::open(
                shell(),
                ModuleDraftCollectorV1::default(),
            );
            let result = builder.observe(
                route,
                candidate
                    .abort(InvocationCandidateFailureStageV1::RootPreflight)
                    .into_proof(),
            );
            if result.is_err() {
                assert_eq!(
                    result.unwrap_err(),
                    InvocationCandidateRouteProofErrorV1::DuplicateRoute {
                        route: route.name()
                    }
                );
                return;
            }
        }
        panic!("duplicate route must be rejected");
    }
}
