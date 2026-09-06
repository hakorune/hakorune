use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    declared_instance_locator::{
        DeclaredInstanceCallLocatorTakeErrorV1, DeclaredInstanceCallPackageLocatorDispositionV1,
    },
    issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageInstallIssueV1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("handoff source");
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

fn admission(key: &CanonicalSameModuleCallableKeyV1) -> NormalCatalogedBoxMethodDraftAdmissionV1 {
    NormalCatalogedBoxMethodDraftAdmissionV1::seal(key.clone()).expect("catalog admission")
}

#[test]
fn selected_static_and_instance_rows_lend_one_signature_sibling() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(951).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source(
            r#"
static box StaticApi {
  run(source: StringBox, needle: StringBox) { return { 0 } }
}
box InstanceApi {
  run(source: StringBox) { return 0 }
}
"#,
        ),
    )
    .expect("selected ordinary package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let static_key =
        CanonicalSameModuleCallableKeyV1::test_static_box_method("StaticApi", "run", 2);
    let instance_key =
        CanonicalSameModuleCallableKeyV1::test_instance_box_method("InstanceApi", "run", 1);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_selected_cataloged_lowering_input_and_signature(
        admission(&static_key),
        |input, signature| {
            input.with_selected_and_admission(|selected, admitted| {
                assert_eq!(selected.source().owner(), signature.owner());
                assert_eq!(selected.block_expr_expectation().pair_count(), 1);
                assert_eq!(
                    admitted.source_key().arity(),
                    signature.source_logical_arity()
                );
                assert_eq!(signature.receiver_lane_count(), 0);
                assert_eq!(signature.physical_formal_lane_count(), 4);
                assert_eq!(signature.physical_callable_lane_count(), 4);
            });
        },
    )
    .expect("static sibling loan");

    port.with_selected_cataloged_lowering_input_and_signature(
        admission(&instance_key),
        |input, signature| {
            input.with_selected_and_admission(|selected, admitted| {
                assert_eq!(selected.source().owner(), signature.owner());
                assert_eq!(
                    admitted.source_key().arity(),
                    signature.source_logical_arity()
                );
                assert_eq!(signature.receiver_lane_count(), 1);
                assert_eq!(signature.physical_formal_lane_count(), 2);
                assert_eq!(signature.physical_callable_lane_count(), 3);
            });
        },
    )
    .expect("instance sibling loan");

    assert_eq!(
        port.with_selected_cataloged_lowering_input_and_signature(
            admission(&static_key),
            |_, _| ()
        ),
        Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey)
    );
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete().expect("all selected rows consumed");
}

fn declared_instance_package() -> super::VerifiedNormalCallableSemanticPackageV1 {
    let mut resolver = FunctionSemanticResolverSessionV1::new(952).expect("resolver");
    issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source("box Counter { call() { return me.value() } value() { return 1 } }"),
    )
    .expect("declared-instance package")
}

fn consume_declared_instance_selected_rows(
    port: &mut super::NormalCallableSemanticPackagePortV1<'_>,
    call_site: Option<&crate::mir::resolved_semantics::OwnedExprSiteV1>,
) {
    for method in ["call", "value"] {
        let key = CanonicalSameModuleCallableKeyV1::test_instance_box_method("Counter", method, 0);
        port.with_selected_cataloged_lowering_input_signature_and_declared_instance_locator(
            admission(&key),
            |input, signature, mut locator| {
                let owner = input.with_selected_and_admission(|selected, admitted| {
                    assert_eq!(selected.source().owner(), signature.owner());
                    assert_eq!(admitted.source_key(), &key);
                    assert_eq!(signature.receiver_lane_count(), 1);
                    selected.source().owner()
                });
                if method == "call" {
                    if let Some(call_site) = call_site {
                        locator
                            .take_exact_relation(call_site, |relation| {
                                assert_eq!(relation.caller_owner(), owner);
                                assert_eq!(relation.call_site(), call_site.site());
                                assert_eq!(relation.receiver_binding().owner(), call_site.owner());
                                Ok(())
                            })
                            .expect("exact call locator");
                        assert_eq!(
                            locator.take_exact_relation(call_site, |_| Ok(())),
                            Err(DeclaredInstanceCallLocatorTakeErrorV1::AlreadyTaken)
                        );
                    }
                }
            },
        )
        .expect("selected instance sibling loan");
    }
}

#[test]
fn declared_instance_selected_handoff_consumes_locator_and_finishes() {
    let package = declared_instance_package();
    let call_site = match package.declared_instance_call_locators() {
        DeclaredInstanceCallPackageLocatorDispositionV1::Published(catalog) => catalog
            .rows()
            .first()
            .expect("one locator row")
            .call_site()
            .clone(),
        DeclaredInstanceCallPackageLocatorDispositionV1::NoRootDeclaredInstanceCall => {
            panic!("declared-instance fixture must publish one locator")
        }
    };
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    consume_declared_instance_selected_rows(&mut port, Some(&call_site));
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete()
        .expect("selected rows and exact locator are consumed once");
}

#[test]
fn declared_instance_selected_handoff_rejects_residual_locator() {
    let package = declared_instance_package();
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    consume_declared_instance_selected_rows(&mut port, None);
    assert_eq!(
        port.complete(),
        Err(NormalCallableSemanticPackageInstallIssueV1::DeclaredInstanceLocatorNotConsumed)
    );
}
