use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};

use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SemanticOwnerSourceKindV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "inventory_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(literal(0)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                }),
                body: vec![ASTNode::Assignment {
                    target: Box::new(variable("i")),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(variable("i")),
                        right: Box::new(literal(1)),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("i"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn stmt(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(node(segments))
}

fn expr(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(node(segments))
}

#[test]
fn resolver_co_seals_branded_statement_and_expression_membership() {
    let tree = fixture();
    let product = FunctionSemanticResolverSessionV1::new(9)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let inventory = product.source_site_inventory();

    assert_eq!(inventory.owner(), product.owner());
    assert_eq!(inventory.function_origin(), product.function_origin());
    assert_eq!(
        inventory.source_kind(),
        SemanticOwnerSourceKindV1::DeclaredFunction
    );

    for site in [
        stmt(vec![SourcePathSegmentV1::Body(0)]),
        stmt(vec![SourcePathSegmentV1::Body(1)]),
        stmt(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
        ]),
        stmt(vec![SourcePathSegmentV1::Body(2)]),
    ] {
        assert!(inventory.contains_statement(&site));
    }

    for site in [
        expr(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ]),
        expr(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopCondition,
        ]),
        expr(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Target,
        ]),
        expr(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Lhs,
        ]),
        expr(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Rhs,
        ]),
        expr(vec![
            SourcePathSegmentV1::Body(2),
            SourcePathSegmentV1::Value,
        ]),
    ] {
        assert!(inventory.contains_expression(&site));
    }
}

#[test]
fn membership_is_typed_and_point_lookup_has_no_synthetic_fallback() {
    let tree = fixture();
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let inventory = product.source_site_inventory();
    let loop_statement = stmt(vec![SourcePathSegmentV1::Body(1)]);
    let loop_as_expression = expr(vec![SourcePathSegmentV1::Body(1)]);
    let absent_statement = stmt(vec![SourcePathSegmentV1::Body(99)]);

    assert!(inventory.contains_statement(&loop_statement));
    assert!(!inventory.contains_expression(&loop_as_expression));
    assert!(!inventory.contains_statement(&absent_statement));
}

#[test]
fn inventory_brand_is_not_reused_across_resolver_sessions() {
    let tree = fixture();
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let first = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap();
    let second = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap();

    assert_ne!(
        first.source_site_inventory().owner(),
        second.source_site_inventory().owner()
    );
}
