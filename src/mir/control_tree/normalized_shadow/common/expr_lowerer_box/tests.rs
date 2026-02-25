use super::super::expr_lowering_contract::{ExprLoweringScope, ImpurePolicy, OutOfScopeReason};
use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, JoinInst, MirLikeInst, UnaryOp};
use crate::mir::types::MirType;
use crate::mir::ValueId;
use std::collections::BTreeMap;

fn span() -> Span {
    Span::unknown()
}

#[test]
fn lower_var_from_env() {
    let mut env = BTreeMap::new();
    env.insert("x".to_string(), ValueId(42));
    let mut body = vec![];
    let mut next = 100;

    let ast = ASTNode::Variable {
        name: "x".to_string(),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(42)));
    assert!(body.is_empty());
    assert_eq!(next, 100);
}

#[test]
fn out_of_scope_var_missing() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 100;

    let ast = ASTNode::Variable {
        name: "missing".to_string(),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, None);
    assert!(body.is_empty());
    assert_eq!(next, 100);
}

#[test]
fn lower_int_literal_const_emits() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 100;

    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(7),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(100)));
    assert_eq!(next, 101);
    assert!(matches!(
        body.as_slice(),
        [JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(100),
            value: ConstValue::Integer(7)
        })]
    ));
}

#[test]
fn lower_bool_literal_const_emits() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 100;

    let ast = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(100)));
    assert_eq!(next, 101);
    assert!(matches!(
        body.as_slice(),
        [JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(100),
            value: ConstValue::Bool(true)
        })]
    ));
}

#[test]
fn lower_unary_minus_int() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 10;

    let ast = ASTNode::UnaryOp {
        operator: UnaryOperator::Minus,
        operand: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: span(),
        }),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(11)));
    assert_eq!(next, 12);
    assert!(matches!(
        body.as_slice(),
        [
            JoinInst::Compute(MirLikeInst::Const { .. }),
            JoinInst::Compute(MirLikeInst::UnaryOp {
                dst: ValueId(11),
                op: UnaryOp::Neg,
                ..
            })
        ]
    ));
}

#[test]
fn lower_unary_not_bool() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 10;

    let ast = ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(false),
            span: span(),
        }),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(11)));
    assert_eq!(next, 12);
    assert!(matches!(
        body.as_slice(),
        [
            JoinInst::Compute(MirLikeInst::Const { .. }),
            JoinInst::Compute(MirLikeInst::UnaryOp {
                dst: ValueId(11),
                op: UnaryOp::Not,
                ..
            })
        ]
    ));
}

#[test]
fn lower_add_sub_mul_div_ints() {
    let mut env = BTreeMap::new();
    env.insert("x".to_string(), ValueId(1));
    let mut body = vec![];
    let mut next = 100;

    let add = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(2),
            span: span(),
        }),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&add, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, Some(ValueId(101)));

    let sub = ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(5),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: span(),
        }),
        span: span(),
    };
    let _ = NormalizedExprLowererBox::lower_expr(&sub, &env, &mut body, &mut next).unwrap();

    let mul = ASTNode::BinaryOp {
        operator: BinaryOperator::Multiply,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(6),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(7),
            span: span(),
        }),
        span: span(),
    };
    let _ = NormalizedExprLowererBox::lower_expr(&mul, &env, &mut body, &mut next).unwrap();

    let div = ASTNode::BinaryOp {
        operator: BinaryOperator::Divide,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(8),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(2),
            span: span(),
        }),
        span: span(),
    };
    let _ = NormalizedExprLowererBox::lower_expr(&div, &env, &mut body, &mut next).unwrap();

    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::BinOp {
            op: BinOpKind::Add,
            ..
        })
    )));
    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::BinOp {
            op: BinOpKind::Sub,
            ..
        })
    )));
    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::BinOp {
            op: BinOpKind::Mul,
            ..
        })
    )));
    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::BinOp {
            op: BinOpKind::Div,
            ..
        })
    )));
}

#[test]
fn lower_compare_eq_lt_ints() {
    let mut env = BTreeMap::new();
    env.insert("x".to_string(), ValueId(1));
    let mut body = vec![];
    let mut next = 100;

    let eq = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: span(),
        }),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&eq, &env, &mut body, &mut next).unwrap();
    assert!(got.is_some());

    let lt = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: span(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: span(),
        }),
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&lt, &env, &mut body, &mut next).unwrap();
    assert!(got.is_some());

    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::Compare {
            op: CompareOp::Eq,
            ..
        })
    )));
    assert!(body.iter().any(|i| matches!(
        i,
        JoinInst::Compute(MirLikeInst::Compare {
            op: CompareOp::Lt,
            ..
        })
    )));
}

#[test]
fn out_of_scope_call() {
    let env = BTreeMap::new();
    let mut body = vec![];
    let mut next = 1;

    let ast = ASTNode::FunctionCall {
        name: "f".to_string(),
        arguments: vec![],
        span: span(),
    };
    let got = NormalizedExprLowererBox::lower_expr(&ast, &env, &mut body, &mut next).unwrap();
    assert_eq!(got, None);
    assert!(body.is_empty());
    assert_eq!(next, 1);
}

#[test]
fn call_is_out_of_scope_in_pure_only() {
    let env = BTreeMap::new();
    let ast = ASTNode::Call {
        callee: Box::new(ASTNode::Variable {
            name: "f".to_string(),
            span: span(),
        }),
        arguments: vec![],
        span: span(),
    };
    assert_eq!(
        NormalizedExprLowererBox::out_of_scope_reason(ExprLoweringScope::PureOnly, &ast, &env),
        Some(OutOfScopeReason::Call)
    );
}

#[test]
fn methodcall_is_out_of_scope_in_pure_only() {
    let env = BTreeMap::new();
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        }),
        method: "m".to_string(),
        arguments: vec![],
        span: span(),
    };
    assert_eq!(
        NormalizedExprLowererBox::out_of_scope_reason(ExprLoweringScope::PureOnly, &ast, &env),
        Some(OutOfScopeReason::MethodCall)
    );
}

#[test]
fn methodcall_length0_is_in_scope_with_known_intrinsic_only() {
    let mut env = BTreeMap::new();
    env.insert("s".to_string(), ValueId(1));

    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "s".to_string(),
            span: span(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: span(),
    };

    assert_eq!(
        NormalizedExprLowererBox::out_of_scope_reason(
            ExprLoweringScope::WithImpure(ImpurePolicy::KnownIntrinsicOnly),
            &ast,
            &env
        ),
        None
    );
}

#[test]
fn lower_methodcall_length0_emits_method_call_inst() {
    let mut env = BTreeMap::new();
    env.insert("s".to_string(), ValueId(7));
    let mut body = vec![];
    let mut next = 100;

    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "s".to_string(),
            span: span(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: span(),
    };

    let got = NormalizedExprLowererBox::lower_expr_with_scope(
        ExprLoweringScope::WithImpure(ImpurePolicy::KnownIntrinsicOnly),
        &ast,
        &env,
        &mut body,
        &mut next,
    )
    .unwrap();

    assert_eq!(got, Some(ValueId(100)));
    assert_eq!(next, 101);
    assert!(matches!(
        body.as_slice(),
        [JoinInst::MethodCall {
            dst: ValueId(100),
            receiver: ValueId(7),
            method,
            args,
            type_hint: Some(MirType::Integer),
        }] if method == "length" && args.is_empty()
    ));
}
