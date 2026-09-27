use super::super::normal_default_program_root::PreparedNormalDefaultProgramRootV1;
use super::super::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
    ModuleBuilderInvocationSessionV1, NormalRuntimeInputSnapshotV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn session() -> ModuleBuilderInvocationSessionV1 {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
    ModuleBuilderInvocationSessionV1::open(&current, config)
}

fn compatibility_root(source: &str) -> PreparedNormalDefaultProgramRootV1 {
    let ast = NyashParser::parse_from_string(source).expect("compatibility source");
    PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program root")
}

/// Source-backed lowering recurses deeply enough in debug builds to exceed
/// the default 8 MiB test-thread stack; run those pins on a widened thread
/// the same way the callable pipeline loop pins do.
fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}

fn source_backed_root(source: &str) -> PreparedNormalDefaultProgramRootV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("source-backed source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    PreparedNormalDefaultProgramRootV1::from_callable_source(source)
}

/// Compatibility-mode loops carry no parser-issued `FunctionDeclaration`, so
/// they cannot satisfy the route_loop lowering-session contract after the
/// registry retirement. This is the accepted non-corpus compatibility
/// boundary: the loop must end at the named freeze, never at a silent
/// fallback or partial MIR.
#[test]
fn compatibility_loop_uses_legacy_child_terminal_without_callable_scope() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            compatibility_root("local i = 0 loop(i < 1) { i = i + 1 } return i"),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("compatibility Loop must reach its named route_loop terminal");
    let error = rejected.error().to_string();
    assert!(
        error.contains("route_loop requires a parser-issued FunctionDeclaration"),
        "{error}"
    );
    rejected.discard();
}

fn lower_source_backed(source: &str) -> crate::mir::MirModule {
    let root = source_backed_root(source);
    crate::test_support::with_env_vars(
        &[
            ("HAKO_JOINIR_STRICT", Some("1")),
            ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
        ],
        || {
            crate::runtime::ring0::ensure_global_ring0_initialized();
            let completed = session()
                .complete_normal_default_program_root_catalog_lifecycle(
                    root,
                    CallableMainMaterializationPolicyV1::Omitted,
                    NormalRuntimeInputSnapshotV1::empty(),
                )
                .expect("call-free scalar LoopCond must finish lowering");
            let (_, module, _) = completed.into_parts();
            module
        },
    )
}

#[cfg(feature = "vm-reference")]
fn run_scan(module: &crate::mir::MirModule, i: i64, limit: i64) -> i64 {
    let key = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Scan.run/2")
        .map(|(key, _)| key.clone())
        .expect("lowered Scan.run/2");
    let value = crate::backend::MirInterpreter::new()
        .execute_function_with_args(
            module,
            &key,
            &[
                crate::backend::VMValue::Integer(i),
                crate::backend::VMValue::Integer(limit),
            ],
        )
        .expect("Scan.run executes");
    let crate::backend::VMValue::Integer(value) = value else {
        panic!("Scan.run must return an integer, got {value:?}")
    };
    value
}

/// A call-free scalar LoopCond is now covered: the bridge-issued CallFree
/// coverage proof admits the empty call inventory and the existing LoopCond
/// physical owner lowers it — one route, no fallback. Interpreter values pin
/// zero/one/multi iterations and the post-loop PHI carrier read.
#[test]
fn source_backed_loop_keeps_invocation_scope_and_ledger_route() {
    run_on_test_thread("source-backed-loop-ledger-route", || {
        let module = lower_source_backed(
            "static box Scan { run(i, limit) { local x = i loop(x < limit) { x = x + 1 } return x } } static box Main { main() { return 0 } }",
        );
        let function = module
            .functions
            .values()
            .find(|function| function.signature.name == "Scan.run/2")
            .expect("lowered Scan.run/2");
        assert!(
            function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .any(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. })),
            "loop carrier must materialize a PHI: {function}"
        );
        #[cfg(feature = "vm-reference")]
        {
            // Zero iterations: x stays i.
            assert_eq!(run_scan(&module, 5, 3), 5);
            // One iteration.
            assert_eq!(run_scan(&module, 0, 1), 1);
            // Multiple iterations: PHI carrier feeds the post-loop read.
            assert_eq!(run_scan(&module, 0, 4), 4);
            assert_eq!(run_scan(&module, 7, 9), 9);
        }
    });
}

/// Two PHI carriers through the same call-free body — a multiplication
/// rebind beside the counter rebind stays inside the accepted grammar.
#[test]
fn source_backed_call_free_loop_carries_two_bindings() {
    run_on_test_thread("call-free-loop-two-carriers", || {
        let module = lower_source_backed(
            "static box Scan { run(i, limit) { local acc = 1 local n = i loop(n < limit) { acc = acc * 2 n = n + 1 } return acc } } static box Main { main() { return 0 } }",
        );
        let function = module
            .functions
            .values()
            .find(|function| function.signature.name == "Scan.run/2")
            .expect("lowered Scan.run/2");
        assert!(
            function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. }))
                .count()
                >= 2,
            "both loop carriers must materialize PHIs: {function}"
        );
        #[cfg(feature = "vm-reference")]
        {
            let value = crate::backend::MirInterpreter::new()
            .execute_function_with_args(
                &module,
                module
                    .functions
                    .iter()
                    .find(|(_, function)| function.signature.name == "Scan.run/2")
                    .map(|(key, _)| key.as_str())
                    .expect("lowered Scan.run/2"),
                &[
                    crate::backend::VMValue::Integer(0),
                    crate::backend::VMValue::Integer(3),
                ],
            )
            .expect("two-carrier Scan.run executes");
            assert_eq!(value, crate::backend::VMValue::Integer(8));
        }
    });
}

fn reject_source_backed(source: &str) -> String {
    let root = source_backed_root(source);
    crate::test_support::with_env_vars(
        &[
            ("HAKO_JOINIR_STRICT", Some("1")),
            ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
        ],
        || {
            crate::runtime::ring0::ensure_global_ring0_initialized();
            let rejected = session()
                .complete_normal_default_program_root_catalog_lifecycle(
                    root,
                    CallableMainMaterializationPolicyV1::Omitted,
                    NormalRuntimeInputSnapshotV1::empty(),
                )
                .expect_err("uncovered scalar grammar must stop at a named boundary");
            let error = rejected.error().to_string();
            rejected.discard();
            error
        },
    )
}

/// A loop whose condition is not `<` — or whose body holds a compound or
/// non-local assignment — is outside the bounded CallFree grammar: the proof
/// is not issued and the lane keeps a named rejection rather than inferring
/// coverage from the empty call inventory. Each shape pins its observed
/// terminal so a silent pass or a CallFree mint is impossible:
///
/// - `<=` / `-` stay on the LoopCond source lane and end at
///   `SourceItemsMissing`, the unproven-empty-inventory boundary.
/// - `x += 1` records a read+write at one statement site and stops earlier
///   at the loop-handoff `duplicate-source-site` freeze.
/// - `break` leaves NoExitBody, takes the composite lane, and stops at its
///   `source-target-empty` terminal.
#[test]
fn source_backed_loop_outside_call_free_grammar_still_stops_at_source_items() {
    run_on_test_thread("call-free-grammar-negative", || {
        // `<=` condition is outside the accepted grammar.
        let less_equal = reject_source_backed(
            "static box Scan { run(i, limit) { local x = i loop(x <= limit) { x = x + 1 } return x } } static box Main { main() { return 0 } }",
        );
        assert!(
            less_equal.contains("[callable-loop/route-not-front-selected]")
                && less_equal.contains("SourceItemsMissing"),
            "{less_equal}"
        );
        // Compound assignment is not a plain binding rebind.
        let compound = reject_source_backed(
            "static box Scan { run(i, limit) { local x = i loop(x < limit) { x += 1 } return x } } static box Main { main() { return 0 } }",
        );
        assert!(compound.contains("duplicate-source-site"), "{compound}");
        // An exit statement under the loop is outside NoExitBody.
        let exit = reject_source_backed(
            "static box Scan { run(i, limit) { local x = i loop(x < limit) { x = x + 1 break } return x } } static box Main { main() { return 0 } }",
        );
        assert!(exit.contains("source-target-empty"), "{exit}");
        // An unsupported expression row (subtraction) stays uncovered.
        let subtraction = reject_source_backed(
            "static box Scan { run(i, limit) { local x = i loop(x < limit) { x = x - 1 } return x } } static box Main { main() { return 0 } }",
        );
        assert!(
            subtraction.contains("[callable-loop/route-not-front-selected]")
                && subtraction.contains("SourceItemsMissing"),
            "{subtraction}"
        );
    });
}

/// A call inside the loop keeps the WithCalls arm — it must never mint a
/// CallFree route and must reach its own named terminal when the call has no
/// selected same-module target relation.
#[test]
fn source_backed_loop_with_hidden_call_never_mints_call_free() {
    run_on_test_thread("call-free-hidden-call", || {
        let root = source_backed_root(
            "static box Scan { bump(x) { return x + 1 } run(i, limit) { local x = i loop(x < limit) { x = Scan.bump(x) } return x } } static box Main { main() { return 0 } }",
        );
        crate::test_support::with_env_vars(
            &[
                ("HAKO_JOINIR_STRICT", Some("1")),
                ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
            ],
            || {
                crate::runtime::ring0::ensure_global_ring0_initialized();
                let outcome = session().complete_normal_default_program_root_catalog_lifecycle(
                    root,
                    CallableMainMaterializationPolicyV1::Omitted,
                    NormalRuntimeInputSnapshotV1::empty(),
                );
                match outcome {
                    Err(rejected) => {
                        let error = rejected.error().to_string();
                        // A WithCalls loop without a selected relation still
                        // fails through the existing named route terminals —
                        // never SourceItemsMissing, which would mean the call
                        // silently classified as call-free.
                        assert!(
                            error.contains("[callable-loop/route-not-front-selected]"),
                            "{error}"
                        );
                        assert!(!error.contains("SourceItemsMissing"), "{error}");
                        rejected.discard();
                    }
                    Ok(completed) => {
                        let (_, module, _) = completed.into_parts();
                        // If the static call bound through the existing
                        // selected-target evidence, WithCalls lowered it —
                        // either way the loop never claimed CallFree.
                        assert!(
                            module
                                .functions
                                .values()
                                .any(|function| function.signature.name == "Scan.run/2"),
                            "WithCalls lowering must publish Scan.run/2"
                        );
                    }
                }
            },
        );
    });
}

#[test]
fn source_bool_call_feeds_existing_composite_condition() {
    run_on_test_thread("source-bool-call-composite-condition", || {
    let source = r#"static box StringHelpers {
          is_space(ch) { return ch == " " || ch == "\t" || ch == "\n" || ch == "\r" }
          skip_ws(src, i) {
            if src == null { return i }
            local s = "" + src
            local n = s.length()
            local j = i
            loop(j < n) {
              if me.is_space(s.substring(j, j+1)) { j = j + 1 } else { break }
            }
            return j
          }
        }"#;
    let root = source_backed_root(source);
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        // Focused source witness copies the production helper bodies unchanged;
        // the separate merged-parser acceptance retains the complete program.
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                root,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("Bool composite source witness must finish lowering");
        let (_, module, _) = completed.into_parts();
        let function = module
            .functions
            .values()
            .find(|function| function.signature.name == "StringHelpers.skip_ws/2")
            .expect("lowered helper");
        let target = super::super::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "StringHelpers",
            "is_space",
            1,
        )
        .canonical_global_target_v1()
        .unwrap();
        let call = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .find_map(|instruction| match instruction {
                crate::mir::MirInstruction::Call(call)
                    if call.callee == crate::mir::Callee::Global(target.clone()) =>
                {
                    Some(call)
                }
                _ => None,
            })
            .expect("source Bool Call");
        let dst = call.dst.expect("Bool destination");
        assert_eq!(
            function.metadata.value_types.get(&dst),
            Some(&crate::mir::MirType::Bool)
        );
        // Existing condition lowering may copy or compare a Bool before Branch.
        // Follow only these explicit value-preserving/condition operations.
        let mut condition_values = std::collections::BTreeSet::from([dst]);
        loop {
            let before = condition_values.len();
            for instruction in function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
            {
                match instruction {
                    crate::mir::MirInstruction::Copy { dst, src }
                        if condition_values.contains(src) =>
                    {
                        condition_values.insert(*dst);
                    }
                    crate::mir::MirInstruction::Compare { dst, lhs, rhs, .. }
                        if condition_values.contains(lhs) || condition_values.contains(rhs) =>
                    {
                        condition_values.insert(*dst);
                    }
                    _ => {}
                }
            }
            if condition_values.len() == before {
                break;
            }
        }
        assert!(function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction,
            crate::mir::MirInstruction::Branch { condition, .. } if condition_values.contains(condition))),
            "source Bool Call must reach Branch through existing condition lowering: {function}");
    });
    });
}
