use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    project_source_body_node_v1, ProjectedSourceNodeV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1,
};
use crate::parser::NyashParser;

use super::source_method_call_site::checked_method_call_arity_for_test;
use super::*;

const SOURCE: &str = r#"
static box Helpers {
  run(x) { return x }
}

static box Caller {
  invoke(x) { return Helpers.run(x) }
  nested(x) { return Wrapper.consume(Helpers.run(x)) }
  repeated(x) {
    local first = Helpers.run(x)
    local second = Helpers.run(first)
    return second
  }
}

static box Wrapper {
  consume(x) { return x }
}
"#;

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("source MethodCall fixture must parse")
}

fn catalog(root: &ASTNode) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(root)
        .expect("declaration catalog must seal")
}

fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            method,
            arity,
        )
        .unwrap_or_else(|| panic!("missing declaration {owner}.{method}/{arity}"))
        .key()
        .clone()
}

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn return_value_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

#[test]
fn seals_exact_catalog_caller_body_site_and_method_call() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "Caller", "invoke", 1);
    let call_site = return_value_site();
    let verified =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, call_site.clone()).unwrap();

    assert_eq!(verified.caller(), &caller);
    assert!(std::ptr::eq(
        verified.caller(),
        verified.declaration().key()
    ));
    assert_eq!(verified.site(), &call_site);
    assert_eq!(verified.method(), "run");
    assert_eq!(verified.arity(), 1);
    assert_eq!(verified.arguments().len(), 1);
    assert!(matches!(
        verified.receiver(),
        ASTNode::Variable { name, .. } if name == "Helpers"
    ));
    assert!(matches!(verified.expression(), ASTNode::MethodCall { .. }));
    assert_eq!(
        verified.receiver_site(),
        &site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Receiver,
        ])
    );
    let Some(ProjectedSourceNodeV1::Node(projected_receiver)) = project_source_body_node_v1(
        verified.declaration().body(),
        verified.receiver_site().node(),
    ) else {
        panic!("derived receiver site must project through the same catalog body");
    };
    assert!(std::ptr::eq(projected_receiver, verified.receiver()));
}

#[test]
fn same_relative_site_is_bound_to_each_catalog_caller_body() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let call_site = return_value_site();
    let invoke = key(&declarations, "Caller", "invoke", 1);
    let nested = key(&declarations, "Caller", "nested", 1);

    let invoke_site =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &invoke, call_site.clone()).unwrap();
    let nested_site =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &nested, call_site).unwrap();

    assert_eq!(invoke_site.method(), "run");
    assert!(matches!(
        invoke_site.receiver(),
        ASTNode::Variable { name, .. } if name == "Helpers"
    ));
    assert_eq!(nested_site.method(), "consume");
    assert!(matches!(
        nested_site.receiver(),
        ASTNode::Variable { name, .. } if name == "Wrapper"
    ));
    assert!(!std::ptr::eq(
        invoke_site.expression(),
        nested_site.expression()
    ));
}

#[test]
fn distinguishes_nested_and_repeated_same_spelled_calls_by_site() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let nested_caller = key(&declarations, "Caller", "nested", 1);
    let nested_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Argument(0),
    ]);
    let nested =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &nested_caller, nested_site.clone())
            .unwrap();
    assert_eq!(nested.method(), "run");
    assert_eq!(nested.site(), &nested_site);

    let repeated_caller = key(&declarations, "Caller", "repeated", 1);
    let first_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let second_site = site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let first =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &repeated_caller, first_site.clone())
            .unwrap();
    let second = VerifiedSourceMethodCallSiteV1::verify(
        &declarations,
        &repeated_caller,
        second_site.clone(),
    )
    .unwrap();
    assert_eq!((first.method(), first.arity()), ("run", 1));
    assert_eq!((second.method(), second.arity()), ("run", 1));
    assert_ne!(first.site(), second.site());
}

#[test]
fn actual_string_helpers_accepts_only_the_exact_digit_value_site() {
    let root = parse(include_str!(concat!(
        "../../../lang/src/shared/common/",
        "string_helpers.hako"
    )));
    let declarations = catalog(&root);
    let caller = key(&declarations, "StringHelpers", "to_i64", 1);
    let exact_site = site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::LoopBody(2),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let verified =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, exact_site.clone()).unwrap();
    assert_eq!(verified.site(), &exact_site);
    assert_eq!(verified.method(), "_digit_value");
    assert_eq!(verified.arity(), 1);
    assert!(matches!(verified.receiver(), ASTNode::Me { .. }));

    let old_handwritten_site = site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::Value,
    ]);
    assert_eq!(
        VerifiedSourceMethodCallSiteV1::verify(
            &declarations,
            &caller,
            old_handwritten_site.clone(),
        )
        .unwrap_err(),
        SourceMethodCallSiteErrorV1::SiteOutsideCallerBody {
            caller,
            site: old_handwritten_site,
        }
    );
}

#[test]
fn actual_parser_string_utils_binds_skip_ws_to_its_catalog_body() {
    let source = format!(
        "{}\n{}",
        include_str!(concat!(
            "../../../lang/src/shared/common/",
            "string_helpers.hako"
        )),
        include_str!(concat!(
            "../../../lang/src/compiler/parser/scan/",
            "parser_string_utils_box.hako"
        )),
    );
    let root = parse(&source);
    let declarations = catalog(&root);
    let caller = key(&declarations, "ParserStringUtilsBox", "skip_ws", 2);
    let exact_site = return_value_site();
    let verified =
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, exact_site.clone()).unwrap();

    assert_eq!(verified.caller(), &caller);
    assert_eq!(verified.site(), &exact_site);
    assert_eq!(verified.method(), "skip_ws");
    assert_eq!(verified.arity(), 2);
    assert!(matches!(
        verified.receiver(),
        ASTNode::Variable { name, .. } if name == "StringHelpers"
    ));
}

#[test]
fn rejects_foreign_missing_and_non_method_call_sites() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "Caller", "invoke", 1);

    let foreign_root = parse("static box Foreign { invoke(x) { return x } }");
    let foreign_declarations = catalog(&foreign_root);
    let foreign = key(&foreign_declarations, "Foreign", "invoke", 1);
    assert_eq!(
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &foreign, return_value_site(),)
            .unwrap_err(),
        SourceMethodCallSiteErrorV1::CallerOutsideCatalog { caller: foreign }
    );

    let missing = site(vec![SourcePathSegmentV1::Body(99)]);
    assert_eq!(
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, missing.clone())
            .unwrap_err(),
        SourceMethodCallSiteErrorV1::SiteOutsideCallerBody {
            caller: caller.clone(),
            site: missing,
        }
    );

    let statement = site(vec![SourcePathSegmentV1::Body(0)]);
    assert_eq!(
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, statement.clone())
            .unwrap_err(),
        SourceMethodCallSiteErrorV1::MethodCallRequired {
            caller,
            site: statement,
        }
    );
}

#[test]
fn rejects_a_nested_lambda_call_as_the_outer_catalog_caller() {
    let mut root = parse(SOURCE);
    let ASTNode::Program { statements, .. } = &mut root else {
        panic!("fixture root must be Program");
    };
    let caller_box = statements
        .iter_mut()
        .find(|node| matches!(node, ASTNode::BoxDeclaration { name, .. } if name == "Caller"))
        .expect("Caller box must exist");
    let ASTNode::BoxDeclaration { methods, .. } = caller_box else {
        unreachable!();
    };
    let mut compatibility = std::mem::take(methods).into_compatibility_map();
    let invoke = compatibility
        .get_mut("invoke")
        .expect("invoke method must exist");
    let ASTNode::FunctionDeclaration { body, .. } = invoke else {
        panic!("invoke must be a function declaration");
    };
    *body = vec![ASTNode::Return {
        value: Some(Box::new(ASTNode::Lambda {
            params: Vec::new(),
            body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::MethodCall {
                    object: Box::new(ASTNode::Variable {
                        name: "Helpers".into(),
                        span: crate::ast::Span::unknown(),
                    }),
                    method: "run".into(),
                    arguments: vec![ASTNode::Variable {
                        name: "x".into(),
                        span: crate::ast::Span::unknown(),
                    }],
                    span: crate::ast::Span::unknown(),
                })),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        })),
        span: crate::ast::Span::unknown(),
    }];
    *methods = crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(compatibility);

    let declarations = catalog(&root);
    let caller = key(&declarations, "Caller", "invoke", 1);
    let nested_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
    ]);
    assert_eq!(
        VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, nested_site.clone())
            .unwrap_err(),
        SourceMethodCallSiteErrorV1::SiteCrossesNestedCallableBoundary {
            caller,
            site: nested_site,
        }
    );
}

#[test]
fn checked_arity_rejects_values_outside_u32() {
    assert_eq!(checked_method_call_arity_for_test(0), Ok(0));
    assert_eq!(
        checked_method_call_arity_for_test(u32::MAX as usize),
        Ok(u32::MAX)
    );
    if usize::BITS > u32::BITS {
        assert_eq!(
            checked_method_call_arity_for_test(u32::MAX as usize + 1),
            Err(())
        );
    }
}

#[test]
fn declaration_reorder_preserves_the_normalized_site_view() {
    let left = parse(SOURCE);
    let right = parse(&format!(
        "{}\n{}",
        "static box Wrapper { consume(x) { return x } }",
        "static box Helpers { run(x) { return x } }\nstatic box Caller { invoke(x) { return Helpers.run(x) } }"
    ));
    let left_catalog = catalog(&left);
    let right_catalog = catalog(&right);
    let left_caller = key(&left_catalog, "Caller", "invoke", 1);
    let right_caller = key(&right_catalog, "Caller", "invoke", 1);
    let left_site =
        VerifiedSourceMethodCallSiteV1::verify(&left_catalog, &left_caller, return_value_site())
            .unwrap();
    let right_site =
        VerifiedSourceMethodCallSiteV1::verify(&right_catalog, &right_caller, return_value_site())
            .unwrap();
    assert_eq!(left_site.caller(), right_site.caller());
    assert_eq!(left_site.site(), right_site.site());
    assert_eq!(left_site.method(), right_site.method());
    assert_eq!(left_site.arity(), right_site.arity());
}
