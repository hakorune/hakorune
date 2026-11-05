// T6: a value bundle of span plus an owned expression.
#![allow(dead_code)]

struct Span {
    start: u32,
    end: u32,
}

struct Expr {
    value: i64,
}

struct NegNode {
    span: Span,
    body: Box<Expr>,
}

struct Parsed {
    span: Span,
    expr: Box<Expr>,
}

fn atom(start: u32) -> Parsed {
    Parsed { span: Span { start, end: start + 1 }, expr: Box::new(Expr { value: start as i64 }) }
}

fn read_value(p: &Parsed) -> i64 {
    p.expr.value
}

fn wrap(p: Parsed) -> NegNode {
    NegNode { span: p.span, body: p.expr }
}

fn main() {
    let p = atom(1);
    assert_eq!(read_value(&p), 1);
    assert_eq!(wrap(p).body.value, 1);
}
