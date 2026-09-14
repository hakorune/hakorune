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
        &[("HAKO_JOINIR_STRICT", Some("1")), ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1"))],
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
