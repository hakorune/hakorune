use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};

use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn expr_site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn local(name: &str, initializer: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initializer))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn lambda(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Lambda {
        params: Vec::new(),
        body,
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "root".into(),
        params: Vec::new(),
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

#[test]
fn recursive_forest_preserves_capture_first_demand_order_not_binding_sort_order() {
    let body = ASTNode::Return {
        value: Some(Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("second")),
            right: Box::new(variable("first")),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    };
    let tree = function(vec![
        local("first", int(1)),
        local("second", int(2)),
        local("f", lambda(vec![body])),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let child = forest
        .child_at(&OwnedExprSiteV1::new(
            root,
            expr_site(vec![
                SourcePathSegmentV1::Body(2),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let second_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Lhs,
    ]);
    let first_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Rhs,
    ]);
    let ResolvedLexicalRefV1::Upvar(second) = forest
        .owner(child)
        .unwrap()
        .variable_ref(&second_site)
        .unwrap()
    else {
        panic!("expected second upvar");
    };
    let ResolvedLexicalRefV1::Upvar(first) = forest
        .owner(child)
        .unwrap()
        .variable_ref(&first_site)
        .unwrap()
    else {
        panic!("expected first upvar");
    };
    let demands = forest.ordered_capture_demands(child);
    assert_eq!(demands.len(), 2);
    assert_eq!(demands[0].source_binding(), second.source());
    assert_eq!(demands[0].first_demand(), &second_site);
    assert_eq!(demands[1].source_binding(), first.source());
    assert_eq!(demands[1].first_demand(), &first_site);
}
