fn pump_forever() {
    let mut i: i64 = 0;
    loop {
        i = i + 1;
        observe(i);
    }
}
