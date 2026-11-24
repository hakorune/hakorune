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

/// Compatibility-mode loops lack a parser-issued `FunctionDeclaration`.
/// The raw Loop entry rejects them with the named checked-source diagnostic
/// before emitting MIR.
#[test]
fn compatibility_loop_uses_legacy_child_terminal_without_callable_scope() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            compatibility_root("local i = 0 loop(i < 1) { i = i + 1 } return i"),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("compatibility Loop must be refused when the checked source route is unavailable");
    let error = rejected.error().to_string();
    assert!(
        error.contains("[raw-loop-child-entry/checked-source-route-unavailable]"),
        "{error}"
    );
    rejected.discard();
}

fn assert_scan_requires_checked_loop_route(source: &str) {
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
                .expect_err("untyped comparison must not lower through an alternate route");
            let error = rejected.error().to_string();
            assert!(
                error.contains("[raw-loop-child-entry/checked-source-route-unavailable]")
                    && error.contains("function=Scan.run/2"),
                "{error}"
            );
            rejected.discard();
        },
    );
}

/// The checked loop requires evidence for the comparison operands.
#[test]
fn untyped_loop_comparison_requires_checked_source_route() {
    run_on_test_thread("untyped-loop-comparison", || {
        assert_scan_requires_checked_loop_route(
            "static box Scan { run(i, limit) { local x = i loop(x < limit) { x = x + 1 } return x } } static box Main { main() { return 0 } }",
        );
    });
}

/// Two loop-carried bindings do not supply evidence for the operands.
#[test]
fn untyped_loop_comparison_with_two_carriers_requires_checked_source_route() {
    run_on_test_thread("untyped-loop-two-carriers", || {
        assert_scan_requires_checked_loop_route(
            "static box Scan { run(i, limit) { local acc = 1 local n = i loop(n < limit) { acc = acc * 2 n = n + 1 } return acc } } static box Main { main() { return 0 } }",
        );
    });
}

/// A static call in the body does not supply evidence for the condition.
#[test]
fn untyped_loop_comparison_with_body_call_requires_checked_source_route() {
    run_on_test_thread("untyped-loop-body-call", || {
        assert_scan_requires_checked_loop_route(
            "static box Scan { bump(x) { return x + 1 } run(i, limit) { local x = i loop(x < limit) { x = Scan.bump(x) } return x } } static box Main { main() { return 0 } }",
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
