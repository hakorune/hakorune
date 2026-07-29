use crate::ast::{ASTNode, BinaryOperator, CheckItem, LiteralValue, Span, UnaryOperator};
use crate::parser::NyashParser;

use super::{
    PreparedRawNonProgramRootV1, PreparedRawRootPartitionV1, RawNonProgramRootCompatibilityClassV1,
    SelectedRawNonProgramRootV1,
};

#[path = "raw_nonprogram_root_descent_tests/parity.rs"]
mod parity;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_owned()),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn awaited(expression: ASTNode) -> ASTNode {
    ASTNode::AwaitExpression {
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn checked(expressions: Vec<ASTNode>) -> ASTNode {
    ASTNode::CheckExpr {
        name: Some("root-partition".to_owned()),
        items: expressions
            .into_iter()
            .enumerate()
            .map(|(index, expression)| CheckItem {
                label: Some(format!("item-{index}")),
                expression,
            })
            .collect(),
        span: Span::unknown(),
    }
}

fn printed(expression: ASTNode) -> ASTNode {
    ASTNode::Print {
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn nowait(variable: &str, expression: ASTNode) -> ASTNode {
    ASTNode::Nowait {
        variable: variable.to_owned(),
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn array(elements: Vec<ASTNode>) -> ASTNode {
    ASTNode::ArrayLiteral {
        elements,
        span: Span::unknown(),
    }
}

fn map(entries: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::MapLiteral {
        entries: entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
        span: Span::unknown(),
    }
}

fn grouped_assignment(variable_name: &str, rhs: ASTNode) -> ASTNode {
    ASTNode::GroupedAssignmentExpr {
        lhs: variable_name.to_owned(),
        rhs: Box::new(rhs),
        span: Span::unknown(),
    }
}

fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn compound_assignment(target: ASTNode, operator: BinaryOperator, value: ASTNode) -> ASTNode {
    ASTNode::CompoundAssignment {
        target: Box::new(target),
        operator,
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn returning(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn indexed(target: ASTNode, index: ASTNode) -> ASTNode {
    ASTNode::Index {
        target: Box::new(target),
        index: Box::new(index),
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

fn local(
    variables: &[&str],
    initial_values: Vec<Option<ASTNode>>,
    declared_type_names: Vec<Option<&str>>,
) -> ASTNode {
    ASTNode::Local {
        variables: variables.iter().map(|name| (*name).to_owned()).collect(),
        initial_values: initial_values
            .into_iter()
            .map(|value| value.map(Box::new))
            .collect(),
        declared_type_names: declared_type_names
            .into_iter()
            .map(|value| value.map(str::to_owned))
            .collect(),
        span: Span::unknown(),
    }
}

fn task_scope(source_keyword: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::TaskScope {
        body,
        source_keyword: source_keyword.to_owned(),
        span: Span::unknown(),
    }
}

fn scope_box(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::ScopeBox {
        body,
        span: Span::unknown(),
    }
}

fn assert_selected(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::ExprTree(_)
        ))
    ));
}

fn assert_selected_print(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::PrintRoot(_)
        ))
    ));
}

fn assert_selected_nowait(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::NowaitRoot(_)
        ))
    ));
}

fn assert_selected_local(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::LocalRoot(_)
        ))
    ));
}

fn assert_selected_task_scope(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::TaskScopeRoot(_)
        ))
    ));
}

fn assert_selected_assignment(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::VariableAssignmentRoot(_)
        ))
    ));
}

fn assert_selected_compound_assignment(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::VariableCompoundAssignmentRoot(_)
        ))
    ));
}

fn assert_selected_return(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::ReturnRoot(_)
        ))
    ));
}

fn assert_selected_scope_box(node: ASTNode) {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(node),
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::SelectedPortParity(
            SelectedRawNonProgramRootV1::PlainScopeBoxRoot(_)
        ))
    ));
}

fn assert_compatibility(node: ASTNode, expected: RawNonProgramRootCompatibilityClassV1) {
    match PreparedRawRootPartitionV1::classify(node) {
        PreparedRawRootPartitionV1::NonProgram(PreparedRawNonProgramRootV1::Compatibility {
            class,
            ..
        }) => assert_eq!(class, expected),
        _ => panic!("root must remain on the compatibility route"),
    }
}

fn first_statement(source: &str) -> ASTNode {
    match NyashParser::parse_from_string(source).expect("root source") {
        ASTNode::Program { mut statements, .. } => statements.remove(0),
        _ => panic!("parser must return Program"),
    }
}

#[test]
fn port_neutral_partition_is_recursive_and_disjoint() {
    assert_selected(integer(1));
    assert_selected(variable("x"));
    assert_selected(ASTNode::Me {
        span: Span::unknown(),
    });
    assert_selected(ASTNode::UnaryOp {
        operator: UnaryOperator::Minus,
        operand: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(integer(2)),
            right: Box::new(variable("x")),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    });
    assert_selected(awaited(awaited(integer(4))));
    assert_selected(ASTNode::UnaryOp {
        operator: UnaryOperator::Minus,
        operand: Box::new(awaited(variable("future"))),
        span: Span::unknown(),
    });
    assert_selected(ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(awaited(integer(5))),
        right: Box::new(integer(6)),
        span: Span::unknown(),
    });
    assert_selected(checked(Vec::new()));
    assert_selected(checked(vec![
        integer(7),
        awaited(checked(vec![variable("ready")])),
    ]));
    assert_selected(awaited(checked(vec![integer(8), integer(9)])));
    assert_selected_print(printed(integer(10)));
    assert_selected_print(printed(awaited(checked(vec![
        integer(11),
        variable("ready"),
    ]))));
    assert_selected_nowait(nowait(
        "pending",
        awaited(checked(vec![integer(12), variable("ready")])),
    ));
    assert_selected(map(Vec::new()));
    assert_selected(array(vec![
        integer(13),
        map(vec![("nested", array(vec![integer(14)]))]),
    ]));
    assert_selected(awaited(array(vec![checked(vec![integer(15)])])));
    assert_selected_print(printed(map(vec![("array", array(vec![integer(16)]))])));
    assert_selected_nowait(nowait("array_future", array(vec![integer(17)])));
    assert_selected(grouped_assignment("x", integer(18)));
    assert_selected(awaited(grouped_assignment(
        "x",
        checked(vec![integer(19), array(vec![integer(20)])]),
    )));
    assert_selected(indexed(array(vec![integer(21)]), integer(0)));
    assert_selected(indexed(map(vec![("key", integer(22))]), string("key")));
    assert_selected(awaited(indexed(
        array(vec![integer(23), integer(24)]),
        integer(1),
    )));
    assert_selected(block_expr(Vec::new(), integer(25)));
    assert_selected(awaited(block_expr(
        Vec::new(),
        indexed(array(vec![integer(26)]), integer(0)),
    )));
    assert_selected(block_expr(
        Vec::new(),
        block_expr(Vec::new(), checked(vec![integer(27)])),
    ));
    assert_selected(block_expr(
        vec![
            local(&["x"], vec![Some(integer(29))], vec![None]),
            printed(variable("x")),
            nowait("pending", variable("x")),
        ],
        variable("pending"),
    ));
    assert_selected(block_expr(
        vec![block_expr(vec![printed(integer(30))], integer(31))],
        integer(32),
    ));
    assert_selected_local(local(&["x"], vec![Some(integer(28))], vec![None]));
    assert_selected_local(local(&["missing"], Vec::new(), Vec::new()));
    assert_selected_local(local(
        &["nested"],
        vec![Some(block_expr(Vec::new(), array(vec![integer(29)])))],
        vec![None],
    ));
    assert_selected_assignment(assignment(variable("x"), awaited(integer(36))));
    assert_selected_compound_assignment(compound_assignment(
        variable("x"),
        BinaryOperator::Add,
        checked(vec![integer(36)]),
    ));
    assert_selected_return(returning(None));
    assert_selected_return(returning(Some(awaited(integer(37)))));
    assert_selected_scope_box(scope_box(Vec::new()));
    assert_selected_scope_box(scope_box(vec![
        printed(integer(38)),
        scope_box(vec![assignment(variable("x"), integer(39))]),
    ]));
    assert_selected_task_scope(task_scope("co", Vec::new()));
    assert_selected_task_scope(task_scope(
        "task_scope",
        vec![
            printed(integer(37)),
            nowait("pending", integer(38)),
            assignment(variable("x"), checked(vec![integer(38)])),
            compound_assignment(variable("x"), BinaryOperator::Multiply, integer(2)),
            scope_box(vec![printed(integer(39))]),
            task_scope("co", vec![checked(vec![integer(39)])]),
        ],
    ));
    assert_selected(block_expr(
        vec![task_scope("co", vec![printed(integer(40))])],
        integer(41),
    ));

    assert_compatibility(
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(ASTNode::New {
                class: "Page".to_owned(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(integer(3)),
            right: Box::new(ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        awaited(ASTNode::New {
            class: "Page".to_owned(),
            arguments: Vec::new(),
            field_initializers: Vec::new(),
            type_arguments: Vec::new(),
            span: Span::unknown(),
        }),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(awaited(ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        },
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        checked(vec![
            integer(10),
            ASTNode::New {
                class: "Page".to_owned(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            },
            integer(11),
        ]),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        awaited(checked(vec![ASTNode::FieldAccess {
            object: Box::new(variable("page")),
            field: "value".to_owned(),
            span: Span::unknown(),
        }])),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        printed(ASTNode::FunctionCall {
            name: "isType".to_owned(),
            arguments: vec![
                integer(12),
                ASTNode::Literal {
                    value: LiteralValue::String("Integer".to_owned()),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        printed(ASTNode::MethodCall {
            object: Box::new(integer(13)),
            method: "is".to_owned(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::String("Integer".to_owned()),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        nowait(
            "pending",
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        map(vec![
            ("safe", integer(18)),
            (
                "unsafe",
                ASTNode::FieldAccess {
                    object: Box::new(variable("page")),
                    field: "value".to_owned(),
                    span: Span::unknown(),
                },
            ),
        ]),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        awaited(array(vec![ASTNode::New {
            class: "Page".to_owned(),
            arguments: Vec::new(),
            field_initializers: Vec::new(),
            type_arguments: Vec::new(),
            span: Span::unknown(),
        }])),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        grouped_assignment(
            "x",
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        indexed(
            ASTNode::New {
                class: "Page".to_owned(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            },
            integer(0),
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        indexed(
            array(vec![integer(28)]),
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "index".to_owned(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_selected(block_expr(
        vec![
            assignment(variable("x"), integer(33)),
            compound_assignment(variable("x"), BinaryOperator::Add, integer(1)),
        ],
        integer(34),
    ));
    assert_compatibility(
        assignment(
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            },
            integer(33),
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        compound_assignment(
            indexed(variable("items"), integer(0)),
            BinaryOperator::Add,
            integer(1),
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        returning(Some(ASTNode::FieldAccess {
            object: Box::new(variable("page")),
            field: "value".to_owned(),
            span: Span::unknown(),
        })),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        block_expr(vec![returning(Some(integer(1)))], integer(2)),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        scope_box(vec![ASTNode::If {
            condition: Box::new(integer(1)),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        }]),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        compound_assignment(
            variable("x"),
            BinaryOperator::Add,
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        assignment(
            variable("x"),
            ASTNode::New {
                class: "Page".to_owned(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        block_expr(
            vec![local(
                &["typed"],
                vec![Some(integer(35))],
                vec![Some("i64")],
            )],
            integer(36),
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        block_expr(
            Vec::new(),
            ASTNode::FieldAccess {
                object: Box::new(variable("page")),
                field: "value".to_owned(),
                span: Span::unknown(),
            },
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        local(&["typed"], vec![Some(integer(30))], vec![Some("i64")]),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        local(
            &["unsafe"],
            vec![Some(ASTNode::New {
                class: "Page".to_owned(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            })],
            vec![None],
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        task_scope(
            "co",
            vec![ASTNode::Return {
                value: Some(Box::new(integer(42))),
                span: Span::unknown(),
            }],
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        task_scope(
            "task_scope",
            vec![local(
                &["typed"],
                vec![Some(integer(43))],
                vec![Some("i64")],
            )],
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
    assert_compatibility(
        block_expr(
            vec![task_scope(
                "co",
                vec![ASTNode::FunctionCall {
                    name: "unsafe_child".to_owned(),
                    arguments: Vec::new(),
                    span: Span::unknown(),
                }],
            )],
            integer(44),
        ),
        RawNonProgramRootCompatibilityClassV1::SeparateDesignStop,
    );
}

#[test]
fn program_box_and_loop_keep_their_existing_root_owners() {
    assert!(matches!(
        PreparedRawRootPartitionV1::classify(ASTNode::Program {
            statements: vec![integer(1)],
            span: Span::unknown(),
        }),
        PreparedRawRootPartitionV1::Program { .. }
    ));
    assert_compatibility(
        first_statement("box Page {}"),
        RawNonProgramRootCompatibilityClassV1::ExplicitRoot,
    );
    assert_compatibility(
        first_statement("loop(true) { break }"),
        RawNonProgramRootCompatibilityClassV1::ExplicitRoot,
    );
}
