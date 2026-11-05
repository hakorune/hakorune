// Expected to fail: E0505 (move out of `n.body` while `n` is borrowed). Method on the parent taking a moved-out field.
// The candidate rejects the same shape, `node.fold(move node.body)`.
struct Inner(i64);

struct N {
    body: Inner,
}

impl N {
    fn fold(&self, v: Inner) -> Inner {
        v
    }
}

fn main() {
    let mut n = N { body: Inner(1) };
    n.body = n.fold(n.body);
    println!("{}", n.body.0);
}
