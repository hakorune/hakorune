use super::*;
use crate::ast::RuneAttr;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

fn make_function(name: &str, is_entry_point: bool) -> MirFunction {
    let signature = FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.metadata.is_entry_point = is_entry_point;
    function
}

fn make_string_loop_function() -> MirFunction {
    let mut function = make_function("main", true);
    let entry = BasicBlockId::new(0);
    let header = BasicBlockId::new(18);
    let body = BasicBlockId::new(19);
    let exit = BasicBlockId::new(21);

    function
        .blocks
        .get_mut(&entry)
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::String("line-seed-abcdef".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(4),
                src: ValueId::new(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(5),
                value: ConstValue::Integer(16),
            },
        ]);

    let mut header_block = BasicBlock::new(header);
    header_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(15),
            inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
            type_hint: Some(MirType::String),
        },
        MirInstruction::Const {
            dst: ValueId::new(41),
            value: ConstValue::Integer(300000),
        },
        MirInstruction::Compare {
            dst: ValueId::new(37),
            op: CompareOp::Lt,
            lhs: ValueId::new(15),
            rhs: ValueId::new(41),
        },
        MirInstruction::Branch {
            condition: ValueId::new(37),
            then_bb: body,
            else_bb: exit,
            then_edge_args: None,
            else_edge_args: None,
        },
    ]);
    function.blocks.insert(header, header_block);

    let mut body_block = BasicBlock::new(body);
    body_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(50),
            value: ConstValue::Integer(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(47),
            op: BinaryOp::Div,
            lhs: ValueId::new(5),
            rhs: ValueId::new(50),
        },
        MirInstruction::Const {
            dst: ValueId::new(66),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Copy {
            dst: ValueId::new(36),
            src: ValueId::new(21),
        },
    ]);
    function.blocks.insert(body, body_block);
    function.blocks.insert(exit, BasicBlock::new(exit));
    function
}

#[test]
fn collect_sorted_user_box_decl_values_sorts_by_box_name() {
    let mut module = crate::mir::MirModule::new("test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Stage1ProgramResultValidationBox".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Main".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Stage1InputContractBox".to_string(), Vec::new());

    let decls = collect_sorted_user_box_decl_values(&module);
    let names: Vec<_> = decls
        .iter()
        .map(|decl| {
            decl.get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
                .to_string()
        })
        .collect();

    assert_eq!(
        names,
        vec![
            "Main".to_string(),
            "Stage1InputContractBox".to_string(),
            "Stage1ProgramResultValidationBox".to_string(),
        ]
    );
}

#[test]
fn collect_sorted_user_box_decl_values_includes_typed_field_decls() {
    let mut module = crate::mir::MirModule::new("test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Point".to_string(), vec!["x".to_string(), "y".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Point".to_string(),
        vec![
            crate::mir::UserBoxFieldDecl {
                name: "x".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            },
            crate::mir::UserBoxFieldDecl {
                name: "y".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: true,
            },
        ],
    );

    let decls = collect_sorted_user_box_decl_values(&module);
    let point = decls
        .iter()
        .find(|decl| decl.get("name").and_then(serde_json::Value::as_str) == Some("Point"))
        .expect("Point decl");
    let field_decls = point
        .get("field_decls")
        .and_then(serde_json::Value::as_array)
        .expect("field_decls array");

    assert_eq!(field_decls.len(), 2);
    assert_eq!(
        field_decls[0]
            .get("name")
            .and_then(serde_json::Value::as_str),
        Some("x")
    );
    assert_eq!(
        field_decls[0]
            .get("declared_type")
            .and_then(serde_json::Value::as_str),
        Some("IntegerBox")
    );
    assert_eq!(
        field_decls[1]
            .get("is_weak")
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
}

#[test]
fn collect_sorted_enum_decl_values_preserves_variant_inventory() {
    let mut module = crate::mir::MirModule::new("test".to_string());
    module.metadata.enum_decls.insert(
        "Option".to_string(),
        crate::mir::MirEnumDecl {
            type_parameters: vec!["T".to_string()],
            variants: vec![
                crate::mir::MirEnumVariantDecl {
                    name: "None".to_string(),
                    payload_type_name: None,
                },
                crate::mir::MirEnumVariantDecl {
                    name: "Some".to_string(),
                    payload_type_name: Some("T".to_string()),
                },
            ],
        },
    );

    let decls = collect_sorted_enum_decl_values(&module);
    assert_eq!(decls.len(), 1);
    assert_eq!(decls[0]["name"], "Option");
    assert_eq!(decls[0]["type_parameters"], json!(["T"]));
    assert_eq!(decls[0]["variants"][1]["name"], "Some");
    assert_eq!(decls[0]["variants"][1]["payload_type"], "T");
}

#[test]
fn ordered_harness_functions_puts_entry_main_first() {
    let mut module = MirModule::new("test".to_string());
    module.functions.insert(
        "Main.equals/1".to_string(),
        make_function("Main.equals/1", false),
    );
    module.functions.insert(
        "condition_fn".to_string(),
        make_function("condition_fn", false),
    );
    module
        .functions
        .insert("main".to_string(), make_function("main", true));

    let ordered: Vec<_> = ordered_harness_functions(&module)
        .into_iter()
        .map(|(name, _)| name.as_str())
        .collect();

    assert_eq!(ordered[0], "main");
    assert_eq!(ordered[1], "Main.equals/1");
    assert_eq!(ordered[2], "condition_fn");
}

#[test]
fn build_mir_json_root_emits_function_runes_as_attrs() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function.metadata.runes = vec![
        RuneAttr {
            name: "Symbol".to_string(),
            args: vec!["main_sym".to_string()],
        },
        RuneAttr {
            name: "CallConv".to_string(),
            args: vec!["c".to_string()],
        },
    ];
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let runes = root["functions"][0]["attrs"]["runes"]
        .as_array()
        .expect("attrs.runes array");
    assert_eq!(runes.len(), 2);
    assert_eq!(runes[0]["name"], "Symbol");
    assert_eq!(runes[0]["args"], serde_json::json!(["main_sym"]));
    assert_eq!(runes[1]["name"], "CallConv");
    assert_eq!(runes[1]["args"], serde_json::json!(["c"]));
}

#[test]
fn build_mir_json_root_emits_thin_entry_candidates() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .thin_entry_candidates
        .push(crate::mir::ThinEntryCandidate {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            value: Some(crate::mir::ValueId::new(7)),
            surface: crate::mir::ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            preferred_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
            current_carrier: crate::mir::ThinEntryCurrentCarrier::CompatBox,
            value_class: crate::mir::ThinEntryValueClass::AggLocal,
            reason: "variant.make stays aggregate-first".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let candidates = root["functions"][0]["metadata"]["thin_entry_candidates"]
        .as_array()
        .expect("thin_entry_candidates array");

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0]["surface"], "variant_make");
    assert_eq!(candidates[0]["subject"], "Option::Some");
    assert_eq!(candidates[0]["preferred_entry"], "thin_internal_entry");
    assert_eq!(candidates[0]["current_carrier"], "compat_box");
    assert_eq!(candidates[0]["value_class"], "agg_local");
    assert_eq!(candidates[0]["value"], 7);
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
                window_contract:
                    crate::mir::StringCorridorWindowContract::PreservePlanWindow,
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
                    start: Some(crate::mir::ValueId::new(2)),
                    end: Some(crate::mir::ValueId::new(3)),
                    known_length: Some(2),
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
        value_candidates[0]["reason"],
        "borrowed slice corridor can target a direct kernel entry before publication"
    );
    assert_eq!(value_candidates[0]["plan"]["corridor_root"], 7);
    assert_eq!(value_candidates[0]["plan"]["source_root"], 1);
    assert_eq!(value_candidates[0]["plan"]["start"], 2);
    assert_eq!(value_candidates[0]["plan"]["end"], 3);
    assert_eq!(value_candidates[0]["plan"]["known_length"], 2);
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
    let plan = crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
        corridor_root: crate::mir::ValueId::new(7),
        source_root: Some(crate::mir::ValueId::new(1)),
        start: Some(crate::mir::ValueId::new(2)),
        end: Some(crate::mir::ValueId::new(3)),
        known_length: Some(2),
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
                plan: Some(plan),
            },
            crate::mir::StringCorridorCandidate {
                kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
                state: crate::mir::StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
            },
        ],
    );
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["string_kernel_plans"]
        .as_object()
        .expect("string_kernel_plans object");
    let plan = &plans["8"];

    assert_eq!(plan["version"], 1);
    assert_eq!(plan["family"], "concat_triplet_window");
    assert_eq!(plan["corridor_root"], 7);
    assert_eq!(plan["source_root"], 1);
    assert_eq!(plan["known_length"], 2);
    assert_eq!(plan["retained_form"], "borrowed_text");
    assert_eq!(plan["barriers"]["publication"], "already_satisfied");
    assert_eq!(plan["consumer"], "direct_kernel_entry");
    assert_eq!(plan["direct_kernel_entry"]["state"], "candidate");
    assert_eq!(plan["legality"]["byte_exact"], true);
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
                    start: Some(ValueId::new(71)),
                    end: Some(ValueId::new(72)),
                    known_length: Some(2),
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
            }],
        );
    module.functions.insert("main".to_string(), function);

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
fn build_mir_json_root_emits_thin_entry_selections() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .thin_entry_selections
        .push(crate::mir::ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 3,
            value: Some(crate::mir::ValueId::new(8)),
            surface: crate::mir::ThinEntrySurface::UserBoxFieldGet,
            subject: "Point.x".to_string(),
            manifest_row: "user_box_field_get.inline_scalar",
            selected_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
            state: crate::mir::ThinEntrySelectionState::AlreadySatisfied,
            current_carrier: crate::mir::ThinEntryCurrentCarrier::BackendTyped,
            value_class: crate::mir::ThinEntryValueClass::InlineI64,
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let selections = root["functions"][0]["metadata"]["thin_entry_selections"]
        .as_array()
        .expect("thin_entry_selections array");

    assert_eq!(selections.len(), 1);
    assert_eq!(
        selections[0]["manifest_row"],
        "user_box_field_get.inline_scalar"
    );
    assert_eq!(selections[0]["selected_entry"], "thin_internal_entry");
    assert_eq!(selections[0]["state"], "already_satisfied");
    assert_eq!(selections[0]["value"], 8);
}

#[test]
fn build_mir_json_root_emits_sum_placement_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_facts
        .push(crate::mir::SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: 4,
            value: Some(crate::mir::ValueId::new(9)),
            surface: crate::mir::ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            value_class: crate::mir::ThinEntryValueClass::AggLocal,
            state: crate::mir::SumPlacementState::LocalAggregateCandidate,
            tag_reads: 1,
            project_reads: 1,
            barriers: vec![crate::mir::SumObjectizationBarrier::Call],
            reason: "variant value stays local until call boundary".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["sum_placement_facts"]
        .as_array()
        .expect("sum_placement_facts array");

    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["surface"], "variant_make");
    assert_eq!(facts[0]["state"], "local_agg_candidate");
    assert_eq!(facts[0]["barriers"][0], "call");
    assert_eq!(facts[0]["value"], 9);
}

#[test]
fn build_mir_json_root_emits_sum_placement_selections() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_selections
        .push(crate::mir::SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: 5,
            value: Some(crate::mir::ValueId::new(10)),
            surface: crate::mir::ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            source_sum: Some(crate::mir::ValueId::new(9)),
            manifest_row: "variant_project.local_aggregate",
            selected_path: crate::mir::SumPlacementPath::LocalAggregate,
            reason: "selected local aggregate projection".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let selections = root["functions"][0]["metadata"]["sum_placement_selections"]
        .as_array()
        .expect("sum_placement_selections array");

    assert_eq!(selections.len(), 1);
    assert_eq!(
        selections[0]["manifest_row"],
        "variant_project.local_aggregate"
    );
    assert_eq!(selections[0]["selected_path"], "local_aggregate");
    assert_eq!(selections[0]["source_sum"], 9);
    assert_eq!(selections[0]["value"], 10);
}

#[test]
fn build_mir_json_root_emits_sum_placement_layouts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_layouts
        .push(crate::mir::SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: 6,
            value: Some(crate::mir::ValueId::new(11)),
            surface: crate::mir::ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            layout: crate::mir::SumLocalAggregateLayout::TagI64Payload,
            reason: "selected local aggregate uses i64 payload lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let layouts = root["functions"][0]["metadata"]["sum_placement_layouts"]
        .as_array()
        .expect("sum_placement_layouts array");

    assert_eq!(layouts.len(), 1);
    assert_eq!(layouts[0]["layout"], "tag_i64_payload");
    assert_eq!(layouts[0]["surface"], "variant_make");
    assert_eq!(layouts[0]["value"], 11);
}

#[test]
fn build_mir_json_root_emits_agg_local_scalarization_routes() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .agg_local_scalarization_routes
        .push(crate::mir::AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(2),
            value: Some(crate::mir::ValueId::new(11)),
            subject: "Option::Some".to_string(),
            kind: crate::mir::AggLocalScalarizationKind::SumLocalLayout(
                crate::mir::SumLocalAggregateLayout::TagI64Payload,
            ),
            reason: "selected local aggregate uses i64 payload lane".to_string(),
        });
    function
        .metadata
        .agg_local_scalarization_routes
        .push(crate::mir::AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(3),
            value: Some(crate::mir::ValueId::new(12)),
            subject: "Point.x".to_string(),
            kind: crate::mir::AggLocalScalarizationKind::UserBoxLocalBody(
                crate::mir::ThinEntryValueClass::InlineI64,
            ),
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    function
        .metadata
        .agg_local_scalarization_routes
        .push(crate::mir::AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(4),
            value: Some(crate::mir::ValueId::new(13)),
            subject: "Point.flag".to_string(),
            kind: crate::mir::AggLocalScalarizationKind::TypedSlotStorage(
                crate::mir::StorageClass::InlineBool,
            ),
            reason: "typed slot stays inline on the scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let routes = root["functions"][0]["metadata"]["agg_local_scalarization_routes"]
        .as_array()
        .expect("agg_local_scalarization_routes array");

    assert_eq!(routes.len(), 3);
    assert_eq!(routes[0]["kind"], "sum_local_layout");
    assert_eq!(routes[0]["layout"], "tag_i64_payload");
    assert_eq!(routes[1]["kind"], "user_box_local_body");
    assert_eq!(routes[1]["value_class"], "inline_i64");
    assert_eq!(routes[2]["kind"], "typed_slot_storage");
    assert_eq!(routes[2]["storage_class"], "inline_bool");
}

#[test]
fn build_mir_json_root_emits_placement_effect_routes() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .placement_effect_routes
        .push(crate::mir::PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(2),
            value: Some(crate::mir::ValueId::new(11)),
            source: crate::mir::PlacementEffectSource::StringCorridor,
            subject: "string.value%11".to_string(),
            decision: crate::mir::PlacementEffectDecision::PublishHandle,
            state: crate::mir::PlacementEffectState::Candidate,
            detail: Some("plan(root=%11 source=- outer=- known_len=- proof=borrowed_slice(src=%1 start=%2 end=%3))".to_string()),
            reason: "publish boundary can sink to the corridor exit".to_string(),
        });
    function
        .metadata
        .placement_effect_routes
        .push(crate::mir::PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(3),
            value: Some(crate::mir::ValueId::new(12)),
            source: crate::mir::PlacementEffectSource::SumPlacement,
            subject: "Option::Some".to_string(),
            decision: crate::mir::PlacementEffectDecision::LocalAggregate,
            state: crate::mir::PlacementEffectState::Selected,
            detail: Some("variant_make.local_aggregate".to_string()),
            reason: "selected local aggregate route".to_string(),
        });
    function
        .metadata
        .placement_effect_routes
        .push(crate::mir::PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(4),
            value: Some(crate::mir::ValueId::new(13)),
            source: crate::mir::PlacementEffectSource::ThinEntry,
            subject: "Point.x".to_string(),
            decision: crate::mir::PlacementEffectDecision::ThinInternalEntry,
            state: crate::mir::PlacementEffectState::AlreadySatisfied,
            detail: Some("user_box_field_get.inline_scalar".to_string()),
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let routes = root["functions"][0]["metadata"]["placement_effect_routes"]
        .as_array()
        .expect("placement_effect_routes array");

    assert_eq!(routes.len(), 3);
    assert_eq!(routes[0]["source"], "string_corridor");
    assert_eq!(routes[0]["decision"], "publish_handle");
    assert_eq!(routes[1]["source"], "sum_placement");
    assert_eq!(routes[1]["state"], "selected");
    assert_eq!(routes[2]["source"], "thin_entry");
    assert_eq!(routes[2]["decision"], "thin_internal_entry");
}
