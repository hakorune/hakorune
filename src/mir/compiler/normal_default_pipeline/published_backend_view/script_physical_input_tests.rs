//! Real retained Script inputs and their rejection boundary; no C activation.
use super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;
use crate::mir::instruction::InvokeOperation;
use crate::mir::{ConstValue, MirInstruction};
use serde_json::Value;

fn rows(json: &Value) -> Vec<&Value> {
    json["functions"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|function| {
            function["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|block| {
                    block["instructions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .chain(std::iter::once(&block["terminator"]))
                        .map(|row| &row["instruction"])
                })
        })
        .collect()
}

#[test]
fn script_physical_input_uses_retained_specs_results_and_native_cleanup() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            for (tag, spec) in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"]
                .iter()
                .enumerate()
            {
                for terminal in ["return 30", "return"] {
                    let source = format!("local before = 9\nlocal a: Array<{spec}> = [10, 20]\nlocal alias = a\nlocal b: Array<{spec}> = []\n{terminal}");
                    MirCompiler::with_options(optimize)
                        .compile_normal_with_published(
                            published_request(&source),
                            |view, verification| -> Result<(), String> {
                                assert!(verification.is_ok());
                                let input = view.issue_lifecycle_physical_abi_input()?;
                                assert_eq!(
                                    input.runtime_requirements(),
                                    PublishedLifecycleRuntimeRequirementsV1::NativeArray
                                );
                                assert!(input.layouts().is_empty());
                                assert_eq!(input.storage_profile(), None);
                                assert!(input.entry().births().is_empty());
                                assert!(input.entry().birth_calls().is_empty());
                                assert_eq!(input.entry().array_claims().len(), 2);
                                assert_eq!(input.entry().array_writes().len(), 2);
                                assert_eq!(input.program().functions().len(), 1);
                                assert_eq!(
                                    input.entry().root_result(),
                                    if terminal == "return" {
                                        CompiledEntryRootResultV1::Unit
                                    } else {
                                        CompiledEntryRootResultV1::I64
                                    }
                                );
                                let text = emit_lifecycle_physical_abi_json(&input)?;
                                let json: Value = serde_json::from_str(&text).unwrap();
                                assert_eq!(json["runtime_requirements"]["kind"], "native_array");
                                assert_eq!(json["runtime_requirements"]["abi_version"], 1);
                                assert!(json.get("storage_profile").is_none());
                                assert!(json.get("layouts").is_none());
                                assert_eq!(
                                    json["functions"][0]["role"],
                                    if terminal == "return" {
                                        "root_unit"
                                    } else {
                                        "root_i64"
                                    }
                                );
                                let all = rows(&json);
                                let claims: Vec<_> = all
                                    .iter()
                                    .filter(|row| row["operation"]["kind"] == "array_claim")
                                    .collect();
                                assert_eq!(claims.len(), 2);
                                assert!(claims
                                    .iter()
                                    .all(|row| row["operation"]["element_tag"] == tag + 1));
                                assert_eq!(
                                    all.iter()
                                        .filter(|row| row["operation"]["kind"] == "array_new")
                                        .count(),
                                    2
                                );
                                assert_eq!(
                                    all.iter()
                                        .filter(|row| row["operation"]["kind"] == "array_append")
                                        .count(),
                                    2
                                );
                                assert_eq!(
                                    all.iter()
                                        .filter(|row| row["op"] == "fault_frame_enter")
                                        .count(),
                                    1
                                );
                                assert_eq!(
                                    all.iter().filter(|row| row["op"] == "return_fault").count(),
                                    4
                                );
                                assert_eq!(
                                    all.iter()
                                        .filter(|row| row["op"] == "array_residence_release")
                                        .count(),
                                    6
                                );
                                assert!(all
                                    .iter()
                                    .filter(|row| row["op"] == "invoke")
                                    .all(|row| row["operation"]["site"].is_u64()));
                                let generic =
                                    PublishedMirBackendView::try_new(view.module()).unwrap();
                                assert!(generic.issue_lifecycle_physical_abi_input().is_err());
                                assert_eq!(
                                    view.route(),
                                    PublishedStaticMethodRouteV1::UnsupportedBeforeObject
                                );
                                Ok(())
                            },
                        )
                        .unwrap();
                }
            }
        }
    });
}

#[test]
fn script_physical_write_representation_preserves_distinct_primitive_payloads() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            MirCompiler::with_options(optimize)
                .compile_normal_with_published(
                    published_request("local a: Array<i64> = [7, true, 1.25, 0.0]\nreturn 30"),
                    |view, _| -> Result<(), String> {
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let json: Value =
                            serde_json::from_str(&emit_lifecycle_physical_abi_json(&input)?)
                                .unwrap();
                        let all = rows(&json);
                        let representations: Vec<_> = all
                            .iter()
                            .filter(|row| row["operation"]["kind"] == "array_append")
                            .map(|row| row["operation"]["representation"].as_str().unwrap())
                            .collect();
                        assert_eq!(representations, ["i64", "bool", "f64", "f64"]);
                        let bits: Vec<_> = all
                            .iter()
                            .filter(|row| row["op"] == "const_f64_bits")
                            .map(|row| row["bits"].as_u64().unwrap())
                            .collect();
                        assert_eq!(bits, [1.25f64.to_bits(), 0.0f64.to_bits()]);
                        assert!(all
                            .iter()
                            .any(|row| row["op"] == "const_bool" && row["value"] == true));
                        Ok(())
                    },
                )
                .unwrap();
        }
    });
}

#[test]
fn script_physical_input_rejects_foreign_missing_and_drifted_handoff_correspondence() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(true)
            .compile_normal_with_published(
                published_request("local a: Array<i64> = [10, 0.0]\nreturn 30"),
                |view, _| -> Result<(), String> {
                    let handoff = view.retained_handoff.unwrap();
                    for mutation in 0..9 {
                        let mut module = view.module().clone();
                        let root = module.functions.get_mut(handoff.root_key()).unwrap();
                        match mutation {
                            0 => {
                                root.metadata.typed_array_contract_sources[0]
                                    .element_spec
                                    .element =
                                    crate::typed_array_contract_spec::ExactArrayElementType::U8
                            }
                            1 | 2 => {
                                let row = root
                                    .blocks
                                    .values_mut()
                                    .flat_map(|body| &mut body.instructions)
                                    .find(|row| {
                                        matches!(
                                            row,
                                            MirInstruction::Const {
                                                value: ConstValue::Float(_),
                                                ..
                                            }
                                        )
                                    })
                                    .unwrap();
                                let MirInstruction::Const {
                                    value: ConstValue::Float(value),
                                    ..
                                } = row
                                else {
                                    unreachable!()
                                };
                                *value = if mutation == 1 {
                                    -0.0
                                } else {
                                    f64::from_bits(0x7ff8_0000_0000_0042)
                                };
                            }
                            3 | 4 => {
                                let row = root
                                    .blocks
                                    .values_mut()
                                    .filter_map(|body| body.terminator.as_mut())
                                    .find(|row| {
                                        matches!(
                                            row,
                                            MirInstruction::Invoke {
                                                operation: InvokeOperation::IntrinsicArrayNew,
                                                ..
                                            }
                                        )
                                    })
                                    .unwrap();
                                let MirInstruction::Invoke {
                                    fault_frame,
                                    normal_landing,
                                    fault_landing,
                                    ..
                                } = row
                                else {
                                    unreachable!()
                                };
                                if mutation == 3 {
                                    *fault_frame = crate::mir::ValueId::new(u32::MAX);
                                } else {
                                    *normal_landing = *fault_landing;
                                }
                            }
                            5 => {
                                let row = root
                                    .blocks
                                    .values_mut()
                                    .flat_map(|body| &mut body.instructions)
                                    .find(|row| {
                                        matches!(row, MirInstruction::ArrayResidenceRelease { .. })
                                    })
                                    .unwrap();
                                let MirInstruction::ArrayResidenceRelease { value } = row else {
                                    unreachable!()
                                };
                                *value = crate::mir::ValueId::new(u32::MAX);
                            }
                            6 => {
                                let row = root
                                    .blocks
                                    .values_mut()
                                    .filter_map(|body| body.terminator.as_mut())
                                    .find(|row| matches!(row, MirInstruction::Return { .. }))
                                    .unwrap();
                                *row = MirInstruction::Return { value: None };
                            }
                            7 => {
                                root.blocks
                                    .get_mut(&root.entry_block)
                                    .unwrap()
                                    .instructions
                                    .push(MirInstruction::ArrayResidenceRelease {
                                        value: crate::mir::ValueId::new(u32::MAX),
                                    });
                            }
                            _ => {
                                root.signature.name = "foreign-root".into();
                            }
                        }
                        let result = PublishedMirBackendView::try_new(&module)
                            .map_err(|error| error.to_string())
                            .and_then(|generic| generic.bind_finalized_root_handoff(Some(handoff)))
                            .and_then(|bound| bound.issue_lifecycle_physical_abi_input());
                        assert!(result.is_err(), "mutation {mutation} accepted");
                    }
                    Ok(())
                },
            )
            .unwrap();
    });
}
