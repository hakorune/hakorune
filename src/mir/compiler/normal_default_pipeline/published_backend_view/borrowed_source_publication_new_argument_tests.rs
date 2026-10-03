//! Constructor-argument view publication: non-literal i64 birth actuals and
//! the dominated `new`-argument tagged spelling through the lifecycle ABI.
use super::*;

#[test]
fn nonliteral_i64_birth_actuals_publish_integer_payload_tag() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Item { id: i64 serial: i64 birth(id, serial) { me.id = id me.serial = serial } } \
            box Store { limit: i64 birth() { me.limit = 10 } \
            check(p): i64 { if p > me.limit { return 0 } local h = new Item(me.limit, 3) return 1 } \
            } \
            static box Main { main() { local s = new Store() return s.check(1) } }";
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                classify_pretransform_report(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let functions = json["functions"].as_array().unwrap();
                let make = functions
                    .iter()
                    .find(|f| f["name"] == "Store.check/1")
                    .expect("Store.check/1 published");
                let birth_ops: Vec<_> = make["blocks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|block| {
                        block["instructions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|row| &row["instruction"])
                            .chain(std::iter::once(
                                &block["terminator"]["instruction"]["operation"],
                            ))
                    })
                    .filter(|ins| ins["op"] == "birth_call" || ins["kind"] == "birth_call")
                    .collect();
                assert_eq!(birth_ops.len(), 1, "{wire}");
                let args = birth_ops[0]["call"]["args"].as_array().unwrap();
                assert_eq!(args.len(), 2);
                assert_eq!(args[0]["kind"], 1, "me.limit field actual is i64 payload");
                assert_eq!(args[1]["kind"], 1, "i64 formal actual is i64 payload");
                for arg in args {
                    assert!(arg["value"].is_u64(), "{arg}");
                }
                std::fs::write(
                    std::env::temp_dir().join("hako-issued-birth-actual-i64.json"),
                    wire,
                )
                .unwrap();
                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn dominated_new_argument_view_publishes_tagged_birth_actual() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (suffix, argument) in [
            ("ok", "5"),
            ("over", "20"),
            ("bool", "true"),
            ("object", "c"),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p): i64 {{ if p > me.limit {{ return 0 }} local h = new Item(p, 3) return 1 }} \
                }} static box Main {{ main() {{ local s = new Store() local c = new Store() \
                return s.check({argument}) }} }}",
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let wire =
                            super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(
                                &input,
                            )?;
                        let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                        let functions = json["functions"].as_array().unwrap();
                        let make = functions
                            .iter()
                            .find(|f| f["name"] == "Store.check/1")
                            .expect("Store.check/1 published");
                        assert!(make["params"].as_array().unwrap().iter().any(|p| {
                            p["representation"] == "borrowed_kind_payload_v1"
                        }));
                        let birth_ops: Vec<_> = make["blocks"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .flat_map(|block| {
                                block["instructions"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|row| &row["instruction"])
                            })
                            .chain(make["blocks"].as_array().unwrap().iter().map(|block| {
                                &block["terminator"]["instruction"]["operation"]
                            }).filter(|op| op["kind"] == "birth_call").map(|op| op))
                            .filter(|ins| {
                                ins["op"] == "birth_call" || ins["kind"] == "birth_call"
                            })
                            .collect();
                        assert_eq!(birth_ops.len(), 1, "{suffix}: {wire}");
                        let args = birth_ops[0]["call"]["args"].as_array().unwrap();
                        assert_eq!(args.len(), 2, "{suffix}");
                        assert_eq!(
                            args[0]["kind"], "tagged",
                            "{suffix}: dominated new argument spells the tagged pair"
                        );
                        assert_eq!(args[1]["kind"], 1, "{suffix}: literal actual stays i64 payload");
                        for arg in args {
                            assert!(arg["value"].is_u64(), "{suffix}: {arg}");
                        }
                        reject_drifted_ctor_view(&input);
                        std::fs::write(
                            std::env::temp_dir()
                                .join(format!("hako-issued-new-argument-{suffix}.json")),
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

/// The dominated `new`-argument view cannot drift to another operand or lose
/// its admission: mutating the published birth-call actual rejects at the
/// same projection boundary as the compare/add/set lanes.
fn reject_drifted_ctor_view(input: &super::super::super::PublishedLifecyclePhysicalAbiInputV1<'_>) {
    let mut drifted = 0;
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block
                .instructions()
                .iter()
                .chain(std::iter::once(&block.terminator()))
                .enumerate()
            {
                let changed = match row.instruction() {
                    MirInstruction::Call(call)
                        if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }) =>
                    {
                        let mut call = call.clone();
                        if call.args.is_empty() {
                            continue;
                        }
                        call.args[0] = ValueId(998);
                        MirInstruction::Call(call)
                    }
                    MirInstruction::Invoke {
                        operation,
                        fault_frame,
                        normal_landing,
                        fault_landing,
                    } => {
                        let InvokeOperation::Call { call, result } = operation else {
                            continue;
                        };
                        if !matches!(
                            call.callee,
                            crate::mir::Callee::BirthConstructor { .. }
                        ) {
                            continue;
                        }
                        let mut call = call.clone();
                        if call.args.is_empty() {
                            continue;
                        }
                        call.args[0] = ValueId(998);
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call,
                                result: *result,
                            },
                            fault_frame: *fault_frame,
                            normal_landing: *normal_landing,
                            fault_landing: *fault_landing,
                        }
                    }
                    _ => continue,
                };
                let mut program = input.program().clone();
                let block_mut = &mut program.functions[fi].blocks[bi];
                if ri < block_mut.instructions.len() {
                    block_mut.instructions[ri].instruction = &changed;
                } else {
                    block_mut.terminator.instruction = &changed;
                }
                let error =
                    super::super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program,
                        Some(input),
                    )
                    .unwrap_err();
                assert!(
                    error.contains("borrowed-carrier-projection-drift")
                        || error.contains("borrowed-use/"),
                    "{error}"
                );
                drifted += 1;
            }
        }
    }
    assert!(drifted > 0, "birth-call actual rows present");
}

#[test]
fn undominated_new_argument_view_still_rejects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, body) in [
            ("unguarded", "local h = new Item(p, 3) return 1"),
            ("pre-guard", "local h = new Item(p, 3) if p > me.limit { return 0 } return 1"),
            ("inside-arm", "if p > me.limit { local h = new Item(p, 3) return 0 } return 1"),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p): i64 {{ {body} }} \
                }} \
                static box Main {{ main() {{ local s = new Store() return s.check(1) }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input()
                        .map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(
                error.contains("actual-kind-unavailable")
                    || error.contains("borrowed-carrier")
                    || error.contains("borrowed-use")
                    || error.contains("borrowed-formal")
                    || error.contains("call-argument")
                    // An unconsumed borrowed carrier fails the terminal-homes
                    // obligation wrapped in the local-commit artifact freeze.
                    || error.contains("artifact-source-unavailable")
                    // A `new` inside a guarded arm lands on an unowned
                    // lifecycle site — still fail-closed, different stage.
                    || error.contains("artifact-unowned-lifecycle-site"),
                "{label}: {error}"
            );
        }
    });
}

#[test]
fn nonscalar_birth_actuals_still_reject() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, body) in [
            ("null", "local h = new Item(null) return 1"),
            ("handle", "local g = new Item(5) local h = new Item(g) return 1"),
        ] {
            let text = format!(
                "box Item {{ id: i64 birth(id) {{ me.id = id }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p): i64 {{ if p > me.limit {{ return 0 }} {body} }} \
                }} \
                static box Main {{ main() {{ local s = new Store() return s.check(1) }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input()
                        .map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains("actual-kind-unavailable"), "{label}: {error}");
        }
    });
}
