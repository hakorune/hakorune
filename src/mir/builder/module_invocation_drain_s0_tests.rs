//! CUT0-S0 disconnected same-state drain fixtures.

use super::drained_module_candidate::CompletedInvocationInventoryV1;
use super::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use super::module_invocation_drain::{
    ConditionFnPolicyV1, InvocationDrainExpectationV1, ModuleLoweringInvocationDrainOwnerV1,
};
use super::module_lowering_invocation_candidate::ModuleLoweringInvocationCandidateV1;
use super::module_lowering_shell::ModuleLoweringShellV1;
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

fn draft(symbol: &str) -> MirFunction {
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

fn collector() -> super::module_draft_collector::ModuleDraftCollectorV1 {
    let mut collector = super::module_draft_collector::ModuleDraftCollectorV1::default();
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("main".into()),
            "main".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("main"))
        .unwrap()
        .collect();
    collector
}

fn expectation() -> InvocationDrainExpectationV1 {
    InvocationDrainExpectationV1::new(
        vec!["main".into()],
        true,
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap()
}

#[test]
fn completed_candidate_drains_to_typed_candidate_without_rebuilding_state() {
    let shell =
        ModuleLoweringShellV1::from_empty_module(MirModule::new("s0".into())).unwrap();
    let mut candidate = ModuleLoweringInvocationCandidateV1::open(shell, collector());
    candidate.capture_main().unwrap();
    let complete = candidate.complete_success().unwrap();
    let prepared = ModuleLoweringInvocationDrainOwnerV1::prepare_complete(complete, expectation())
        .unwrap();
    let root = RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap();
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["main".into()],
        root,
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    let drained = prepared.drain_candidate(inventory).unwrap();
    assert!(drained.module().functions.contains_key("main"));
    assert_eq!(drained.inventory().symbols(), ["main"]);
}
