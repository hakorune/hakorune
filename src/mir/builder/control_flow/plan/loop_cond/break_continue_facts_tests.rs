use super::super::MAX_NESTED_LOOPS;
use super::try_extract_loop_cond_break_continue_facts_inner;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::policies::BodyLoweringPolicy;

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

fn cond_lt(var: &str, value: i64) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v(var)),
        right: Box::new(lit_int(value)),
        span: Span::unknown(),
    }
}

fn cond_eq_zero(var: &str) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(v(var)),
        right: Box::new(lit_int(0)),
        span: Span::unknown(),
    }
}

fn cond_ge_zero(var: &str) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::GreaterEqual,
        left: Box::new(v(var)),
        right: Box::new(lit_int(0)),
        span: Span::unknown(),
    }
}

fn cond_gt(var: &str, value: i64) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Greater,
        left: Box::new(v(var)),
        right: Box::new(lit_int(value)),
        span: Span::unknown(),
    }
}

fn assign_inc(var: &str) -> ASTNode {
    assign_add(var, 1)
}

fn assign_add(var: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(v(var)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v(var)),
            right: Box::new(lit_int(value)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn local_int(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(lit_int(value)))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn if_break_eq_zero(var: &str) -> ASTNode {
    ASTNode::If {
        condition: Box::new(cond_eq_zero(var)),
        then_body: vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    }
}

#[test]
fn policy_recipe_only_when_not_extended() {
    let condition = cond_lt("i", 1);
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond_eq_zero("i")),
            then_body: vec![assign_inc("i")],
            else_body: None,
            span: Span::unknown(),
        },
        assign_inc("i"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        false,
        false,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
}

#[test]
fn policy_recipe_only_with_then_only_break() {
    let condition = cond_lt("i", 2);
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond_eq_zero("i")),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: Some(vec![assign_inc("i")]),
            span: Span::unknown(),
        },
        assign_inc("i"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        true,
        true,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
}

#[test]
fn accepts_nested_guard_break_if_as_program_block_recipe_only() {
    let condition = cond_lt("scan", 10);
    let body = vec![
        local_int("t_idx", 1),
        if_break_eq_zero("t_idx"),
        local_int("atype", 1),
        if_break_eq_zero("atype"),
        ASTNode::If {
            condition: Box::new(cond_eq_zero("atype")),
            then_body: vec![
                local_int("v_idx", 1),
                if_break_eq_zero("v_idx"),
                local_int("x", 1),
            ],
            else_body: Some(vec![
                local_int("n_idx", 2),
                if_break_eq_zero("n_idx"),
                local_int("y", 2),
            ]),
            span: Span::unknown(),
        },
        assign_inc("scan"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        true,
        true,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
    assert!(matches!(
        facts.recipe.items[4],
        LoopCondBreakContinueItem::ProgramBlock {
            stmt_only: None,
            ..
        }
    ));
}

#[test]
fn accepts_nested_loop_if_as_program_block_recipe_only() {
    let condition = cond_lt("pos", 10);
    let nested_loop = ASTNode::Loop {
        condition: Box::new(cond_lt("j", 3)),
        body: vec![if_break_eq_zero("j"), assign_inc("j")],
        span: Span::unknown(),
    };
    let body = vec![
        local_int("name_idx", 1),
        if_break_eq_zero("name_idx"),
        local_int("params_idx", 1),
        ASTNode::If {
            condition: Box::new(cond_ge_zero("params_idx")),
            then_body: vec![local_int("j", 0), nested_loop],
            else_body: None,
            span: Span::unknown(),
        },
        assign_inc("pos"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        true,
        false,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
    assert!(matches!(
        facts.recipe.items[3],
        LoopCondBreakContinueItem::ProgramBlock {
            stmt_only: None,
            ..
        }
    ));
}

#[test]
fn accepts_multidelta_break_continue_as_recipe_only() {
    let condition = cond_lt("j", 6);
    let body = vec![
        ASTNode::If {
            condition: Box::new(cond_eq_zero("j")),
            then_body: vec![
                assign_add("out", 10),
                assign_add("j", 2),
                ASTNode::Continue {
                    span: Span::unknown(),
                },
            ],
            else_body: None,
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(v("j")),
                right: Box::new(lit_int(2)),
                span: Span::unknown(),
            }),
            then_body: vec![
                assign_add("out", 20),
                assign_add("j", 3),
                ASTNode::Continue {
                    span: Span::unknown(),
                },
            ],
            else_body: None,
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(cond_gt("j", 10)),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign_add("out", 1),
        assign_inc("j"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        true,
        true,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
}

#[test]
fn program_block_with_exit_signals_prefers_recipe_only() {
    let condition = cond_lt("j", 3);
    let body = vec![
        assign_inc("j"),
        ASTNode::If {
            condition: Box::new(cond_eq_zero("j")),
            then_body: vec![
                assign_inc("x"),
                ASTNode::Continue {
                    span: Span::unknown(),
                },
            ],
            else_body: Some(vec![
                ASTNode::If {
                    condition: Box::new(cond_ge_zero("x")),
                    then_body: vec![assign_inc("x")],
                    else_body: Some(vec![assign_inc("x")]),
                    span: Span::unknown(),
                },
                ASTNode::Break {
                    span: Span::unknown(),
                },
            ]),
            span: Span::unknown(),
        },
        assign_inc("j"),
    ];

    let facts = try_extract_loop_cond_break_continue_facts_inner(
        &condition,
        &body,
        true,
        true,
        false,
        MAX_NESTED_LOOPS,
        None,
    )
    .expect("freeze")
    .expect("facts");

    assert!(matches!(
        facts.recipe.items[1],
        LoopCondBreakContinueItem::ProgramBlock { .. }
    ));
    assert!(matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    ));
}
