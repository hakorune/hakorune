use super::*;
use crate::ast::{BinaryOperator, LiteralValue, Span};
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{BinOpKind, JoinInst, MirLikeInst};

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

fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: op,
        left: Box::new(left),
        right: Box::new(right),
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

#[test]
fn if_sum_lowering_supports_i_mod_2_eq_1_filter() {
    let mut join_value_space = JoinValueSpace::new();
    let mut cond_env = ConditionEnv::new();
    let i_id = join_value_space.alloc_param();
    let len_id = join_value_space.alloc_param();
    cond_env.insert("i".to_string(), i_id);
    cond_env.insert("len".to_string(), len_id);

    let loop_condition = bin(BinaryOperator::Less, var("i"), var("len"));
    let if_condition = bin(
        BinaryOperator::Equal,
        bin(BinaryOperator::Modulo, var("i"), int_lit(2)),
        int_lit(1),
    );

    let sum_update = assignment(var("sum"), bin(BinaryOperator::Add, var("sum"), int_lit(1)));
    let counter_update = assignment(var("i"), bin(BinaryOperator::Add, var("i"), int_lit(1)));

    let if_stmt = ASTNode::If {
        condition: Box::new(if_condition),
        then_body: vec![sum_update],
        else_body: None,
        span: Span::unknown(),
    };
    let body = vec![if_stmt.clone(), counter_update];

    let (module, _meta) = lower_if_sum_pattern(
        &loop_condition,
        &if_stmt,
        &body,
        &cond_env,
        &mut join_value_space,
        &[],
    )
    .expect("if-sum lowering should succeed");

    let mut has_mod = false;
    let mut has_compare = false;

    for func in module.functions.values() {
        for inst in &func.body {
            match inst {
                JoinInst::Compute(MirLikeInst::BinOp {
                    op: BinOpKind::Mod, ..
                }) => {
                    has_mod = true;
                }
                JoinInst::Compute(MirLikeInst::Compare { .. }) => {
                    has_compare = true;
                }
                _ => {}
            }
        }
    }

    assert!(has_mod, "expected modulo lowering in JoinIR output");
    assert!(has_compare, "expected compare lowering in JoinIR output");
}
