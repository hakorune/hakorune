use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::{
    BodyExpressionShapeV1, BodyStatementShapeV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, SourcePathSegmentV1,
};

fn function(body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "shape_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn return_value(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

#[test]
fn resolved_shape_issues_return_and_exact_value_site() {
    let tree = function(vec![return_value(Some(int(0)))], true);
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();

    assert_eq!(product.body_shape().owner(), product.function().owner());
    assert_eq!(
        product.body_shape().body_root(),
        &SourcePathSegmentV1::FunctionBody
    );
    assert!(matches!(
        &product.body_shape().statements()[0],
        BodyStatementShapeV1::Return { value: Some(site), .. }
            if site.node().segments()
                == [SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Value]
    ));
    assert!(product.body_shape().relations().iter().any(|relation| {
        relation.role == SourcePathSegmentV1::Value
            && relation.child.node().segments()
                == [SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Value]
    }));
}

#[test]
fn resolved_shape_seals_me_as_the_lexical_receiver_binding() {
    let tree = function(
        vec![return_value(Some(ASTNode::Me {
            span: Span::unknown(),
        }))],
        false,
    );
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let me_site = product
        .body_shape()
        .expressions()
        .iter()
        .find_map(|row| match row {
            BodyExpressionShapeV1::Me { site, receiver } => Some((site, *receiver)),
            _ => None,
        })
        .expect("Me shape should be present");

    assert_eq!(
        me_site.0.node().segments(),
        [SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Value]
    );
    assert_eq!(
        product
            .function()
            .binding(me_site.1)
            .unwrap()
            .diagnostic_name(),
        "me"
    );
}

#[test]
fn resolved_shape_keeps_empty_body_as_complete_empty_inventory() {
    let tree = function(Vec::new(), true);
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();

    assert!(product.body_shape().statements().is_empty());
    assert!(product.body_shape().expressions().is_empty());
    assert!(product.body_shape().relations().is_empty());
}
