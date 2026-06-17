struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn len2(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}
