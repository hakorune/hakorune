use super::*;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};
use crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1;
use crate::parser::NyashParser;
use std::collections::HashMap;

fn request(source: &str) -> NormalCompileRequestV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("exact callable parse");
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
        .expect("exact callable transform");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source identity must remain intact")
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, HashMap::new())
}

#[test]
fn unannotated_pair_issues_tagged_input_from_retained_contract() {
    use crate::mir::normal_callable_semantic_package::BirthFormalPhysicalDispositionV1;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut block_counts = Vec::new();
        for optimize in [false, true] {
            let mut compiler = MirCompiler::with_options(optimize);
            compiler.compile_normal_with_published(
            request(include_str!("../../../../../apps/typed-object-birth-min/main.hako")),
            |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let entry = view.issue_lifecycle_compiled_entry_contract()?;
                let formals = entry.births()[0].formals();
                assert_eq!(formals.len(), 3);
                assert_eq!(formals[0].disposition(), None);
                for formal in &formals[1..] {
                    assert_eq!(formal.disposition(), Some(
                        BirthFormalPhysicalDispositionV1::UnavailableTaggedOrCheckedRepresentation));
                }
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = emit_lifecycle_physical_abi_json(&input)?;
                let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(decoded["schema"], "hako.published-lifecycle-physical-program.v2");
                assert_eq!(decoded["functions"][1]["params"].as_array().unwrap().len(), 2);
                assert_eq!(decoded["functions"][1]["params"][0]["representation"], "kind_payload_v1");
                let root = input.program().functions().first().unwrap();
                block_counts.push(root.blocks().len());
                if optimize {
                    let release = root.blocks().iter().find(|block| matches!(
                        block.terminator().instruction(), MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::HomeRelease { .. }, .. }
                    )).expect("source Home release");
                    assert!(release.instructions().iter().any(|row| matches!(row.instruction(),
                        MirInstruction::BinOp { op: BinaryOp::Add, .. })), "release must move after Add");
                    let MirInstruction::Invoke { normal_landing, fault_landing, .. }
                        = release.terminator().instruction() else { unreachable!() };
                    assert!(matches!(root.blocks().iter().find(|b| b.id() == *normal_landing)
                        .unwrap().terminator().instruction(), MirInstruction::Return { .. }));
                    assert!(matches!(root.blocks().iter().find(|b| b.id() == *fault_landing)
                        .unwrap().terminator().instruction(), MirInstruction::ReturnFault { .. }));
                }
                // Exact source-issued inputs for both finishing schedules.
                let file = if optimize { "hako-issued-physical-v2-optimized.json" }
                    else { "hako-issued-physical-v2.json" };
                std::fs::write(std::env::temp_dir().join(file), json).unwrap();
                Ok(())
            },
        ).unwrap();
        }
        assert_eq!(
            block_counts[0] - block_counts[1],
            3,
            "three cleanup Jump contractions"
        );
    });
}

#[test]
fn serializer_rejects_nonissued_instruction_vocabulary() {
    let instruction = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Float(1.0),
    };
    assert!(matches!(
        encode_instruction(
            None,
            &instruction,
            &BTreeMap::new(),
            0,
            None,
            None,
            &BTreeMap::new(),
        ),
        Err(error) if error.contains("instruction-unsupported"),
    ));
}

#[test]
fn direct_physical_input_rejects_unit_root_before_c() {
    let source = r#"
box Pair { left: i64 right: i64
  birth(left, right) { me.left = left me.right = right }
}
static box Main { main() { local pair = new Pair(10, 20) return } }
"#;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler.compile_normal_with_published(request(source), |view, _| match view
            .issue_lifecycle_physical_abi_input()
        {
            Err(error) if error.contains("root-result-unavailable") => Ok(()),
            Err(error) => Err(format!("unexpected direct-input rejection: {error}")),
            Ok(_) => Err("Unit root unexpectedly entered direct physical input".into()),
        });
        if let Err(error) = result {
            panic!("{error}");
        }
    });
}

#[test]
fn ordinary_instance_function_publishes_canonical_receiver_object() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(true);
        compiler
            .compile_normal_with_published(
                request(include_str!(
                    "../../../../../apps/typed-object-method-min/main.hako"
                )),
                |view, verification| {
                    assert!(verification.is_ok(), "{verification:?}");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let ordinary = input
                        .program()
                        .functions()
                        .iter()
                        .find(|function| {
                            matches!(
                                function.role(),
                                PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { .. }
                            )
                        })
                        .ok_or_else(|| "ordinary method function missing".to_owned())?;
                    let object = ordinary
                        .role()
                        .receiver_object()
                        .ok_or_else(|| "ordinary method receiver object missing".to_owned())?;
                    let prepared_field = ordinary
                        .blocks()
                        .iter()
                        .flat_map(|block| {
                            block
                                .instructions()
                                .iter()
                                .copied()
                                .chain(std::iter::once(block.terminator()))
                        })
                        .find_map(|row| {
                            matches!(row.instruction(), MirInstruction::FieldGet { .. })
                                .then_some(row.field_ref())
                        })
                        .flatten()
                        .ok_or_else(|| "ordinary method prepared FieldGet missing".to_owned())?;
                    let layout_field = input
                        .layouts()
                        .iter()
                        .find(|layout| layout.object_id() == prepared_field.object().declaration_index())
                        .and_then(|layout| {
                            layout
                                .fields()
                                .iter()
                                .find(|field| field.declaration_ordinal() == prepared_field.declaration_ordinal())
                        })
                        .ok_or_else(|| "prepared FieldGet layout missing".to_owned())?;
                    let json = emit_lifecycle_physical_abi_json(&input)?;
                    let decoded: Value = serde_json::from_str(&json).unwrap();
                    let row = decoded["functions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|row| row["role"] == "ordinary_i64")
                        .ok_or_else(|| "ordinary method JSON row missing".to_owned())?;
                    assert_eq!(row["receiver_object"], object.declaration_index());
                    assert!(row["receiver"].is_number());
                    let json_field = row["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| block["instructions"].as_array().into_iter().flatten())
                        .find(|instruction| instruction["instruction"]["op"] == "object_field_get")
                        .ok_or_else(|| "ordinary method JSON FieldGet missing".to_owned())?;
                    assert_eq!(
                        json_field["instruction"]["object_id"],
                        prepared_field.object().declaration_index()
                    );
                    assert_eq!(
                        json_field["instruction"]["field_ordinal"],
                        prepared_field.declaration_ordinal()
                    );
                    assert_eq!(
                        layout_field.declaration_ordinal(),
                        prepared_field.declaration_ordinal()
                    );
                    Ok::<(), String>(())
                },
            )
            .unwrap();
    });
}

#[test]
fn bool_actuals_keep_kind_and_do_not_specialize_the_birth_definition() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let original = include_str!("../../../../../apps/typed-object-birth-min/main.hako");
        let mut birth = None;
        for optimize in [false, true] {
            for (ordinal, arguments) in ["true, 20", "10, false"].iter().enumerate() {
                let source =
                    original.replace("new Pair(10, 20)", &format!("new Pair({arguments})"));
                MirCompiler::with_options(optimize)
                    .compile_normal_with_published(request(&source), |view, verification| {
                        assert!(verification.is_ok(), "{verification:?}");
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let text = emit_lifecycle_physical_abi_json(&input)?;
                        let json: Value = serde_json::from_str(&text).unwrap();
                        if let Some(expected) = &birth {
                            assert_eq!(expected, &json["functions"][1]);
                        } else {
                            birth = Some(json["functions"][1].clone());
                        }
                        let instructions: Vec<_> = json["functions"][0]["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|block| block["instructions"].as_array().unwrap())
                            .map(|row| &row["instruction"])
                            .collect();
                        let constant = instructions
                            .iter()
                            .find(|row| row["op"] == "const_bool")
                            .unwrap();
                        assert_eq!(constant["value"], ordinal == 0);
                        let args = input.entry().birth_calls()[0].actual().arguments();
                        assert_eq!(
                            super::super::physical_abi::scalar_actual_kind(
                                args[ordinal].source().kind()
                            )?,
                            2
                        );
                        std::fs::write(
                            std::env::temp_dir().join(if optimize {
                                format!("hako-issued-physical-v2-bool-{ordinal}-optimized.json")
                            } else {
                                format!("hako-issued-physical-v2-bool-{ordinal}.json")
                            }),
                            text,
                        )
                        .unwrap();
                        Ok::<(), String>(())
                    })
                    .unwrap();
            }
        }
    });
}

#[test]
fn exact_annotation_remains_unavailable_despite_integer_actual() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = include_str!("../../../../../apps/typed-object-birth-min/main.hako")
            .replace("birth(left, right)", "birth(left: i64, right)");
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(&source), |view, _| {
                assert!(view
                    .issue_lifecycle_physical_abi_input()
                    .unwrap_err()
                    .contains("formal-declaration-unavailable"));
                Ok::<(), String>(())
            })
            .unwrap();
    });
}

#[test]
fn diagnostic_finishing_accepts_the_same_optimized_pair_cleanup() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = include_str!("../../../../../apps/typed-object-birth-min/main.hako");
        let result = MirCompiler::with_options(true)
            .compile_normal(request(source))
            .unwrap();
        assert!(result.module.functions.values().any(|function| {
            function.blocks.values().any(|block| {
                matches!(
                    block.terminator,
                    Some(MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::HomeRelease { .. },
                        ..
                    })
                ) && block.instructions.iter().any(|instruction| {
                    matches!(
                        instruction,
                        MirInstruction::BinOp {
                            op: BinaryOp::Add,
                            ..
                        }
                    )
                })
            })
        }));
    });
}

#[test]
fn native_float_wire_preserves_signed_zero_and_nan_payload_bits() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(true)
            .compile_normal_with_published(
                request("local a: Array<i64> = [0.0]\nreturn 30"),
                |view, _| -> Result<(), String> {
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    // Encoder-only evidence: this does not admit NaN or unary minus in source.
                    for bits in [
                        0u64,
                        0x8000_0000_0000_0000,
                        0x7ff8_0000_0000_0042,
                        0xfff8_0000_0000_0123,
                        0x7ff0_0000_0000_0001,
                    ] {
                        let instruction = MirInstruction::Const {
                            dst: ValueId::new(0),
                            value: ConstValue::Float(f64::from_bits(bits)),
                        };
                        let encoded = encode_instruction(
                            None,
                            &instruction,
                            &BTreeMap::new(),
                            0,
                            None,
                            Some(&input),
                            &BTreeMap::new(),
                        )?;
                        let decoded: Value =
                            serde_json::from_str(&serde_json::to_string(&encoded).unwrap())
                                .unwrap();
                        assert_eq!(decoded["op"], "const_f64_bits");
                        assert_eq!(decoded["bits"].as_u64(), Some(bits));
                    }
                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn map_value_wire_kind_is_explicit_and_has_no_object_identity() {
    use crate::mir::instruction::{MapInvokeOperation as Map, MapValueKind};
    for (kind, wire) in [(MapValueKind::I64, 1), (MapValueKind::Bool, 2)] {
        let op = InvokeOperation::Map(Map::InstallValue {
            map: ValueId(1),
            key: ValueId(2),
            value: ValueId(3),
            kind,
        });
        let encoded =
            encode_invoke(&op, &BTreeMap::new(), &BTreeMap::new(), 0, Some(42), None).unwrap();
        assert_eq!(
            encoded,
            json!({"kind": "map_install_value", "map": 1,
            "key": 2, "value": 3, "value_kind": wire, "site": 42})
        );
    }
}
