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
