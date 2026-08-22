use crate::mir::builder::with_common_v2_canonical_session;
use crate::mir::builder::CompilationContext;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::common_v2_session_admission::with_loop_v2_canonical_session_admission;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("admission source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

#[test]
fn admission_co_seals_loop_outer_if_block_expr_envelope_and_completion() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(971).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source(include_str!(
            "../../../apps/tests/scan_with_init_typed_ok_min.hako"
        )),
    )
    .expect("same-cohort package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        with_loop_v2_canonical_session_admission(&loan, |admission| {
            let owner = admission.input().owner();
            assert_eq!(owner, admission.completion().owner());
            assert_eq!(owner, admission.envelope().owner());
            assert_eq!(owner, admission.outer_if().owner());
            assert_eq!(admission.block_expr_expectation().owner(), owner);
            assert_eq!(admission.envelope().coverage().placement_count(), 15);
            assert_eq!(admission.completion().explicit_sites().len(), 2);
            assert_eq!(admission.outer_if().row_count(), 0);
            let seed = loan.envelope().initial_index_seed();
            assert_eq!(seed.owner(), owner);
            assert_eq!(seed.declared_type_name(), Some("i64"));
            assert_eq!(
                seed.literal(),
                &crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(0)
            );
            assert_eq!(seed.index_carrier_entry().raw(), 2);

            with_common_v2_canonical_session(admission, |session| {
                assert_eq!(session.owner(), owner);
                assert!(!session.completion_is_implicit());
                assert_eq!(session.envelope().coverage().placement_count(), 15);
                assert_eq!(session.return_source_binding().owner(), owner);
                assert_eq!(session.return_source_binding().join_exit_item().raw(), 10);
                assert!(session.physical_entry_stamp().is_err());
                assert!(session.issue_length_call_target_plan().is_err());
                assert!(session.issue_length_call_materialization_canary().is_err());
            })
            .expect("canonical session open");
        })
        .expect("callback-scoped admission");
    })
    .expect("common V2 loan");

    assert_eq!(
        port.with_s6c_common_v2_pre_session(|_| ()),
        Err(crate::mir::normal_callable_semantic_package::
            NormalCallableSemanticPackageInstallIssueV1::S6CChildAlreadyConsumed)
    );
}
