use crate::mir::builder::CompilationContext;
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_v1, InstalledNormalCallableSemanticPackageV1,
    NormalCallableSemanticPackageInstallIssueV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::main0_derived_predicate_root_selection::{
    select_main0_derived_predicate_source_v1, Main0DerivedPredicateSourceSelectionV1,
};

const DERIVED_PREDICATE_SOURCE: &str = r#"
static box Main {
    main() {
        local j = 0
        local m = 0
        local n = 3
        loop(j + m <= n) {
            j = j + 1
        }
        return j
    }
}
"#;

const NON_PROFILE_SOURCE: &str = r#"
static box Main {
    main() {
        local i = 0
        return i
    }
}
"#;

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

fn installed_package(source: &str) -> (InstalledNormalCallableSemanticPackageV1, CompilationContext) {
    let mut resolver = FunctionSemanticResolverSessionV1::new(7001).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
        .expect("source-backed package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    (installed, context)
}

fn app_main_relation(
    context: &CompilationContext,
) -> (
    crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    crate::parser::CallableDeclarationIdentityV1,
) {
    let catalog = context
        .callable_declaration_catalog()
        .expect("installed catalog");
    let app_main = catalog
        .source_backed_app_main()
        .expect("source-backed App Main");
    (
        app_main.catalog_key().clone(),
        app_main.parser_identity().clone(),
    )
}

#[test]
fn selects_exact_profile_without_consuming_the_one_shot_loan() {
    let (installed, context) = installed_package(DERIVED_PREDICATE_SOURCE);
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    let (key, identity) = app_main_relation(&context);

    let selection = select_main0_derived_predicate_source_v1(&port, &key, &identity)
        .expect("selection observes the installed source");
    let Main0DerivedPredicateSourceSelectionV1::Selected(product) = selection else {
        panic!("exact Main0 derived-predicate source must be selected")
    };
    assert_eq!(
        product.operations().core().recipe().as_recipe().loops.len(),
        1,
        "selected product carries the one canonical loop"
    );

    // Observation left the one-shot loan unconsumed: the selected branch can
    // still perform the single consumption for physical lowering.
    let owner = port
        .with_app_main_root_lowering_input(&key, &identity, |input, _| input.owner())
        .expect("loan remains available after observation");
    assert_eq!(owner, product.tail().owner());

    // The consumption is still exactly once.
    assert!(matches!(
        port.with_app_main_root_lowering_input(&key, &identity, |_, _| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::MainRootAlreadyConsumed)
    ));
    // And observation after consumption rejects instead of re-reading a
    // claimed root.
    assert!(matches!(
        port.observe_app_main_root_source_v1(&key, &identity, |_, _| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::MainRootAlreadyConsumed)
    ));
}

#[test]
fn declines_non_profile_source_and_preserves_the_loan() {
    let (installed, context) = installed_package(NON_PROFILE_SOURCE);
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    let (key, identity) = app_main_relation(&context);

    let selection = select_main0_derived_predicate_source_v1(&port, &key, &identity)
        .expect("non-profile source still resolves the root loan");
    assert!(matches!(
        selection,
        Main0DerivedPredicateSourceSelectionV1::Unselected
    ));

    // The declined observation consumed nothing: the legacy hook would still
    // receive its one consumption.
    port.with_app_main_root_lowering_input(&key, &identity, |_, _| ())
        .expect("legacy consumption intact after decline");
}
