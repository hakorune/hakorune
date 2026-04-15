#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::cleanup::policies::p5b_escape_derived_policy::{
    classify_p5b_escape_derived, P5bEscapeDerivedDecision,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, Span};
    use crate::tests::helpers::joinir_env::with_joinir_env_lock;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }
    }

    fn int_lit(v: i64) -> ASTNode {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(v),
            span: Span::unknown(),
        }
    }

    fn binop(op: BinaryOperator, lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(lhs),
            right: Box::new(rhs),
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

    fn method_call(obj: &str, method: &str, args: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(obj)),
            method: method.to_string(),
            arguments: args,
            span: Span::unknown(),
        }
    }

    #[test]
    fn detects_p5b_shape_and_builds_recipe() {
        // Body layout:
        // 0: local ch = s.substring(i, i+1)
        // 1: if ch == "\"" { break }
        // 2: if ch == "\\" { i = i + 1; ch = s.substring(i, i+1) }
        // 3: i = i + 1
        let body = vec![
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![Some(Box::new(method_call(
                    "s",
                    "substring",
                    vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                )))],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\""))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\\"))),
                then_body: vec![
                    assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
                    assignment(
                        var("ch"),
                        method_call(
                            "s",
                            "substring",
                            vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                        ),
                    ),
                ],
                else_body: None,
                span: Span::unknown(),
            },
            assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
        ];

        match classify_p5b_escape_derived(&body, "i") {
            P5bEscapeDerivedDecision::Use(recipe) => {
                assert_eq!(recipe.name, "ch");
                assert_eq!(recipe.loop_counter_name, "i");
                assert_eq!(recipe.pre_delta, 1);
                assert_eq!(recipe.post_delta, 1);
                match recipe.override_expr {
                    ASTNode::MethodCall { ref method, .. } => assert_eq!(method, "substring"),
                    other => panic!("expected override MethodCall, got {:?}", other),
                }
            }
            other => panic!("expected UseDerived recipe, got {:?}", other),
        }
    }

    #[test]
    fn strict_rejects_when_local_init_missing() {
        with_joinir_env_lock(|| {
            // escape pattern exists, but `local ch = ...` is absent -> strict should reject
            let body = vec![
                ASTNode::If {
                    condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\""))),
                    then_body: vec![ASTNode::Break {
                        span: Span::unknown(),
                    }],
                    else_body: None,
                    span: Span::unknown(),
                },
                ASTNode::If {
                    condition: Box::new(binop(BinaryOperator::Equal, var("ch"), str_lit("\\"))),
                    then_body: vec![
                        assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
                        assignment(
                            var("ch"),
                            method_call(
                                "s",
                                "substring",
                                vec![var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))],
                            ),
                        ),
                    ],
                    else_body: None,
                    span: Span::unknown(),
                },
                assignment(var("i"), binop(BinaryOperator::Add, var("i"), int_lit(1))),
            ];

            let prev = crate::config::env::joinir_dev::strict_enabled();
            std::env::set_var("HAKO_JOINIR_STRICT", "1");
            let decision = classify_p5b_escape_derived(&body, "i");
            if prev {
                std::env::set_var("HAKO_JOINIR_STRICT", "1");
            } else {
                std::env::remove_var("HAKO_JOINIR_STRICT");
            }

            match decision {
                P5bEscapeDerivedDecision::Reject(reason) => {
                    assert!(
                        reason.contains("missing_local_init"),
                        "unexpected reason: {}",
                        reason
                    );
                }
                other => panic!("expected Reject, got {:?}", other),
            }
        });
    }
}
