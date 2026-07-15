use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

use super::*;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn call(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn function(call_name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "countdown".into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(call(call_name, arguments))),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn call_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]))
}

fn argument_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Argument(0),
    ]))
}

fn nested_call_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::IfThen(0),
    ]))
}

fn resolve_callable(
    tree: &ASTNode,
) -> Result<VerifiedResolvedCallableForestV1, ResolveOwnerForestErrorV1> {
    let views = CallableFunctionSyntaxViewV1::from_function_ast(tree).unwrap();
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_forest_with_root_callable(views)
}

#[test]
fn seals_exact_self_call_target_and_argument_use() {
    let tree = function("countdown", vec![variable("n")]);
    let unit = resolve_callable(&tree).unwrap();
    let root = unit.forest().roots()[0];
    let function = unit.forest().owner(root).unwrap();
    let target = function.direct_call_target(&call_site()).unwrap();

    assert_eq!(target.callable().owner(), root);
    assert_eq!(
        unit.callable_index().only_header().callable(),
        target.callable()
    );
    assert!(matches!(
        function.variable_ref(&argument_site()),
        Some(ResolvedLexicalRefV1::Local(_))
    ));
    assert_eq!(function.direct_call_targets().count(), 1);
}

#[test]
fn preserves_exact_nested_if_call_site() {
    let mut tree = function("countdown", vec![variable("n")]);
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        unreachable!()
    };
    *body = vec![ASTNode::If {
        condition: Box::new(variable("n")),
        then_body: vec![call("countdown", vec![variable("n")])],
        else_body: None,
        span: Span::unknown(),
    }];
    let unit = resolve_callable(&tree).unwrap();
    let root = unit.forest().roots()[0];

    assert_eq!(
        unit.forest()
            .owner(root)
            .unwrap()
            .direct_call_target(&nested_call_site())
            .unwrap()
            .callable()
            .owner(),
        root
    );
}

#[test]
fn exact_lookup_rejects_wrong_name_wrong_arity_and_physical_spelling() {
    let wrong_name = function("other", vec![variable("n")]);
    assert!(matches!(
        resolve_callable(&wrong_name),
        Err(ResolveOwnerForestErrorV1::Function(
            ResolveFunctionErrorV1::CallableLookup(CallableLookupErrorV1::MissingExactSourceKey)
        ))
    ));

    let wrong_arity = function("countdown", Vec::new());
    assert!(matches!(
        resolve_callable(&wrong_arity),
        Err(ResolveOwnerForestErrorV1::Function(
            ResolveFunctionErrorV1::CallableLookup(CallableLookupErrorV1::MissingExactSourceKey)
        ))
    ));

    let physical = function("countdown/1", vec![variable("n")]);
    assert!(matches!(
        resolve_callable(&physical),
        Err(ResolveOwnerForestErrorV1::Function(
            ResolveFunctionErrorV1::CallableLookup(
                CallableLookupErrorV1::PhysicalSymbolSpellingInSource
            )
        ))
    ));
}

#[test]
fn body_only_resolver_stays_disconnected_from_callable_targets() {
    let tree = function("countdown", vec![variable("n")]);
    let forest = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];

    assert_eq!(forest.owner(root).unwrap().direct_call_targets().count(), 0);
}

#[test]
fn source_unit_seal_rejects_foreign_catalog_and_forest_pairing() {
    let tree = function("countdown", vec![variable("n")]);
    let first = resolve_callable(&tree).unwrap();
    let second = resolve_callable(&tree).unwrap();
    let (first_forest, _) = first.into_parts();
    let (_, second_index) = second.into_parts();

    assert!(matches!(
        VerifiedResolvedCallableForestV1::seal(first_forest, second_index),
        Err(ResolvedCallableForestVerificationErrorV1::IndexOwnerIsNotSoleRoot { .. })
    ));
}
