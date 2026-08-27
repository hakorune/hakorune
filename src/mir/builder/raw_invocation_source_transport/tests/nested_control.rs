use super::*;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};

#[test]
fn print_statement_and_value_are_exactly_located() {
    let statement = ASTNode::Print {
        expression: Box::new(integer(5)),
        span: Span::unknown(),
    };
    let root = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::function_body().node(),
        body_kind: Some(SourceBodyKindV1::Function),
    };
    let (_, child) =
        RawInvocationSourceContextV1::from_transport(root.body_statement(statement.clone(), 4));
    assert_eq!(
        child.site().unwrap().segments(),
        &[SourcePathSegmentV1::Body(4)]
    );
    let value = child
        .child_expression(&statement, ExprChildRoleV1::PrintValue)
        .unwrap();
    assert_eq!(
        value.site().unwrap().segments(),
        &[SourcePathSegmentV1::Body(4), SourcePathSegmentV1::Value]
    );
}

#[test]
fn if_roles_issue_exact_condition_and_branch_roots() {
    let statement = ASTNode::If {
        condition: Box::new(integer(1)),
        then_body: vec![integer(2)],
        else_body: Some(vec![integer(3)]),
        span: Span::unknown(),
    };
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(4).node(),
        body_kind: None,
    };

    let condition = parent
        .child_expression(&statement, ExprChildRoleV1::IfCondition)
        .expect("condition role");
    let then_body = parent
        .child_body(&statement, BodyChildRoleV1::IfThen)
        .expect("then role");
    let else_body = parent
        .child_body(&statement, BodyChildRoleV1::IfElse)
        .expect("else role");

    assert_eq!(
        condition.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfCondition
        ]
    );
    assert_eq!(
        then_body.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfThenBody
        ]
    );
    assert_eq!(
        else_body.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfElseBody
        ]
    );
}

#[test]
fn try_roles_issue_exact_try_first_catch_and_cleanup_roots() {
    let statement = ASTNode::TryCatch {
        try_body: vec![integer(1)],
        catch_clauses: vec![crate::ast::CatchClause {
            exception_type: Some("Error".into()),
            variable_name: None,
            body: vec![integer(2)],
            span: Span::unknown(),
        }],
        finally_body: Some(vec![integer(3)]),
        span: Span::unknown(),
    };
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(6).node(),
        body_kind: None,
    };

    for (role, expected) in [
        (
            BodyChildRoleV1::TryBody,
            vec![
                SourcePathSegmentV1::Body(6),
                SourcePathSegmentV1::TryBodyRoot,
            ],
        ),
        (
            BodyChildRoleV1::FirstCatchBody,
            vec![
                SourcePathSegmentV1::Body(6),
                SourcePathSegmentV1::CatchClause(0),
                SourcePathSegmentV1::CatchBodyRoot,
            ],
        ),
        (
            BodyChildRoleV1::CleanupBody,
            vec![
                SourcePathSegmentV1::Body(6),
                SourcePathSegmentV1::CleanupBodyRoot,
            ],
        ),
    ] {
        let child = parent.child_body(&statement, role).expect("TryCatch role");
        assert_eq!(child.site().unwrap().segments(), expected);
    }
}

#[test]
fn script_direct_if_and_fastmem_start_from_exact_statement_index() {
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));
    let if_node = ASTNode::If {
        condition: Box::new(integer(1)),
        then_body: vec![integer(2)],
        else_body: None,
        span: Span::unknown(),
    };
    let if_parent = root.child_statement(&if_node, 5).expect("If statement");
    let condition = if_parent
        .child_expression(&if_node, ExprChildRoleV1::IfCondition)
        .expect("If condition");
    let then_body = if_parent
        .child_body(&if_node, BodyChildRoleV1::IfThen)
        .expect("If body");
    assert_eq!(
        condition.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::IfCondition
        ]
    );
    assert_eq!(
        then_body.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::IfThenBody
        ]
    );

    let fastmem = ASTNode::FastMemRegion {
        contract: "PageMapV0".to_owned(),
        body: vec![integer(3)],
        span: Span::unknown(),
    };
    let fastmem_parent = root
        .child_statement(&fastmem, 7)
        .expect("FastMem statement");
    let body = fastmem_parent
        .child_body(&fastmem, BodyChildRoleV1::FastMemBody)
        .expect("FastMem body");
    assert_eq!(
        body.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(7),
            SourcePathSegmentV1::FastMemBodyRoot
        ]
    );
}

#[test]
fn match_and_enum_roles_issue_only_the_selected_exact_children() {
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(8).node(),
        body_kind: None,
    };
    let match_node = ASTNode::MatchExpr {
        scrutinee: Box::new(integer(1)),
        arms: vec![
            (LiteralValue::Integer(1), integer(2)),
            (LiteralValue::Integer(2), integer(3)),
        ],
        else_expr: Box::new(integer(4)),
        span: Span::unknown(),
    };
    for (role, segment) in [
        (
            ExprChildRoleV1::MatchScrutinee,
            SourcePathSegmentV1::MatchScrutinee,
        ),
        (
            ExprChildRoleV1::MatchArm(0),
            SourcePathSegmentV1::MatchArm(0),
        ),
        (
            ExprChildRoleV1::MatchArm(1),
            SourcePathSegmentV1::MatchArm(1),
        ),
        (ExprChildRoleV1::MatchElse, SourcePathSegmentV1::MatchElse),
    ] {
        let child = parent
            .child_expression(&match_node, role)
            .expect("exact Match child");
        assert_eq!(
            child.site().unwrap().segments(),
            &[SourcePathSegmentV1::Body(8), segment]
        );
    }
    assert!(parent
        .child_expression(&match_node, ExprChildRoleV1::MatchArm(2))
        .is_err());

    let enum_match = ASTNode::EnumMatchExpr {
        enum_name: "Result".into(),
        scrutinee: Box::new(integer(5)),
        arms: Vec::new(),
        else_expr: None,
        span: Span::unknown(),
    };
    let child = parent
        .child_expression(&enum_match, ExprChildRoleV1::EnumMatchScrutinee)
        .expect("exact EnumMatch scrutinee");
    assert_eq!(
        child.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(8),
            SourcePathSegmentV1::EnumMatchScrutinee,
        ]
    );
}
