use std::hint::black_box;

fn main() {
    let ops = black_box(300_000usize);
    let text = black_box("line-seed-abcdef");
    let len = black_box(text.len());
    let split = black_box(len / 2);
    let mut left = black_box(text);
    let mut right = black_box(text);

    for _ in 0..ops {
        left = black_box(&text[..split]);
        right = black_box(&text[split..]);
    }

    let checksum = black_box(left.len() + right.len() + len);
    std::process::exit((checksum & 0xff) as i32);
}
