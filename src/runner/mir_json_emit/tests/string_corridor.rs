use super::super::build_mir_json_root;
use super::{make_function, make_string_loop_function};
use crate::mir::{MirModule, ValueConsumerFacts, ValueId};

#[test]
fn build_mir_json_root_emits_value_consumer_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function.metadata.value_consumer_facts.insert(
        ValueId::new(7),
        ValueConsumerFacts {
            direct_set_consumer: true,
        },
    );
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["value_consumer_facts"]
        .as_object()
        .expect("value_consumer_facts object");

    assert_eq!(facts["7"]["direct_set_consumer"], true);
}

#[test]
fn build_mir_json_root_emits_string_corridor_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function.metadata.string_corridor_facts.insert(
        crate::mir::ValueId::new(7),
        crate::mir::StringCorridorFact::str_slice(crate::mir::StringCorridorCarrier::MethodCall),
    );
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["string_corridor_facts"]
        .as_object()
        .expect("string_corridor_facts object");

    assert_eq!(facts["7"]["op"], "str.slice");
    assert_eq!(facts["7"]["role"], "borrow_producer");
    assert_eq!(facts["7"]["carrier"], "method_call");
    assert_eq!(facts["7"]["borrow_contract"], "borrow_text_from_obj");
    assert!(facts["7"]["outcome"].is_null());
    assert_eq!(facts["7"]["objectize"], "?");
    assert_eq!(facts["7"]["publish"], "?");
    assert_eq!(facts["7"]["materialize"], "?");
}

#[test]
fn build_mir_json_root_emits_string_corridor_candidates() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function.metadata.string_corridor_relations.insert(
        crate::mir::ValueId::new(7),
        vec![crate::mir::StringCorridorRelation {
            kind: crate::mir::StringCorridorRelationKind::PhiCarryBase,
            base_value: crate::mir::ValueId::new(6),
            window_contract: crate::mir::StringCorridorWindowContract::PreservePlanWindow,
            witness_value: None,
            reason: "single-input phi continuity keeps the current string corridor lane and preserves the proof-bearing plan window",
        }],
    );
    function.metadata.string_corridor_candidates.insert(
        crate::mir::ValueId::new(8),
        vec![crate::mir::StringCorridorCandidate {
            kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
            state: crate::mir::StringCorridorCandidateState::Candidate,
            reason:
                "borrowed slice corridor can target a direct kernel entry before publication",
            plan: Some(crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
                corridor_root: crate::mir::ValueId::new(7),
                source_root: Some(crate::mir::ValueId::new(1)),
                borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
                publish_reason: None,
                publish_repr_policy: None,
                stable_view_provenance: None,
                start: Some(crate::mir::ValueId::new(2)),
                end: Some(crate::mir::ValueId::new(3)),
                known_length: Some(2),
                publication_contract: Some(
                    crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
                ),
                proof:
                    crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                        left_value: Some(crate::mir::ValueId::new(4)),
                        left_source: crate::mir::ValueId::new(1),
                        left_start: crate::mir::ValueId::new(4),
                        left_end: crate::mir::ValueId::new(5),
                        middle: crate::mir::ValueId::new(6),
                        right_value: Some(crate::mir::ValueId::new(8)),
                        right_source: crate::mir::ValueId::new(1),
                        right_start: crate::mir::ValueId::new(5),
                        right_end: crate::mir::ValueId::new(9),
                        shared_source: true,
                    },
            }),
            publication_boundary: Some(
                crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary,
            ),
        }],
    );
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let candidates = root["functions"][0]["metadata"]["string_corridor_candidates"]
        .as_object()
        .expect("string_corridor_candidates object");
    let value_candidates = candidates["8"]
        .as_array()
        .expect("string corridor candidate array");

    assert_eq!(value_candidates.len(), 1);
    assert_eq!(value_candidates[0]["kind"], "direct_kernel_entry");
    assert_eq!(value_candidates[0]["state"], "candidate");
    assert_eq!(
        value_candidates[0]["publication_boundary"],
        "first_external_boundary"
    );
    assert_eq!(
        value_candidates[0]["reason"],
        "borrowed slice corridor can target a direct kernel entry before publication"
    );
    assert_eq!(value_candidates[0]["plan"]["corridor_root"], 7);
    assert_eq!(value_candidates[0]["plan"]["source_root"], 1);
    assert_eq!(
        value_candidates[0]["plan"]["borrow_contract"],
        "borrow_text_from_obj"
    );
    assert!(value_candidates[0]["plan"]["publish_reason"].is_null());
    assert!(value_candidates[0]["plan"]["publish_repr_policy"].is_null());
    assert!(value_candidates[0]["plan"]["stable_view_provenance"].is_null());
    assert_eq!(value_candidates[0]["plan"]["start"], 2);
    assert_eq!(value_candidates[0]["plan"]["end"], 3);
    assert_eq!(value_candidates[0]["plan"]["known_length"], 2);
    assert_eq!(
        value_candidates[0]["plan"]["publication_contract"],
        "publish_now_not_required_before_first_external_boundary"
    );
    assert_eq!(
        value_candidates[0]["plan"]["proof"]["kind"],
        "concat_triplet"
    );
    assert_eq!(value_candidates[0]["plan"]["proof"]["left_value"], 4);
    assert_eq!(value_candidates[0]["plan"]["proof"]["middle"], 6);
    assert_eq!(value_candidates[0]["plan"]["proof"]["right_value"], 8);
    assert_eq!(value_candidates[0]["plan"]["proof"]["shared_source"], true);

    let relations = root["functions"][0]["metadata"]["string_corridor_relations"]
        .as_object()
        .expect("string_corridor_relations object");
    let value_relations = relations["7"]
        .as_array()
        .expect("string corridor relation array");
    assert_eq!(value_relations[0]["kind"], "phi_carry_base");
    assert_eq!(value_relations[0]["base_value"], 6);
    assert_eq!(value_relations[0]["witness_value"], serde_json::Value::Null);
    assert_eq!(
        value_relations[0]["window_contract"],
        "preserve_plan_window"
    );
}

#[test]
fn build_mir_json_root_emits_string_kernel_plans() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    let publication_plan = crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
        corridor_root: crate::mir::ValueId::new(7),
        source_root: Some(crate::mir::ValueId::new(1)),
        borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: Some(crate::mir::StringPublishReason::StableObjectDemand),
        publish_repr_policy: Some(crate::mir::StringPublishReprPolicy::StableOwned),
        stable_view_provenance: None,
        start: Some(crate::mir::ValueId::new(2)),
        end: Some(crate::mir::ValueId::new(3)),
        known_length: Some(2),
        publication_contract: Some(
            crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(crate::mir::ValueId::new(4)),
            left_source: crate::mir::ValueId::new(1),
            left_start: crate::mir::ValueId::new(4),
            left_end: crate::mir::ValueId::new(5),
            middle: crate::mir::ValueId::new(6),
            right_value: Some(crate::mir::ValueId::new(8)),
            right_source: crate::mir::ValueId::new(1),
            right_start: crate::mir::ValueId::new(5),
            right_end: crate::mir::ValueId::new(9),
            shared_source: true,
        },
    };
    let direct_plan = crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
        corridor_root: crate::mir::ValueId::new(7),
        source_root: Some(crate::mir::ValueId::new(1)),
        borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        start: Some(crate::mir::ValueId::new(2)),
        end: Some(crate::mir::ValueId::new(3)),
        known_length: Some(2),
        publication_contract: Some(
            crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        proof: crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(crate::mir::ValueId::new(4)),
            left_source: crate::mir::ValueId::new(1),
            left_start: crate::mir::ValueId::new(4),
            left_end: crate::mir::ValueId::new(5),
            middle: crate::mir::ValueId::new(6),
            right_value: Some(crate::mir::ValueId::new(8)),
            right_source: crate::mir::ValueId::new(1),
            right_start: crate::mir::ValueId::new(5),
            right_end: crate::mir::ValueId::new(9),
            shared_source: true,
        },
    };
    function.metadata.string_corridor_candidates.insert(
        crate::mir::ValueId::new(8),
        vec![
            crate::mir::StringCorridorCandidate {
                kind: crate::mir::StringCorridorCandidateKind::PublicationSink,
                state: crate::mir::StringCorridorCandidateState::AlreadySatisfied,
                reason: "publish boundary is already sunk at the current corridor exit",
                plan: Some(publication_plan),
                publication_boundary: Some(
                    crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
            crate::mir::StringCorridorCandidate {
                kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
                state: crate::mir::StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(direct_plan),
                publication_boundary: Some(
                    crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
        ],
    );
    module.functions.insert("main".to_string(), function);
    crate::mir::refresh_module_string_kernel_plans(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["string_kernel_plans"]
        .as_object()
        .expect("string_kernel_plans object");
    let plan = &plans["8"];

    assert_eq!(plan["version"], 1);
    assert_eq!(plan["plan_value"], 8);
    assert_eq!(plan["family"], "concat_triplet_window");
    assert_eq!(plan["corridor_root"], 7);
    assert_eq!(plan["source_root"], 1);
    assert_eq!(plan["borrow_contract"], "borrow_text_from_obj");
    assert_eq!(plan["publish_reason"], "stable_object_demand");
    assert_eq!(plan["publish_repr_policy"], "stable_owned");
    assert!(plan["stable_view_provenance"].is_null());
    assert_eq!(plan["known_length"], 2);
    assert_eq!(plan["retained_form"], "borrowed_text");
    assert_eq!(plan["publication_boundary"], "first_external_boundary");
    assert_eq!(
        plan["publication_contract"],
        "publish_now_not_required_before_first_external_boundary"
    );
    assert_eq!(plan["barriers"]["publication"], "already_satisfied");
    assert_eq!(plan["consumer"], "direct_kernel_entry");
    assert_eq!(plan["text_consumer"], serde_json::Value::Null);
    assert_eq!(plan["carrier"], serde_json::Value::Null);
    assert_eq!(plan["verifier_owner"], "lowering_direct_kernel_entry");
    assert_eq!(plan["direct_kernel_entry"]["state"], "candidate");
    assert_eq!(plan["legality"]["byte_exact"], true);
    assert_eq!(plan["legality"]["requires_kernel_text_slot"], false);
    assert_eq!(plan["read_alias"]["same_receiver"], true);
    assert_eq!(plan["read_alias"]["source_window"], true);
    assert_eq!(plan["read_alias"]["followup_substring"], false);
    assert_eq!(plan["read_alias"]["piecewise_subrange"], false);
    assert_eq!(plan["read_alias"]["direct_set_consumer"], false);
    assert_eq!(plan["read_alias"]["shared_receiver"], false);
    assert_eq!(plan["parts"][0]["kind"], "slice");
    assert_eq!(plan["parts"][1]["kind"], "const");
    assert_eq!(plan["parts"][1]["known_length"], 2);
    assert_eq!(plan["parts"][2]["kind"], "slice");
}

#[test]
fn build_mir_json_root_emits_string_kernel_plan_loop_payload() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_string_loop_function();
    function.metadata.string_corridor_candidates.insert(
        ValueId::new(21),
        vec![crate::mir::StringCorridorCandidate {
            kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
            state: crate::mir::StringCorridorCandidateState::Candidate,
            reason: "substring concat loop can target a direct kernel entry",
            plan: Some(crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
                corridor_root: ValueId::new(21),
                source_root: Some(ValueId::new(21)),
                borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
                publish_reason: None,
                publish_repr_policy: None,
                stable_view_provenance: None,
                start: Some(ValueId::new(71)),
                end: Some(ValueId::new(72)),
                known_length: Some(2),
                publication_contract: Some(
                    crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
                ),
                proof:
                    crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                        left_value: Some(ValueId::new(26)),
                        left_source: ValueId::new(21),
                        left_start: ValueId::new(46),
                        left_end: ValueId::new(47),
                        middle: ValueId::new(66),
                        right_value: Some(ValueId::new(27)),
                        right_source: ValueId::new(21),
                        right_start: ValueId::new(47),
                        right_end: ValueId::new(42),
                        shared_source: true,
                    },
            }),
            publication_boundary: None,
        }],
    );
    module.functions.insert("main".to_string(), function);
    crate::mir::refresh_module_string_kernel_plans(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plan = &root["functions"][0]["metadata"]["string_kernel_plans"]["21"];

    assert_eq!(plan["parts"][1]["literal"], "xx");
    assert_eq!(plan["loop_payload"]["seed_value"], 3);
    assert_eq!(plan["loop_payload"]["seed_literal"], "line-seed-abcdef");
    assert_eq!(plan["loop_payload"]["seed_length"], 16);
    assert_eq!(plan["loop_payload"]["loop_bound"], 300000);
    assert_eq!(plan["loop_payload"]["split_length"], 8);
}

#[test]
fn build_mir_json_root_emits_string_kernel_plan_slot_hop_substring() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    let block = function
        .blocks
        .get_mut(&crate::mir::BasicBlockId::new(0))
        .expect("entry block");
    block.instructions.extend([
        crate::mir::MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::String("xx".to_string()),
        },
        crate::mir::MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(6),
        },
        crate::mir::MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(1),
        },
        crate::mir::MirInstruction::Const {
            dst: ValueId::new(4),
            value: crate::mir::ConstValue::Integer(5),
        },
        crate::mir::MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern(
                "nyash.string.substring_concat3_hhhii".to_string(),
            )),
            args: vec![
                ValueId::new(0),
                ValueId::new(1),
                ValueId::new(0),
                ValueId::new(3),
                ValueId::new(4),
            ],
            effects: crate::mir::EffectMask::PURE,
        },
        crate::mir::MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern(
                "nyash.string.substring_hii".to_string(),
            )),
            args: vec![ValueId::new(10), ValueId::new(3), ValueId::new(4)],
            effects: crate::mir::EffectMask::PURE,
        },
    ]);
    block.set_terminator(crate::mir::MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    function.metadata.string_corridor_candidates.insert(
        ValueId::new(10),
        vec![crate::mir::StringCorridorCandidate {
            kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
            state: crate::mir::StringCorridorCandidateState::Candidate,
            reason: "direct kernel entry candidate",
            plan: Some(crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
                corridor_root: ValueId::new(10),
                source_root: Some(ValueId::new(0)),
                borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
                publish_reason: None,
                publish_repr_policy: None,
                stable_view_provenance: None,
                start: Some(ValueId::new(3)),
                end: Some(ValueId::new(4)),
                known_length: Some(2),
                publication_contract: Some(
                    crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
                ),
                proof:
                    crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                        left_value: Some(ValueId::new(0)),
                        left_source: ValueId::new(0),
                        left_start: ValueId::new(3),
                        left_end: ValueId::new(2),
                        middle: ValueId::new(1),
                        right_value: Some(ValueId::new(0)),
                        right_source: ValueId::new(0),
                        right_start: ValueId::new(2),
                        right_end: ValueId::new(4),
                        shared_source: true,
                    },
            }),
            publication_boundary: Some(
                crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary,
            ),
        }],
    );
    module.functions.insert("main".to_string(), function);
    crate::mir::refresh_module_string_kernel_plans(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plan = &root["functions"][0]["metadata"]["string_kernel_plans"]["10"];

    assert_eq!(plan["text_consumer"], "slot_text");
    assert_eq!(plan["carrier"], "kernel_text_slot");
    assert_eq!(plan["slot_hop_substring"]["consumer_value"], 11);
    assert_eq!(plan["slot_hop_substring"]["start"], 3);
    assert_eq!(plan["slot_hop_substring"]["end"], 4);
    assert_eq!(plan["slot_hop_substring"]["instruction_index"], 5);
    assert!(plan["slot_hop_substring"]["copy_instruction_indices"]
        .as_array()
        .expect("copy indices")
        .is_empty());
}
