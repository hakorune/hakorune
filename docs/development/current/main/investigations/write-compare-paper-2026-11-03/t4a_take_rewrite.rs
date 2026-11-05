// T4a: tree rewrite with nullable children using take, including `node = fold(node)`.
#![allow(dead_code)]

// === COMMON ===
#[derive(Clone, Copy, PartialEq)]
enum Op {
    Lit,
    Neg,
    Add,
}

#[derive(Clone)]
struct Term {
    op: Op,
    value: i64,
    lhs: Option<Box<Term>>,
    rhs: Option<Box<Term>>,
}

// === nullable children ===
fn fold(mut node: Term) -> Term {
    node.lhs = fold_opt(node.lhs.take());
    node.rhs = fold_opt(node.rhs.take());
    match node.op {
        Op::Lit => node,
        Op::Neg => simplify_neg(node),
        Op::Add => simplify_add(node),
    }
}

fn fold_opt(child: Option<Box<Term>>) -> Option<Box<Term>> {
    child.map(|c| Box::new(fold(*c)))
}

fn simplify_neg(mut node: Term) -> Term {
    let Some(mut inner) = node.lhs.take() else {
        return node;
    };
    if inner.op == Op::Lit {
        inner.value = -inner.value;
        return *inner;
    }
    if inner.op == Op::Neg {
        if let Some(x) = inner.lhs.take() {
            return *x;
        }
    }
    node.lhs = Some(inner);
    node
}

fn simplify_add(node: Term) -> Term {
    if let (Some(a), Some(b)) = (&node.lhs, &node.rhs) {
        if a.op == Op::Lit && b.op == Op::Lit {
            return Term { op: Op::Lit, value: a.value + b.value, lhs: None, rhs: None };
        }
    }
    node
}

// === array children ===
struct Call {
    callee: String,
    args: Vec<Term>,
}

fn fold_args(call: &mut Call) {
    call.args = std::mem::take(&mut call.args).into_iter().map(fold).collect();
}

// === caller ===
fn lit(v: i64) -> Term {
    Term { op: Op::Lit, value: v, lhs: None, rhs: None }
}

fn main() {
    let neg = Term { op: Op::Neg, value: 0, lhs: Some(Box::new(lit(7))), rhs: None };
    let mut root = Term { op: Op::Neg, value: 0, lhs: Some(Box::new(neg)), rhs: None };
    for _ in 0..2 {
        root = fold(root);
    }
    assert_eq!(root.value, 7);
    let mut call = Call { callee: "f".to_string(), args: vec![lit(1)] };
    fold_args(&mut call);
    assert_eq!(call.args.len(), 1);
}
