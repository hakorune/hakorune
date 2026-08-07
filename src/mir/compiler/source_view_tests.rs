use std::num::NonZeroU32;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
};

use super::located::{ConsumedSourceRangeV1, LocatedBodySuffixV1, SourceBodyKindV1};
use super::lowering_input::verified_source_unit_for_test;
use super::source_projection::{SourceNavigationErrorV1, VerifiedSourceProjectionV1};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

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

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn fixture_function() -> ASTNode {
    let nested = ASTNode::BlockExpr {
        prelude_stmts: Vec::new(),
        tail_expr: Box::new(variable("x")),
        span: Span::unknown(),
    };
    let outer = ASTNode::BlockExpr {
        prelude_stmts: vec![local("y", literal(2)), assignment("x", variable("y"))],
        tail_expr: Box::new(nested),
        span: Span::unknown(),
    };
    let lambda = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            local("x", literal(1)),
            assignment("x", outer),
            local("f", lambda),
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

#[test]
fn located_navigation_keeps_exact_nested_blockexpr_sites() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();

    assert_eq!(body.site().kind(), SourceBodyKindV1::Function);
    assert_eq!(
        body.site().root().segments(),
        &[SourcePathSegmentV1::FunctionBody]
    );

    let local_x = view.body_stmt(&body, 0).unwrap();
    let assign = view.body_stmt(&body, 1).unwrap();
    assert_ne!(local_x.site(), assign.site());
    assert_eq!(local_x.node().span(), assign.node().span());

    let initializer = view
        .child_expr_from_stmt(&local_x, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    assert_eq!(
        initializer.site().node().segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ]
    );

    let target = view
        .child_expr_from_stmt(&assign, ExprChildRoleV1::AssignmentTarget)
        .unwrap();
    let outer = view
        .child_expr_from_stmt(&assign, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    assert!(matches!(target.node(), ASTNode::Variable { name, .. } if name == "x"));
    assert!(matches!(outer.node(), ASTNode::BlockExpr { .. }));

    let prelude = view
        .child_body_from_expr(&outer, BodyChildRoleV1::BlockExprPrelude)
        .unwrap();
    assert_eq!(prelude.statements().len(), 2);
    assert_eq!(
        prelude.site().root().segments(),
        &[
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::BlockExprPreludeRoot,
        ]
    );

    let nested = view
        .child_expr_from_expr(&outer, ExprChildRoleV1::BlockExprTail)
        .unwrap();
    let nested_prelude = view
        .child_body_from_expr(&nested, BodyChildRoleV1::BlockExprPrelude)
        .unwrap();
    assert!(nested_prelude.statements().is_empty());
    let nested_tail = view
        .child_expr_from_expr(&nested, ExprChildRoleV1::BlockExprTail)
        .unwrap();
    assert_eq!(
        nested_tail.site().node().segments(),
        &[
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::BlockExprTail,
            SourcePathSegmentV1::BlockExprTail,
        ]
    );

    let suffix = view.body_suffix(body, 1).unwrap();
    assert_eq!(suffix.start_index(), 1);
    assert_eq!(suffix.body().statements().len(), 3);
}

#[test]
fn navigator_rejects_wrong_roles_and_out_of_bounds_sites() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let local_x = view.body_stmt(&body, 0).unwrap();

    assert!(matches!(
        view.child_expr_from_stmt(&local_x, ExprChildRoleV1::AssignmentTarget),
        Err(SourceNavigationErrorV1::InvalidSite {
            reason: "expression_role_parent_mismatch",
            ..
        })
    ));
    assert!(matches!(
        view.child_expr_from_stmt(&local_x, ExprChildRoleV1::LocalInitializer(9)),
        Err(SourceNavigationErrorV1::InvalidSite {
            reason: "segment_does_not_match_syntax",
            ..
        })
    ));
    assert!(matches!(
        view.body_stmt(&body, 99),
        Err(SourceNavigationErrorV1::BodyIndexOutOfBounds { .. })
    ));
    assert!(matches!(
        view.body_suffix(body, 99),
        Err(SourceNavigationErrorV1::SuffixStartOutOfBounds { .. })
    ));
}

#[test]
fn stmt_at_reopens_only_resolver_inventory_sites() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();

    let site = SourcePathV1::root_body(1).stmt();
    let located = view.stmt_at(&site).unwrap();
    assert_eq!(located.site(), &site);
    assert!(matches!(located.node(), ASTNode::Assignment { .. }));

    let synthetic = SourcePathV1::root_body(99).stmt();
    assert!(matches!(
        view.stmt_at(&synthetic),
        Err(SourceNavigationErrorV1::InvalidSite {
            reason: "statement_not_in_resolver_inventory",
            ..
        })
    ));
}

#[test]
fn lambda_child_view_preserves_owner_and_rejects_foreign_carriers() {
    let unit = verified_source_unit_for_test(fixture_function());
    let root_owner = unit.forest().roots()[0];
    let root_view = unit.function_source_view(root_owner).unwrap();
    let root_body = root_view.root_body().unwrap();
    let lambda_local = root_view.body_stmt(&root_body, 2).unwrap();
    let lambda = root_view
        .child_expr_from_stmt(&lambda_local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let child_view = root_view.child_function(&lambda).unwrap();

    assert_ne!(child_view.owner(), root_owner);
    assert!(matches!(child_view.root(), ASTNode::Lambda { .. }));
    let child_body = child_view.root_body().unwrap();
    assert_eq!(
        child_body.site().root().segments(),
        &[SourcePathSegmentV1::LambdaBodyRoot]
    );
    let return_stmt = child_view.body_stmt(&child_body, 0).unwrap();
    let returned = child_view
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .unwrap();
    assert_eq!(
        returned.site().node().segments(),
        &[
            SourcePathSegmentV1::LambdaBody(0),
            SourcePathSegmentV1::Value,
        ]
    );

    assert!(matches!(
        root_view.body_stmt(&child_body, 0),
        Err(SourceNavigationErrorV1::ForeignOwner { .. })
    ));
    assert!(matches!(
        child_view.child_expr_from_stmt(&lambda_local, ExprChildRoleV1::LocalInitializer(0)),
        Err(SourceNavigationErrorV1::ForeignOwner { .. })
    ));
}

#[test]
fn projection_rejects_syntax_and_semantic_product_mismatch() {
    let syntax = fixture_function();
    let forest = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&syntax).unwrap())
        .unwrap();
    let mut foreign_syntax = fixture_function();
    let ASTNode::FunctionDeclaration { params, .. } = &mut foreign_syntax else {
        unreachable!()
    };
    params.push("foreign_parameter".into());

    assert!(matches!(
        VerifiedSourceProjectionV1::seal(&foreign_syntax, &forest),
        Err(SourceNavigationErrorV1::SignatureMismatch { .. })
    ));
}

#[test]
fn suffix_range_navigation_is_exact_and_advances_to_body_end() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let start = view.body_suffix(body.clone(), 0).unwrap();
    let first = view.suffix_first_stmt(&start).unwrap();
    let first_range = view
        .consumed_prefix(&start, NonZeroU32::new(1).unwrap())
        .unwrap();

    assert_eq!(
        first.site().node().segments(),
        &[SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(first_range.body(), body.site());
    assert_eq!(first_range.start(), 0);
    assert_eq!(first_range.count(), NonZeroU32::new(1).unwrap());

    let middle = view.advance_body_suffix(start, &first_range).unwrap();
    assert_eq!(middle.start_index(), 1);
    let second = view.suffix_first_stmt(&middle).unwrap();
    assert_ne!(first.site(), second.site());
    assert_eq!(first.node().span(), second.node().span());

    let remaining = view
        .consumed_prefix(&middle, NonZeroU32::new(2).unwrap())
        .unwrap();
    let end = view.advance_body_suffix(middle, &remaining).unwrap();
    assert_eq!(end.start_index(), 3);
    assert!(matches!(
        view.suffix_first_stmt(&end),
        Err(SourceNavigationErrorV1::EmptyBodySuffix { start: 3, .. })
    ));
}

#[test]
fn suffix_navigation_rejects_empty_bounds_and_integer_overflow() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();

    assert!(matches!(
        view.body_stmt(&body, usize::MAX),
        Err(SourceNavigationErrorV1::SourceIndexOverflow {
            role: "body_statement_index",
            ..
        })
    ));
    assert!(matches!(
        view.body_suffix(body.clone(), usize::MAX),
        Err(SourceNavigationErrorV1::SourceIndexOverflow {
            role: "body_suffix_start",
            ..
        })
    ));

    let end = view
        .body_suffix(body.clone(), body.statements().len())
        .unwrap();
    assert!(matches!(
        view.consumed_prefix(&end, NonZeroU32::new(1).unwrap()),
        Err(SourceNavigationErrorV1::EmptyBodySuffix { .. })
    ));
    let middle = view.body_suffix(body.clone(), 1).unwrap();
    assert!(matches!(
        view.consumed_prefix(&middle, NonZeroU32::new(3).unwrap()),
        Err(SourceNavigationErrorV1::ConsumedRangeOutOfBounds { .. })
    ));

    let overflow_suffix = LocatedBodySuffixV1::new_for_test(body.clone(), u32::MAX);
    let overflow_range = ConsumedSourceRangeV1::new_for_test(
        body.site().clone(),
        u32::MAX,
        NonZeroU32::new(1).unwrap(),
    );
    assert!(matches!(
        view.advance_body_suffix(overflow_suffix, &overflow_range),
        Err(SourceNavigationErrorV1::ConsumedRangeEndOverflow { .. })
    ));
}

#[test]
fn suffix_advance_rejects_foreign_body_gap_overlap_and_reuse() {
    let unit = verified_source_unit_for_test(fixture_function());
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let suffix0 = view.body_suffix(body.clone(), 0).unwrap();
    let suffix1 = view.body_suffix(body.clone(), 1).unwrap();
    let range0 = view
        .consumed_prefix(&suffix0, NonZeroU32::new(1).unwrap())
        .unwrap();
    let range1 = view
        .consumed_prefix(&suffix1, NonZeroU32::new(1).unwrap())
        .unwrap();

    assert!(matches!(
        view.advance_body_suffix(suffix0.clone(), &range1),
        Err(SourceNavigationErrorV1::ConsumedRangeStartMismatch {
            expected: 0,
            actual: 1,
            ..
        })
    ));
    assert!(matches!(
        view.advance_body_suffix(suffix1, &range0),
        Err(SourceNavigationErrorV1::ConsumedRangeStartMismatch {
            expected: 1,
            actual: 0,
            ..
        })
    ));
    let advanced = view.advance_body_suffix(suffix0, &range0).unwrap();
    assert!(matches!(
        view.advance_body_suffix(advanced, &range0),
        Err(SourceNavigationErrorV1::ConsumedRangeStartMismatch { .. })
    ));

    let assign = view.body_stmt(&body, 1).unwrap();
    let block = view
        .child_expr_from_stmt(&assign, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    let prelude = view
        .child_body_from_expr(&block, BodyChildRoleV1::BlockExprPrelude)
        .unwrap();
    let prelude_suffix = view.body_suffix(prelude, 0).unwrap();
    assert!(matches!(
        view.advance_body_suffix(prelude_suffix, &range0),
        Err(SourceNavigationErrorV1::ConsumedRangeBodyMismatch { .. })
    ));

    let lambda_local = view.body_stmt(&body, 2).unwrap();
    let lambda = view
        .child_expr_from_stmt(&lambda_local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let child_view = view.child_function(&lambda).unwrap();
    let child_body = child_view.root_body().unwrap();
    let child_suffix = child_view.body_suffix(child_body, 0).unwrap();
    assert!(matches!(
        child_view.advance_body_suffix(child_suffix, &range0),
        Err(SourceNavigationErrorV1::ForeignOwner { .. })
    ));
}
