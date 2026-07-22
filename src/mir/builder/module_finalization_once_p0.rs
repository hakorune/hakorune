//! CUT0-S0 finalizer consume-once fixtures.

use super::drained_module_candidate::{CompletedInvocationInventoryV1, DrainedModuleCandidateV1};
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_finalization_once::{finalize_drained_module_once, FinalizedModuleCandidateV1};
use super::module_finalization_split::DrainedModuleFinalizationInputV1;
use super::module_invocation_drain::ConditionFnPolicyV1;
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};
use std::collections::BTreeMap;

fn candidate() -> DrainedModuleCandidateV1 {
    let mut module = MirModule::new("finalized".into());
    module.add_function(MirFunction::new(
        FunctionSignature {
            name: "main".into(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    ));
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["main".into()],
        RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::NoValue)
            .unwrap(),
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    DrainedModuleCandidateV1::from_drained_module(module, inventory).unwrap()
}

fn facts() -> SealedModuleDeclarationFactsV1 {
    SealedModuleDeclarationFactsV1::new(
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn finalize() -> FinalizedModuleCandidateV1 {
    finalize_drained_module_once(DrainedModuleFinalizationInputV1::new(candidate(), facts()))
}

#[test]
fn finalizer_consumes_co_sealed_input_without_bare_module_or_builder() {
    let finalized = finalize();
    assert!(finalized.candidate().module().functions.contains_key("main"));
    assert!(finalized.declaration_facts().user_box_decls().is_empty());
}

#[test]
fn finalizer_output_is_single_use_and_retains_root_inventory() {
    let finalized = finalize();
    assert_eq!(finalized.candidate().inventory().symbols(), ["main"]);
    assert_eq!(
        finalized.candidate().inventory().root_body().result(),
        RootBodyResultV1::NoValue
    );
}
