fn pump_until_zero(mut n: i64) {
    loop {
        if n == 0 {
            break;
        }
        n = n - 1;
        continue;
    }
}
