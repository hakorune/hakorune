#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::policies::BodyLoweringPolicy;
    use super::super::v0::try_extract_generic_loop_v0_facts;
    use super::super::v1::try_extract_generic_loop_v1_facts;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_i(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn local_init(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            span: Span::unknown(),
        }
    }

    fn method_call(obj: &str, method: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(obj)),
            method: method.to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    fn exit_if_break(condition: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(condition),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn simple_general_if(condition: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(condition),
            then_body: vec![assign("tmp", lit_i(1))],
            else_body: Some(vec![assign("tmp", lit_i(2))]),
            span: Span::unknown(),
        }
    }

    fn assert_is_loop_var_plus_one(expr: &ASTNode, loop_var: &str) {
        match expr {
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                right,
                ..
            } => {
                let left_is_var = matches!(
                    left.as_ref(),
                    ASTNode::Variable { name, .. } if name == loop_var
                );
                let right_is_var = matches!(
                    right.as_ref(),
                    ASTNode::Variable { name, .. } if name == loop_var
                );
                let left_is_one = matches!(
                    left.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                );
                let right_is_one = matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                );
                assert!(
                    (left_is_var && right_is_one) || (right_is_var && left_is_one),
                    "expected {loop_var}+1 style step, got {:?}",
                    expr
                );
            }
            _ => panic!("expected binary add step, got {:?}", expr),
        }
    }

    #[test]
    fn generic_loop_v0_allows_loop_var_from_add_expr_in_condition() {
        let cond = bin(
            BinaryOperator::LessEqual,
            bin(BinaryOperator::Add, var("j"), var("m")),
            var("n"),
        );
        let body = vec![assign("j", bin(BinaryOperator::Add, var("j"), lit_i(1)))];

        let facts = try_extract_generic_loop_v0_facts(&cond, &body)
            .expect("no freeze")
            .expect("should match");
        assert_eq!(facts.loop_var, "j");
    }

    #[test]
    fn generic_loop_v0_rejects_control_flow_after_step() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let cond = bin(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
            ASTNode::Break {
                span: Span::unknown(),
            },
        ];

        let facts = try_extract_generic_loop_v0_facts(&cond, &body).expect("no freeze");
        assert!(facts.is_none());
    }

    #[test]
    fn generic_loop_v0_rejects_v1_shape_effect_step_only() {
        let cond = bin(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            method_call("s", "len"),
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
        ];

        let facts = try_extract_generic_loop_v0_facts(&cond, &body).expect("no freeze");
        assert!(facts.is_none());
    }

    #[test]
    fn generic_loop_v1_policy_exit_allowed_without_break() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let cond = bin(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local_init("tmp0", lit_i(0)),
            ASTNode::If {
                condition: Box::new(cond.clone()),
                then_body: vec![assign("tmp1", lit_i(1))],
                else_body: Some(vec![assign("tmp2", lit_i(2))]),
                span: Span::unknown(),
            },
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
        ];

        let facts = try_extract_generic_loop_v1_facts(&cond, &body)
            .expect("no freeze")
            .expect("should match");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
    }

    #[test]
    fn generic_loop_v1_policy_recipe_only_with_break() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let cond = bin(BinaryOperator::Less, var("i"), var("n"));
        let general_if = simple_general_if(cond.clone());
        let else_body = vec![
            local_init("tmp0", lit_i(0)),
            local_init("tmp1", lit_i(1)),
            assign("tmp2", lit_i(2)),
            assign("tmp3", lit_i(3)),
            simple_general_if(cond.clone()),
            assign("tmp4", lit_i(4)),
            local_init("tmp5", lit_i(5)),
            assign("tmp6", lit_i(6)),
            simple_general_if(cond.clone()),
            assign("tmp7", lit_i(7)),
            assign("tmp8", lit_i(8)),
            simple_general_if(cond.clone()),
        ];
        let parse_map_if = ASTNode::If {
            condition: Box::new(cond.clone()),
            then_body: vec![assign("tmp9", lit_i(9))],
            else_body: Some(else_body),
            span: Span::unknown(),
        };
        let body = vec![
            assign("tmp10", lit_i(10)),
            exit_if_break(cond.clone()),
            exit_if_break(cond.clone()),
            parse_map_if,
            assign("tmp11", lit_i(11)),
            general_if,
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
        ];

        let facts = try_extract_generic_loop_v1_facts(&cond, &body)
            .expect("no freeze")
            .expect("should match");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
    }

    #[test]
    fn generic_loop_nested_program_stmt_preserves_outer_loop_step_expr() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let outer_cond = bin(BinaryOperator::Less, var("i"), var("n"));
        let inner_cond = bin(BinaryOperator::Less, var("j"), var("n"));
        let inner_body = vec![
            ASTNode::If {
                condition: Box::new(bin(BinaryOperator::Equal, var("j"), lit_i(1))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign("j", bin(BinaryOperator::Add, var("j"), lit_i(1))),
        ];
        let body = vec![
            ASTNode::If {
                condition: Box::new(bin(BinaryOperator::Equal, var("i"), lit_i(2))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            ASTNode::Program {
                statements: vec![
                    local_init("j", lit_i(0)),
                    ASTNode::Loop {
                        condition: Box::new(inner_cond),
                        body: inner_body,
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            },
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
        ];

        if let Some(v0) = try_extract_generic_loop_v0_facts(&outer_cond, &body)
            .expect("v0: no freeze")
        {
            assert_eq!(v0.loop_var, "i");
            assert_is_loop_var_plus_one(&v0.loop_increment, "i");
        }

        let v1 = try_extract_generic_loop_v1_facts(&outer_cond, &body)
            .expect("v1: no freeze")
            .expect("v1 should match");
        assert_eq!(v1.loop_var, "i");
        assert_is_loop_var_plus_one(&v1.loop_increment, "i");
    }

    #[test]
    fn generic_loop_v1_accepts_if_condition_with_blockexpr_loop_prelude() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let cond = bin(BinaryOperator::Less, var("i"), lit_i(3));
        let prelude_loop = ASTNode::Loop {
            condition: Box::new(bin(BinaryOperator::Less, var("j"), lit_i(1))),
            body: vec![assign("j", bin(BinaryOperator::Add, var("j"), lit_i(1)))],
            span: Span::unknown(),
        };
        let if_cond = ASTNode::BlockExpr {
            prelude_stmts: vec![local_init("j", lit_i(0)), prelude_loop],
            tail_expr: Box::new(bin(BinaryOperator::Equal, var("j"), lit_i(1))),
            span: Span::unknown(),
        };
        let body = vec![
            ASTNode::If {
                condition: Box::new(if_cond),
                then_body: vec![assign("sum", bin(BinaryOperator::Add, var("sum"), lit_i(1)))],
                else_body: None,
                span: Span::unknown(),
            },
            assign("i", bin(BinaryOperator::Add, var("i"), lit_i(1))),
        ];

        let facts = try_extract_generic_loop_v1_facts(&cond, &body)
            .expect("v1: no freeze")
            .expect("v1 should match");
        assert_eq!(facts.loop_var, "i");
        assert!(facts.body_exit_allowed.is_some());
    }
}
