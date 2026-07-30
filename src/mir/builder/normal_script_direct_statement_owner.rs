//! Selected-normal Script statement terminals that already have typed owners.
//!
//! Source classification stays in `normal_script_nonbox_statement_disposition`.
//! This sibling only hands a preselected statement to its existing production
//! owner through the caller's current invocation port.

use crate::ast::ASTNode;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::recursive_child_lowering::drive_legacy_expression_v1;
use crate::mir::builder::stmts::if_statement_descent::{
    complete_if_statement_v1, drive_raw_if_statement_with_port_v1,
};
use crate::mir::builder::stmts::print_stmt::{
    lower_prepared_raw_print_with_port_v1, PreparedRawPrintV1,
};
use crate::mir::{MirBuilder, ValueId};

pub(super) fn lower_direct_print_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::Print { expression, .. } = statement else {
        return Err("[freeze:contract][mir/script-runtime/print-source-drift]".to_owned());
    };
    lower_prepared_raw_print_with_port_v1(
        builder,
        port,
        PreparedRawPrintV1::prepare((**expression).clone()),
    )
}

pub(super) fn lower_direct_port_aware_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    drive_legacy_expression_v1(builder, port, statement.clone())
}

pub(super) fn lower_direct_if_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/if-source-drift]".to_owned());
    };
    builder.metadata_ctx.set_current_span(statement.span());
    let lowering = drive_raw_if_statement_with_port_v1(
        builder,
        port,
        (**condition).clone(),
        then_body.clone(),
        else_body.clone(),
    );
    complete_if_statement_v1(builder, lowering)
}

pub(super) fn lower_direct_static_const_runtime_completion_v1(
    builder: &mut MirBuilder,
    statement: &ASTNode,
) -> Result<ValueId, String> {
    if !matches!(statement, ASTNode::StaticConstTable { .. }) {
        return Err("[freeze:contract][mir/script-runtime/static-const-source-drift]".to_owned());
    }
    builder.metadata_ctx.set_current_span(statement.span());
    emit_void(builder)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::{ASTNode, BinaryOperator, CheckItem, LiteralValue, Span, UnaryOperator};
    use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

    fn integer(value: i64, line: usize) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::new(line, line + 1, line, 1),
        }
    }

    fn compare_normal_and_legacy(root: ASTNode, name: &str) {
        let program = ASTNode::Program {
            statements: vec![root],
            span: Span::new(0, 100, 1, 1),
        };
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler.compile_with_source(program.clone(), Some(name));
        let mut normal_compiler = MirCompiler::with_options(false);
        let normal = normal_compiler.compile_normal(
            NormalCompileRequestV1::for_mir_mode(program, Some(name), HashMap::new())
                .expect("normal request"),
        );

        match (normal, legacy) {
            (Ok(normal), Ok(legacy)) => {
                assert_eq!(
                    MirPrinter::new().print_module(&normal.module),
                    MirPrinter::new().print_module(&legacy.module),
                    "{name}"
                );
                assert_eq!(
                    normal.verification_result, legacy.verification_result,
                    "{name}"
                );
            }
            (Err(normal), Err(legacy)) => assert_eq!(normal, legacy, "{name}"),
            (normal, legacy) => panic!(
                "normal/legacy outcome drift for {name}: normal={}, legacy={}",
                normal.is_ok(),
                legacy.is_ok()
            ),
        }
    }

    #[test]
    fn direct_expression_roots_keep_full_legacy_outcomes_and_spans() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let literal = integer(1, 11);
        let variable = ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::new(20, 27, 20, 3),
        };
        let me = ASTNode::Me {
            span: Span::new(30, 32, 30, 5),
        };
        let binary = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(integer(2, 41)),
            right: Box::new(integer(3, 42)),
            span: Span::new(40, 43, 40, 1),
        };
        let unary = ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(binary),
            span: Span::new(39, 44, 39, 1),
        };
        let await_expression = ASTNode::AwaitExpression {
            expression: Box::new(integer(4, 51)),
            span: Span::new(50, 52, 50, 1),
        };
        let check = ASTNode::CheckExpr {
            name: Some("direct".to_owned()),
            items: vec![CheckItem {
                label: Some("item".to_owned()),
                expression: integer(1, 61),
            }],
            span: Span::new(60, 62, 60, 1),
        };
        let nested_call = ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(ASTNode::FunctionCall {
                name: "isType".to_owned(),
                arguments: vec![
                    integer(42, 71),
                    ASTNode::Literal {
                        value: LiteralValue::String("Integer".to_owned()),
                        span: Span::new(72, 79, 72, 4),
                    },
                ],
                span: Span::new(70, 80, 70, 2),
            }),
            span: Span::new(69, 81, 69, 1),
        };
        let function_call = ASTNode::FunctionCall {
            name: "isType".to_owned(),
            arguments: vec![
                integer(42, 82),
                ASTNode::Literal {
                    value: LiteralValue::String("Integer".to_owned()),
                    span: Span::new(83, 90, 83, 4),
                },
            ],
            span: Span::new(81, 91, 81, 1),
        };
        let method_call = ASTNode::MethodCall {
            object: Box::new(ASTNode::Literal {
                value: LiteralValue::String("abc".to_owned()),
                span: Span::new(92, 95, 92, 1),
            }),
            method: "length".to_owned(),
            arguments: Vec::new(),
            span: Span::new(92, 102, 92, 1),
        };
        let allocation = ASTNode::ArrayLiteral {
            elements: vec![integer(1, 103), integer(2, 104)],
            span: Span::new(103, 105, 103, 1),
        };
        let construction = ASTNode::New {
            class: "Page".to_owned(),
            arguments: Vec::new(),
            field_initializers: Vec::new(),
            type_arguments: Vec::new(),
            span: Span::new(106, 114, 106, 1),
        };
        let qmark = ASTNode::QMarkPropagate {
            expression: Box::new(ASTNode::Variable {
                name: "missing_qmark".to_owned(),
                span: Span::new(115, 116, 115, 2),
            }),
            span: Span::new(115, 117, 115, 1),
        };
        let match_expression = ASTNode::MatchExpr {
            scrutinee: Box::new(integer(1, 118)),
            arms: vec![(LiteralValue::Integer(1), integer(2, 119))],
            else_expr: Box::new(integer(3, 120)),
            span: Span::new(118, 121, 118, 1),
        };
        let lambda = ASTNode::Lambda {
            params: Vec::new(),
            body: vec![integer(1, 122)],
            span: Span::new(122, 124, 122, 1),
        };
        let block_expression = ASTNode::BlockExpr {
            prelude_stmts: vec![integer(1, 125)],
            tail_expr: Box::new(integer(2, 126)),
            span: Span::new(125, 127, 125, 1),
        };
        let void_return = ASTNode::Return {
            value: None,
            span: Span::new(128, 129, 128, 1),
        };
        let value_return = ASTNode::Return {
            value: Some(Box::new(function_call.clone())),
            span: Span::new(130, 131, 130, 1),
        };
        let static_table = ASTNode::StaticConstTable {
            name: "DIRECT_TABLE".to_owned(),
            element_type: "u16".to_owned(),
            values: vec![1, 2],
            span: Span::new(160, 161, 160, 1),
        };
        let if_without_else = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::new(162, 163, 162, 3),
            }),
            then_body: vec![integer(1, 163)],
            else_body: None,
            span: Span::new(162, 164, 162, 1),
        };
        let if_with_else = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(false),
                span: Span::new(165, 166, 165, 3),
            }),
            then_body: vec![integer(1, 166)],
            else_body: Some(vec![integer(2, 167)]),
            span: Span::new(165, 168, 165, 1),
        };

        for (root, name) in [
            (literal, "direct-literal.hako"),
            (variable, "direct-variable.hako"),
            (me, "direct-me.hako"),
            (unary, "direct-unary-binary.hako"),
            (await_expression, "direct-await.hako"),
            (check, "direct-check.hako"),
            (nested_call, "direct-nested-call.hako"),
            (function_call, "direct-function-call.hako"),
            (method_call, "direct-method-call.hako"),
            (allocation, "direct-array.hako"),
            (construction, "direct-new.hako"),
            (qmark, "direct-qmark.hako"),
            (match_expression, "direct-match.hako"),
            (lambda, "direct-lambda.hako"),
            (block_expression, "direct-block-expression.hako"),
            (void_return, "direct-void-return.hako"),
            (value_return, "direct-value-return.hako"),
            (static_table, "direct-static-table.hako"),
            (if_without_else, "direct-if-no-else.hako"),
            (if_with_else, "direct-if-else.hako"),
        ] {
            compare_normal_and_legacy(root, name);
        }
    }

    #[test]
    fn direct_expression_failure_discards_candidate_and_reuses_compiler() {
        let mut compiler = MirCompiler::with_options(false);
        let failing = ASTNode::Program {
            statements: vec![ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::new(10, 17, 10, 2),
            }],
            span: Span::unknown(),
        };
        let request = NormalCompileRequestV1::for_mir_mode(
            failing,
            Some("direct-failure.hako"),
            HashMap::new(),
        )
        .expect("failing request");
        let error = compiler
            .compile_normal(request)
            .expect_err("undefined direct expression must reject");
        assert!(error.contains("Undefined variable: missing"), "{error}");

        let fresh = ASTNode::Program {
            statements: vec![integer(7, 20)],
            span: Span::unknown(),
        };
        compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    fresh,
                    Some("direct-reuse.hako"),
                    HashMap::new(),
                )
                .expect("fresh request"),
            )
            .expect("fresh candidate after direct expression rejection");
    }

    #[test]
    fn statement_surface_fallthrough_roots_keep_exact_legacy_outcomes() {
        let variable = |name: &str, line| ASTNode::Variable {
            name: name.to_owned(),
            span: Span::new(line, line + 1, line, 1),
        };
        let boolean = |value, line| ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::new(line, line + 1, line, 1),
        };

        let roots = [
            (
                ASTNode::Assignment {
                    target: Box::new(variable("missing_assignment", 170)),
                    value: Box::new(integer(1, 171)),
                    span: Span::new(170, 172, 170, 1),
                },
                "direct-assignment.hako",
            ),
            (
                ASTNode::CompoundAssignment {
                    target: Box::new(variable("missing_compound", 173)),
                    operator: BinaryOperator::Add,
                    value: Box::new(integer(1, 174)),
                    span: Span::new(173, 175, 173, 1),
                },
                "direct-compound-assignment.hako",
            ),
            (
                ASTNode::Loop {
                    condition: Box::new(boolean(false, 176)),
                    body: Vec::new(),
                    span: Span::new(176, 177, 176, 1),
                },
                "direct-loop.hako",
            ),
            (
                ASTNode::Nowait {
                    variable: "pending".to_owned(),
                    expression: Box::new(integer(1, 178)),
                    span: Span::new(178, 179, 178, 1),
                },
                "direct-nowait.hako",
            ),
            (
                ASTNode::TaskScope {
                    body: vec![integer(1, 180)],
                    source_keyword: "co".to_owned(),
                    span: Span::new(180, 181, 180, 1),
                },
                "direct-task-scope.hako",
            ),
            (
                ASTNode::ContextScope {
                    name: "ctx".to_owned(),
                    declared_type_name: None,
                    value: Box::new(integer(1, 182)),
                    body: Vec::new(),
                    source_keyword: "context".to_owned(),
                    span: Span::new(182, 183, 182, 1),
                },
                "direct-context-scope.hako",
            ),
            (
                ASTNode::TryCatch {
                    try_body: vec![integer(1, 184)],
                    catch_clauses: Vec::new(),
                    finally_body: None,
                    span: Span::new(184, 185, 184, 1),
                },
                "direct-try-catch.hako",
            ),
            (
                ASTNode::Throw {
                    expression: Box::new(integer(1, 186)),
                    span: Span::new(186, 187, 186, 1),
                },
                "direct-throw.hako",
            ),
            (
                ASTNode::Local {
                    variables: vec!["local_value".to_owned()],
                    initial_values: vec![Some(Box::new(integer(1, 188)))],
                    declared_type_names: vec![None],
                    span: Span::new(188, 189, 188, 1),
                },
                "direct-local.hako",
            ),
            (
                ASTNode::ScopeBox {
                    body: vec![integer(1, 190)],
                    span: Span::new(190, 191, 190, 1),
                },
                "direct-scope-box.hako",
            ),
            (
                ASTNode::Outbox {
                    variables: vec!["out".to_owned()],
                    initial_values: vec![None],
                    span: Span::new(192, 193, 192, 1),
                },
                "direct-outbox.hako",
            ),
            (
                ASTNode::Program {
                    statements: vec![integer(1, 194)],
                    span: Span::new(194, 195, 194, 1),
                },
                "direct-nested-program.hako",
            ),
            (
                ASTNode::UsingStatement {
                    namespace_name: "std.math".to_owned(),
                    span: Span::new(196, 197, 196, 1),
                },
                "direct-using.hako",
            ),
        ];

        for (root, name) in roots {
            compare_normal_and_legacy(root, name);
        }
    }

    #[test]
    fn direct_return_stops_the_suffix_and_failure_keeps_compiler_reusable() {
        let request = |statements, name| {
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements,
                    span: Span::unknown(),
                },
                Some(name),
                HashMap::new(),
            )
            .expect("normal request")
        };
        let mut compiler = MirCompiler::with_options(false);
        let suffix = ASTNode::Print {
            expression: Box::new(ASTNode::Variable {
                name: "must_not_be_lowered".to_owned(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        compiler
            .compile_normal(request(
                vec![
                    ASTNode::Return {
                        value: Some(Box::new(integer(7, 140))),
                        span: Span::new(139, 141, 139, 1),
                    },
                    suffix,
                ],
                "direct-return-suffix.hako",
            ))
            .expect("Return must stop Script suffix descent");

        let error = compiler
            .compile_normal(request(
                vec![ASTNode::Return {
                    value: Some(Box::new(ASTNode::Variable {
                        name: "missing".to_owned(),
                        span: Span::unknown(),
                    })),
                    span: Span::unknown(),
                }],
                "direct-return-failure.hako",
            ))
            .expect_err("missing Return value must fail");
        assert!(error.contains("Undefined variable: missing"), "{error}");

        compiler
            .compile_normal(request(
                vec![ASTNode::Return {
                    value: Some(Box::new(integer(9, 150))),
                    span: Span::unknown(),
                }],
                "direct-return-reuse.hako",
            ))
            .expect("fresh request must reuse compiler after Return failure");
    }

    #[test]
    fn direct_if_branch_failure_discards_candidate_and_reuses_compiler() {
        let request = |condition, then_body, else_body, name| {
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![ASTNode::If {
                        condition: Box::new(condition),
                        then_body,
                        else_body,
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                },
                Some(name),
                HashMap::new(),
            )
            .expect("normal If request")
        };
        let missing = |name: &str| ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        };
        let boolean = |value| ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        };
        let mut compiler = MirCompiler::with_options(false);

        for (condition, then_body, else_body, expected, name) in [
            (
                missing("missing_condition"),
                Vec::new(),
                None,
                "missing_condition",
                "direct-if-condition-failure.hako",
            ),
            (
                boolean(true),
                vec![missing("missing_then")],
                Some(vec![missing("must_not_demand_else")]),
                "missing_then",
                "direct-if-then-failure.hako",
            ),
            (
                boolean(false),
                vec![integer(1, 210)],
                Some(vec![missing("missing_else")]),
                "missing_else",
                "direct-if-else-failure.hako",
            ),
        ] {
            let error = compiler
                .compile_normal(request(condition, then_body, else_body, name))
                .expect_err("selected If child failure must reject");
            assert!(error.contains(expected), "{name}: {error}");
        }

        compiler
            .compile_normal(request(
                boolean(true),
                vec![integer(7, 220)],
                Some(vec![integer(8, 221)]),
                "direct-if-reuse.hako",
            ))
            .expect("fresh If candidate after child failures");
    }
}
