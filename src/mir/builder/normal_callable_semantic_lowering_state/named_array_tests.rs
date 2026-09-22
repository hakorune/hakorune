use super::*;

use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalRootExecutionConsumerV1,
    SelectedNormalCallableKeyV1,
};
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, SourceExprSiteV1, SourcePathV1,
};
use crate::mir::ValueId;
use crate::parser::{NyashParser, ParserBuildConfig};

const SOURCE: &str = r#"
static box Scan {
  run() {
    local arr = new ArrayBox()
    loop(true) {
      arr.push("text")
    }
    return 0
  }
}
"#;

fn package() -> VerifiedNormalCallableSemanticPackageV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        SOURCE,
        ParserBuildConfig::default(),
    )
    .expect("named-array source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("named-array fixture must remain source-backed")
    };
    let brands = crate::analysis::brand_program_declaration_catalog::
        issue_brand_program_declaration_catalog_v1(source.ast())
        .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(991).expect("resolver");
    issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&brands),
    )
    .expect("named-array package")
}

struct Fixture {
    state: CallableSemanticLoweringState,
    push_site: SourceExprSiteV1,
}

fn fixture() -> Fixture {
    let mut context = CompilationContext::new();
    let installed = package()
        .prepare_install(&mut context)
        .expect("package install")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("lowering loan");
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Scan", "run", 0),
    );
    port.with_selected_lowering_input_and_core_methods(&key, |input, rows, _| {
        let push_site = rows
            .iter()
            .find(|(_, row)| row.contract().named_array_requirement().is_some())
            .map(|(site, _)| site.clone())
            .expect("named ArrayPush row");
        let construction_site = rows
            .get(&push_site)
            .expect("named ArrayPush row")
            .contract()
            .named_array_requirement()
            .expect("ArrayPush construction")
            .construction()
            .clone();
        let mut state =
            CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods(
                input.source(),
                None,
                rows,
            )
            .expect("callable state");

        let local_site = state.locals.keys().next().cloned().expect("ArrayBox local");
        let local_value = ValueId::new(17);
        state
            .record_completed_local(
                &local_site,
                &crate::mir::builder::stmts::CompletedLocalStatementV1::from_parts(
                    local_value,
                    vec![crate::mir::builder::stmts::CompletedLocalBindingV1::new(
                        0,
                        local_value,
                        local_value,
                    )],
                ),
            )
            .expect("local materialization");
        state
            .record_named_array_allocation(&construction_site, local_value)
            .expect("named allocation");

        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("Scan/run/0".to_owned());
        let entry = PreparedCallableEntryValuesV1::static_function(&builder, 0)
            .expect("static entry values");
        state.install_entry_values(&entry).expect("entry install");
        Ok(Fixture { state, push_site })
    })
    .expect("selected named-array loan")
}

#[test]
fn named_array_state_rejects_call_shape_drift() {
    let mut fixture = fixture();
    let error = fixture
        .state
        .take_source_array_push(&fixture.push_site, "append", 1)
        .expect_err("wrong method must freeze");
    assert!(error.contains("named-array-call-shape"), "{error}");
}

#[test]
fn named_array_state_rejects_duplicate_exact_site_consumption() {
    let mut fixture = fixture();
    fixture
        .state
        .take_source_array_push(&fixture.push_site, "push", 1)
        .expect("first ArrayPush take")
        .expect("named ArrayPush row");
    let error = fixture
        .state
        .take_source_array_push(&fixture.push_site, "push", 1)
        .expect_err("second exact take must freeze");
    assert!(
        error.contains("duplicate-core-method-call-consumption"),
        "{error}"
    );
}

#[test]
fn named_array_state_finish_rejects_missing_source_row() {
    let mut fixture = fixture();
    let missing_site = SourcePathV1::root_body(99).expr();
    assert!(fixture
        .state
        .take_source_array_push(&missing_site, "push", 1)
        .expect("missing source lookup")
        .is_none());
    let error = fixture
        .state
        .finish_with_named_arrays()
        .expect_err("required caller must reject missing source row at finish");
    assert!(error.contains("incomplete-consumption"), "{error}");
}

#[test]
fn named_array_state_finish_rejects_write_without_physical_emission() {
    let mut fixture = fixture();
    fixture
        .state
        .take_source_array_push(&fixture.push_site, "push", 1)
        .expect("ArrayPush take")
        .expect("named ArrayPush row");
    let error = fixture
        .state
        .finish_with_named_arrays()
        .expect_err("unemitted named write must freeze");
    assert!(error.contains("write-not-emitted"), "{error}");
}
