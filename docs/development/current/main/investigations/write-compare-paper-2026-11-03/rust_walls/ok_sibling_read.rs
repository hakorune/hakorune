// Expected to compile. The candidate rejects `fold(move node.body, node.kind)`. Rust accepts it:
// the sibling field stays readable while `body` is moved out, and nothing can observe the hole.
struct Inner(i64);

struct N {
    kind: i64,
    body: Inner,
}

fn fold(v: Inner, kind: i64) -> Inner {
    Inner(v.0 + kind)
}

fn main() {
    let mut n = N { kind: 1, body: Inner(1) };
    n.body = fold(n.body, n.kind);
    assert_eq!(n.body.0, 2);
}
