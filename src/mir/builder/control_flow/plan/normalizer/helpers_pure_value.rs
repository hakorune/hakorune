pub(super) fn is_pure_value_expr(ast: &crate::ast::ASTNode) -> bool {
    use crate::ast::{ASTNode, BinaryOperator};

    fn is_known_pure_method_call_for_value_if(object: &ASTNode, method: &str) -> bool {
        if matches!(
            (object, method),
            // Stage-B/JsonFrag normalizer uses ternary value-if with this helper.
            // It is deterministic and side-effect free for the current semantics.
            (ASTNode::Variable { name, .. }, "int_to_str") if name == "StringHelpers"
        ) {
            return true;
        }

        // Selfhost FuncLowering uses ternary value-if with String slice helpers.
        // These methods are pure reads and safe for Select-based lowering.
        if matches!(method, "substring" | "length" | "contains") {
            return matches!(
                object,
                ASTNode::Variable { .. }
                    | ASTNode::FieldAccess { .. }
                    | ASTNode::ThisField { .. }
                    | ASTNode::MeField { .. }
            );
        }

        false
    }

    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            is_known_pure_method_call_for_value_if(object, method)
                && arguments.iter().all(is_pure_value_expr)
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let Some(else_body) = else_body else {
                return false;
            };
            if then_body.len() != 1 || else_body.len() != 1 {
                return false;
            }
            is_pure_value_expr(condition)
                && is_pure_value_expr(&then_body[0])
                && is_pure_value_expr(&else_body[0])
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => prelude_stmts.is_empty() && is_pure_value_expr(tail_expr),
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr(left)
                && is_pure_value_expr(right)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_pure_value_expr;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
    use crate::mir::builder::control_flow::plan::CoreEffectPlan;
    use crate::mir::builder::control_flow::plan::PlanNormalizer;
    use crate::mir::builder::MirBuilder;
    use std::collections::BTreeMap;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    fn empty_blockexpr(tail_expr: ASTNode) -> ASTNode {
        ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(tail_expr),
            span: Span::unknown(),
        }
    }

    fn blockexpr(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr: Box::new(tail_expr),
            span: Span::unknown(),
        }
    }

    fn local_stmt(name: &str, init: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(init))],
            span: Span::unknown(),
        }
    }

    fn assign_stmt(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn value_if_allows_pure_string_substring() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(var("dot")),
            right: Box::new(int_lit(0)),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::MethodCall {
            object: Box::new(var("last_val")),
            method: "substring".to_string(),
            arguments: vec![int_lit(0), var("dot")],
            span: Span::unknown(),
        };
        let else_expr = var("last_val");
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr(&value_if));
    }

    #[test]
    fn value_if_allows_empty_blockexpr_wrapped_branches() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("a")),
            right: Box::new(var("b")),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(int_lit(10)),
            span: Span::unknown(),
        };
        let else_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(int_lit(20)),
            span: Span::unknown(),
        };
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr(&value_if));
    }

    #[test]
    fn value_if_rejects_blockexpr_with_prelude_side_effect() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("a")),
            right: Box::new(var("b")),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Local {
                variables: vec!["tmp".to_string()],
                initial_values: vec![Some(Box::new(int_lit(1)))],
                span: Span::unknown(),
            }],
            tail_expr: Box::new(int_lit(10)),
            span: Span::unknown(),
        };
        let else_expr = int_lit(20);
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(!is_pure_value_expr(&value_if));
    }

    #[test]
    fn lower_value_ast_accepts_map_literal_and_emits_set_calls() {
        let map_expr = ASTNode::MapLiteral {
            entries: vec![("x".to_string(), int_lit(1)), ("y".to_string(), int_lit(2))],
            span: Span::unknown(),
        };
        let mut builder = MirBuilder::new();
        let (map_id, effects) =
            PlanNormalizer::lower_value_ast(&map_expr, &mut builder, &BTreeMap::new())
                .expect("MapLiteral should lower in value context");

        match effects.first() {
            Some(CoreEffectPlan::NewBox {
                dst,
                box_type,
                args,
            }) => {
                assert_eq!(*dst, map_id);
                assert_eq!(box_type, "MapBox");
                assert!(args.is_empty());
            }
            other => panic!("first effect must be NewBox(MapBox), got {:?}", other),
        }
        match effects.get(1) {
            Some(CoreEffectPlan::MethodCall {
                dst: None,
                object,
                method,
                args,
                ..
            }) => {
                assert_eq!(*object, map_id);
                assert_eq!(method, "birth");
                assert!(args.is_empty());
            }
            other => panic!("second effect must be birth() call, got {:?}", other),
        }

        let set_calls = effects
            .iter()
            .filter(|effect| {
                if let CoreEffectPlan::MethodCall {
                    dst: None,
                    object,
                    method,
                    args,
                    ..
                } = effect
                {
                    *object == map_id && method == "set" && args.len() == 2
                } else {
                    false
                }
            })
            .count();
        assert_eq!(set_calls, 2);
    }

    #[test]
    fn lower_value_ast_accepts_empty_blockexpr_wrapper() {
        let wrapped_expr = empty_blockexpr(int_lit(42));

        let mut wrapped_builder = MirBuilder::new();
        let (wrapped_id, wrapped_effects) = PlanNormalizer::lower_value_ast(
            &wrapped_expr,
            &mut wrapped_builder,
            &BTreeMap::new(),
        )
        .expect("empty BlockExpr should lower in value context");

        let mut tail_builder = MirBuilder::new();
        let (tail_id, tail_effects) =
            PlanNormalizer::lower_value_ast(&int_lit(42), &mut tail_builder, &BTreeMap::new())
                .expect("tail literal should lower in value context");

        assert_eq!(
            format!("{:?}", wrapped_effects),
            format!("{:?}", tail_effects)
        );
        assert_eq!(
            wrapped_builder.type_ctx.get_type(wrapped_id),
            tail_builder.type_ctx.get_type(tail_id)
        );
    }

    #[test]
    fn lower_value_ast_accepts_bool_or_with_unary_not() {
        let expr = ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(bool_lit(true)),
                span: Span::unknown(),
            }),
            right: Box::new(bool_lit(false)),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr(&expr));
    }

    #[test]
    fn lower_value_ast_accepts_blockexpr_with_local_prelude() {
        let expr = blockexpr(vec![local_stmt("tmp", int_lit(10))], var("tmp"));

        let mut builder = MirBuilder::new();
        let (value_id, effects) =
            PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
                .expect("BlockExpr prelude should lower in value context");

        assert_eq!(builder.variable_ctx.variable_map.get("tmp"), Some(&value_id));
        assert!(matches!(
            effects.first(),
            Some(CoreEffectPlan::Const {
                value: crate::mir::ConstValue::Integer(10),
                ..
            })
        ));
    }

    #[test]
    fn lower_value_ast_blockexpr_if_merges_only_preexisting_bindings() {
        let expr = blockexpr(
            vec![
                local_stmt("a", int_lit(0)),
                ASTNode::If {
                    condition: Box::new(bool_lit(true)),
                    then_body: vec![
                        assign_stmt("a", int_lit(10)),
                        local_stmt("tmp", int_lit(1)),
                    ],
                    else_body: Some(vec![
                        assign_stmt("a", int_lit(20)),
                        local_stmt("tmp", int_lit(2)),
                    ]),
                    span: Span::unknown(),
                },
            ],
            var("a"),
        );

        let mut builder = MirBuilder::new();
        let (value_id, effects) =
            PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
                .expect("BlockExpr if-prelude should lower in value context");

        let select_dst = effects
            .iter()
            .find_map(|effect| {
                if let CoreEffectPlan::Select { dst, .. } = effect {
                    Some(*dst)
                } else {
                    None
                }
            })
            .expect("merged pre-existing binding should produce Select");
        assert_eq!(value_id, select_dst);
        assert_eq!(builder.variable_ctx.variable_map.get("a"), Some(&value_id));
        assert!(!builder.variable_ctx.variable_map.contains_key("tmp"));
    }

    #[test]
    fn lower_value_ast_blockexpr_exit_in_prelude_is_forbidden() {
        let expr = blockexpr(
            vec![ASTNode::If {
                condition: Box::new(bool_lit(true)),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(int_lit(1))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            }],
            int_lit(0),
        );

        let mut builder = MirBuilder::new();
        let err = PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
            .expect_err("BlockExpr prelude exit must stay fail-fast");
        assert!(err.contains("[freeze:contract][blockexpr]"));
    }
}
