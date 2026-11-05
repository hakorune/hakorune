// T2: from a borrowed child, read a grandchild (chained borrow through two calls).
#![allow(dead_code)]

// === required chain ===
struct Block {
    count: i64,
}

struct FnDecl {
    body: Block,
}

impl FnDecl {
    fn body_ref(&self) -> &Block {
        &self.body
    }
}

struct Module {
    entry: FnDecl,
}

impl Module {
    fn entry_ref(&self) -> &FnDecl {
        &self.entry
    }
    fn entry_body(&self) -> &Block {
        self.entry_ref().body_ref()
    }
}

// === nullable chain ===
enum Expr {
    Num(i64),
    Neg(Box<Expr>),
    If { cond: Box<Expr>, then_e: Box<Expr>, else_e: Option<Box<Expr>> },
}

fn first_child(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::Num(_) => None,
        Expr::Neg(b) => Some(b),
        Expr::If { cond, .. } => Some(cond),
    }
}

fn first_grandchild(e: &Expr) -> Option<&Expr> {
    first_child(first_child(e)?)
}

fn grandchild_value(e: &Expr) -> i64 {
    match first_grandchild(e) {
        Some(Expr::Num(v)) => *v,
        _ => 0,
    }
}

// === temporary root ===
fn make() -> Expr {
    Expr::Num(7)
}

fn value_of_fresh() -> i64 {
    match make() {
        Expr::Num(v) => v,
        _ => 0,
    }
}

// === caller ===
fn main() {
    assert_eq!(value_of_fresh(), 7);
    let m = Module { entry: FnDecl { body: Block { count: 3 } } };
    let b = m.entry_body();
    let root = Expr::Neg(Box::new(Expr::Neg(Box::new(Expr::Num(7)))));
    assert_eq!(b.count + grandchild_value(&root), 10);
}
