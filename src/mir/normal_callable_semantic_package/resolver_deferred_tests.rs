use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::callable_semantic_batch::ResolvedCallableSemanticBatchIssueV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1, ResolveOwnerForestErrorV1,
    ScriptResolverDeferredCauseV1, ScriptResolverDeferredSiteV1, ShadowResolveErrorV0,
    SourceResolverDeferredV1,
};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1,
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    NormalCallableSemanticPackageIssueV1,
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

#[test]
fn constructor_resolver_deferred_keeps_the_exact_parser_source_id() {
    let source = final_source("box Holder { init() { return missing_constructor_value } }");
    let source_id = source
        .with_constructor_semantic_syntax(|loan| {
            loan.rows()
                .first()
                .expect("constructor row")
                .source_id()
                .clone()
        })
        .expect("constructor identity loan");
    let mut resolver = FunctionSemanticResolverSessionV1::new(94).unwrap();
    let deferred = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::InstanceConstructors {
            _error:
                super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1::ResolverDeferred(
                    deferred,
                ),
        }) => deferred,
        other => panic!("expected constructor resolver deferral, got {other:?}"),
    };

    assert_eq!(deferred.len(), 1);
    assert!(deferred
        .first()
        .source()
        .constructor_source_id()
        .expect("constructor source id")
        .same_as(&source_id));
    match deferred.first().observation() {
        SourceResolverDeferredV1::Located {
            cause: ScriptResolverDeferredCauseV1::UnresolvedName { name },
            site: ScriptResolverDeferredSiteV1::Expression(_),
        } => assert_eq!(&**name, "missing_constructor_value"),
        other => panic!("expected located constructor unresolved name, got {other:?}"),
    }
}

#[test]
fn constructor_construction_reject_keeps_the_exact_parser_source_id() {
    let source = final_source(
        "brand PageId: i64\n\
         box Holder { init() { return PageId(1, 2) } }",
    );
    let source_id = source
        .with_constructor_semantic_syntax(|loan| {
            loan.rows()
                .first()
                .expect("constructor row")
                .source_id()
                .clone()
        })
        .expect("constructor identity loan");
    let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(
        source.ast(),
    )
    .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(95).unwrap();
    let reject = match issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&catalog),
    ) {
        Err(NormalCallableSemanticPackageIssueV1::InstanceConstructors {
            _error:
                super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1::Resolver(
                    reject,
                ),
        }) => reject,
        other => panic!("expected constructor construction reject, got {other:?}"),
    };

    assert!(reject
        .source()
        .constructor_source_id()
        .expect("constructor source id")
        .same_as(&source_id));
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::BrandConstructorArity { actual: 2, .. }
        ))
    ));
}

#[test]
fn unissued_direct_call_observation_rejects_package_before_install() {
    let source = final_source("function caller() { return helper() }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(96).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected unissued direct-call package terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn nested_lambda_direct_call_observation_rejects_package_before_install() {
    let source = final_source("function caller() { local f = fn() { return helper() } return 0 }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(97).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected nested unissued direct-call package terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn root_and_nested_direct_call_observations_share_one_package_gate() {
    let source =
        final_source("function caller() { local f = fn() { return helper() } return helper() }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(98).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected mixed-owner package terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn cataloged_static_direct_call_observation_rejects_before_install() {
    let source =
        final_source("static box Api { caller() { return helper() } helper() { return 0 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(99).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected cataloged unissued direct-call terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn cataloged_nested_lambda_direct_call_observation_rejects_before_install() {
    let source = final_source(
        "static box Api { caller() { local f = fn() { return helper() } return 0 } helper() { return 0 } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(100).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected nested cataloged direct-call terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn app_main_owner_forest_relation_is_validated_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return 1 } return 0 } helper() { 2 } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(101).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("valid App Main package");
    let app_main = package
        .declaration_catalog()
        .source_backed_app_main()
        .expect("App Main companion");
    let batch = package.batch();
    let (batch_slot, owner, function_origin, identity) = {
        let mut declarations = batch
            .declarations()
            .filter(|declaration| declaration.identity().same_as(app_main.parser_identity()));
        let declaration = declarations.next().expect("exact Main batch row");
        assert!(declarations.next().is_none());
        assert_eq!(
            declaration.mode(),
            crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::StaticBoxMethod
        );
        assert_eq!(
            declaration.parameter_count(),
            app_main.catalog_key().arity()
        );
        (
            declaration.batch_slot(),
            declaration.owner(),
            declaration.function_origin(),
            declaration.identity().clone(),
        )
    };
    let coherent = batch
        .with_lowering_input_and_source_identity(batch_slot, |input, observed| {
            let forest_owner_count = input.forest().owners().count();
            observed.identity().same_as(&identity)
                && observed.owner() == owner
                && input.owner() == owner
                && input.forest().roots() == std::slice::from_ref(&owner)
                && forest_owner_count >= 2
                && input.function().owner() == owner
                && input.function().function_origin() == function_origin
                && input.function().source_site_inventory().owner() == owner
                && input.function().source_site_inventory().function_origin() == function_origin
        })
        .expect("Main owner/forest relation");
    assert!(coherent);
}

#[test]
fn app_main_direct_call_observation_rejects_before_install() {
    let source =
        final_source("static box Main { main() { return helper() } helper() { return 2 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(102).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected App Main direct-call terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn app_main_nested_direct_call_observation_rejects_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return helper() } return 0 } helper() { return 2 } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(103).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected nested App Main direct-call terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}

#[test]
fn app_main_root_and_nested_direct_call_observations_reject_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return helper() } return helper() } helper() { return 2 } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(104).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        }) => (),
        other => panic!("expected mixed App Main direct-call terminal, got {other:?}"),
    };
    assert_eq!(reject, ());
}
