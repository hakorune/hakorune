//! HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0: disconnected boundary fixtures.

use super::drained_module_candidate::{CompletedInvocationInventoryV1, DrainedModuleCandidateV1};
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_finalization_split::DrainedModuleFinalizationInputV1;
use super::module_invocation_drain::ConditionFnPolicyV1;
use super::root_body_completion::{RootBodyCompletionTrackerV1, RootBodyResultV1};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};
use std::collections::BTreeMap;

fn module() -> MirModule {
    let mut module = MirModule::new("finalize-split".into());
    module.add_function(MirFunction::new(
        FunctionSignature {
            name: "main".into(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    ));
    module
}

fn candidate() -> DrainedModuleCandidateV1 {
    let root = RootBodyCompletionTrackerV1::new()
        .complete(RootBodyResultV1::NoValue)
        .unwrap();
    let inventory = CompletedInvocationInventoryV1::new(
        vec!["main".into()],
        root,
        ConditionFnPolicyV1::Forbidden,
    )
    .unwrap();
    DrainedModuleCandidateV1::from_drained_module(module(), inventory).unwrap()
}

fn declaration_facts() -> SealedModuleDeclarationFactsV1 {
    SealedModuleDeclarationFactsV1::new(
        BTreeMap::from([("Main".into(), vec!["entry".into()])]),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

#[test]
fn post_drain_input_co_seals_candidate_and_declaration_facts() {
    let input = DrainedModuleFinalizationInputV1::new(candidate(), declaration_facts());
    assert!(input.candidate().module().functions.contains_key("main"));
    assert!(input
        .declaration_facts()
        .user_box_decls()
        .contains_key("Main"));
}

#[test]
fn post_drain_input_consumes_both_owners_once() {
    let input = DrainedModuleFinalizationInputV1::new(candidate(), declaration_facts());
    let (candidate, facts) = input.into_parts();
    assert!(candidate.module().functions.contains_key("main"));
    assert!(facts.user_box_decls().contains_key("Main"));
}
