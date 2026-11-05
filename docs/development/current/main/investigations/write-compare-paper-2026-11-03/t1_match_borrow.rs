// T1: AST with required and nullable children. Match on a node and return a child as a borrow.
#![allow(dead_code)]

struct Span {
    start: u32,
    end: u32,
}

enum Expr {
    Num { span: Span, value: i64 },
    Var { span: Span, name: String },
    Neg { span: Span, body: Box<Expr> },
    If { span: Span, cond: Box<Expr>, then_e: Box<Expr>, else_e: Option<Box<Expr>> },
}

fn first_child(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::Num { .. } | Expr::Var { .. } => None,
        Expr::Neg { body, .. } => Some(body),
        Expr::If { cond, .. } => Some(cond),
    }
}

fn else_of(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::If { else_e, .. } => else_e.as_deref(),
        _ => None,
    }
}

fn sp(start: u32, end: u32) -> Span {
    Span { start, end }
}

fn main() {
    let root = Expr::If {
        span: sp(0, 9),
        cond: Box::new(Expr::Num { span: sp(3, 4), value: 1 }),
        then_e: Box::new(Expr::Var { span: sp(5, 6), name: "x".to_string() }),
        else_e: None,
    };
    let c = first_child(&root);
    let e = else_of(&root);
    assert!(c.is_some() && e.is_none());
}
