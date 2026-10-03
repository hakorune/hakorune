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

/// A borrowed nullable result (`return null` / `return new ..`) reached
/// through `local h = recv.m(..)` publishes the sole physical lane: the
/// callee row carries `ordinary_nullable_handle`, the invoke spells
/// `"nullable_handle"`, the callee materializes `const_null`, and the
/// caller owes exactly one `home_release_if_live` — never an unconditional
/// release on a maybe-null handle.
#[test]
fn borrowed_nullable_result_lexical_call_publishes_checked_release() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Item { id: i64 serial: i64 birth(id, serial) { me.id = id me.serial = serial } } \
            box Store { limit: i64 birth() { me.limit = 10 } \
            check(p) { if p > me.limit { return null } return new Item(p, 3) } } \
            static box Main { main() { local s = new Store() local h = s.check(5) return 0 } }";
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
                classify_pretransform_report(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                let functions = json["functions"].as_array().unwrap();
                let callee = functions
                    .iter()
                    .find(|f| f["name"] == "Store.check/1")
                    .expect("Store.check/1 published");
                assert_eq!(callee["role"], "ordinary_nullable_handle", "{wire}");
                let callee_instructions: Vec<&serde_json::Value> = callee["blocks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|block| {
                        block["instructions"].as_array().unwrap().iter().chain(
                            std::iter::once(&block["terminator"]["instruction"]),
                        )
                    })
                    .map(|row| row.get("instruction").unwrap_or(row))
                    .collect();
                assert!(
                    callee_instructions
                        .iter()
                        .any(|row| row["op"] == "const_null"),
                    "callee materializes the Void null sentinel: {wire}"
                );
                let caller = functions
                    .iter()
                    .find(|f| f["name"] == "main")
                    .expect("caller row");
                let instructions: Vec<&serde_json::Value> = caller["blocks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .flat_map(|block| {
                        block["instructions"].as_array().unwrap().iter().chain(
                            std::iter::once(&block["terminator"]["instruction"]),
                        )
                    })
                    .map(|row| row.get("instruction").unwrap_or(row))
                    .collect();
                let call = instructions
                    .iter()
                    .find(|row| row["operation"]["kind"] == "ordinary_call")
                    .expect("nullable call edge");
                assert_eq!(call["operation"]["result"], "nullable_handle", "{wire}");
                let checked: Vec<&serde_json::Value> = instructions
                    .iter()
                    .filter(|row| row["operation"]["kind"] == "home_release_if_live")
                    .copied()
                    .collect();
                assert_eq!(
                    checked.len(),
                    1,
                    "exactly one checked release for the nullable result: {wire}"
                );
                let released = checked[0]["operation"]["value"].clone();
                assert!(
                    !instructions
                        .iter()
                        .any(|row| row["operation"]["kind"] == "home_release"
                            && row["operation"]["value"] == released),
                    "a maybe-null result never owes an unconditional release: {wire}"
                );
                Ok(())
            })
            .unwrap();
    });
}

/// The borrowed nullable class is bounded: exits splitting scalar i64 and
/// nullable object forms, or return shapes outside `null`/`new ..`, freeze
/// at the source gate instead of falling back to the scalar lane.
#[test]
fn borrowed_nullable_result_rejects_mixed_and_unproved_returns() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, callee, token) in [
            (
                "mixed",
                "check(p) { if p > me.limit { return null } return 1 }",
                "source-class-mixed",
            ),
            (
                "string-return",
                "check(p) { return \"s\" }",
                "source-not-i64",
            ),
            (
                "bool-return",
                "check(p) { return true }",
                "source-not-i64",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} {callee} }} \
                static box Main {{ main() {{ local s = new Store() local h = s.check(5) return 0 }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input()
                        .map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains(token), "{label}: {error}");
        }
    });
}

/// The `local h = recv.m(..)` lane is the sole admitted caller edge for
/// borrowed nullable results today: `me.`-receiver forwarding inside a
/// nested method and field reads through the received handle stay
/// fail-closed at their own boundaries.
#[test]
fn borrowed_nullable_result_frontiers_stay_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, caller, callee, token) in [
            (
                "me-receiver",
                "main() { local s = new Store() return s.run(5) }",
                "run(q) { local h = me.check(q) return 0 }",
                "artifact-source-unavailable",
            ),
            (
                "forward",
                "main() { local s = new Store() local h = s.run(5) return 0 }",
                "run(q) { local h = me.check(q) if h == null { return 0 } return h.id }",
                "literal-physical-drift",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p) {{ if p > me.limit {{ return null }} return new Item(p, 3) }} {callee} }} \
                static box Main {{ {caller} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input()
                        .map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains(token), "{label}: {error}");
        }
    });
}

/// A terminated `h == null` arm narrows the surviving fall-through: the
/// received nullable binding is proven non-null, its sealed `NullableObject`
/// claim names the field's class authority, and the checked release still
/// discharges the caller's ownership exactly once. Both published shapes —
/// the terminal `return h.id` and the `local v = h.id` initializer —
/// emit `object_field_get` on the invoke result.
#[test]
fn nullable_field_read_publishes_guarded_object_field_get() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, tail) in [
            (
                "terminal",
                "if h == null { return s.limit } return h.id",
            ),
            (
                "initializer",
                "if h == null { return s.limit } local v = h.id return s.limit",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p) {{ if p > me.limit {{ return null }} return new Item(p, 3) }} }} \
                static box Main {{ main() {{ local s = new Store() local h = s.check(5) {tail} }} }}",
            );
            MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                    classify_pretransform_report(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let functions = json["functions"].as_array().unwrap();
                    let caller = functions
                        .iter()
                        .find(|f| f["name"] == "main")
                        .expect("caller row");
                    let instructions: Vec<&serde_json::Value> = caller["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|block| {
                            block["instructions"].as_array().unwrap().iter().chain(
                                std::iter::once(&block["terminator"]["instruction"]),
                            )
                        })
                        .map(|row| row.get("instruction").unwrap_or(row))
                        .collect();
                    let call = instructions
                        .iter()
                        .find(|row| row["operation"]["kind"] == "ordinary_call")
                        .expect("nullable call edge");
                    assert_eq!(
                        call["operation"]["result"], "nullable_handle",
                        "{label}: {wire}"
                    );
                    let result_values: Vec<serde_json::Value> = instructions
                        .iter()
                        .filter(|row| row["op"] == "invoke_normal_result")
                        .map(|row| row["dst"].clone())
                        .collect();
                    let reads: Vec<&serde_json::Value> = instructions
                        .iter()
                        .filter(|row| {
                            row["op"] == "object_field_get"
                                && result_values.contains(&row["base"])
                        })
                        .copied()
                        .collect();
                    assert_eq!(
                        reads.len(),
                        1,
                        "{label}: exactly one field read on the nullable invoke result: {wire}"
                    );
                    let exits = instructions
                        .iter()
                        .filter(|row| row["op"] == "return")
                        .count();
                    let checked: Vec<&serde_json::Value> = instructions
                        .iter()
                        .filter(|row| row["operation"]["kind"] == "home_release_if_live")
                        .copied()
                        .collect();
                    assert_eq!(
                        checked.len(),
                        exits,
                        "{label}: one checked release per exit for the nullable result: {wire}"
                    );
                    assert!(
                        result_values.iter().all(|result| !instructions.iter().any(
                            |row| row["operation"]["kind"] == "home_release"
                                && row["operation"]["value"] == *result
                        )),
                        "{label}: a maybe-null result never owes an unconditional release: {wire}"
                    );
                    Ok(())
                })
                .unwrap_or_else(|error| panic!("{label}: {error}"));
        }
    });
}

/// Unguarded `h.id`, a read inside the null-check arm before its `return`,
/// and an undeclared field all stay fail-closed — the narrow proof comes
/// only from a terminated `== null` arm on the surviving fall-through.
#[test]
fn nullable_field_read_stays_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, tail) in [
            (
                "unguarded",
                "return h.id",
            ),
            (
                "inside-null-arm",
                "if h == null { return h.id } return s.limit",
            ),
            (
                "unknown-field",
                "if h == null { return s.limit } return h.missing",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p) {{ if p > me.limit {{ return null }} return new Item(p, 3) }} }} \
                static box Main {{ main() {{ local s = new Store() local h = s.check(5) {tail} }} }}",
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
                error.contains("IncompleteOrdinaryNewCoverage")
                    || error.contains("artifact-source-unavailable"),
                "{label}: {error}"
            );
        }
    });
}

/// PARAMFIELD census pin: a nullable/object formal's guarded use inside an
/// instance callee stays fail-closed at the observed named terminals until
/// the authority forks recorded in the task-4 card are decided — the
/// unannotated i64 result contract, the main-owner birth-actual root
/// source, the null-compare envelope, the formal field-read class
/// authority, and the null/nullable actual transport.
#[test]
fn parameter_field_frontiers_stay_fail_closed() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (label, callee_sig, callee_body, caller_tail) in [
            (
                "pa-guarded-nullarg",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "return s.release(null)",
            ),
            (
                "pa-guarded-nullarg-cmp",
                "",
                "if handle == null { return 0 } if handle.page_id < 0 { return 0 } return handle.page_id",
                "return s.release(null)",
            ),
            (
                "pa-unguarded-nullarg",
                "",
                "return handle.page_id",
                "return s.release(null)",
            ),
            (
                "pa-guarded-handlearg",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "local h = s.check(5) return s.release(h)",
            ),
            (
                "pa-newarg-unguarded",
                "",
                "return handle.page_id",
                "return s.release(new Handle(1, 2))",
            ),
            (
                "pa-newarg-guarded",
                "",
                "if handle == null { return 0 } return handle.page_id",
                "return s.release(new Handle(1, 2))",
            ),
            (
                "pa-trivial-newarg",
                "",
                "return 0",
                "return s.release(new Handle(1, 2))",
            ),
            (
                "pa-trivial-handlearg",
                "",
                "return 0",
                "local h = s.check(5) return s.release(h)",
            ),
            (
                "pa-eqnull-newarg",
                "",
                "if handle == null { return 0 } return 1",
                "return s.release(new Handle(1, 2))",
            ),
            (
                "pa-trivial-localnewarg",
                "",
                "return 0",
                "local h = new Handle(1, 2) return s.release(h)",
            ),
            (
                "pa-eqnull-localnewarg",
                "",
                "if handle == null { return 0 } return 1",
                "local h = new Handle(1, 2) return s.release(h)",
            ),
            (
                "pa-trivial-localnewarg-bind",
                "",
                "return 0",
                "local h = new Handle(1, 2) local r = s.release(h) return r",
            ),
            (
                "pa-trivial-localnewarg-discard",
                "",
                "return 0",
                "local h = new Handle(1, 2) s.release(h) return 1",
            ),
            (
                "pa-trivial-localnewarg-annotated",
                ": i64",
                "return 0",
                "local h = new Handle(1, 2) local r = s.release(h) return r",
            ),
        ] {
            let text = format!(
                "box Handle {{ page_id: i64 block_id: i64 birth(pid, bid) {{ me.page_id = pid me.block_id = bid }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} \
                check(p) {{ if p > me.limit {{ return null }} return new Handle(p, 3) }} \
                release(handle){callee_sig} {{ {callee_body} }} }} \
                static box Main {{ main() {{ local s = new Store() {caller_tail} }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    classify_pretransform_report(verification);
                    view.issue_lifecycle_physical_abi_input().map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(
                error.contains("freeze:contract")
                    || error.contains("IncompleteOrdinaryNewCoverage")
                    || error.contains("artifact-source-unavailable")
                    || error.contains("BorrowedFormalIngress")
                    || error.contains("LexicalInstanceCall"),
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
