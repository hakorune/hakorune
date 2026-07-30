use super::*;
use crate::ast::{LiteralValue, Span};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

#[test]
fn located_root_derives_exact_body_item_without_reissuing_lineage() {
    let box_statement = ASTNode::BoxDeclaration {
        name: "Page".to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: std::collections::HashMap::new(),
        constructors: std::collections::HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: false,
        is_static: false,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        static_init: None,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: crate::ast::Span::unknown(),
    };
    let root = RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
        0,
        "Main",
        "main",
        "Main.main/0",
        0,
    ));
    let (_, context) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root.clone()),
    );
    let (_, child) =
        RawInvocationSourceContextV1::from_transport(context.body_statement(box_statement, 3));

    assert!(matches!(
        child,
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Main(_),
            ..
        }
    ));
    assert_eq!(
        child.site().expect("located child").segments(),
        &[SourcePathSegmentV1::Body(3)]
    );
}

#[test]
fn compatibility_reason_is_forwarded_once_to_body_item() {
    let (_, context) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::unlocated(
            Vec::<ASTNode>::new(),
            RawUnlocatedPortalV1::NestedBoxAdmission,
        ));
    let (_, child) = RawInvocationSourceContextV1::from_transport(context.body_statement(
        ASTNode::Break {
            span: crate::ast::Span::unknown(),
        },
        0,
    ));
    assert_eq!(
        child,
        RawInvocationSourceContextV1::UnlocatedCompatibility(
            RawUnlocatedPortalV1::NestedBoxAdmission
        )
    );
}

#[test]
fn located_controls_and_diagnostic_terminals_keep_exact_parent_sites() {
    let controls = [
        ASTNode::If {
            condition: Box::new(integer(1)),
            then_body: vec![integer(2)],
            else_body: Some(vec![integer(3)]),
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(integer(1)),
            body: vec![integer(2)],
            span: Span::unknown(),
        },
        ASTNode::TaskScope {
            body: vec![integer(1)],
            source_keyword: "co".to_owned(),
            span: Span::unknown(),
        },
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(),
            body: vec![integer(1)],
            span: Span::unknown(),
        },
        ASTNode::ScopeBox {
            body: vec![integer(1)],
            span: Span::unknown(),
        },
        ASTNode::BlockExpr {
            prelude_stmts: vec![integer(1)],
            tail_expr: Box::new(integer(2)),
            span: Span::unknown(),
        },
        ASTNode::LoopRange {
            var_name: "i".to_owned(),
            start: Box::new(integer(0)),
            end: Box::new(integer(1)),
            body: Vec::new(),
            span: Span::unknown(),
        },
        ASTNode::ContextScope {
            name: "ctx".to_owned(),
            declared_type_name: None,
            value: Box::new(integer(1)),
            body: Vec::new(),
            source_keyword: "context".to_owned(),
            span: Span::unknown(),
        },
    ];
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    for (index, control) in controls.into_iter().enumerate() {
        let (_, child) =
            RawInvocationSourceContextV1::from_transport(root.body_statement(control, index));
        assert!(matches!(
            child,
            RawInvocationSourceContextV1::Located { .. }
        ));
        assert_eq!(
            child.site().expect("structured control site").segments(),
            &[SourcePathSegmentV1::Body(index as u32)]
        );
    }
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
