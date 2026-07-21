//! HEADERPORT0-I0-DRAIN0-P0: disconnected drain/candidate failure matrix.
//!
//! These fixtures exercise the old disconnected drain owner and the new
//! route-owned candidate boundary together.  They do not connect either
//! product to a production lowering route.

use super::drained_module_candidate::{
    CompletedInvocationInventoryV1, DrainedModuleCandidateErrorV1, DrainedModuleCandidateV1,
};
use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use super::module_invocation_drain::{
    ConditionFnPolicyV1, InvocationDrainExpectationV1, ModuleLoweringInvocationDrainOwnerV1,
};
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

fn collector(symbols: &[&str]) -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    for symbol in symbols {
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol((*symbol).to_owned()),
                (*symbol).to_owned(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft(symbol))
            .unwrap()
            .collect();
    }
    collector
}

fn empty_shell() -> ModuleLoweringShellV1 {
    ModuleLoweringShellV1::from_empty_module(MirModule::new("main".into())).unwrap()
}

fn root_body() -> super::root_body_completion::CompletedRootBodyV1 {
    RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap()
}

fn drain(
    symbols: &[&str],
    condition_fn: ConditionFnPolicyV1,
) -> Result<MirModule, super::module_invocation_drain::InvocationDrainPreflightErrorV1> {
    let owner = ModuleLoweringInvocationDrainOwnerV1::new(empty_shell(), collector(symbols));
    let expectation = InvocationDrainExpectationV1::new(
        symbols.iter().map(|symbol| (*symbol).to_owned()),
        true,
        condition_fn,
    )
    .unwrap();
    owner.prepare(expectation).map(|prepared| prepared.drain())
}

#[test]
fn exact_drain_inventory_and_candidate_policy_co_seal() {
    let module = drain(&["condition_fn", "main"], ConditionFnPolicyV1::Required).unwrap();
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["main".into(), "condition_fn".into()],
        root_body(),
        ConditionFnPolicyV1::Required,
    )
    .unwrap();
    let candidate = DrainedModuleCandidateV1::from_drained_module(module, inventory).unwrap();
    assert_eq!(candidate.inventory().symbols(), ["condition_fn", "main"]);
    assert!(candidate.module().functions.contains_key("main"));
}

#[test]
fn drain_inventory_order_is_deterministic_before_candidate_issue() {
    let first = drain(&["z/0", "main", "a/0"], ConditionFnPolicyV1::Forbidden);
    let second = drain(&["a/0", "z/0", "main"], ConditionFnPolicyV1::Forbidden);
    let first = first.unwrap();
    let second = second.unwrap();
    let first_symbols = first.functions.keys().cloned().collect::<Vec<_>>();
    let second_symbols = second.functions.keys().cloned().collect::<Vec<_>>();
    assert_eq!(first_symbols, second_symbols);
}

#[test]
fn drain_rejects_missing_main_before_candidate_issue() {
    let owner = ModuleLoweringInvocationDrainOwnerV1::new(empty_shell(), collector(&["child/0"]));
    let expectation = InvocationDrainExpectationV1::new(
        vec!["child/0".into()],
        true,
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    assert!(matches!(
        owner.prepare(expectation),
        Err(super::module_invocation_drain::InvocationDrainPreflightErrorV1::MissingMain)
    ));
}

#[test]
fn drain_condition_policy_matrix_is_explicit() {
    assert!(drain(&["main"], ConditionFnPolicyV1::Optional).is_ok());
    assert!(drain(&["main", "condition_fn"], ConditionFnPolicyV1::Optional).is_ok());
    assert!(matches!(
        drain(&["main"], ConditionFnPolicyV1::Required),
        Err(super::module_invocation_drain::InvocationDrainPreflightErrorV1::MissingConditionFn)
    ));
    assert!(matches!(
        drain(&["main", "condition_fn"], ConditionFnPolicyV1::Forbidden),
        Err(super::module_invocation_drain::InvocationDrainPreflightErrorV1::UnexpectedConditionFn)
    ));
}

#[test]
fn candidate_rejects_inventory_mismatch_without_exposing_module() {
    let module = drain(&["main"], ConditionFnPolicyV1::Forbidden).unwrap();
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["main".into(), "child/0".into()],
        root_body(),
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    assert!(matches!(
        DrainedModuleCandidateV1::from_drained_module(module, inventory),
        Err(DrainedModuleCandidateErrorV1::InventoryMismatch { .. })
    ));
}

#[test]
fn candidate_rejects_missing_main_even_when_inventory_matches() {
    let mut module = MirModule::new("main".into());
    module.add_function(draft("child/0"));
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["child/0".into()],
        root_body(),
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    assert_eq!(
        DrainedModuleCandidateV1::from_drained_module(module, inventory).unwrap_err(),
        DrainedModuleCandidateErrorV1::MissingMain
    );
}

#[test]
fn duplicate_inventory_is_rejected_before_drain_candidate_creation() {
    assert_eq!(
        CompletedInvocationInventoryV1::new(
            vec!["main".into(), "main".into()],
            root_body(),
            ConditionFnPolicyV1::Optional,
        )
        .unwrap_err(),
        DrainedModuleCandidateErrorV1::DuplicateInventorySymbol {
            symbol: "main".into(),
        }
    );
}
