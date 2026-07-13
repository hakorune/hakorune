use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourceBindingSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};

fn fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["arg".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: false,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn local_site() -> SourceBindingSiteV1 {
    SourceBindingSiteV1::Local {
        statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ])),
        ordinal: 0,
    }
}

#[test]
fn canonical_resolver_seals_receiver_parameter_and_local() {
    let tree = fixture();
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let product = session.resolve(view).unwrap();

    assert_eq!(product.binding_count(), 3);
    let local = product.declaration_binding(&local_site()).unwrap();
    assert_eq!(product.binding(local).unwrap().diagnostic_name(), "x");
}

#[test]
fn canonical_resolver_normalizes_across_compilation_sessions() {
    let tree = fixture();
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let first = FunctionSemanticResolverSessionV1::new(7)
        .unwrap()
        .resolve(view)
        .unwrap();
    let second = FunctionSemanticResolverSessionV1::new(7)
        .unwrap()
        .resolve(view)
        .unwrap();

    assert_ne!(first.owner(), second.owner());
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}
