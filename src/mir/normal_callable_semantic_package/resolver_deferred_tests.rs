use crate::mir::builder::{CompilationContext, NormalRootExecutionConsumerV1};
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
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => reject,
        other => panic!("expected unissued direct-call package terminal, got {other:?}"),
    };
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::CallableLookup(
            crate::mir::resolved_semantics::CallableLookupErrorV1::MissingExactSourceKey
        ))
    ));
}

#[test]
fn nested_lambda_direct_call_observation_rejects_package_before_install() {
    let source = final_source("function caller() { local f = fn() { return helper() } return 0 }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(97).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => reject,
        other => panic!("expected nested unissued direct-call package terminal, got {other:?}"),
    };
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::DraftInvariant(
            "direct calls require a callable index"
        ))
    ));
}

#[test]
fn root_and_nested_direct_call_observations_share_one_package_gate() {
    let source =
        final_source("function caller() { local f = fn() { return helper() } return helper() }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(98).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => reject,
        other => panic!("expected mixed-owner package terminal, got {other:?}"),
    };
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::CallableLookup(
            crate::mir::resolved_semantics::CallableLookupErrorV1::MissingExactSourceKey
        ))
    ));
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
fn cataloged_typed_static_direct_call_uses_source_index_target() {
    let source = final_source(
        "static box Api { caller(value: i64): i64 { return helper(value) } helper(value: i64): i64 { return value } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(1001).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("typed cataloged direct call has an exact source target");
}

#[test]
fn cataloged_nested_lambda_direct_call_observation_rejects_before_install() {
    let source = final_source(
        "static box Api { caller() { local f = fn() { return helper() } return 0 } helper() { return 0 } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(100).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => reject,
        other => panic!("expected nested cataloged direct-call terminal, got {other:?}"),
    };
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::DraftInvariant(
            "direct calls require a callable index"
        ))
    ));
}

#[test]
fn app_main_owner_forest_relation_is_validated_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return 1 } return 0 } helper(value: i64): i64 { return value } }",
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
fn app_main_direct_call_observation_issues_one_affine_loan() {
    let source = final_source(
        "static box Main { main() { return helper(2) } helper(value: i64): i64 { return value } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(102).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("exact App Main direct-call package");
    assert!(package.has_direct_call_loan());
}

#[test]
fn app_main_qualified_receiver_relation_retains_exact_catalog_row() {
    let source = final_source(
        "static box Helpers { run(value: i64): i64 { return value } }\n\
         static box Main { main() { return Helpers.run(2) } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(108).unwrap();
    let mut package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("qualified App Main package");
    let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        package.declaration_catalog(),
        std::iter::empty::<(String, String)>(),
    )
    .expect("empty invocation import view");
    let relation = package
        .issue_app_main_qualified_receiver_catalog_relation(&imports)
        .expect("qualified receiver relation");
    let relation = relation.expect("source-backed Main relation");
    assert_eq!(relation.rows().len(), 1);
    {
        let row = &relation.rows()[0];
        assert_eq!(row.receiver(), "Helpers");
        assert_eq!(row.canonical_owner(), "Helpers");
        assert_eq!(row.declaration_key().owner(), "Helpers");
        assert_eq!(row.declaration_key().name(), "run");
        assert_eq!(row.declaration_key().arity(), 1);
        assert_eq!(row.selector(), "run");
        assert_eq!(row.arity(), 1);
        assert_eq!(
            row.admission(),
            super::model::QualifiedReceiverCatalogAdmissionV1::DirectCanonicalOwner
        );
    }
    package
        .retain_app_main_qualified_receiver_catalog(relation)
        .expect("owned Main relation retention");
    assert_eq!(
        package
            .app_main_qualified_receiver_catalog
            .as_ref()
            .expect("retained Main relation")
            .rows()
            .len(),
        1
    );
}

#[test]
fn app_main_qualified_receiver_relation_resolves_imported_alias_in_shared_view() {
    let source = final_source(
        "static box Helpers { run(value: i64): i64 { return value } }\n\
         static box Main { main() { return HelpersAlias.run(2) } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(111).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("qualified alias App Main package");
    let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        package.declaration_catalog(),
        [("HelpersAlias".to_owned(), "Helpers".to_owned())],
    )
    .expect("invocation import view");
    let relation = package
        .issue_app_main_qualified_receiver_catalog_relation(&imports)
        .expect("qualified alias relation")
        .expect("source-backed Main relation");
    assert_eq!(relation.rows().len(), 1);
    let row = &relation.rows()[0];
    assert_eq!(row.receiver(), "HelpersAlias");
    assert_eq!(row.canonical_owner(), "Helpers");
    assert_eq!(
        row.admission(),
        super::model::QualifiedReceiverCatalogAdmissionV1::ImportedAlias
    );
}

#[test]
fn app_main_qualified_receiver_relation_rejects_foreign_import_view() {
    let source = final_source(
        "static box Helpers { run(value: i64): i64 { return value } }\n\
         static box Main { main() { return Helpers.run(2) } }",
    );
    let foreign_source = final_source("static box Main { main() { return 0 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(109).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("qualified App Main package");
    let mut foreign_resolver = FunctionSemanticResolverSessionV1::new(110).unwrap();
    let foreign_package =
        issue_normal_callable_semantic_package_v1(&mut foreign_resolver, foreign_source)
            .expect("foreign package");
    let foreign_imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        foreign_package.declaration_catalog(),
        std::iter::empty::<(String, String)>(),
    )
    .expect("foreign import view");
    let error = package
        .issue_app_main_qualified_receiver_catalog_relation(&foreign_imports)
        .expect_err("foreign import view must fail closed");
    assert_eq!(
        &*error,
        "[freeze:contract][mir/main-import-view/catalog-brand]"
    );
}

#[test]
fn app_main_qualified_receiver_package_port_is_one_shot() {
    let source = final_source(
        "static box Helpers { run(value: i64): i64 { return value } }\n\
         static box Main { main() { return Helpers.run(2) } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(112).unwrap();
    let mut package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("qualified App Main package");
    let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
        package.declaration_catalog(),
        std::iter::empty::<(String, String)>(),
    )
    .expect("empty invocation import view");
    let relation = package
        .issue_app_main_qualified_receiver_catalog_relation(&imports)
        .expect("qualified receiver relation")
        .expect("source-backed Main relation");
    package
        .retain_app_main_qualified_receiver_catalog(relation)
        .expect("owned Main relation retention");

    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let first = port
        .take_app_main_qualified_receiver_catalog()
        .expect("first relation take");
    assert_eq!(first.expect("owned relation").rows().len(), 1);
    let error = match port.take_app_main_qualified_receiver_catalog() {
        Ok(_) => panic!("second relation take must fail"),
        Err(error) => error,
    };
    assert_eq!(
        &*error,
        "[freeze:contract][mir/main-qualified-relation/already-taken]"
    );
}

#[test]
fn app_main_without_qualified_receiver_is_unavailable_but_still_one_shot() {
    let source = final_source("static box Main { main() { return 0 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(113).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("plain App Main package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    assert!(port
        .take_app_main_qualified_receiver_catalog()
        .expect("unavailable relation take")
        .is_none());
    let error = match port.take_app_main_qualified_receiver_catalog() {
        Ok(_) => panic!("second unavailable-relation take must fail"),
        Err(error) => error,
    };
    assert_eq!(
        &*error,
        "[freeze:contract][mir/main-qualified-relation/already-taken]"
    );
}

#[test]
fn app_main_direct_call_accepts_top_level_free_function() {
    let source = final_source(
        "function helper(value: i64): i64 { return value }\n\
         static box Main { main() { return helper(2) } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(107).unwrap();
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, source)
        .expect("mixed App Main plus top-level FreeFunction package");
    assert!(package.has_direct_call_loan());
    assert!(package
        .declaration_catalog()
        .declaration(
            &crate::mir::builder::CanonicalSameModuleCallableKeyV1::free_function("helper", 1,)
        )
        .is_some());
}

#[test]
fn app_main_direct_call_wrong_arity_rejects_before_install() {
    let source = final_source(
        "static box Main { main() { return helper() } helper(value: i64): i64 { return value } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(105).unwrap();
    let reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => reject,
        other => panic!("expected wrong-arity App Main direct-call terminal, got {other:?}"),
    };
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::AppMainDirectCall(
            crate::mir::resolved_semantics::AppMainFreeStaticResolverIssueV1::ArityMismatch
        ))
    ));
}

#[test]
fn app_main_non_freestatic_direct_call_rejects_before_install() {
    let source =
        final_source("static box Main { main() { return helper() } helper() { return 1 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(106).unwrap();
    // An unannotated callee seals into the index now; the App Main loan
    // rejects the scalar call lane when the target has no `:i64` header
    // and no sealed Map result.
    assert!(matches!(
        issue_normal_callable_semantic_package_v1(&mut resolver, source),
        Err(NormalCallableSemanticPackageIssueV1::DirectCall {
            _error: super::issuer::DirectCallDispositionIssueV1::Loan(
                super::direct_call_loan::DirectCallLoanErrorV1::LifecycleSourceMismatch
            ),
        })
    ));
}

#[test]
fn app_main_nested_direct_call_observation_rejects_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return helper(2) } return 0 } helper(value: i64): i64 { return value } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(103).unwrap();
    let _reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => assert!(matches!(
            reject.error(),
            ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::AppMainDirectCall(
                crate::mir::resolved_semantics::AppMainFreeStaticResolverIssueV1::NestedOwnerObservation
            ))
        )),
        other => panic!("expected nested App Main direct-call terminal, got {other:?}"),
    };
}

#[test]
fn app_main_root_and_nested_direct_call_observations_reject_before_install() {
    let source = final_source(
        "static box Main { main() { local f = fn() { return helper(2) } return helper(2) } helper(value: i64): i64 { return value } }",
    );
    let mut resolver = FunctionSemanticResolverSessionV1::new(104).unwrap();
    let _reject = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::Resolver(reject),
        }) => assert!(matches!(
            reject.error(),
            ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::AppMainDirectCall(
                crate::mir::resolved_semantics::AppMainFreeStaticResolverIssueV1::NestedOwnerObservation
            ))
        )),
        other => panic!("expected mixed App Main direct-call terminal, got {other:?}"),
    };
}
