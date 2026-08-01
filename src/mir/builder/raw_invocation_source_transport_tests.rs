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

fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn new_value(class: &str) -> ASTNode {
    ASTNode::New {
        class: class.to_owned(),
        arguments: Vec::new(),
        type_arguments: Vec::new(),
        field_initializers: Vec::new(),
        span: Span::unknown(),
    }
}

fn local(
    variables: &[&str],
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<&str>>,
) -> ASTNode {
    ASTNode::Local {
        variables: variables.iter().map(|name| (*name).to_owned()).collect(),
        initial_values,
        declared_type_names: declared_type_names
            .into_iter()
            .map(|name| name.map(str::to_owned))
            .collect(),
        span: Span::unknown(),
    }
}

#[test]
fn scalar_single_value_statements_keep_exact_parent_sites() {
    let field_assignment = assignment(
        ASTNode::FieldAccess {
            object: Box::new(variable("page")),
            field: "value".to_owned(),
            span: Span::unknown(),
        },
        integer(4),
    );
    let statements = [
        assignment(variable("x"), integer(1)),
        ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_owned(),
            rhs: Box::new(integer(2)),
            span: Span::unknown(),
        },
        ASTNode::CompoundAssignment {
            target: Box::new(variable("x")),
            operator: crate::ast::BinaryOperator::Add,
            value: Box::new(integer(3)),
            span: Span::unknown(),
        },
        ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            }),
            operator: crate::ast::BinaryOperator::Add,
            value: Box::new(integer(4)),
            span: Span::unknown(),
        },
        ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::Index {
                target: Box::new(variable("items")),
                index: Box::new(integer(0)),
                span: Span::unknown(),
            }),
            operator: crate::ast::BinaryOperator::Add,
            value: Box::new(integer(4)),
            span: Span::unknown(),
        },
        field_assignment,
        assignment(
            ASTNode::Index {
                target: Box::new(variable("items")),
                index: Box::new(integer(0)),
                span: Span::unknown(),
            },
            integer(4),
        ),
        ASTNode::Return {
            value: Some(Box::new(integer(8))),
            span: Span::unknown(),
        },
    ];
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    for (index, statement) in statements.into_iter().enumerate() {
        let (_, body_child) = RawInvocationSourceContextV1::from_transport(
            root.body_statement(statement.clone(), index),
        );
        assert!(matches!(
            body_child,
            RawInvocationSourceContextV1::Located { .. }
        ));
        assert_eq!(
            body_child
                .site()
                .expect("located scalar statement")
                .segments(),
            &[SourcePathSegmentV1::Body(index as u32)]
        );
        let role = match &statement {
            ASTNode::Assignment { .. } => ExprChildRoleV1::AssignmentValue,
            ASTNode::GroupedAssignmentExpr { .. } => ExprChildRoleV1::GroupedAssignmentValue,
            ASTNode::CompoundAssignment { .. } => ExprChildRoleV1::CompoundAssignmentValue,
            ASTNode::Return { .. } => ExprChildRoleV1::ReturnValue,
            _ => unreachable!("scalar single-value fixture"),
        };
        let value = body_child
            .child_expression(&statement, role)
            .expect("exact scalar value source");
        assert_eq!(
            value.site().expect("located scalar value").segments(),
            &[
                SourcePathSegmentV1::Body(index as u32),
                SourcePathSegmentV1::Value
            ]
        );
        if let ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. } =
            &statement
        {
            let target_source = body_child
                .child_expression(
                    &statement,
                    if matches!(statement, ASTNode::Assignment { .. }) {
                        ExprChildRoleV1::AssignmentTarget
                    } else {
                        ExprChildRoleV1::CompoundAssignmentTarget
                    },
                )
                .expect("assignment target source");
            if matches!(target.as_ref(), ASTNode::FieldAccess { .. }) {
                let receiver_source = target_source
                    .child_expression(target, ExprChildRoleV1::Receiver)
                    .expect("field receiver source");
                assert_eq!(
                    receiver_source.site().unwrap().segments(),
                    &[
                        SourcePathSegmentV1::Body(index as u32),
                        SourcePathSegmentV1::Target,
                        SourcePathSegmentV1::Receiver,
                    ]
                );
            } else if matches!(target.as_ref(), ASTNode::Index { .. }) {
                let index_target = target_source
                    .child_expression(target, ExprChildRoleV1::IndexTarget)
                    .expect("index target source");
                let index_subscript = target_source
                    .child_expression(target, ExprChildRoleV1::IndexSubscript)
                    .expect("index subscript source");
                assert_eq!(
                    index_target.site().unwrap().segments(),
                    &[
                        SourcePathSegmentV1::Body(index as u32),
                        SourcePathSegmentV1::Target,
                        SourcePathSegmentV1::Target,
                    ]
                );
                assert_eq!(
                    index_subscript.site().unwrap().segments(),
                    &[
                        SourcePathSegmentV1::Body(index as u32),
                        SourcePathSegmentV1::Target,
                        SourcePathSegmentV1::Argument(0),
                    ]
                );
            }
        }

        let direct_child = root
            .child_statement(&statement, index)
            .expect("direct scalar statement");
        assert_eq!(
            direct_child
                .site()
                .expect("located direct statement")
                .segments(),
            &[SourcePathSegmentV1::Body(index as u32)]
        );
    }
}

#[test]
fn scalar_statements_are_located_including_unsupported_targets() {
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    let (_, void_return) = RawInvocationSourceContextV1::from_transport(root.body_statement(
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        },
        0,
    ));
    assert_eq!(
        void_return.site().expect("located void Return").segments(),
        &[SourcePathSegmentV1::Body(0)]
    );

    let (_, child) = RawInvocationSourceContextV1::from_transport(root.body_statement(
        ASTNode::CompoundAssignment {
            target: Box::new(integer(0)),
            operator: crate::ast::BinaryOperator::Add,
            value: Box::new(integer(5)),
            span: Span::unknown(),
        },
        1,
    ));
    assert_eq!(
        child
            .site()
            .expect("located unsupported CompoundAssignment")
            .segments(),
        &[SourcePathSegmentV1::Body(1)]
    );

    let match_return = ASTNode::Return {
        value: Some(Box::new(ASTNode::MatchExpr {
            scrutinee: Box::new(integer(6)),
            arms: Vec::new(),
            else_expr: Box::new(integer(7)),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    };
    let (_, located) =
        RawInvocationSourceContextV1::from_transport(root.body_statement(match_return.clone(), 2));
    assert_eq!(
        located.site().expect("located Match Return").segments(),
        &[SourcePathSegmentV1::Body(2)]
    );
    assert_eq!(
        located
            .child_expression(&match_return, ExprChildRoleV1::ReturnValue)
            .unwrap()
            .site()
            .unwrap()
            .segments(),
        &[SourcePathSegmentV1::Body(2), SourcePathSegmentV1::Value]
    );
}

#[test]
fn local_initializers_keep_exact_active_index_paths() {
    let statement = local(
        &["x", "missing", "z"],
        vec![
            Some(Box::new(integer(1))),
            None,
            Some(Box::new(integer(3))),
            Some(Box::new(new_value("Surplus"))),
        ],
        vec![None, None, None, Some("Array<u8>")],
    );
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));
    let (_, child) =
        RawInvocationSourceContextV1::from_transport(root.body_statement(statement.clone(), 4));

    assert_eq!(
        child.site().expect("located Local").segments(),
        &[SourcePathSegmentV1::Body(4)]
    );
    for index in [0, 2] {
        let initializer = child
            .child_expression(&statement, ExprChildRoleV1::LocalInitializer(index))
            .expect("active initializer source");
        assert_eq!(
            initializer.site().unwrap().segments(),
            &[
                SourcePathSegmentV1::Body(4),
                SourcePathSegmentV1::Initializer(index),
            ]
        );
    }
}

#[test]
fn local_selection_ignores_missing_and_surplus_vector_entries() {
    let selected = [
        local(
            &["x", "missing"],
            vec![Some(Box::new(integer(1)))],
            vec![None],
        ),
        local(
            &["x"],
            vec![
                Some(Box::new(integer(1))),
                Some(Box::new(new_value("Surplus"))),
            ],
            vec![None, Some("Array<u8>")],
        ),
        local(
            &["xs"],
            vec![Some(Box::new(ASTNode::ArrayLiteral {
                elements: Vec::new(),
                span: Span::unknown(),
            }))],
            vec![Some("Array<not-valid")],
        ),
    ];
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    for (index, statement) in selected.into_iter().enumerate() {
        let (_, child) =
            RawInvocationSourceContextV1::from_transport(root.body_statement(statement, index));
        assert!(matches!(
            child,
            RawInvocationSourceContextV1::Located { .. }
        ));
    }
}

#[test]
fn local_special_initializer_hooks_keep_exact_nested_sources() {
    let hooks = [
        local(
            &["value"],
            vec![Some(Box::new(ASTNode::New {
                class: "Page".to_owned(),
                arguments: vec![integer(7)],
                type_arguments: Vec::new(),
                field_initializers: Vec::new(),
                span: Span::unknown(),
            }))],
            vec![None],
        ),
        local(
            &["values"],
            vec![Some(Box::new(ASTNode::ArrayLiteral {
                elements: vec![integer(1)],
                span: Span::unknown(),
            }))],
            vec![Some("Array<u8>")],
        ),
    ];
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    for (index, statement) in hooks.into_iter().enumerate() {
        let (_, child) = RawInvocationSourceContextV1::from_transport(
            root.body_statement(statement.clone(), index),
        );
        assert_eq!(
            child.site().expect("located special Local").segments(),
            &[SourcePathSegmentV1::Body(index as u32)]
        );
        let initializer = child
            .child_expression(&statement, ExprChildRoleV1::LocalInitializer(0))
            .unwrap();
        let role = if index == 0 {
            ExprChildRoleV1::CallArgument(0)
        } else {
            ExprChildRoleV1::ArrayElement(0)
        };
        let ASTNode::Local { initial_values, .. } = &statement else {
            unreachable!("fixture is Local")
        };
        let initializer_node = initial_values[0].as_deref().expect("special initializer");
        assert_eq!(
            initializer
                .child_expression(initializer_node, role)
                .unwrap()
                .site()
                .unwrap()
                .segments()
                .len(),
            3
        );
    }
}

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
