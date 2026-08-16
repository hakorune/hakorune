use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageInstallIssueV1,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("S6C source");
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
fn installed_s6c_common_v2_loan_lends_one_cohort_exactly_once() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(602).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source(FIXTURE))
        .expect("same-cohort S6C child");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    port.with_s6c_common_v2_pre_session(|loan| {
        let child = loan.callable();
        assert_eq!(
            child.result(),
            crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64
        );
        assert_eq!(child.signature().source_logical_arity(), 2);
        assert_eq!(child.signature().physical_formal_lane_count(), 4);
        assert_eq!(child.selected().parameter_contracts().count(), 2);
        assert_eq!(
            child.storage_header().params(),
            ["s".to_owned(), "ch".to_owned()]
        );
        assert_eq!(child.storage_header().return_type_name(), Some("i64"));
        assert_eq!(
            child.physical_effects().effect_mask(),
            crate::mir::EffectMask::READ
        );
        assert_eq!(child.physical_effects().external_call_count(), 2);
        assert_eq!(loan.envelope().operations().operation_count(), 13);
        assert_eq!(loan.envelope().control().control_count(), 2);
        assert_eq!(loan.envelope().coverage().placement_count(), 15);
        child.with_completion_parity(|completion| {
            assert!(completion.cleanup_empty());
            assert_eq!(completion.explicit_exit_count(), 2);
        });
    })
    .expect("first child loan");
    assert_eq!(
        port.with_s6c_common_v2_pre_session(|_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::S6CChildAlreadyConsumed)
    );
    port.complete().expect("selected child coverage");
}

#[test]
fn s6c_child_cannot_be_taken_through_generic_key_loan() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(603).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source(FIXTURE))
        .expect("same-cohort S6C child");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Main", "find_ok", 2),
    );
    assert_eq!(
        port.with_selected_lowering_input(&key, |_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired)
    );
    port.with_s6c_child(|child| {
        assert_eq!(
            child.result(),
            crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64
        )
    })
    .expect("typed child loan");
    port.complete().expect("selected child coverage");
}
