use std::{hint::black_box, ptr};

fn main() {
    let ops = black_box(300_000usize);
    let text = black_box(b"line-seed-abcdef" as &[u8]);
    let len = black_box(text.len());
    let split = black_box(len / 2);
    let mut left = [0u8; 32];
    let mut right = [0u8; 32];

    for _ in 0..ops {
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), left.as_mut_ptr(), split);
            *left.get_unchecked_mut(split) = 0;
            ptr::copy_nonoverlapping(
                text.as_ptr().add(split),
                right.as_mut_ptr(),
                len - split,
            );
            *right.get_unchecked_mut(len - split) = 0;
        }
    }

    let checksum = black_box(left[0] as usize + right[0] as usize + len);
    std::process::exit((checksum & 0xff) as i32);
}
