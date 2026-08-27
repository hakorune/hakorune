use super::*;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};

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
