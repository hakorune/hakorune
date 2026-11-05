// T4b: the T4a rewrite where a child is a required non-nullable field.
// Rewrites: -(num) => num, -(-x) => x, if(const) a else b => a or b.
#![allow(dead_code)]

// === V1 struct nodes, field re-store (the candidate's Rust equivalent) ===
struct Span {
    start: u32,
    end: u32,
}

struct NegNode {
    span: Span,
    body: Box<Expr>,
}

struct IfNode {
    span: Span,
    cond: Box<Expr>,
    then_e: Box<Expr>,
    else_e: Option<Box<Expr>>,
}

enum Expr {
    Num(i64),
    Neg(NegNode),
    If(IfNode),
}

fn fold(e: Expr) -> Expr {
    match e {
        Expr::Num(v) => Expr::Num(v),
        Expr::Neg(n) => fold_neg(n),
        Expr::If(n) => fold_if(n),
    }
}

fn fold_neg(mut node: NegNode) -> Expr {
    node.body = Box::new(fold(*node.body));
    match *node.body {
        Expr::Num(v) => Expr::Num(-v),
        // Lifting out of the dying parent is an ordinary move here.
        Expr::Neg(inner) => *inner.body,
        other => {
            node.body = Box::new(other);
            Expr::Neg(node)
        }
    }
}

fn fold_if(mut node: IfNode) -> Expr {
    node.cond = Box::new(fold(*node.cond));
    node.then_e = Box::new(fold(*node.then_e));
    node.else_e = node.else_e.map(|e| Box::new(fold(*e)));
    if let Expr::Num(v) = *node.cond {
        if v != 0 {
            return *node.then_e;
        }
        return match node.else_e {
            Some(e) => *e,
            None => Expr::Num(0),
        };
    }
    Expr::If(node)
}

// === V3 enum payload layout ===
enum E {
    Num(i64),
    Neg(Box<E>),
    If { cond: Box<E>, then_e: Box<E>, else_e: Option<Box<E>> },
}

fn fold_e(e: E) -> E {
    match e {
        E::Num(v) => E::Num(v),
        E::Neg(x) => match fold_e(*x) {
            E::Num(v) => E::Num(-v),
            E::Neg(y) => *y,
            other => E::Neg(Box::new(other)),
        },
        E::If { cond, then_e, else_e } => match fold_e(*cond) {
            E::Num(v) if v != 0 => fold_e(*then_e),
            E::Num(_) => else_e.map_or(E::Num(0), |x| fold_e(*x)),
            c => E::If {
                cond: Box::new(c),
                then_e: Box::new(fold_e(*then_e)),
                else_e: else_e.map(|x| Box::new(fold_e(*x))),
            },
        },
    }
}

// === caller ===
fn main() {
    let leaf = Expr::Num(7);
    let inner = Expr::Neg(NegNode { span: Span { start: 1, end: 3 }, body: Box::new(leaf) });
    let mut root = Expr::Neg(NegNode { span: Span { start: 0, end: 3 }, body: Box::new(inner) });
    root = fold(root);
    assert!(matches!(root, Expr::Num(7)));
    let cond = Expr::Num(1);
    let iff = IfNode {
        span: Span { start: 0, end: 1 },
        cond: Box::new(cond),
        then_e: Box::new(Expr::Num(5)),
        else_e: None,
    };
    assert!(matches!(fold(Expr::If(iff)), Expr::Num(5)));
    let e = E::Neg(Box::new(E::Neg(Box::new(E::Num(3)))));
    assert!(matches!(fold_e(e), E::Num(3)));
}
