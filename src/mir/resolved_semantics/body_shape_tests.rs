use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::{
    BodyExpressionShapeV1, BodyStatementShapeV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ReceiverPolicyV1, SourcePathSegmentV1,
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

#[test]
fn resolved_shape_keeps_static_current_owner_me_without_forging_a_receiver_binding() {
    let tree = function(
        vec![return_value(Some(ASTNode::MethodCall {
            object: Box::new(ASTNode::Me {
                span: Span::unknown(),
            }),
            method: "read".into(),
            arguments: Vec::new(),
            span: Span::unknown(),
        }))],
        true,
    );
    let ASTNode::FunctionDeclaration { params, body, .. } = &tree else {
        unreachable!("fixture is a function")
    };
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_borrowed_function_parts(
            params,
            body,
            ReceiverPolicyV1::StaticCurrentOwner,
        ))
        .unwrap();

    assert!(product
        .body_shape()
        .expressions()
        .iter()
        .any(|row| matches!(
            row,
            BodyExpressionShapeV1::Me {
                receiver: super::BodyMeReceiverV1::StaticCurrentOwner,
                ..
            }
        )));
    assert!(product
        .function()
        .declaration_sites()
        .all(|site| !matches!(site, super::SourceBindingSiteV1::Receiver)));
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

fn block_expr(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(tail_expr),
        span: Span::unknown(),
    }
}

#[test]
fn resolved_shape_issues_typed_block_expr_and_exact_expectation() {
    let tree = function(
        vec![return_value(Some(block_expr(Vec::new(), int(1))))],
        true,
    );
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    assert!(product
        .body_shape()
        .expressions()
        .iter()
        .any(|row| matches!(row, BodyExpressionShapeV1::BlockExpr { .. })));
    let expectation =
        super::issue_resolved_block_expr_expectation_v1(product.function(), product.body_shape())
            .unwrap();
    assert_eq!(expectation.owner(), product.function().owner());
    assert_eq!(
        expectation.function_origin(),
        product.function().function_origin()
    );
    assert_eq!(expectation.pair_count(), 1);
}

#[test]
fn resolved_shape_issues_zero_block_expr_expectation_without_defaulting_a_count() {
    let tree = function(vec![return_value(Some(int(1)))], true);
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let expectation =
        super::issue_resolved_block_expr_expectation_v1(product.function(), product.body_shape())
            .unwrap();
    assert_eq!(expectation.pair_count(), 0);
}

#[test]
fn resolved_shape_counts_nested_block_expr_pairs_from_the_same_source_product() {
    let tree = function(
        vec![return_value(Some(block_expr(
            Vec::new(),
            block_expr(Vec::new(), int(1)),
        )))],
        true,
    );
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let expectation =
        super::issue_resolved_block_expr_expectation_v1(product.function(), product.body_shape())
            .unwrap();
    assert_eq!(expectation.pair_count(), 2);
}

#[test]
fn resolved_shape_expectation_rejects_a_foreign_body_shape_owner() {
    let with_block = function(
        vec![return_value(Some(block_expr(Vec::new(), int(1))))],
        true,
    );
    let without_block = function(vec![return_value(Some(int(1)))], true);
    let first = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&with_block).unwrap())
        .unwrap();
    let second = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&without_block).unwrap())
        .unwrap();
    assert!(matches!(
        super::issue_resolved_block_expr_expectation_v1(first.function(), second.body_shape()),
        Err(super::ResolvedBlockExpressionExpectationIssueV1::OwnerMismatch)
    ));
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
            BodyExpressionShapeV1::Me {
                site,
                receiver: super::BodyMeReceiverV1::Lexical(receiver),
            } => Some((site, *receiver)),
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

#[test]
fn method_call_source_issuer_rejects_incomplete_or_reordered_argument_relations() {
    let tree = function(
        vec![return_value(Some(ASTNode::MethodCall {
            object: Box::new(ASTNode::Me {
                span: Span::unknown(),
            }),
            method: "slice".into(),
            arguments: vec![int(1), int(2)],
            span: Span::unknown(),
        }))],
        false,
    );
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let shape = product.body_shape();
    assert!(
        super::body_shape::duplicate_shadow_body_shape_relation_rejects_for_test(
            shape.relations().first().expect("MethodCall relation"),
        ),
        "the production shadow seal must reject duplicates before publication",
    );
    let call_site = shape
        .expressions()
        .iter()
        .find_map(|row| match row {
            BodyExpressionShapeV1::MethodCall { site, .. } => Some(site.clone()),
            _ => None,
        })
        .unwrap();

    let mut missing = shape.relations().to_vec();
    missing.retain(|row| {
        !(row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Argument(1))
    });
    assert_eq!(
        super::body_shape::issue_resolved_method_call_sources_with_relations_for_test(
            shape, &missing,
        ),
        Err(
            super::body_shape::ResolvedMethodCallSourceIssueV1::MissingArgumentRelation {
                site: call_site.clone(),
                ordinal: 1,
            }
        )
    );

    let mut duplicate = shape.relations().to_vec();
    let argument_zero = duplicate
        .iter()
        .find(|row| row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Argument(0))
        .unwrap()
        .clone();
    duplicate.push(argument_zero);
    assert_eq!(
        super::body_shape::issue_resolved_method_call_sources_with_relations_for_test(
            shape, &duplicate,
        ),
        Err(
            super::body_shape::ResolvedMethodCallSourceIssueV1::DuplicateArgumentRelation {
                site: call_site.clone(),
                ordinal: 0,
            }
        )
    );

    let mut reordered = shape.relations().to_vec();
    let argument_zero = reordered
        .iter()
        .position(|row| {
            row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Argument(0)
        })
        .unwrap();
    let argument_one = reordered
        .iter()
        .position(|row| {
            row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Argument(1)
        })
        .unwrap();
    let zero_site = reordered[argument_zero].child.clone();
    reordered[argument_zero].child = reordered[argument_one].child.clone();
    reordered[argument_one].child = zero_site;
    assert_eq!(
        super::body_shape::issue_resolved_method_call_sources_with_relations_for_test(
            shape, &reordered,
        ),
        Err(
            super::body_shape::ResolvedMethodCallSourceIssueV1::ArgumentSourceMismatch {
                site: call_site.clone(),
                ordinal: 0,
            }
        )
    );

    let mut wrong_receiver = shape.relations().to_vec();
    let argument_site = wrong_receiver
        .iter()
        .find(|row| row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Argument(0))
        .unwrap()
        .child
        .clone();
    wrong_receiver
        .iter_mut()
        .find(|row| row.parent == *call_site.node() && row.role == SourcePathSegmentV1::Receiver)
        .unwrap()
        .child = argument_site;
    assert_eq!(
        super::body_shape::issue_resolved_method_call_sources_with_relations_for_test(
            shape,
            &wrong_receiver,
        ),
        Err(super::body_shape::ResolvedMethodCallSourceIssueV1::ReceiverSourceMismatch(call_site,))
    );
}
