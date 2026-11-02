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
                require_artifact_verification(verification);
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
fn dominated_new_argument_preserves_checked_caller_representation() {
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
                        require_artifact_verification(verification);
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
                        let signed = matches!(suffix, "ok" | "over");
                        assert_eq!(make["params"][0]["integer_view"] == true, signed,
                            "{suffix}: only Integer callers lend a signed entry");
                        assert_eq!(args[0]["kind"],
                            if signed { serde_json::json!(1) } else { serde_json::json!("tagged") },
                            "{suffix}: the actual preserves the caller's checked representation");
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
    use crate::mir::instruction::{BirthArgumentV1 as Actual, StringInvokeOperand};
    let mut drifted = [0, 0];
    for (fi, function) in input.program().functions().iter().enumerate() {
        for (bi, block) in function.blocks().iter().enumerate() {
            for (ri, row) in block.instructions().iter().chain(std::iter::once(&block.terminator())).enumerate() {
                for text_kind in [false, true] {
                    let mut changed = row.instruction().clone();
                    match &mut changed {
                        MirInstruction::Call(call) if !text_kind
                            && matches!(call.callee, crate::mir::Callee::BirthConstructor { .. })
                            && !call.args.is_empty() => call.args[0] = ValueId(998),
                        MirInstruction::Invoke { operation: InvokeOperation::Birth(call), .. } => {
                            let Some(Actual::Value(value)) = call.arguments.first() else { continue };
                            // Same SSA dependency, different logical kind: Text
                            // must not inherit the original Integer admission.
                            call.arguments[0] = if text_kind { Actual::Text(StringInvokeOperand::BorrowedText(*value)) }
                                else { Actual::Value(ValueId(998)) };
                        }
                        _ => continue,
                    }
                    let mut program = input.program().clone();
                    let block_mut = &mut program.functions[fi].blocks[bi];
                    if ri < block_mut.instructions.len() { block_mut.instructions[ri].instruction = &changed; }
                    else { block_mut.terminator.instruction = &changed; }
                    let error = super::super::super::physical_program_json::emit_lifecycle_physical_program_value(
                        &program, Some(input)).unwrap_err();
                    assert!(error.contains("borrowed-carrier-projection-drift") || error.contains("borrowed-use/"), "{error}");
                    drifted[usize::from(text_kind)] += 1;
                }
            }
        }
    }
    assert!(drifted.iter().all(|count| *count > 0), "value and same-SSA Text mutations must both hit original Birth actuals");
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
                    require_artifact_verification(verification);
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
                require_artifact_verification(verification);
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
                "artifact-source-unavailable",
            ),
        ] {
            let text = format!(
                "box Item {{ id: i64 serial: i64 birth(id, serial) {{ me.id = id me.serial = serial }} }} \
                box Store {{ limit: i64 birth() {{ me.limit = 10 }} {callee} }} \
                static box Main {{ main() {{ local s = new Store() local h = s.check(5) return 0 }} }}",
            );
            let error = MirCompiler::with_options(false)
                .compile_normal_with_published(request(&text), |view, verification| {
                    require_artifact_verification(verification);
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
                "i64-result-mismatch",
            ),
            (
                "forward",
                "main() { local s = new Store() local h = s.run(5) return 0 }",
                "run(q) { local h = me.check(q) if h == null { return 0 } return h.id }",
                "source-not-i64",
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
                    require_artifact_verification(verification);
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
                    require_artifact_verification(verification);
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
                    require_artifact_verification(verification);
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
                    require_artifact_verification(verification);
                    view.issue_lifecycle_physical_abi_input()
                        .map(|_| ())
                })
                .err()
                .unwrap_or_else(|| panic!("{label}: unexpectedly admitted"));
            assert!(error.contains("actual-kind-unavailable"), "{label}: {error}");
        }
    });
}

/// A borrowed child comes from the original returned wrapper, without its own
/// Home. Both physical optimization modes must preserve its exact argument use.
#[test]
fn returned_wrapper_field_argument_reaches_published_call() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Token {} box Wrapper { child: Token birth(child) { me.child = child } }
            box Maker { make() { return new Token() }
                wrap() { local h = me.make() return new Wrapper(h) }
                consume(h) { if h == null { return 0 } return 1 } }
            static box Main { main() { local maker = new Maker()
                local w = maker.wrap() maker.consume(w.child) return 0 } }";
        for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(
                request(text), |view, verification| -> Result<(), String> {
                    require_artifact_verification(verification);
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let main = json["functions"].as_array().unwrap().iter()
                        .find(|row| row["name"] == "main").unwrap();
                    let instructions: Vec<_> = main["blocks"].as_array().unwrap().iter()
                        .flat_map(|block| block["instructions"].as_array().unwrap().iter()
                            .chain(std::iter::once(&block["terminator"]["instruction"])))
                        .map(|row| row.get("instruction").unwrap_or(row)).collect();
                    let reads: Vec<_> = instructions.iter().filter(|row| row["op"] == "object_field_get").collect();
                    assert_eq!(reads.len(), 1, "{wire}");
                    let child = &reads[0]["dst"];
                    assert!(instructions.iter().any(|row| row["operation"]["kind"] == "ordinary_call"
                        && row["operation"]["call"]["args"].as_array().is_some_and(|args|
                            args.iter().any(|arg| arg.get("value").unwrap_or(arg) == child))), "{wire}");
                    Ok(())
                }).unwrap_or_else(|issue| panic!("optimize={optimize}: {issue}"));
        }
    });
}

#[test]
fn declared_read_result_keeps_local_support_through_field_publication() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for n in [0, 1] {
            let source = format!("box Leaf {{ value: i64 birth() {{ me.value = 7 }} maybe(n: i64, p: Leaf): Leaf from p {{ if n == 0 {{ return null }} return p }} }} static box Main {{ main() {{ local p = new Leaf() local q = p.maybe({n}, p) return 7 }} }}");
            MirCompiler::with_options(false).compile_normal_with_published(request(&source), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-nullable-{n}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable capture {n}: {issue}"));
            let guarded = source.replace("return 7", "if q == null { return 7 } return 9");
            MirCompiler::with_options(false).compile_normal_with_published(request(&guarded), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-guarded-{n}.json")), wire).unwrap();
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable comparison {n}: {issue}"));
            let guarded_field = source.replace("return 7", "if q == null { return 0 } return q.value");
            for optimize in [false, true] {
            MirCompiler::with_options(optimize).compile_normal_with_published(request(&guarded_field), |view, verification| -> Result<(), String> {
                require_artifact_verification(verification);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                std::fs::write(std::env::temp_dir().join(format!("hako-issued-declared-read-guarded-field-{n}.json")), wire).unwrap();
                let mut changed = view.module().clone();
                let mut comparisons = 0;
                for block in changed.functions.get_mut("main").unwrap().blocks.values_mut() {
                    for row in &mut block.instructions {
                        if let MirInstruction::Compare { op, .. } = row {
                            *op = crate::mir::CompareOp::Ne;
                            comparisons += 1;
                        }
                    }
                }
                assert_eq!(comparisons, 1);
                let handoff = view.retained_handoff.ok_or("handoff missing")?;
                assert!(crate::mir::verification::MirVerifier::new()
                    .verify_module_with_finalized_root(&changed, Some(handoff)).is_err(),
                    "the independent verifier must reject the swapped nullable guard");
                Ok(())
            }).unwrap_or_else(|issue| panic!("nullable field {n} optimize={optimize}: {issue}"));
            }
            for (label, tail) in [
                ("unguarded", "return q.value"),
                ("support-mutated", "if q == null { return 0 } p.value = 8 return q.value"),
                ("null-arm-rejoins", "if q == null { local ignored = 0 } return q.value"),
            ] {
                let unsafe_read = source.replace("return 7", tail);
                assert!(MirCompiler::with_options(false).compile_normal_with_published(request(&unsafe_read), |_, _| Ok(())).is_err(), "{label} must not gain nullable field permission");
            }
        }
        for (text, symbol, receiver, ordinal, artifact) in [
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } } static box Main { main() { local p = new Leaf() local q = p.identity(p) return q.value } }", "Leaf.identity/1", false, 0, "hako-issued-declared-read-local.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(p: Leaf): Leaf from p { return p } forward(n: i64, p: Leaf): Leaf from p { local q = me.identity(p) return q } } static box Main { main() { local p = new Leaf() local q = p.forward(0, p) return q.value } }", "Leaf.forward/2", false, 1, "hako-issued-declared-read-forward.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } } static box Main { main() { local p = new Leaf() local q = p.identity() return q.value } }", "Leaf.identity/0", true, 0, "hako-issued-declared-read-receiver.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } identity(): Leaf from me { return me } forward(): Leaf from me { local q = me.identity() return q } } static box Main { main() { local p = new Leaf() local q = p.forward() return q.value } }", "Leaf.forward/0", true, 0, "hako-issued-declared-read-receiver-forward.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } } box Parent { child: Leaf marker: i64 birth(child) { me.child = child me.marker = 0 } identity(): Leaf from me { return me.child } } static box Main { main() { local child = new Leaf() local p = new Parent(child) local q = p.identity() return q.value } }", "Parent.identity/0", true, 0, "hako-issued-declared-read-field.json"),
            ("box Leaf { value: i64 birth() { me.value = 7 } } box Parent { child: Leaf marker: i64 birth(child) { me.child = child me.marker = 0 } get(): Leaf from me { return me.child } forward(): Leaf from me { local q = me.get() return q } } static box Main { main() { local child = new Leaf() local p = new Parent(child) local q = p.forward() return q.value } }", "Parent.forward/0", true, 0, "hako-issued-declared-read-field-forward.json"),
        ] {
        MirCompiler::with_options(artifact.ends_with("-forward.json")).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
            require_artifact_verification(verification);
            let input = view.issue_lifecycle_physical_abi_input()?;
            let wire = super::super::super::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
            let functions = json["functions"].as_array().unwrap();
            let (target, callee) = functions.iter().enumerate().find(|(_, function)| function["name"] == symbol).unwrap();
            assert_eq!(callee["role"], "ordinary_borrowed_nullable_handle");
            assert_eq!(callee["result_read"], true);
            if receiver {
                assert_eq!(callee["result_support"], serde_json::json!({"kind":"receiver"}));
                assert!(callee.get("result_formal_ordinal").is_none());
            } else { assert_eq!(callee["result_formal_ordinal"], ordinal); }
            let calls: Vec<_> = functions.iter().flat_map(|function| function["blocks"].as_array().unwrap())
                .map(|block| &block["terminator"]["instruction"]["operation"])
                .filter(|op| op["kind"] == "ordinary_call" && op["call"]["target"] == target).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0]["result_read"], true);
            assert_eq!(calls[0]["result"], "borrowed_nullable_handle");
            // Erasing both qualifiers remains internally consistent, but must
            // disagree with the ORIGINAL From declaration retained by input.
            let original_call = input.program().functions().iter().filter(|f| f.name() == "main").flat_map(|f| f.blocks())
                .map(|block| block.terminator().instruction()).find(|row| matches!(row,
                    crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                        result: crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
                            access: crate::mir::instruction::BorrowedResultAccessV1::ReadOnly, .. }, .. }, .. })).unwrap();
            let mut erased_call = original_call.clone();
            if let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call {
                result: crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { access, .. }, .. }, .. } = &mut erased_call {
                *access = crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified;
            }
            let mut changed = input.program().clone();
            for function in changed.functions.iter_mut() {
                if let PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryBorrowedNullableHandle { access, read_result, .. } = &mut function.role {
                    *access = crate::mir::instruction::BorrowedResultAccessV1::LegacyUnqualified;
                    *read_result = None;
                }
                for block in function.blocks.iter_mut() {
                    if std::ptr::eq(block.terminator.instruction(), original_call) { block.terminator.instruction = &erased_call; }
                }
            }
            let erased_result = match &erased_call {
                crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { result, .. }, .. } => *result,
                _ => unreachable!(),
            };
            let (ordinal, callee) = changed.functions.iter().enumerate().find(|(_, function)| function.name() == symbol).unwrap();
            assert_eq!(callee.role().ordinary_result(), Some(erased_result), "both downgraded descriptors deliberately agree");
            assert!(super::super::super::physical_program_json::corroborate_borrowed_function_snapshot(
                callee, ordinal as u32, &input).unwrap_err().contains("borrowed-carrier-function-drift"));
            let issue = super::super::super::physical_program_json::emit_lifecycle_physical_program_value(
                &changed, Some(&input)).unwrap_err();
            assert!(issue.contains("borrowed-carrier-projection-drift"), "{issue}");
            std::fs::write(std::env::temp_dir().join(artifact), wire).unwrap();
            Ok(())
        }).unwrap_or_else(|issue| panic!("{symbol}: {issue}"));
        let forwarding = match artifact {
            "hako-issued-declared-read-forward.json" => Some(("local q = me.identity(p) return q", "local q = me.identity(p) p.value = 9 return q", "forward(n: i64, p: Leaf): Leaf from p", "forward(n: i64, p: Leaf)")),
            "hako-issued-declared-read-receiver-forward.json" => Some(("local q = me.identity() return q", "local q = me.identity() me.value = 9 return q", "forward(): Leaf from me", "forward()")),
            "hako-issued-declared-read-field-forward.json" => Some(("local q = me.get() return q", "local q = me.get() me.marker = 9 return q", "forward(): Leaf from me", "forward()")),
            _ => None,
        };
        if let Some((body, mutated, declaration, unqualified)) = forwarding {
            for source in [text.replace(body, mutated), text.replace(declaration, unqualified)] {
                assert!(MirCompiler::with_options(false).compile_normal_with_published(request(&source), |_, _| Ok(())).is_err(), "invalid forwarding source accepted: {source}");
            }
        }
        for (label, tail, refusal) in [
            ("write-result", "q.value = 9 return 0", "[freeze:contract]"),
            ("write-support-before-read", "p.value = 9 return q.value", "[freeze:contract]"),
            ("move-support-before-read", "local moved = move p return q.value", "[freeze:contract]"),
            ("move-read-result", "local moved = move q return 0", "MoveUnsupported"),
        ] {
            let tail = if symbol.starts_with("Parent.") && label == "write-support-before-read" { "p.marker = 9 return q.value" } else { tail };
            let source = text.replace("return q.value", tail);
            let issue = MirCompiler::with_options(false).compile_normal_with_published(request(&source),
                |_, _| Ok(())).err().expect(label);
            assert!(issue.to_string().contains(refusal), "{label}: {issue}");
        }
        }
    });
}
