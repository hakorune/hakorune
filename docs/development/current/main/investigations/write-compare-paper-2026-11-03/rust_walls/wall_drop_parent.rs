// Expected to fail: E0509. Field move out of an owned parent that has Drop.
// This is the Rust counterpart of the candidate's "parent has no fini" condition.
struct Inner(i64);

struct N {
    body: Inner,
}

impl Drop for N {
    fn drop(&mut self) {}
}

fn fold(v: Inner) -> Inner {
    v
}

fn main() {
    let mut n = N { body: Inner(1) };
    n.body = fold(n.body);
    println!("{}", n.body.0);
}
