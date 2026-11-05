// T4c: the rewrite when the node is only available through a `&mut` borrow.
#![allow(dead_code)]
use std::mem;

// === COMMON ===
struct Holder {
    required: Box<Leaf>,
    optional: Option<Box<Leaf>>,
}

struct Leaf {
    value: i64,
}

fn fold(mut l: Leaf) -> Leaf {
    l.value += 1;
    l
}

// === nullable through mut ===
fn rewrite_opt(h: &mut Holder) {
    h.optional = h.optional.take().map(|l| Box::new(fold(*l)));
}

// === required through mut ===
// Rejected by rustc: `h.required = Box::new(fold(*h.required));` is E0507.
// See rust_walls/wall_mut_parent.rs. Rust's usual answers follow.
fn rewrite_dummy(h: &mut Holder) {
    let old = mem::replace(&mut *h.required, Leaf { value: 0 });
    *h.required = fold(old);
}

fn rebuild(h: &mut Holder) {
    h.required = Box::new(Leaf { value: h.required.value + 1 });
}

fn in_place(h: &mut Holder) {
    bump(&mut h.required);
}

fn bump(l: &mut Leaf) {
    l.value += 1;
}

// === whole node through mut ===
fn replace_whole(node: &mut Leaf) {
    *node = Leaf { value: 0 };
}

// === enum child through mut ===
enum Expr {
    Num(Leaf),
    Neg(Box<Expr>),
}

fn visit(e: &mut Expr) {
    match e {
        Expr::Num(l) => l.value = 0,
        Expr::Neg(b) => visit(b),
    }
}

// === caller ===
fn main() {
    let mut h = Holder {
        required: Box::new(Leaf { value: 1 }),
        optional: Some(Box::new(Leaf { value: 2 })),
    };
    rewrite_opt(&mut h);
    rewrite_dummy(&mut h);
    rebuild(&mut h);
    in_place(&mut h);
    assert_eq!(h.required.value, 4);
    replace_whole(&mut h.required);
    assert_eq!(h.required.value, 0);
    let mut e = Expr::Neg(Box::new(Expr::Num(Leaf { value: 9 })));
    visit(&mut e);
}
