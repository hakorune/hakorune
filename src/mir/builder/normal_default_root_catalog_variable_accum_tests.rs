use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::raw_loop_child_entry::test_route_observation as route;
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}

fn fixture_source() -> crate::mir::builder::PreparedNormalDefaultProgramRootV1 {
    callable_source(
        include_str!("../../../apps/tests/loop_simple_while_inline_explicit_step_min.hako"),
        ParserBuildConfig::default(),
    )
}

#[test]
fn accepted_variable_accum_callable_completes_source_backed_mir_lifecycle() {
    run_on_test_thread("var-accum-mir-lifecycle", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        route::reset();
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                fixture_source(),
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("accepted callable recurrence reaches the source-backed MIR lifecycle");
        let (_, module, _) = completed.into_parts();
        assert!(module
            .functions
            .iter()
            .any(|(_, function)| function.signature.name == "main"));
        assert_eq!(
            route::counts(),
            (1, 0),
            "the accepted source row selects VAR exactly once and never enters the shared non-callable route"
        );
    });
}

#[test]
fn late_var_failure_discards_lowered_invocation_and_fresh_call_succeeds() {
    run_on_test_thread("var-accum-late-failure", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        route::reset();
        route::inject_failure_after_variable_accum_consume_once();

        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                fixture_source(),
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("post-physicalization fault must discard the unpublished invocation");
        assert!(rejected
            .error()
            .to_string()
            .contains("failure-after-source-consume"));
        assert_eq!(
            route::counts(),
            (1, 0),
            "the exact source rows were consumed on VAR and no fallback ran before the injected late fault"
        );
        rejected.discard();

        route::reset();
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                fixture_source(),
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("a fresh invocation succeeds after discarding the failed one");
        let (_, module, _) = completed.into_parts();
        assert!(module
            .functions
            .iter()
            .any(|(_, function)| function.signature.name == "main"));
        assert_eq!(route::counts(), (1, 0));
    });
}
