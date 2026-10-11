//! Independent wire corruption and source-image checks, without execution admission.
use super::*;
use crate::mir::instruction::{FaultFrameMode, InvokeCallResultKind, InvokeOperation};
use crate::mir::{ConstValue, MirInstruction as I, MirModule, MirType};
use serde_json::json;

fn fixture() -> MirModule {
    let mut module = MirModule::new("transport-data".into());
    let mut leaf = MirFunction::new(
        FunctionSignature {
            name: "Leaf.pick/0".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(7),
    );
    leaf.next_value_id = 1;
    leaf.metadata
        .value_types
        .insert(ValueId::new(0), MirType::Integer);
    let entry = leaf.blocks.get_mut(&BasicBlockId::new(7)).unwrap();
    entry.add_instruction(I::Const {
        dst: ValueId::new(0),
        value: ConstValue::Integer(7),
    });
    entry.set_terminator(I::Return {
        value: Some(ValueId::new(0)),
    });
    module.canonical_callable_definitions.insert(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Leaf", "pick", 0),
        "Leaf.pick/0".into(),
    );
    module.add_function(leaf);
    let mut caller = MirFunction::new(
        FunctionSignature {
            name: "main".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO | EffectMask::READ,
        },
        BasicBlockId::new(3),
    );
    caller.next_value_id = 3;
    caller.metadata.source_file = Some("original.hako".into());
    caller.metadata.line_number = Some(12);
    caller
        .metadata
        .value_types
        .insert(ValueId::new(1), MirType::Bool);
    caller
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Integer);
    let mut call = crate::mir::definitions::MirCall::global(
        None,
        hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
            "Leaf".into(),
            "pick".into(),
            0,
        )
        .unwrap(),
        vec![],
    );
    call.flags.can_inline = true;
    call.effects = EffectMask::READ | EffectMask::IO;
    let entry = caller.blocks.get_mut(&BasicBlockId::new(3)).unwrap();
    entry.add_instruction(I::FaultFrameEnter {
        dst: ValueId::new(0),
        mode: FaultFrameMode::RootOwned,
    });
    entry.add_instruction_with_span(
        I::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        },
        crate::ast::Span::new(10, 14, 12, 3),
    );
    entry.set_terminator(I::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        },
        fault_frame: ValueId::new(0),
        normal_landing: BasicBlockId::new(4),
        fault_landing: BasicBlockId::new(5),
    });
    let mut normal = BasicBlock::new(BasicBlockId::new(4));
    normal.add_instruction(I::InvokeNormalResult {
        invoke_block: BasicBlockId::new(3),
        dst: ValueId::new(2),
    });
    normal.set_terminator(I::Return {
        value: Some(ValueId::new(2)),
    });
    let mut fault = BasicBlock::new(BasicBlockId::new(5));
    fault.set_terminator(I::ReturnFault {
        fault_frame: ValueId::new(0),
    });
    caller.add_block(normal);
    caller.add_block(fault);
    module.add_function(caller);
    module
}

fn emit(module: &MirModule) -> Value {
    let wire = crate::runner::mir_json_emit::emit_mir_json_string_for_unpublished_candidate(module)
        .unwrap();
    parse_canonical_document_transport_v1(&wire).unwrap()
}

fn caller(document: &mut Value) -> &mut Value {
    document["functions"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|f| f["name"] == "main")
        .unwrap()
}

#[test]
fn canonical_document_transport_retains_flags_effects_bool_and_source_positions() {
    let module = fixture();
    let document = emit(&module);
    let caller = document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "main")
        .unwrap();
    assert_eq!(
        caller["metadata"]["canonical_transport_v1"]["entry_block"],
        3
    );
    assert_eq!(
        caller["metadata"]["canonical_transport_v1"]["source_file"],
        "original.hako"
    );
    assert_eq!(
        caller["metadata"]["canonical_transport_v1"]["block_spans"][0]["instructions"][1],
        json!({"start":10,"end":14,"line":12,"column":3})
    );
    let rows = caller["blocks"][0]["instructions"].as_array().unwrap();
    assert_eq!(rows[1]["value"], json!({"type":"i1","value":true}));
    assert_eq!(
        rows[2]["operation"]["call"]["effects_bits"],
        (EffectMask::READ | EffectMask::IO).bits()
    );
    assert_eq!(
        rows[2]["operation"]["call"]["flags"],
        json!({"tail_call":false,"no_return":false,"can_inline":true,"is_constructor":false})
    );
    assert!(
        crate::runner::json_v1_bridge::try_parse_v1_to_module(&document.to_string()).is_err(),
        "transport does not open executable reader"
    );
}

#[test]
fn canonical_document_transport_rejects_independent_wire_corruption() {
    let original = emit(&fixture());
    for mutation in 0..20 {
        let mut document = original.clone();
        match mutation {
            0 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["call"]
                    ["target"]["owner"] = json!("Foreign")
            }
            1 => caller(&mut document)["blocks"][0]["instructions"][2]["normal"] = json!(5),
            2 => caller(&mut document)["blocks"][1]["instructions"][0]["invoke_block"] = json!(7),
            3 => caller(&mut document)["blocks"][2]["instructions"][0]["fault_frame"] = json!(1),
            4 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["call"]["dst"] =
                    json!(2)
            }
            5 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["call"]["args"] =
                    json!([1])
            }
            6 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["call"]
                    ["flags"]["hidden"] = json!(true)
            }
            7 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["call"]
                    ["effects_bits"] = json!(65536)
            }
            8 => {
                caller(&mut document)["blocks"][0]["instructions"][1]["dst"] = json!(4294967296u64)
            }
            9 => caller(&mut document)["blocks"][0]["instructions"][1]["dst"] = json!(0),
            10 => {
                caller(&mut document)["metadata"]["canonical_transport_v1"]["entry_block"] =
                    json!(99)
            }
            11 => {
                caller(&mut document)["metadata"]["canonical_transport_v1"]["next_value_id"] =
                    json!(2)
            }
            12 => caller(&mut document)["blocks"][0]["instructions"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            13 => caller(&mut document)["blocks"][1]["instructions"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            14 => {
                let row = document["canonical_transport_v1"]["callable_definitions"][0].clone();
                document["canonical_transport_v1"]["callable_definitions"]
                    .as_array_mut()
                    .unwrap()
                    .push(row);
            }
            15 => {
                let row = caller(&mut document)["blocks"][0].clone();
                caller(&mut document)["blocks"]
                    .as_array_mut()
                    .unwrap()
                    .push(row);
            }
            16 => {
                caller(&mut document)["metadata"]["canonical_transport_v1"]
                    ["physical_param_carriers"] = json!(["borrowed_tagged_value"])
            }
            17 => {
                caller(&mut document)["blocks"][0]["instructions"][2]["operation"]["kind"] =
                    json!("home_release")
            }
            18 => {
                caller(&mut document)["blocks"][0]["instructions"][1]["value"]["value"] = json!(1)
            }
            _ => {
                caller(&mut document)["metadata"]["canonical_transport_v1"]["block_spans"][0]
                    ["instructions"] = json!([])
            }
        }
        assert!(
            parse_canonical_document_transport_v1(&document.to_string()).is_err(),
            "mutation {mutation}"
        );
    }
    assert!(parse_canonical_document_transport_v1(
        "{\"schema_version\":\"1.0\",\"schema_version\":\"1.0\"}"
    )
    .is_err());
}

#[test]
fn canonical_document_transport_preserves_original_size_class_published_image() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(|| {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let original = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("lang/src/hako_alloc/memory/size_class_box.hako")).unwrap();
            let source = format!("{original}\nstatic box Main {{ main() {{ return 0 }} }}");
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(&source, crate::parser::ParserBuildConfig::default()).unwrap();
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else { panic!("source-backed"); };
            let request = crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, std::collections::HashMap::new());
            crate::mir::MirCompiler::with_options(false)
                .compile_normal_for_mir_json(request, |view, verification| -> Result<(), String> {
                    if let Err(errors) = verification { return Err(format!("source verification: {errors:?}")); }
                    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
                    let path = directory.path().join("canonical.json");
                    crate::runner::mir_json_emit::emit_mir_json_for_published_view(view, &path)?;
                    let wire = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
                    let document = parse_canonical_document_transport_v1(&wire)?;
                    let functions = document["functions"].as_array().unwrap();
                    assert_eq!(functions.len(), view.module().functions.len());
                    let definitions = document["canonical_transport_v1"]["callable_definitions"].as_array().unwrap().iter()
                        .map(|row| (key(&row["key"]).unwrap(), functions.iter().find(|function| function["name"] == row["symbol"]).unwrap())).collect::<Definitions<'_>>();
                    assert_eq!(document["canonical_transport_v1"]["module_name"], view.module().name);
                    assert_eq!(definitions.len(), view.module().canonical_callable_definitions.len());
                    for (original_key, symbol) in &view.module().canonical_callable_definitions {
                        use hakorune_mir_defs::SameModuleCallableNamespaceV1 as Namespace;
                        let namespace = match original_key.namespace() {
                            Namespace::FreeFunction => "free_function",
                            Namespace::StaticBoxMethod => "static_box_method",
                            Namespace::InstanceBoxMethod => "instance_box_method",
                            Namespace::BirthConstructor => "birth_constructor",
                        };
                        let row = definitions.get(&(namespace, original_key.owner(), original_key.name(), original_key.arity())).unwrap();
                        assert_eq!(row["name"], *symbol);
                    }
                    let mut invokes = 0;
                    for function in view.module().functions.values() {
                        let encoded = functions.iter().find(|value| value["name"] == function.signature.name).unwrap();
                        let contract = &encoded["metadata"]["canonical_transport_v1"];
                        assert_eq!(id(contract, "entry_block")?, function.entry_block.as_u32());
                        assert_eq!(id(contract, "next_value_id")?, function.next_value_id);
                        assert_eq!(contract["physical_param_carriers"], json!(function.metadata.physical_param_carrier_transport_tags_v1()));
                        assert_eq!(array(&contract["signature"], "params")?.iter().map(ty).collect::<Result<Vec<_>, _>>()?, function.signature.params);
                        assert_eq!(ty(&contract["signature"]["return_type"])?, function.signature.return_type);
                        assert_eq!(effects(&contract["signature"])?, function.signature.effects);
                        assert_eq!(array(encoded, "params")?.iter().map(id_value).collect::<Result<Vec<_>, _>>()?, function.params.iter().map(|id| id.as_u32()).collect::<Vec<_>>());
                        assert_eq!(array(contract, "locals")?.iter().map(ty).collect::<Result<Vec<_>, _>>()?, function.locals);
                        assert_eq!(contract["source_file"], json!(function.metadata.source_file));
                        assert_eq!(contract["line_number"], json!(function.metadata.line_number));
                        assert_eq!(array(contract, "declared_params")?.len(), function.metadata.declared_param_decls.len());
                        for (row, original) in array(contract, "declared_params")?.iter().zip(&function.metadata.declared_param_decls) {
                            assert_eq!(row, &json!({"name":original.name,"declared_type_name":original.declared_type_name,"implicit_receiver":original.implicit_receiver}));
                        }
                        for block in function.blocks.values() {
                            let encoded = encoded["blocks"].as_array().unwrap().iter().find(|row| row["id"] == block.id.as_u32()).unwrap();
                            let spans = array(contract, "block_spans")?.iter().find(|row| row["block"] == block.id.as_u32()).unwrap();
                            let span_json = |span: crate::ast::Span| json!({"start":span.start,"end":span.end,"line":span.line,"column":span.column});
                            assert_eq!(spans["instructions"], json!(block.instruction_spans.iter().copied().map(span_json).collect::<Vec<_>>()));
                            assert_eq!(spans["terminator"], json!(block.terminator_span.map(span_json)));
                            assert_eq!(spans["return_env"], json!(block.return_env.as_ref().map(|values| values.iter().map(|v| v.as_u32()).collect::<Vec<_>>())));
                            let layout_tag = |layout| match layout {
                                crate::mir::edge_args::JumpArgsLayout::CarriersOnly => "carriers_only",
                                crate::mir::edge_args::JumpArgsLayout::ExprResultPlusCarriers => "expr_result_plus_carriers",
                            };
                            assert_eq!(spans["return_env_layout"], json!(block.return_env_layout.map(layout_tag)));
                            let original = block.instructions.iter().chain(block.terminator.iter()).collect::<Vec<_>>();
                            let rows = array(encoded, "instructions")?;
                            assert_eq!(rows.len(), original.len());
                            for (row, original) in rows.iter().zip(original) {
                                assert_eq!(&instruction::parse(row, &definitions)?, original);
                                if matches!(original, I::Invoke { .. }) { invokes += 1; }
                            }
                        }
                    }
                    assert!(invokes > 0, "must exercise the original target's fallible graph");
                    assert!(crate::runner::json_v1_bridge::try_parse_v1_to_module(&wire).is_err());
                    Ok(())
                }).unwrap();
        });
    }).unwrap().join().unwrap();
}

#[test]
fn canonical_document_transport_checks_cross_surface_types_without_narrowing_tagged_args() {
    let mut tagged = emit(&fixture());
    let definitions = tagged["canonical_transport_v1"]["callable_definitions"]
        .as_array_mut()
        .unwrap();
    definitions[0]["key"]["arity"] = json!(1);
    let leaf = tagged["functions"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|function| function["name"] == "Leaf.pick/0")
        .unwrap();
    leaf["params"] = json!([0]);
    leaf["metadata"]["value_types"] = json!({"0":"i64", "1":"i64"});
    let contract = &mut leaf["metadata"]["canonical_transport_v1"];
    contract["signature"]["params"] = json!(["i64"]);
    contract["physical_param_carriers"] = json!(["borrowed_tagged_value"]);
    contract["next_value_id"] = json!(2);
    leaf["blocks"][0]["instructions"][0]["dst"] = json!(1);
    leaf["blocks"][0]["instructions"][1]["value"] = json!(1);
    let call = &mut caller(&mut tagged)["blocks"][0]["instructions"][2]["operation"]["call"];
    call["target"]["arity"] = json!(1);
    call["args"] = json!([1]);
    parse_canonical_document_transport_v1(&tagged.to_string()).unwrap();

    for mutation in 0..8 {
        let mut broken = tagged.clone();
        let leaf = broken["functions"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|function| function["name"] == "Leaf.pick/0")
            .unwrap();
        match mutation {
            0 => {
                leaf["metadata"]["canonical_transport_v1"]["physical_param_carriers"] =
                    json!(["existing_callable_i64"])
            }
            1 => leaf["metadata"]["value_types"]["0"] = json!("i1"),
            2 => {
                leaf["metadata"]["canonical_transport_v1"]["declared_params"] =
                    json!([{"name":"p", "declared_type_name":null, "implicit_receiver":false}])
            }
            3 => {
                let main = caller(&mut broken);
                main["blocks"][0]["instructions"].as_array_mut().unwrap().insert(2,
                    json!({"op":"compare", "operation":"==", "lhs":1, "rhs":1, "dst":3, "dst_type":"i64"}));
                main["metadata"]["value_types"]["3"] = json!("i1");
                main["metadata"]["canonical_transport_v1"]["next_value_id"] = json!(4);
                main["metadata"]["canonical_transport_v1"]["block_spans"][0]["instructions"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"start":0,"end":0,"line":0,"column":0}));
            }
            4 => caller(&mut broken)["metadata"]["value_types"]["1"] = json!("i64"),
            _ => {
                leaf["metadata"]["canonical_transport_v1"]["physical_param_carriers"] =
                    if mutation == 5 {
                        Value::Null
                    } else {
                        json!(["existing_callable_i64"])
                    };
                if mutation == 6 {
                    caller(&mut broken)["metadata"]["value_types"]["1"] = json!("unknown");
                }
                if mutation == 7 {
                    caller(&mut broken)["metadata"]["value_types"]
                        .as_object_mut()
                        .unwrap()
                        .remove("1");
                }
            }
        }
        let error = parse_canonical_document_transport_v1(&broken.to_string()).unwrap_err();
        let expected = [
            "invoke-argument-carrier-drift",
            "parameter-type-drift",
            "declared-parameter-drift",
            "destination-type-drift",
            "producer-type-drift",
            "invoke-argument-carrier-drift",
            "invoke-argument-carrier-drift",
            "invoke-argument-carrier-drift",
        ][mutation];
        assert!(
            error.contains(expected),
            "cross-surface mutation {mutation}: {error}"
        );
    }
}

fn all_source_fixture() -> MirModule {
    use crate::mir::edge_args::{EdgeArgs, JumpArgsLayout};
    let mut module = fixture();
    let mut leaf = module.functions.remove("Leaf.pick/0").unwrap();
    leaf.signature.name = "Leaf.pick/1".into();
    leaf.signature.params = vec![MirType::Unknown];
    leaf.params = vec![ValueId::new(0)];
    leaf.next_value_id = 2;
    leaf.metadata
        .value_types
        .insert(ValueId::new(0), MirType::Unknown);
    leaf.metadata
        .value_types
        .insert(ValueId::new(1), MirType::Integer);
    let entry = leaf.blocks.get_mut(&BasicBlockId::new(7)).unwrap();
    entry.instructions[0] = I::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    };
    entry.set_terminator(I::Return {
        value: Some(ValueId::new(1)),
    });
    module.canonical_callable_definitions.clear();
    module.canonical_callable_definitions.insert(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Leaf", "pick", 1),
        leaf.signature.name.clone(),
    );
    module.add_function(leaf);
    let mut predicate = MirFunction::new(
        FunctionSignature {
            name: "Predicate.yes/0".into(),
            params: vec![],
            return_type: MirType::Bool,
            effects: EffectMask::from_bits(u16::MAX),
        },
        BasicBlockId::new(8),
    );
    predicate.next_value_id = 1;
    predicate
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Bool);
    let entry = predicate.blocks.get_mut(&BasicBlockId::new(8)).unwrap();
    entry.add_instruction(I::Const {
        dst: ValueId::new(0),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(I::Return {
        value: Some(ValueId::new(0)),
    });
    module.canonical_callable_definitions.insert(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "Predicate",
            "yes",
            0,
        ),
        predicate.signature.name.clone(),
    );
    module.add_function(predicate);
    let main = module.functions.get_mut("main").unwrap();
    if let Some(I::Invoke {
        operation: InvokeOperation::Call { call, .. },
        ..
    }) = &mut main
        .blocks
        .get_mut(&BasicBlockId::new(3))
        .unwrap()
        .terminator
    {
        call.callee = crate::mir::Callee::Global(
            hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
                "Leaf".into(),
                "pick".into(),
                1,
            )
            .unwrap(),
        );
        call.args = vec![ValueId::new(1)];
    } else {
        panic!("original invoke");
    }
    main.next_value_id = 5;
    main.metadata
        .value_types
        .insert(ValueId::new(4), MirType::Bool);
    let normal = main.blocks.get_mut(&BasicBlockId::new(4)).unwrap();
    normal.add_instruction(I::Call(crate::mir::definitions::MirCall::global(
        Some(ValueId::new(4)),
        hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
            "Predicate".into(),
            "yes".into(),
            0,
        )
        .unwrap(),
        vec![],
    )));
    normal.return_env = Some(vec![ValueId::new(2)]);
    normal.return_env_layout = Some(JumpArgsLayout::CarriersOnly);
    normal.set_terminator(I::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(6),
        else_bb: BasicBlockId::new(9),
        then_edge_args: Some(EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        }),
        else_edge_args: Some(EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId::new(2)],
        }),
    });
    for id in [6, 9] {
        let mut block = BasicBlock::new(BasicBlockId::new(id));
        block.set_terminator(I::Return {
            value: Some(ValueId::new(2)),
        });
        main.add_block(block);
    }
    module
}

#[test]
fn canonical_document_transport_retains_unknown_calls_edges_and_return_environment() {
    let module = all_source_fixture();
    let document = emit(&module);
    let definitions = document["canonical_transport_v1"]["callable_definitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                key(&row["key"]).unwrap(),
                document["functions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|f| f["name"] == row["symbol"])
                    .unwrap(),
            )
        })
        .collect::<Definitions<'_>>();
    for function in module.functions.values() {
        let encoded = document["functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["name"] == function.signature.name)
            .unwrap();
        assert_eq!(
            ty(&encoded["metadata"]["canonical_transport_v1"]["signature"]["return_type"]).unwrap(),
            function.signature.return_type
        );
        assert_eq!(
            effects(&encoded["metadata"]["canonical_transport_v1"]["signature"]).unwrap(),
            function.signature.effects
        );
        for block in function.blocks.values() {
            let encoded = encoded["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|b| b["id"] == block.id.as_u32())
                .unwrap();
            for (row, original) in array(encoded, "instructions")
                .unwrap()
                .iter()
                .zip(block.instructions.iter().chain(block.terminator.iter()))
            {
                assert_eq!(&instruction::parse(row, &definitions).unwrap(), original);
            }
        }
    }
    let leaf = document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "Leaf.pick/1")
        .unwrap();
    assert_eq!(leaf["metadata"]["value_types"]["0"], "unknown");
    assert_eq!(
        leaf["metadata"]["canonical_transport_v1"]["signature"]["params"],
        json!(["unknown"])
    );
    assert!(leaf["metadata"]["canonical_transport_v1"]["physical_param_carriers"].is_null());
    let mut conservative = document.clone();
    let leaf = conservative["functions"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|f| f["name"] == "Leaf.pick/1")
        .unwrap();
    leaf["metadata"]["canonical_transport_v1"]["physical_param_carriers"] =
        json!(["existing_callable_i64"]);
    caller(&mut conservative)["blocks"][0]["instructions"][1]["value"] =
        json!({"type":"i64","value":1});
    caller(&mut conservative)["metadata"]["value_types"]["1"] = json!("i64");
    let call = &mut caller(&mut conservative)["blocks"][0]["instructions"][2]["operation"]["call"];
    call["args"] = json!([1]);
    parse_canonical_document_transport_v1(&conservative.to_string()).unwrap();
    assert!(crate::runner::json_v1_bridge::try_parse_v1_to_module(&document.to_string()).is_err());
    let mut allocated = document.clone();
    caller(&mut allocated)["metadata"]["value_types"]["3"] = json!("unknown");
    parse_canonical_document_transport_v1(&allocated.to_string()).unwrap();
    for mutation in 0..7 {
        let mut broken = document.clone();
        let main = caller(&mut broken);
        match mutation {
            0 => main["blocks"][1]["instructions"][2]["then_edge_args"]["layout"] = json!("bogus"),
            1 => {
                main["blocks"][1]["instructions"][2]["then_edge_args"]
                    .as_object_mut()
                    .unwrap()
                    .remove("values");
            }
            2 => main["blocks"][1]["instructions"][2]["else_edge_args"]["values"] = json!([999]),
            3 => {
                main["metadata"]["canonical_transport_v1"]["block_spans"][2]["return_env"] =
                    json!([2])
            }
            4 => {
                main["metadata"]["canonical_transport_v1"]["block_spans"][1]["return_env"] =
                    json!([0])
            }
            5 => main["blocks"][1]["instructions"][1]["call"]["target"]["name"] = json!("pick"),
            _ => {
                main["metadata"]["canonical_transport_v1"]["signature"]["params"] = json!(["bogus"])
            }
        }
        assert!(
            parse_canonical_document_transport_v1(&broken.to_string()).is_err(),
            "AllSource mutation {mutation}"
        );
    }
}
