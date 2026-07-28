use crate::ast::{ASTNode, BinaryOperator, CheckItem, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_raw_legacy_expression_v1, RawInvocationChildPortV1,
};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{MirBuilder, MirType};
use crate::parser::NyashParser;

use super::{
    PreparedRawNonProgramRootV1, PreparedRawRootPartitionV1, RawNonProgramRootCompatibilityClassV1,
    SelectedRawNonProgramRootV1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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
}

fn spanned_instructions(builder: &MirBuilder) -> Vec<(String, Span)> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.all_spanned_instructions())
        .map(|instruction| (format!("{:?}", instruction.inst), instruction.span))
        .collect()
}

#[test]
fn selected_print_root_matches_the_raw_legacy_port_exactly() {
    let root = || printed(awaited(checked(vec![integer(14), integer(15)])));
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("print_root_parity/0".to_owned());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("print_root_parity/0".to_owned());
    let selected_value = {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut selected,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            drive_legacy_expression_v1(builder, &mut port, root())
        })
    }
    .unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
}

#[test]
fn selected_nowait_root_matches_raw_legacy_effects_exactly() {
    let root = || nowait("pending", checked(vec![integer(16), integer(17)]));
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("nowait_root_parity/0".to_owned());
    legacy.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("nowait_root_parity/0".to_owned());
    selected.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let selected_value = {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut selected,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            drive_legacy_expression_v1(builder, &mut port, root())
        })
    }
    .unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    let selected_binding = selected
        .function_state
        .variable_ctx
        .variable_map
        .get("pending");
    let legacy_binding = legacy
        .function_state
        .variable_ctx
        .variable_map
        .get("pending");
    assert_eq!(selected_binding, Some(&selected_value));
    assert_eq!(legacy_binding, Some(&legacy_value));
    assert_eq!(
        selected
            .function_state
            .type_ctx
            .value_types
            .get(&selected_value),
        legacy
            .function_state
            .type_ctx
            .value_types
            .get(&legacy_value)
    );
    assert!(matches!(
        selected
            .function_state
            .type_ctx
            .value_types
            .get(&selected_value),
        Some(MirType::Future(inner)) if **inner == MirType::Integer
    ));
    let selected_slot = selected
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .and_then(|registry| registry.get_slot("pending"));
    let legacy_slot = legacy
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .and_then(|registry| registry.get_slot("pending"));
    assert_eq!(selected_slot, legacy_slot);
    assert!(selected_slot.is_some());
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
