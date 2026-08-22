use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageInstallIssueV1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(193).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

fn main_fixture() -> String {
    include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako")
        .replace("ParserScanLoopBox", "Main")
        .replace(
            "static box Main {",
            "static box Main {\n  main() { return 0 }",
        )
}

#[test]
fn main_static_children_are_role_sealed_and_root_is_omitted() {
    let package = issue(&main_fixture()).expect("valid source-backed Main expansion");
    let roles = package
        .catalog
        .selected_identities()
        .map(|(_, _, role)| role)
        .collect::<Vec<_>>();
    assert_eq!(roles.len(), 4);
    assert!(roles.iter().all(|role| role.is_main_static_child()));
}

#[test]
fn main_static_child_cannot_use_generic_key_only_admission() {
    let package = issue(&main_fixture()).expect("valid source-backed Main expansion");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Main", "skip_while", 4),
    )
    .expect("catalog admission");
    assert_eq!(
        port.with_selected_cataloged_lowering_input(admission, |_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired)
    );
}

#[test]
fn main_static_child_port_consumes_all_role_rows_once() {
    let package = issue(&main_fixture()).expect("valid source-backed Main expansion");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let result =
        crate::mir::builder::with_test_main_static_children(installed.source_ast(), |children| {
            let mut port = installed
                .begin_lowering(&context)
                .expect("same installed catalog");
            assert_eq!(children.len(), 4);
            for child in children {
                port.with_main_static_child_lowering_input(child, |input| {
                    let (selected, admission) = input.into_lowering_and_admission();
                    assert!(matches!(
                        selected.semantic(),
                        super::SelectedCallableSemanticRefV1::Ordinary
                    ));
                    assert_eq!(admission.source_key().owner(), "Main");
                })
                .expect("typed Main-child Port loan");
            }
            port.complete()
        })
        .expect("Main expansion");
    result.expect("all Main-child rows consumed exactly once");
}

#[test]
fn main_static_child_role_does_not_enter_dynamic_candidate_gate() {
    let package = issue(&main_fixture()).expect("valid source-backed Main expansion");
    assert!(matches!(
        package.dynamic_projection(),
        super::NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert!(
        package
            .batch()
            .declarations()
            .filter(|declaration| declaration.mode()
                == ResolvedCallableDeclarationModeV1::StaticBoxMethod)
            .count()
            >= 4
    );
}
