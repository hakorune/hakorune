use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

use super::resolve_function_shadow_v0;

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["x".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn expr_site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn resolve(body: Vec<ASTNode>) -> super::ShadowResolvedFunctionV0 {
    resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &function(body)).unwrap()
}

#[test]
fn print_payload_has_an_exact_value_site() {
    let product = resolve(vec![ASTNode::Print {
        expression: Box::new(variable("x")),
        span: Span::unknown(),
    }]);
    let site = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    assert!(product.variable_uses.contains_key(&site));
}

#[test]
fn general_call_distinguishes_callee_and_arguments() {
    let product = resolve(vec![ASTNode::Call {
        callee: Box::new(variable("x")),
        arguments: vec![variable("x")],
        span: Span::unknown(),
    }]);
    let callee = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Callee,
    ]);
    let argument = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Argument(0),
    ]);
    assert!(product.variable_uses.contains_key(&callee));
    assert!(product.variable_uses.contains_key(&argument));
    assert_ne!(callee, argument);
}

#[test]
fn collection_entries_preserve_ordered_child_roles() {
    let product = resolve(vec![ASTNode::ArrayLiteral {
        elements: vec![
            variable("x"),
            ASTNode::MapLiteral {
                entries: vec![("key".into(), variable("x"))],
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    }]);
    let first = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Element(0),
    ]);
    let nested = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Element(1),
        SourcePathSegmentV1::EntryValue(0),
    ]);
    assert!(product.variable_uses.contains_key(&first));
    assert!(product.variable_uses.contains_key(&nested));
    assert_ne!(first, nested);
}
