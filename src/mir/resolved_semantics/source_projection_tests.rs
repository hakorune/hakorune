use crate::ast::{
    ASTNode, BinaryOperator, CheckItem, DeclarationAttrs, LiteralValue, Span, UnaryOperator,
};

use super::{
    project_source_body_node_v1, project_source_node_v1, ProjectedSourceNodeV1, SourceNodeSiteV1,
    SourcePathSegmentV1,
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

fn local(value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec!["value".into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "project".into(),
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

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn projector_admits_segment_kind(segment: &SourcePathSegmentV1) -> bool {
    match segment {
        SourcePathSegmentV1::FunctionBody
        | SourcePathSegmentV1::Body(_)
        | SourcePathSegmentV1::ScopeBodyRoot
        | SourcePathSegmentV1::ScopeBody(_)
        | SourcePathSegmentV1::TaskScopeBodyRoot
        | SourcePathSegmentV1::TaskScopeBody(_)
        | SourcePathSegmentV1::FastMemBodyRoot
        | SourcePathSegmentV1::FastMemBody(_)
        | SourcePathSegmentV1::IfCondition
        | SourcePathSegmentV1::IfThenBody
        | SourcePathSegmentV1::IfThen(_)
        | SourcePathSegmentV1::IfElseBody
        | SourcePathSegmentV1::IfElse(_)
        | SourcePathSegmentV1::LoopCondition
        | SourcePathSegmentV1::LoopBodyRoot
        | SourcePathSegmentV1::LoopBody(_)
        | SourcePathSegmentV1::Receiver
        | SourcePathSegmentV1::Callee
        | SourcePathSegmentV1::Argument(_)
        | SourcePathSegmentV1::Element(_)
        | SourcePathSegmentV1::EntryValue(_)
        | SourcePathSegmentV1::FieldValue(_)
        | SourcePathSegmentV1::UpdateValue(_)
        | SourcePathSegmentV1::Base
        | SourcePathSegmentV1::CheckItem(_)
        | SourcePathSegmentV1::Target
        | SourcePathSegmentV1::Value
        | SourcePathSegmentV1::Lhs
        | SourcePathSegmentV1::Rhs
        | SourcePathSegmentV1::Operand
        | SourcePathSegmentV1::Initializer(_)
        | SourcePathSegmentV1::LambdaBodyRoot
        | SourcePathSegmentV1::LambdaBody(_)
        | SourcePathSegmentV1::BlockExprPreludeRoot
        | SourcePathSegmentV1::BlockExprPrelude(_)
        | SourcePathSegmentV1::BlockExprTail => true,
        SourcePathSegmentV1::Binding(_)
        | SourcePathSegmentV1::QMarkOperand
        | SourcePathSegmentV1::MatchScrutinee
        | SourcePathSegmentV1::MatchArm(_)
        | SourcePathSegmentV1::MatchElse
        | SourcePathSegmentV1::EnumMatchScrutinee
        | SourcePathSegmentV1::EnumMatchArm(_)
        | SourcePathSegmentV1::EnumMatchElse
        | SourcePathSegmentV1::TryBodyRoot
        | SourcePathSegmentV1::TryBody(_)
        | SourcePathSegmentV1::CatchClause(_)
        | SourcePathSegmentV1::CatchBodyRoot
        | SourcePathSegmentV1::CatchBody(_)
        | SourcePathSegmentV1::CleanupBodyRoot
        | SourcePathSegmentV1::CleanupBody(_) => false,
    }
}

fn assert_literal(root: &ASTNode, segments: Vec<SourcePathSegmentV1>, expected: i64) {
    let projected = project_source_node_v1(root, &site(segments)).expect("projected node");
    assert!(matches!(
        projected,
        ProjectedSourceNodeV1::Node(ASTNode::Literal {
            value: LiteralValue::Integer(actual),
            ..
        }) if *actual == expected
    ));
}

fn assert_body_len(root: &ASTNode, segments: Vec<SourcePathSegmentV1>, expected: usize) {
    let projected = project_source_node_v1(root, &site(segments)).expect("projected body");
    assert!(matches!(projected, ProjectedSourceNodeV1::Body(body) if body.len() == expected));
}

#[test]
fn projects_every_admitted_body_and_control_segment() {
    let root = function(vec![
        local(literal(10)),
        ASTNode::ScopeBox {
            body: vec![literal(11)],
            span: Span::unknown(),
        },
        ASTNode::TaskScope {
            body: vec![literal(12)],
            source_keyword: "co".into(),
            span: Span::unknown(),
        },
        ASTNode::FastMemRegion {
            contract: "Proof".into(),
            body: vec![literal(13)],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(literal(14)),
            then_body: vec![literal(15)],
            else_body: Some(vec![literal(16)]),
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(literal(17)),
            body: vec![literal(18)],
            span: Span::unknown(),
        },
        local(ASTNode::BlockExpr {
            prelude_stmts: vec![literal(19)],
            tail_expr: Box::new(literal(20)),
            span: Span::unknown(),
        }),
        local(ASTNode::Lambda {
            params: Vec::new(),
            body: vec![literal(21)],
            span: Span::unknown(),
        }),
    ]);

    assert_body_len(&root, vec![SourcePathSegmentV1::FunctionBody], 8);
    assert!(matches!(
        project_source_node_v1(&root, &site(vec![SourcePathSegmentV1::Body(0)])),
        Some(ProjectedSourceNodeV1::Node(ASTNode::Local { .. }))
    ));
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ],
        10,
    );

    for (body_index, root_role, item_role, expected) in [
        (
            1,
            SourcePathSegmentV1::ScopeBodyRoot,
            SourcePathSegmentV1::ScopeBody(0),
            11,
        ),
        (
            2,
            SourcePathSegmentV1::TaskScopeBodyRoot,
            SourcePathSegmentV1::TaskScopeBody(0),
            12,
        ),
        (
            3,
            SourcePathSegmentV1::FastMemBodyRoot,
            SourcePathSegmentV1::FastMemBody(0),
            13,
        ),
    ] {
        assert_body_len(
            &root,
            vec![SourcePathSegmentV1::Body(body_index), root_role],
            1,
        );
        assert_literal(
            &root,
            vec![SourcePathSegmentV1::Body(body_index), item_role],
            expected,
        );
    }

    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfCondition,
        ],
        14,
    );
    assert_body_len(
        &root,
        vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfThenBody,
        ],
        1,
    );
    assert_literal(
        &root,
        vec![SourcePathSegmentV1::Body(4), SourcePathSegmentV1::IfThen(0)],
        15,
    );
    assert_body_len(
        &root,
        vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfElseBody,
        ],
        1,
    );
    assert_literal(
        &root,
        vec![SourcePathSegmentV1::Body(4), SourcePathSegmentV1::IfElse(0)],
        16,
    );
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::LoopCondition,
        ],
        17,
    );
    assert_body_len(
        &root,
        vec![
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::LoopBodyRoot,
        ],
        1,
    );
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(5),
            SourcePathSegmentV1::LoopBody(0),
        ],
        18,
    );
    assert_body_len(
        &root,
        vec![
            SourcePathSegmentV1::Body(6),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::BlockExprPreludeRoot,
        ],
        1,
    );
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(6),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::BlockExprPrelude(0),
        ],
        19,
    );
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(6),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::BlockExprTail,
        ],
        20,
    );
    assert_body_len(
        &root,
        vec![
            SourcePathSegmentV1::Body(7),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::LambdaBodyRoot,
        ],
        1,
    );
    assert_literal(
        &root,
        vec![
            SourcePathSegmentV1::Body(7),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::LambdaBody(0),
        ],
        21,
    );
}

#[test]
fn projects_every_admitted_statement_and_expression_segment() {
    let root = function(vec![
        ASTNode::Assignment {
            target: Box::new(literal(30)),
            value: Box::new(literal(31)),
            span: Span::unknown(),
        },
        ASTNode::Print {
            expression: Box::new(literal(32)),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(literal(33))),
            span: Span::unknown(),
        },
        local(ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(literal(34)),
            span: Span::unknown(),
        }),
        local(ASTNode::AwaitExpression {
            expression: Box::new(literal(35)),
            span: Span::unknown(),
        }),
        local(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(literal(36)),
            right: Box::new(literal(37)),
            span: Span::unknown(),
        }),
        local(ASTNode::ArrayLiteral {
            elements: vec![literal(38)],
            span: Span::unknown(),
        }),
        local(ASTNode::MapLiteral {
            entries: vec![("key".into(), literal(39))],
            span: Span::unknown(),
        }),
        local(ASTNode::RecordLiteral {
            record_type_name: "Pair".into(),
            fields: vec![("field".into(), literal(40))],
            span: Span::unknown(),
        }),
        local(ASTNode::RecordUpdate {
            base: Box::new(literal(41)),
            updates: vec![("field".into(), literal(42))],
            span: Span::unknown(),
        }),
        local(ASTNode::CheckExpr {
            name: Some("proof".into()),
            items: vec![CheckItem {
                label: Some("item".into()),
                expression: literal(43),
            }],
            span: Span::unknown(),
        }),
        local(ASTNode::GroupedAssignmentExpr {
            lhs: "x".into(),
            rhs: Box::new(literal(44)),
            span: Span::unknown(),
        }),
    ]);

    for (body_index, role, expected) in [
        (0, SourcePathSegmentV1::Target, 30),
        (0, SourcePathSegmentV1::Value, 31),
        (1, SourcePathSegmentV1::Value, 32),
        (2, SourcePathSegmentV1::Value, 33),
    ] {
        assert_literal(
            &root,
            vec![SourcePathSegmentV1::Body(body_index), role],
            expected,
        );
    }

    for (body_index, role, expected) in [
        (3, SourcePathSegmentV1::Operand, 34),
        (4, SourcePathSegmentV1::Operand, 35),
        (5, SourcePathSegmentV1::Lhs, 36),
        (5, SourcePathSegmentV1::Rhs, 37),
        (6, SourcePathSegmentV1::Element(0), 38),
        (7, SourcePathSegmentV1::EntryValue(0), 39),
        (8, SourcePathSegmentV1::FieldValue(0), 40),
        (9, SourcePathSegmentV1::Base, 41),
        (9, SourcePathSegmentV1::UpdateValue(0), 42),
        (10, SourcePathSegmentV1::CheckItem(0), 43),
        (11, SourcePathSegmentV1::Value, 44),
    ] {
        assert_literal(
            &root,
            vec![
                SourcePathSegmentV1::Body(body_index),
                SourcePathSegmentV1::Initializer(0),
                role,
            ],
            expected,
        );
    }

    assert!(matches!(
        project_source_node_v1(
            &root,
            &site(vec![
                SourcePathSegmentV1::Body(11),
                SourcePathSegmentV1::Initializer(0),
                SourcePathSegmentV1::Target,
            ])
        ),
        Some(ProjectedSourceNodeV1::SyntheticName)
    ));
}

#[test]
fn projects_shared_variant_arms_and_rejects_absent_optional_children() {
    let root = function(vec![
        ASTNode::Outbox {
            variables: vec!["value".into()],
            initial_values: vec![Some(Box::new(literal(45)))],
            span: Span::unknown(),
        },
        ASTNode::CompoundAssignment {
            target: Box::new(literal(46)),
            operator: BinaryOperator::Add,
            value: Box::new(literal(47)),
            span: Span::unknown(),
        },
        ASTNode::Nowait {
            variable: "future".into(),
            expression: Box::new(literal(48)),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(literal(49)),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        },
    ]);

    for (body_index, role, expected) in [
        (0, SourcePathSegmentV1::Initializer(0), 45),
        (1, SourcePathSegmentV1::Target, 46),
        (1, SourcePathSegmentV1::Value, 47),
        (2, SourcePathSegmentV1::Value, 48),
    ] {
        assert_literal(
            &root,
            vec![SourcePathSegmentV1::Body(body_index), role],
            expected,
        );
    }

    for absent in [
        vec![SourcePathSegmentV1::Body(3), SourcePathSegmentV1::Value],
        vec![
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::IfElseBody,
        ],
        vec![SourcePathSegmentV1::Body(4), SourcePathSegmentV1::IfElse(0)],
    ] {
        assert!(project_source_node_v1(&root, &site(absent)).is_none());
    }
}

#[test]
fn projects_every_admitted_receiver_callee_argument_and_new_field_segment() {
    let root = function(vec![
        local(ASTNode::MethodCall {
            object: Box::new(literal(50)),
            method: "run".into(),
            arguments: vec![literal(51)],
            span: Span::unknown(),
        }),
        local(ASTNode::FieldAccess {
            object: Box::new(literal(52)),
            field: "value".into(),
            span: Span::unknown(),
        }),
        local(ASTNode::Index {
            target: Box::new(literal(53)),
            index: Box::new(literal(54)),
            span: Span::unknown(),
        }),
        local(ASTNode::Call {
            callee: Box::new(literal(55)),
            arguments: vec![literal(56)],
            span: Span::unknown(),
        }),
        local(ASTNode::FunctionCall {
            name: "call".into(),
            arguments: vec![literal(57)],
            span: Span::unknown(),
        }),
        local(ASTNode::FromCall {
            parent: "Parent".into(),
            method: "call".into(),
            arguments: vec![literal(58)],
            span: Span::unknown(),
        }),
        local(ASTNode::New {
            class: "Box".into(),
            arguments: vec![literal(59)],
            field_initializers: vec![("field".into(), literal(60))],
            type_arguments: Vec::new(),
            span: Span::unknown(),
        }),
    ]);

    for (body_index, role, expected) in [
        (0, SourcePathSegmentV1::Receiver, 50),
        (0, SourcePathSegmentV1::Argument(0), 51),
        (1, SourcePathSegmentV1::Receiver, 52),
        (2, SourcePathSegmentV1::Target, 53),
        (2, SourcePathSegmentV1::Argument(0), 54),
        (3, SourcePathSegmentV1::Callee, 55),
        (3, SourcePathSegmentV1::Argument(0), 56),
        (4, SourcePathSegmentV1::Argument(0), 57),
        (5, SourcePathSegmentV1::Argument(0), 58),
        (6, SourcePathSegmentV1::Argument(0), 59),
        (6, SourcePathSegmentV1::Initializer(0), 60),
    ] {
        assert_literal(
            &root,
            vec![
                SourcePathSegmentV1::Body(body_index),
                SourcePathSegmentV1::Initializer(0),
                role,
            ],
            expected,
        );
    }
}

#[test]
fn rejects_malformed_paths_without_partial_publication() {
    let root = function(vec![local(literal(1))]);
    for malformed in [
        vec![SourcePathSegmentV1::Body(9)],
        vec![
            SourcePathSegmentV1::FunctionBody,
            SourcePathSegmentV1::Body(0),
        ],
        vec![SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Receiver],
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(1),
        ],
    ] {
        assert!(project_source_node_v1(&root, &site(malformed)).is_none());
    }

    assert!(matches!(
        project_source_node_v1(&variable("root"), &site(Vec::new())),
        Some(ProjectedSourceNodeV1::Node(ASTNode::Variable { name, .. })) if name == "root"
    ));
}

#[test]
fn keeps_the_parked_segment_vocabulary_explicitly_rejected() {
    let parked = [
        SourcePathSegmentV1::Binding(0),
        SourcePathSegmentV1::QMarkOperand,
        SourcePathSegmentV1::MatchScrutinee,
        SourcePathSegmentV1::MatchArm(0),
        SourcePathSegmentV1::MatchElse,
        SourcePathSegmentV1::EnumMatchScrutinee,
        SourcePathSegmentV1::EnumMatchArm(0),
        SourcePathSegmentV1::EnumMatchElse,
        SourcePathSegmentV1::TryBodyRoot,
        SourcePathSegmentV1::TryBody(0),
        SourcePathSegmentV1::CatchClause(0),
        SourcePathSegmentV1::CatchBodyRoot,
        SourcePathSegmentV1::CatchBody(0),
        SourcePathSegmentV1::CleanupBodyRoot,
        SourcePathSegmentV1::CleanupBody(0),
    ];
    let root = variable("root");
    for segment in parked {
        assert!(!projector_admits_segment_kind(&segment));
        assert!(project_source_node_v1(&root, &site(vec![segment])).is_none());
    }
}

#[test]
fn projects_catalog_owned_function_bodies_without_reconstructing_a_declaration() {
    let body = vec![local(ASTNode::MethodCall {
        object: Box::new(variable("Helpers")),
        method: "run".into(),
        arguments: vec![literal(70)],
        span: Span::unknown(),
    })];
    let call_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    assert!(matches!(
        project_source_body_node_v1(&body, &call_site),
        Some(ProjectedSourceNodeV1::Node(ASTNode::MethodCall { method, .. }))
            if method == "run"
    ));
    assert_literal(
        &function(body.clone()),
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Argument(0),
        ],
        70,
    );
    assert!(
        project_source_body_node_v1(&body, &site(vec![SourcePathSegmentV1::FunctionBody]))
            .is_none()
    );
    assert!(project_source_body_node_v1(&body, &site(Vec::new())).is_none());
}
