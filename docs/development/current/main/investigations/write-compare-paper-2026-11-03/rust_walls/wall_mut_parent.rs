// Expected to fail: E0507. Field re-store through a `&mut` parent.
struct N {
    body: Box<i64>,
}

fn fold(v: i64) -> i64 {
    v + 1
}

fn rewrite(n: &mut N) {
    n.body = Box::new(fold(*n.body));
    let moved = n.body;
    n.body = moved;
}

fn main() {
    let mut n = N { body: Box::new(1) };
    rewrite(&mut n);
}
