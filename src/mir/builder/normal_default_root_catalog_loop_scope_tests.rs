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

#[test]
fn compatibility_loop_uses_legacy_child_terminal_without_callable_scope() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            compatibility_root("local i = 0 loop(i < 1) { i = i + 1 } return i"),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("compatibility Loop must reach its existing legacy terminal");
    let (_, module, _) = completed.into_parts();
    assert!(module.functions.iter().any(|(_, function)| {
        function.signature.name == "main"
            || function
                .blocks
                .values()
                .any(|block| block.instructions.iter().any(|_| true))
    }));
}

#[test]
fn source_backed_loop_keeps_invocation_scope_and_ledger_route() {
    let root = source_backed_root(
        "static box Scan { run(i, limit) { local x = i loop(x < limit) { x = x + 1 } return x } } static box Main { main() { return 0 } }",
    );
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
                .expect_err("source-backed Loop must reach its existing recipe boundary");
            let error = rejected.error().to_string();
            assert!(error.contains("[callable-loop/recipe]"), "{error}");
            assert!(!error.contains("callable-ledger-missing"), "{error}");
            assert!(rejected.session.builder().current_module.is_some());
            rejected.discard();
        },
    );
}

#[test]
fn source_bool_call_feeds_existing_composite_condition() {
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
}
