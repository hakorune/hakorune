//! Original source through production lowering, final handoff and ABI writer.
//! Immutable source loans remain authority across the selected continuations.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};

fn request(text: &str) -> NormalCompileRequestV1 {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    else {
        panic!("source-backed")
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(
        source,
        None,
        std::collections::HashMap::new(),
    )
}

#[test]
fn borrowed_original_source_publishes_all_continuations_and_closed_domains() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (domain, argument, prefix, kind) in [
            ("zero", "0", "", 1),
            ("negative", "-7", "", 1),
            ("true", "true", "", 2),
            ("false", "false", "", 2),
            ("object", "recv", "", 3),
            ("scalar", "n", "local n = 8", 1),
            ("bool-scalar", "n", "local n = true", 2),
        ] {
            for shape in ["local", "discard", "return", "nested"] {
                let continuation = match shape {
                    "local" => format!("local result = recv.probe({argument}) return 7"),
                    "discard" => format!("recv.probe({argument}) return 7"),
                    "return" => format!("return recv.probe({argument})"),
                    _ => format!("return recv.wrap(recv.probe({argument}))"),
                };
                let text = format!("box Transport {{ flag: i64 birth() {{ me.flag = 0 }} probe(p): i64 {{ local scratch = new Transport() return 7 }} wrap(q: i64): i64 {{ return q }} }} static box Main {{ main() {{ local recv = new Transport() {prefix} {continuation} }} }}");
                MirCompiler::with_options(false).compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                    classify_pretransform_report(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let callee = functions.iter().find(|f| f["name"] == "Transport.probe/1").unwrap();
                    assert_eq!(callee["params"][0]["representation"], "borrowed_kind_payload_v1");
                    let calls: Vec<_> = functions.iter().flat_map(|f| f["blocks"].as_array().unwrap()).filter_map(|b| {
                        let op = &b["terminator"]["instruction"]["operation"];
                        (op["kind"] == "ordinary_call").then_some(op)
                    }).collect();
                    assert_eq!(calls.len(), if shape == "nested" { 2 } else { 1 });
                    assert!(calls.iter().any(|op| op["call"]["args"][0]["kind"] == kind));
                    for row in input.entry().ordinary_calls() {
                        let borrowed = input.program().functions()[row.function_index() as usize].name() == "Transport.probe/1";
                        assert_eq!(row.borrowed_actuals().is_some(), borrowed);
                    }
                    reject_erased_carriers(&input);
                    reject_changed_producers(&input);
                    std::fs::write(std::env::temp_dir().join(format!("hako-issued-borrowed-ingress-{shape}-{domain}.json")), wire).unwrap();
                    Ok(())
                }).unwrap_or_else(|error| panic!("{shape}/{domain}: {error}"));
            }
        }
    });
}

#[test]
fn borrowed_original_forwarded_copy_and_entry_receiver_publish_without_producer_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, prefix, argument, kind) in [
            (
                "forwarded",
                "local alias = q",
                "alias",
                serde_json::json!("tagged"),
            ),
            ("entry-receiver", "", "me", serde_json::json!(3)),
        ] {
            let text = format!("box Transport {{ flag: i64 birth() {{ me.flag = 0 }} probe(p): i64 {{ local scratch = new Transport() return 7 }} forward(q): i64 {{ {prefix} local recv = new Transport() local out = recv.probe({argument}) return 7 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.forward(true) return 7 }} }}");
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let forward = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "Transport.forward/1")
                            .unwrap();
                        assert!(forward["blocks"].as_array().unwrap().iter().any(|b| b
                            ["terminator"]["instruction"]["operation"]["call"]["args"][0]["kind"]
                            == kind));
                        reject_erased_carriers(&input);
                        assert!(
                            reject_changed_producers(&input) > 0,
                            "original Copy producer"
                        );
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-borrowed-ingress-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}

fn reject_erased_carriers(input: &super::super::PublishedLifecyclePhysicalAbiInputV1<'_>) {
    for replacement in [
        None,
        Some(&[Carrier::ExistingCallableI64, Carrier::ExistingCallableI64][..]),
    ] {
        let mut program = input.program().clone();
        let mut count = 0;
        for function in &mut program.functions {
            if function
                .param_carriers
                .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue))
            {
                function.param_carriers = replacement;
                count += 1;
            }
        }
        assert!(count > 0);
        let error = super::super::physical_program_json::emit_lifecycle_physical_program_value(
            &program,
            Some(input),
        )
        .unwrap_err();
        assert!(error.contains("borrowed-carrier-function-drift"), "{error}");
    }
}

fn reject_changed_producers(
    input: &super::super::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> usize {
    let mut copies = 0;
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block.instructions().iter().enumerate() {
                let changed = match row.instruction() {
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Integer(_),
                    } => MirInstruction::Const {
                        dst: *dst,
                        value: ConstValue::Integer(999),
                    },
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Bool(flag),
                    } => MirInstruction::Const {
                        dst: *dst,
                        value: ConstValue::Bool(!flag),
                    },
                    MirInstruction::Copy { dst, .. } => {
                        copies += 1;
                        MirInstruction::Copy {
                            dst: *dst,
                            src: ValueId(998),
                        }
                    }
                    _ => continue,
                };
                let mut program = input.program().clone();
                program.functions[fi].blocks[bi].instructions[ri].instruction = &changed;
                let error =
                    super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program,
                        Some(input),
                    )
                    .unwrap_err();
                assert!(
                    error.contains("borrowed-carrier-projection-drift"),
                    "{error}"
                );
            }
        }
    }
    copies
}

/// Preserve the existing historical report; final strict verification is a
/// separate mandatory gate before this callback can publish any ABI input.
fn classify_pretransform_report(
    report: &Result<(), Vec<crate::mir::verification_types::VerificationError>>,
) {
    if let Err(errors) = report {
        assert!(!errors.is_empty());
        assert!(
            errors.iter().all(|error| error
                .to_string()
                .contains("[freeze:contract][mir/invoke/call-argument-type-drift]")),
            "unexpected pretransform failure: {errors:?}"
        );
    }
}

/// The borrowed tagged carrier keeps its physical identity while the
/// checked-compare view lends exactly one Normal-Integer operand read: the
/// `me.<numeric field>` sibling resolves through the entry-receiver proof,
/// and the edge-port model may evaluate the same projection more than once.
#[test]
fn checked_compare_view_publishes_from_original_source() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, sibling, argument) in [
            ("hi", "me.limit", "15"),
            ("lo", "me.limit", "5"),
            ("neg", "me.limit", "-1"),
            ("bool", "me.limit", "true"),
            ("object", "me.limit", "c"),
            ("literal", "10", "15"),
        ] {
            let text = format!(
                "box Counter {{ limit: usize birth() {{ me.limit = 10 }} \
                 check(requested): i64 {{ if requested > {sibling} {{ return 7 }} \
                 return 3 }} }} static box Main {{ main() {{ \
                 local c = new Counter() return c.check({argument}) }} }}"
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let callee = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "Counter.check/1")
                            .unwrap();
                        assert!(callee["params"].as_array().unwrap().iter().any(|p| {
                            p["representation"] == "borrowed_kind_payload_v1"
                        }));
                        let compares: Vec<_> = callee["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|b| b["instructions"].as_array().unwrap())
                            .filter(|row| row["instruction"]["op"] == "compare")
                            .collect();
                        // One source compare; the edge-port model evaluates
                        // the same projection twice.
                        assert_eq!(compares.len(), 2, "{suffix}");
                        assert!(compares.iter().all(|row| {
                            row["instruction"]["predicate"] == "sgt"
                        }));
                        reject_erased_carriers(&input);
                        reject_drifted_compare_view(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-checked-compare-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}

/// The dominated ordered-Add view reads the same lent Normal-Integer
/// projection inside the checked compare's dominance cone; the fresh i64
/// result writes back through the checked usize store lane, whose emitted
/// range check is the sole write-side authority.
#[test]
fn dominated_add_view_publishes_from_original_source() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, argument) in [
            ("hi", "5"),
            ("over", "20"),
            ("neg", "-1"),
            ("bool", "true"),
            ("object", "c"),
        ] {
            let text = format!(
                "box Counter {{ limit: usize total: usize \
                 birth() {{ me.limit = 10 me.total = 0 }} \
                 check(requested): i64 {{ if requested > me.limit {{ return 0 }} \
                 me.total = me.total + requested return 1 }} }} static box Main {{ \
                 main() {{ local c = new Counter() return c.check({argument}) }} }}"
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let callee = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "Counter.check/1")
                            .unwrap();
                        assert!(callee["params"].as_array().unwrap().iter().any(|p| {
                            p["representation"] == "borrowed_kind_payload_v1"
                        }));
                        let rows: Vec<_> = callee["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|b| b["instructions"].as_array().unwrap())
                            .collect();
                        let compares: Vec<_> = rows
                            .iter()
                            .filter(|row| row["instruction"]["op"] == "compare")
                            .collect();
                        // One source compare; the edge-port model evaluates
                        // the same projection twice.
                        assert_eq!(compares.len(), 2, "{suffix}");
                        assert!(compares.iter().all(|row| {
                            row["instruction"]["predicate"] == "sgt"
                        }));
                        let adds: Vec<_> = rows
                            .iter()
                            .filter(|row| row["instruction"]["op"] == "add")
                            .collect();
                        assert_eq!(adds.len(), 2, "{suffix}");
                        let stores: Vec<_> = rows
                            .iter()
                            .filter(|row| row["instruction"]["op"] == "field_set")
                            .collect();
                        assert_eq!(stores.len(), 1, "{suffix}");
                        let store = stores[0]["instruction"].clone();
                        assert!(store["site"].is_u64(), "{suffix}");
                        assert_eq!(
                            store["exact_numeric_runtime_check"]["kind"],
                            "dynamic_integer_range",
                            "{suffix}"
                        );
                        assert_eq!(
                            store["exact_numeric_runtime_check"]["declared_type"],
                            "usize",
                            "{suffix}"
                        );
                        reject_erased_carriers(&input);
                        reject_drifted_add_view(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-add-view-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}

/// The lent operand projection is pinned to its carrier: rewiring the view
/// copy to any other source is projection drift, never a silently widened
/// or dropped admitted use.
fn reject_drifted_compare_view(input: &super::super::PublishedLifecyclePhysicalAbiInputV1<'_>) {
    let mut views = 0;
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block.instructions().iter().enumerate() {
                let MirInstruction::Copy { dst, .. } = row.instruction() else {
                    continue;
                };
                let operand = *dst;
                let feeds_compare = block.instructions().iter().any(|candidate| {
                    matches!(
                        candidate.instruction(),
                        MirInstruction::Compare { lhs, rhs, .. }
                            if *lhs == operand || *rhs == operand
                    )
                });
                if !feeds_compare {
                    continue;
                }
                let mut program = input.program().clone();
                let changed = MirInstruction::Copy {
                    dst: operand,
                    src: ValueId(998),
                };
                program.functions[fi].blocks[bi].instructions[ri].instruction = &changed;
                let error =
                    super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program,
                        Some(input),
                    )
                    .unwrap_err();
                assert!(
                    error.contains("borrowed-carrier-projection-drift"),
                    "{error}"
                );
                views += 1;
            }
        }
    }
    assert!(views > 0, "checked-compare view copies present");
}

/// `me.sizes.set(index, requested)` reads the same lent Normal-Integer view
/// as the dominating compare: the ArrayBox receiver is proven by its own
/// resolver route, the index stays a committed i64, and the kernel call owns
/// bounds/element faults at the exact site. `index == len` is the array's
/// own append-at-end contract; index 3 on an empty array is the bounds arm.
#[test]
fn dominated_set_view_publishes_from_original_source() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, index, argument) in [
            ("ok", "0", "5"),
            ("over", "0", "20"),
            ("oob", "3", "5"),
            ("bool", "0", "true"),
            ("object", "0", "c"),
        ] {
            let text = format!(
                "box Store {{ limit: usize sizes: ArrayBox = new ArrayBox() \
                 birth() {{ me.limit = 10 }} \
                 check(requested): i64 {{ if requested > me.limit {{ return 0 }} \
                 me.sizes.set({index}, requested) return 1 }} }} static box Main {{ \
                 main() {{ local s = new Store() local c = new Store() return s.check({argument}) }} }}"
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let callee = json["functions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|f| f["name"] == "Store.check/1")
                            .unwrap();
                        assert!(callee["params"].as_array().unwrap().iter().any(|p| {
                            p["representation"] == "borrowed_kind_payload_v1"
                        }));
                        let rows: Vec<_> = callee["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|b| b["instructions"].as_array().unwrap())
                            .collect();
                        let compares: Vec<_> = rows
                            .iter()
                            .filter(|row| row["instruction"]["op"] == "compare")
                            .collect();
                        assert_eq!(compares.len(), 2, "{suffix}");
                        let sets: Vec<_> = rows
                            .iter()
                            .filter(|row| row["instruction"]["op"] == "array_set")
                            .collect();
                        assert_eq!(sets.len(), 1, "{suffix}");
                        let set = sets[0]["instruction"].clone();
                        assert!(set["site"].is_u64(), "{suffix}");
                        assert!(set["array"].is_u64(), "{suffix}");
                        assert!(set["index"].is_u64(), "{suffix}");
                        assert!(set["value"].is_u64(), "{suffix}");
                        reject_erased_carriers(&input);
                        reject_drifted_set_view(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-set-view-{suffix}.json")),
                            wire,
                        )
                        .unwrap();
                        Ok(())
                    },
                )
                .unwrap_or_else(|error| panic!("{suffix}: {error}"));
        }
    });
}

/// The dominated `.set` element value is pinned to its carrier: rewiring the
/// element operand to any other value is projection drift, never a silently
/// widened or dropped admitted use.
fn reject_drifted_set_view(input: &super::super::PublishedLifecyclePhysicalAbiInputV1<'_>) {
    let mut views = 0;
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block.instructions().iter().enumerate() {
                let MirInstruction::ArrayElementWrite {
                    site_id,
                    dst,
                    kind: crate::mir::ArrayElementWriteKind::Set,
                    producer,
                    receiver,
                    index,
                    ..
                } = row.instruction()
                else {
                    continue;
                };
                let changed = MirInstruction::ArrayElementWrite {
                    site_id: *site_id,
                    dst: *dst,
                    kind: crate::mir::ArrayElementWriteKind::Set,
                    producer: *producer,
                    receiver: *receiver,
                    index: *index,
                    value: ValueId(998),
                };
                let mut program = input.program().clone();
                program.functions[fi].blocks[bi].instructions[ri].instruction = &changed;
                let error =
                    super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program,
                        Some(input),
                    )
                    .unwrap_err();
                assert!(
                    error.contains("borrowed-carrier-projection-drift"),
                    "{error}"
                );
                views += 1;
            }
        }
    }
    assert!(views > 0, "set view element operands present");
}

/// The dominated Add reads the same lent projection: rewiring the view copy
/// that feeds `Add(NormalInteger, NormalInteger)` is projection drift, never
/// a silently widened or dropped admitted use.
fn reject_drifted_add_view(input: &super::super::PublishedLifecyclePhysicalAbiInputV1<'_>) {
    let mut views = 0;
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block.instructions().iter().enumerate() {
                let MirInstruction::Copy { dst, .. } = row.instruction() else {
                    continue;
                };
                let operand = *dst;
                let feeds_add = block.instructions().iter().any(|candidate| {
                    matches!(
                        candidate.instruction(),
                        MirInstruction::BinOp { lhs, rhs, .. }
                            if *lhs == operand || *rhs == operand
                    )
                });
                if !feeds_add {
                    continue;
                }
                let mut program = input.program().clone();
                let changed = MirInstruction::Copy {
                    dst: operand,
                    src: ValueId(998),
                };
                program.functions[fi].blocks[bi].instructions[ri].instruction = &changed;
                let error =
                    super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program,
                        Some(input),
                    )
                    .unwrap_err();
                assert!(
                    error.contains("borrowed-carrier-projection-drift"),
                    "{error}"
                );
                views += 1;
            }
        }
    }
    assert!(views > 0, "add view copies present");
}

#[path = "borrowed_source_publication_new_argument_tests.rs"]
mod new_argument_tests;

#[path = "borrowed_source_publication_i64_result_tests.rs"]
mod i64_result_tests;

#[path = "borrowed_source_publication_root_source_tests.rs"]
mod root_source_tests;

#[path = "borrowed_source_publication_null_compare_tests.rs"]
mod null_compare_tests;

#[path = "borrowed_source_publication_nullable_actual_tests.rs"]
mod nullable_actual_tests;

#[path = "borrowed_source_publication_param_field_tests.rs"]
mod param_field_tests;

#[path = "borrowed_source_publication_param_field_acceptance_tests.rs"]
mod param_field_acceptance_tests;

#[path = "borrowed_source_publication_declared_tests.rs"]
mod declared_tests;

#[path = "borrowed_source_publication_field_return_projection_tests.rs"]
mod field_return_projection_tests;
