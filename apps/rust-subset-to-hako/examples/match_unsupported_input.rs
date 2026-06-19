fn classify(x: i64) {
    match x {
        0 => observe(0),
        _ => observe(x),
    };
}
