//! CUT0-P0: one disconnected outer adapter for every invocation route.
//!
//! The route row is metadata only.  Every row enters the same move-only
//! candidate -> drain -> finalizer adapter, while injected faults stop before
//! external publication.  This module is test-only until the atomic CUT0-I0.

use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

use super::drained_module_candidate::CompletedInvocationInventoryV1;
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_finalization_once::{finalize_drained_module_once, FinalizedModuleCandidateV1};
use super::module_finalization_split::DrainedModuleFinalizationInputV1;
use super::module_invocation_drain::{
    ConditionFnPolicyV1, InvocationDrainExpectationV1, ModuleLoweringInvocationDrainOwnerV1,
};
use super::module_invocation_route_matrix::{InvocationRouteMatrixRowV1, InvocationRouteMatrixV1};
use super::module_lowering_invocation_candidate::{
    InvocationCandidateFailureStageV1, ModuleLoweringInvocationCandidateV1,
};
use super::module_lowering_shell::ModuleLoweringShellV1;
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use super::route_owned_invocation_inventory::{
    InvocationInventoryAuthorityV2, RouteOwnedInvocationInventoryV2,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cut0P0ScenarioV1 {
    Success,
    Primary,
    Cleanup,
    Admission,
    Root,
    Drain,
    Finalizer,
    Panic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cut0P0AuthorityLaneV1 {
    RawLedger,
    SingleOwnerHeader,
    CallableBatch,
}

fn authority_lane(route: InvocationRouteMatrixRowV1) -> Cut0P0AuthorityLaneV1 {
    let inventory = RouteOwnedInvocationInventoryV2::derive(route.family()).unwrap();
    match inventory.policy().inventory_authority() {
        InvocationInventoryAuthorityV2::RawExpansionReceipts => Cut0P0AuthorityLaneV1::RawLedger,
        InvocationInventoryAuthorityV2::CanonicalResolvedOwner => {
            Cut0P0AuthorityLaneV1::SingleOwnerHeader
        }
        InvocationInventoryAuthorityV2::CanonicalCallableCatalog => {
            Cut0P0AuthorityLaneV1::CallableBatch
        }
    }
}

const SCENARIOS: [Cut0P0ScenarioV1; 8] = [
    Cut0P0ScenarioV1::Success,
    Cut0P0ScenarioV1::Primary,
    Cut0P0ScenarioV1::Cleanup,
    Cut0P0ScenarioV1::Admission,
    Cut0P0ScenarioV1::Root,
    Cut0P0ScenarioV1::Drain,
    Cut0P0ScenarioV1::Finalizer,
    Cut0P0ScenarioV1::Panic,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cut0P0TerminalV1 {
    Success,
    Failed(Cut0P0ScenarioV1),
    Panicked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cut0P0ObservationV1 {
    route: &'static str,
    authority: Cut0P0AuthorityLaneV1,
    scenario: Cut0P0ScenarioV1,
    terminal: Cut0P0TerminalV1,
    external_commit_count: usize,
    retry: bool,
}

#[derive(Debug, Default)]
struct ExternalCommitProbeV1 {
    count: usize,
}

impl ExternalCommitProbeV1 {
    fn commit(&mut self, _candidate: FinalizedModuleCandidateV1) {
        self.count += 1;
    }
}

/// The sole disconnected CUT0-P0 outer owner.  Route rows never choose an
/// alternate adapter or a retry path.
struct Cut0P0OuterAdapterV1;

impl Cut0P0OuterAdapterV1 {
    fn execute(
        route: InvocationRouteMatrixRowV1,
        scenario: Cut0P0ScenarioV1,
    ) -> Cut0P0ObservationV1 {
        let authority = authority_lane(route);
        let result = catch_unwind(AssertUnwindSafe(|| Self::execute_inner(scenario)));
        let (terminal, external_commit_count) = match result {
            Ok((terminal, count)) => (terminal, count),
            Err(_) => (Cut0P0TerminalV1::Panicked, 0),
        };
        Cut0P0ObservationV1 {
            route: route.name(),
            authority,
            scenario,
            terminal,
            external_commit_count,
            retry: false,
        }
    }

    fn execute_inner(scenario: Cut0P0ScenarioV1) -> (Cut0P0TerminalV1, usize) {
        let candidate = ModuleLoweringInvocationCandidateV1::open(shell(), collector());
        if scenario == Cut0P0ScenarioV1::Panic {
            panic!("CUT0-P0 injected outer panic");
        }
        if let Some(stage) = failure_stage(scenario) {
            let proof = candidate.abort(stage).into_proof();
            assert!(proof.boundary_unchanged());
            assert_eq!(
                proof.retry_disposition(),
                super::module_lowering_invocation_candidate::InvocationCandidateRetryV1::Forbidden
            );
            return (Cut0P0TerminalV1::Failed(scenario), 0);
        }

        let mut candidate = candidate;
        candidate.capture_main().unwrap();
        let complete = candidate.complete_success().unwrap();
        let expectation = if scenario == Cut0P0ScenarioV1::Drain {
            InvocationDrainExpectationV1::new(
                vec!["wrong".into()],
                true,
                ConditionFnPolicyV1::Forbidden,
            )
        } else {
            InvocationDrainExpectationV1::new(
                vec!["main".into()],
                true,
                ConditionFnPolicyV1::Forbidden,
            )
        }
        .unwrap();
        let prepared =
            match ModuleLoweringInvocationDrainOwnerV1::prepare_complete(complete, expectation) {
                Ok(prepared) => prepared,
                Err(_) => return (Cut0P0TerminalV1::Failed(Cut0P0ScenarioV1::Drain), 0),
            };
        let inventory = CompletedInvocationInventoryV1::new(
            vec!["main".into()],
            RootBodyCompletionTrackerV1::new()
                .complete(RootBodyResultV1::NoValue)
                .unwrap(),
            ConditionFnPolicyV1::Forbidden,
        )
        .unwrap();
        let drained = prepared.drain_candidate(inventory).unwrap();
        let input = DrainedModuleFinalizationInputV1::new(drained, facts());
        if scenario == Cut0P0ScenarioV1::Finalizer {
            drop(input);
            return (Cut0P0TerminalV1::Failed(Cut0P0ScenarioV1::Finalizer), 0);
        }
        let finalized = finalize_drained_module_once(input);
        let mut commit = ExternalCommitProbeV1::default();
        commit.commit(finalized);
        (Cut0P0TerminalV1::Success, commit.count)
    }
}

fn failure_stage(scenario: Cut0P0ScenarioV1) -> Option<InvocationCandidateFailureStageV1> {
    Some(match scenario {
        Cut0P0ScenarioV1::Primary => InvocationCandidateFailureStageV1::ChildPrimary,
        Cut0P0ScenarioV1::Cleanup => InvocationCandidateFailureStageV1::ChildCleanup,
        Cut0P0ScenarioV1::Admission => InvocationCandidateFailureStageV1::Admission,
        Cut0P0ScenarioV1::Root => InvocationCandidateFailureStageV1::RootPreflight,
        _ => return None,
    })
}

fn shell() -> ModuleLoweringShellV1 {
    ModuleLoweringShellV1::from_empty_module(MirModule::new("cut0-p0".into())).unwrap()
}

fn function(symbol: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn collector() -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("main".into()),
            "main".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(function("main"))
        .unwrap()
        .collect();
    collector
}

fn facts() -> SealedModuleDeclarationFactsV1 {
    SealedModuleDeclarationFactsV1::new(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

#[test]
fn one_outer_adapter_executes_all_nine_routes_and_eight_outcomes() {
    let observations = InvocationRouteMatrixV1::rows()
        .iter()
        .flat_map(|route| {
            SCENARIOS
                .iter()
                .map(move |scenario| Cut0P0OuterAdapterV1::execute(*route, *scenario))
        })
        .collect::<Vec<_>>();
    assert_eq!(observations.len(), 9 * 8);
    assert_eq!(
        observations
            .iter()
            .filter(|observation| observation.terminal == Cut0P0TerminalV1::Success)
            .count(),
        9
    );
    assert!(observations.iter().all(|observation| !observation.retry));
    assert_eq!(
        observations
            .iter()
            .filter(|observation| observation.authority == Cut0P0AuthorityLaneV1::RawLedger)
            .count(),
        4 * SCENARIOS.len()
    );
    assert_eq!(
        observations
            .iter()
            .filter(|observation| observation.authority == Cut0P0AuthorityLaneV1::SingleOwnerHeader)
            .count(),
        3 * SCENARIOS.len()
    );
    assert_eq!(
        observations
            .iter()
            .filter(|observation| observation.authority == Cut0P0AuthorityLaneV1::CallableBatch)
            .count(),
        2 * SCENARIOS.len()
    );
    for route in InvocationRouteMatrixV1::rows() {
        assert_eq!(
            observations
                .iter()
                .filter(|observation| observation.route == route.name())
                .count(),
            SCENARIOS.len()
        );
    }
}

#[test]
fn every_failure_outcome_keeps_external_commit_zero() {
    for route in InvocationRouteMatrixV1::rows() {
        for scenario in SCENARIOS {
            let observation = Cut0P0OuterAdapterV1::execute(*route, scenario);
            assert_eq!(observation.scenario, scenario);
            let expected = usize::from(scenario == Cut0P0ScenarioV1::Success);
            assert_eq!(observation.external_commit_count, expected);
            if scenario == Cut0P0ScenarioV1::Panic {
                assert_eq!(observation.terminal, Cut0P0TerminalV1::Panicked);
            } else if scenario == Cut0P0ScenarioV1::Success {
                assert_eq!(observation.terminal, Cut0P0TerminalV1::Success);
            } else {
                assert_eq!(observation.terminal, Cut0P0TerminalV1::Failed(scenario));
            }
        }
    }
}
