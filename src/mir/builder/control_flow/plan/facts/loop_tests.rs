//! Tests for LoopFacts

#![allow(dead_code)]

#[cfg(test)]
mod tests_invariants {
    use super::super::{try_build_loop_facts, LoopFacts};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[allow(dead_code)]
    fn len_call(var: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(var)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn loop_facts_require_skeleton_and_features_when_present() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let facts = try_build_loop_facts(&condition, &body)
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.skeleton.kind, SkeletonKind::Loop);
        assert!(!facts.features.exit_usage.has_break);
        assert!(!facts.features.exit_usage.has_continue);
        assert!(!facts.features.exit_usage.has_return);
        let _: LoopFacts = facts;
    }
}

#[cfg(test)]
mod tests {
    use super::super::{try_build_loop_facts, try_build_loop_facts_with_ctx};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::planner::PlannerContext;
    use crate::mir::loop_pattern_detection::LoopPatternKind;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn len_call(var: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(var)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn loopfacts_ok_some_for_canonical_scan_with_init_minimal() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("ch")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt, step]).expect("Ok");
        assert!(facts.is_some());
    }

    #[test]
    fn loopfacts_ok_some_for_reverse_scan_with_init_minimal() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("ch")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt, step])
            .expect("Ok")
            .expect("Some");
        let scan = facts.scan_with_init.expect("scan facts");
        assert_eq!(scan.loop_var, "i");
        assert_eq!(scan.haystack, "s");
        assert_eq!(scan.step_lit, -1);
    }

    #[test]
    fn loopfacts_ok_some_for_reverse_scan_with_init_starts_with() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::MethodCall {
                object: Box::new(v("me")),
                method: "starts_with".to_string(),
                arguments: vec![v("s"), v("i"), v("pat")],
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt, step])
            .expect("Ok")
            .expect("Some");
        let scan = facts.scan_with_init.expect("scan facts");
        assert_eq!(scan.loop_var, "i");
        assert_eq!(scan.haystack, "s");
        assert_eq!(scan.needle, "pat");
        assert_eq!(scan.step_lit, -1);
        assert!(scan.dynamic_needle);
    }

    #[test]
    fn loopfacts_ok_some_for_dynamic_needle_scan_with_init() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(len_call("s")),
                right: Box::new(len_call("needle")),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(len_call("needle")),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("needle")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt, step])
            .expect("Ok")
            .expect("Some");
        let scan = facts.scan_with_init.expect("scan facts");
        assert_eq!(scan.loop_var, "i");
        assert_eq!(scan.haystack, "s");
        assert_eq!(scan.needle, "needle");
        assert_eq!(scan.step_lit, 1);
        assert!(scan.dynamic_needle);
    }

    #[test]
    fn loopfacts_ok_some_for_index_of_add_bound_return_step() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("j")),
                right: Box::new(v("m")),
                span: Span::unknown(),
            }),
            right: Box::new(v("n")),
            span: Span::unknown(),
        };
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::MethodCall {
                object: Box::new(v("me")),
                method: "starts_with".to_string(),
                arguments: vec![v("src"), v("j"), v("pat")],
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("j"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("j")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("j")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt, step])
            .expect("Ok")
            .expect("Some");
        assert!(
            facts.scan_with_init.is_some() || facts.loop_cond_return_in_body.is_some(),
            "expected scan_with_init or loop_cond_return_in_body facts"
        );
    }

    #[test]
    fn loopfacts_ctx_skips_pattern1_when_kind_mismatch() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let ctx = PlannerContext {
            pattern_kind: Some(LoopPatternKind::Pattern2Break),
            in_static_box: false,
            debug: false,
        };

        let facts = try_build_loop_facts_with_ctx(&ctx, &condition, &[step]).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn loopfacts_ctx_allows_pattern1_when_kind_matches() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let ctx = PlannerContext {
            pattern_kind: Some(LoopPatternKind::Pattern1SimpleWhile),
            in_static_box: false,
            debug: false,
        };

        let facts = try_build_loop_facts_with_ctx(&ctx, &condition, &[step]).expect("Ok");
        let facts = facts.expect("Some");
        assert!(facts.pattern1_simplewhile.is_some());
    }

    #[test]
    fn loopfacts_ctx_allows_pattern8_in_static_box() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let predicate_if = ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: crate::ast::UnaryOperator::Not,
                operand: Box::new(ASTNode::MethodCall {
                    object: Box::new(ASTNode::This {
                        span: Span::unknown(),
                    }),
                    method: "is_digit".to_string(),
                    arguments: vec![ASTNode::MethodCall {
                        object: Box::new(v("s")),
                        method: "substring".to_string(),
                        arguments: vec![
                            v("i"),
                            ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(v("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            },
                        ],
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = vec![predicate_if, step];
        let allow_ctx = PlannerContext {
            pattern_kind: None,
            in_static_box: false,
            debug: false,
        };
        let block_ctx = PlannerContext {
            pattern_kind: None,
            in_static_box: true,
            debug: false,
        };

        let allow = try_build_loop_facts_with_ctx(&allow_ctx, &condition, &body).expect("Ok");
        assert!(allow
            .as_ref()
            .and_then(|facts| facts.pattern8_bool_predicate_scan.as_ref())
            .is_some());

        let allow_static = try_build_loop_facts_with_ctx(&block_ctx, &condition, &body).expect("Ok");
        assert!(allow_static
            .as_ref()
            .and_then(|facts| facts.pattern8_bool_predicate_scan.as_ref())
            .is_some());
    }

    #[test]
    fn loopfacts_ok_none_when_condition_not_supported() {
        let condition = v("i"); // not `i < n`
        let facts = try_build_loop_facts(&condition, &[]).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn loopfacts_ok_none_when_step_var_differs_from_condition_var() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("j")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("j")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[step]).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn loopfacts_ok_some_for_canonical_split_scan_minimal() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("separator")),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::MethodCall {
                                object: Box::new(v("separator")),
                                method: "length".to_string(),
                                arguments: vec![],
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("separator")),
                span: Span::unknown(),
            }),
            then_body: vec![
                ASTNode::MethodCall {
                    object: Box::new(v("result")),
                    method: "push".to_string(),
                    arguments: vec![ASTNode::MethodCall {
                        object: Box::new(v("s")),
                        method: "substring".to_string(),
                        arguments: vec![v("start"), v("i")],
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
                    target: Box::new(v("start")),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(ASTNode::MethodCall {
                            object: Box::new(v("separator")),
                            method: "length".to_string(),
                            arguments: vec![],
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
                    target: Box::new(v("i")),
                    value: Box::new(v("start")),
                    span: Span::unknown(),
                },
            ],
            else_body: Some(vec![ASTNode::Assignment {
                target: Box::new(v("i")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("i")),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt]).expect("Ok");
        let split_scan = facts.and_then(|facts| facts.split_scan);
        let split_scan = split_scan.expect("SplitScan facts");
        assert_eq!(split_scan.s_var, "s");
        assert_eq!(split_scan.sep_var, "separator");
        assert_eq!(split_scan.result_var, "result");
        assert_eq!(split_scan.i_var, "i");
        assert_eq!(split_scan.start_var, "start");
        assert!(matches!(
            split_scan.shape,
            crate::mir::builder::control_flow::plan::facts::scan_shapes::SplitScanShape::Minimal
        ));
    }

    #[test]
    fn loopfacts_ok_none_when_split_scan_missing_else() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("separator")),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::MethodCall {
                                object: Box::new(v("separator")),
                                method: "length".to_string(),
                                arguments: vec![],
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("separator")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::MethodCall {
                object: Box::new(v("result")),
                method: "push".to_string(),
                arguments: vec![ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![v("start"), v("i")],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };

        let facts = try_build_loop_facts(&condition, &[if_stmt]).expect("Ok");
        assert!(facts.is_none());
    }
}
