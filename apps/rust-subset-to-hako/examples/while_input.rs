fn count_to(limit: i64) -> i64 {
    let mut i: i64 = 0;
    let mut sum: i64 = 0;
    while i < limit {
        sum = sum + i;
        i = i + 1;
    }
    sum
}
