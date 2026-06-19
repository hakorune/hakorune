use super::super::try_build_loop_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakAcceptKind;
use std::ffi::OsString;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvRestore {
    joinir_dev: Option<OsString>,
    planner_required: Option<OsString>,
}

impl EnvRestore {
    fn set_planner_required() -> Self {
        let restore = Self {
            joinir_dev: std::env::var_os("NYASH_JOINIR_DEV"),
            planner_required: std::env::var_os("HAKO_JOINIR_PLANNER_REQUIRED"),
        };
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");
        restore
    }
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        restore_var("NYASH_JOINIR_DEV", self.joinir_dev.take());
        restore_var("HAKO_JOINIR_PLANNER_REQUIRED", self.planner_required.take());
    }
}

fn restore_var(name: &str, value: Option<OsString>) {
    match value {
        Some(value) => std::env::set_var(name, value),
        None => std::env::remove_var(name),
    }
}

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_i(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bin(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(v(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

#[test]
fn loopfacts_keeps_loop_cond_owner_for_multi_candidate_conditional_update() {
    let _lock = ENV_LOCK.lock().expect("joinir env lock poisoned");
    let _restore = EnvRestore::set_planner_required();
    let condition = bin(
        BinaryOperator::Or,
        bin(BinaryOperator::Greater, v("a"), lit_i(0)),
        bin(BinaryOperator::Greater, v("b"), lit_i(0)),
    );
    let body = vec![
        assign("abit", bin(BinaryOperator::Modulo, v("a"), lit_i(2))),
        assign("bbit", bin(BinaryOperator::Modulo, v("b"), lit_i(2))),
        ASTNode::If {
            condition: Box::new(bin(
                BinaryOperator::And,
                bin(BinaryOperator::Equal, v("abit"), lit_i(1)),
                bin(BinaryOperator::Equal, v("bbit"), lit_i(1)),
            )),
            then_body: vec![assign("out", bin(BinaryOperator::Add, v("out"), v("bit")))],
            else_body: None,
            span: Span::unknown(),
        },
        assign("a", bin(BinaryOperator::Divide, v("a"), lit_i(2))),
        assign("b", bin(BinaryOperator::Divide, v("b"), lit_i(2))),
        assign("bit", bin(BinaryOperator::Multiply, v("bit"), lit_i(2))),
    ];

    let facts = try_build_loop_facts(&condition, &body)
        .expect("multi-candidate loop facts should not freeze")
        .expect("loop_cond owner should produce facts");
    let loop_cond = facts
        .loop_cond_break_continue
        .expect("loop_cond_break_continue owns conditional-update multi-candidate loop");
    assert_eq!(
        loop_cond.accept_kind,
        LoopCondBreakAcceptKind::ConditionalUpdate
    );
    assert!(facts.generic_loop_v1.is_none());
}
