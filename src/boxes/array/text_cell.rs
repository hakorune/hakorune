#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum ArrayTextCell {
    Flat(String),
}

impl ArrayTextCell {
    #[inline(always)]
    pub(super) fn flat(value: String) -> Self {
        Self::Flat(value)
    }

    #[inline(always)]
    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::Flat(value) => value.as_str(),
        }
    }

    #[inline(always)]
    pub(super) fn as_mut_string(&mut self) -> &mut String {
        match self {
            Self::Flat(value) => value,
        }
    }

    #[inline(always)]
    pub(super) fn into_string(self) -> String {
        match self {
            Self::Flat(value) => value,
        }
    }

    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline(always)]
    pub(super) fn contains_literal(&self, needle: &str) -> bool {
        match self {
            Self::Flat(value) => text_contains_literal(value, needle),
        }
    }

    #[inline(always)]
    pub(super) fn append_suffix(&mut self, suffix: &str) {
        match self {
            Self::Flat(value) => append_text_suffix(value, suffix),
        }
    }

    #[inline(always)]
    pub(super) fn string_contains_literal(value: &str, needle: &str) -> bool {
        text_contains_literal(value, needle)
    }

    #[inline(always)]
    pub(super) fn append_suffix_to_string(value: &mut String, suffix: &str) {
        append_text_suffix(value, suffix)
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf(&mut self, middle: &str) -> i64 {
        match self {
            Self::Flat(value) => Self::insert_const_mid_lenhalf_string(value, middle),
        }
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string(value: &mut String, middle: &str) -> i64 {
        let split = (value.len() / 2) as i64;
        insert_const_mid_flat(value, middle, split);
        value.len() as i64
    }
}

impl From<String> for ArrayTextCell {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::flat(value)
    }
}

#[inline(always)]
fn insert_const_mid_flat(value: &mut String, middle: &str, split: i64) {
    if value.is_empty() {
        value.push_str(middle);
        return;
    }
    if middle.is_empty() {
        return;
    }
    let split = split.clamp(0, value.len() as i64) as usize;
    if value.is_char_boundary(split) {
        value.insert_str(split, middle);
        return;
    }
    *value = materialize_insert_const_mid_flat(value.as_str(), middle, split as i64);
}

#[inline(always)]
fn materialize_insert_const_mid_flat(source: &str, middle: &str, split: i64) -> String {
    if source.is_empty() {
        return middle.to_owned();
    }
    if middle.is_empty() {
        return source.to_owned();
    }
    let split = split.clamp(0, source.len() as i64) as usize;
    let prefix = source.get(0..split).unwrap_or("");
    let suffix = source.get(split..).unwrap_or("");
    let total = prefix.len() + middle.len() + suffix.len();
    let mut out = String::with_capacity(total);
    out.push_str(prefix);
    out.push_str(middle);
    out.push_str(suffix);
    out
}

#[inline(always)]
fn text_contains_literal(value: &str, needle: &str) -> bool {
    let needle_text = needle;
    let haystack = value.as_bytes();
    let needle = needle.as_bytes();
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    match needle.len() {
        1..=8 => {
            if short_literal_prefix_eq(haystack, needle) {
                return true;
            }
            contains_short_literal_from(haystack, needle, 1)
        }
        _ => value.contains(needle_text),
    }
}

#[inline(always)]
fn append_text_suffix(value: &mut String, suffix: &str) {
    match suffix.len() {
        0 => {}
        1..=8 => append_short_text_suffix(value, suffix),
        _ => value.push_str(suffix),
    }
}

#[inline(always)]
fn append_short_text_suffix(value: &mut String, suffix: &str) {
    let bytes = suffix.as_bytes();
    let len = bytes.len();
    debug_assert!((1..=8).contains(&len));
    // SAFETY: `suffix` is valid UTF-8, so appending its bytes to an existing
    // valid `String` preserves the UTF-8 invariant.
    unsafe {
        let vec = value.as_mut_vec();
        let base_len = vec.len();
        vec.reserve(len);
        let dst = vec.as_mut_ptr().add(base_len);
        match len {
            1 => dst.write(bytes[0]),
            2 => write_u16_unaligned(dst, read_u16_unaligned(bytes.as_ptr())),
            3 => {
                write_u16_unaligned(dst, read_u16_unaligned(bytes.as_ptr()));
                dst.add(2).write(bytes[2]);
            }
            4 => write_u32_unaligned(dst, read_u32_unaligned(bytes.as_ptr())),
            5 => {
                write_u32_unaligned(dst, read_u32_unaligned(bytes.as_ptr()));
                dst.add(4).write(bytes[4]);
            }
            6 => {
                write_u32_unaligned(dst, read_u32_unaligned(bytes.as_ptr()));
                write_u16_unaligned(dst.add(4), read_u16_unaligned(bytes.as_ptr().add(4)));
            }
            7 => {
                write_u32_unaligned(dst, read_u32_unaligned(bytes.as_ptr()));
                write_u16_unaligned(dst.add(4), read_u16_unaligned(bytes.as_ptr().add(4)));
                dst.add(6).write(bytes[6]);
            }
            8 => write_u64_unaligned(dst, read_u64_unaligned(bytes.as_ptr())),
            _ => unreachable!("short suffix length is checked above"),
        }
        vec.set_len(base_len + len);
    }
}

#[inline(always)]
fn short_literal_prefix_eq(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    match needle.len() {
        0 => true,
        1 => haystack[0] == needle[0],
        2 => read_u16_unaligned(haystack.as_ptr()) == read_u16_unaligned(needle.as_ptr()),
        3 => {
            read_u16_unaligned(haystack.as_ptr()) == read_u16_unaligned(needle.as_ptr())
                && haystack[2] == needle[2]
        }
        4 => read_u32_unaligned(haystack.as_ptr()) == read_u32_unaligned(needle.as_ptr()),
        5 => {
            read_u32_unaligned(haystack.as_ptr()) == read_u32_unaligned(needle.as_ptr())
                && haystack[4] == needle[4]
        }
        6 => {
            read_u32_unaligned(haystack.as_ptr()) == read_u32_unaligned(needle.as_ptr())
                && read_u16_unaligned(unsafe { haystack.as_ptr().add(4) })
                    == read_u16_unaligned(unsafe { needle.as_ptr().add(4) })
        }
        7 => {
            read_u32_unaligned(haystack.as_ptr()) == read_u32_unaligned(needle.as_ptr())
                && read_u16_unaligned(unsafe { haystack.as_ptr().add(4) })
                    == read_u16_unaligned(unsafe { needle.as_ptr().add(4) })
                && haystack[6] == needle[6]
        }
        8 => read_u64_unaligned(haystack.as_ptr()) == read_u64_unaligned(needle.as_ptr()),
        _ => haystack.starts_with(needle),
    }
}

#[inline(always)]
fn read_u16_unaligned(src: *const u8) -> u16 {
    unsafe { core::ptr::read_unaligned(src.cast::<u16>()) }
}

#[inline(always)]
fn read_u32_unaligned(src: *const u8) -> u32 {
    unsafe { core::ptr::read_unaligned(src.cast::<u32>()) }
}

#[inline(always)]
fn read_u64_unaligned(src: *const u8) -> u64 {
    unsafe { core::ptr::read_unaligned(src.cast::<u64>()) }
}

#[inline(always)]
fn write_u16_unaligned(dst: *mut u8, value: u16) {
    unsafe { core::ptr::write_unaligned(dst.cast::<u16>(), value) }
}

#[inline(always)]
fn write_u32_unaligned(dst: *mut u8, value: u32) {
    unsafe { core::ptr::write_unaligned(dst.cast::<u32>(), value) }
}

#[inline(always)]
fn write_u64_unaligned(dst: *mut u8, value: u64) {
    unsafe { core::ptr::write_unaligned(dst.cast::<u64>(), value) }
}

#[inline(always)]
fn contains_short_literal_from(haystack: &[u8], needle: &[u8], start: usize) -> bool {
    let limit = haystack.len() - needle.len();
    if start > limit {
        return false;
    }
    match needle.len() {
        1 => contains_one_byte_from(haystack, needle[0], start, limit),
        2 => contains_two_bytes_from(haystack, needle[0], needle[1], start, limit),
        3 => contains_three_bytes_from(haystack, needle[0], needle[1], needle[2], start, limit),
        4 => contains_four_bytes_from(
            haystack, needle[0], needle[1], needle[2], needle[3], start, limit,
        ),
        _ => contains_short_slice_from(haystack, needle, start, limit),
    }
}

#[inline(always)]
fn contains_one_byte_from(haystack: &[u8], b0: u8, mut index: usize, limit: usize) -> bool {
    while index <= limit {
        if haystack[index] == b0 {
            return true;
        }
        index += 1;
    }
    false
}

#[inline(always)]
fn contains_two_bytes_from(
    haystack: &[u8],
    b0: u8,
    b1: u8,
    mut index: usize,
    limit: usize,
) -> bool {
    while index <= limit {
        if haystack[index] == b0 && haystack[index + 1] == b1 {
            return true;
        }
        index += 1;
    }
    false
}

#[inline(always)]
fn contains_three_bytes_from(
    haystack: &[u8],
    b0: u8,
    b1: u8,
    b2: u8,
    mut index: usize,
    limit: usize,
) -> bool {
    while index <= limit {
        if haystack[index] == b0 && haystack[index + 1] == b1 && haystack[index + 2] == b2 {
            return true;
        }
        index += 1;
    }
    false
}

#[inline(always)]
fn contains_four_bytes_from(
    haystack: &[u8],
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    mut index: usize,
    limit: usize,
) -> bool {
    while index <= limit {
        if haystack[index] == b0
            && haystack[index + 1] == b1
            && haystack[index + 2] == b2
            && haystack[index + 3] == b3
        {
            return true;
        }
        index += 1;
    }
    false
}

#[inline(always)]
fn contains_short_slice_from(
    haystack: &[u8],
    needle: &[u8],
    mut index: usize,
    limit: usize,
) -> bool {
    while index <= limit {
        let end = index + needle.len();
        if haystack[index] == needle[0] && &haystack[index..end] == needle {
            return true;
        }
        index += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{append_text_suffix, text_contains_literal};

    #[test]
    fn text_contains_literal_matches_str_contains() {
        let values = [
            "",
            "line-seed",
            "xxline-seed",
            "seed-line",
            "naive cafe",
            "東京line大阪",
            "abc日本語def",
        ];
        let needles = [
            "", "l", "li", "line", "seed", "cafe", "東京", "日本", "absent",
        ];
        for value in values {
            for needle in needles {
                assert_eq!(
                    text_contains_literal(value, needle),
                    value.contains(needle),
                    "value={value:?} needle={needle:?}"
                );
            }
        }
    }

    #[test]
    fn append_text_suffix_matches_push_str() {
        let suffixes = ["", "l", "ln", "東京", "🙂", "abcdefghi"];
        for suffix in suffixes {
            let mut actual = String::from("line-seed");
            append_text_suffix(&mut actual, suffix);

            let mut expected = String::from("line-seed");
            expected.push_str(suffix);
            assert_eq!(actual, expected, "suffix={suffix:?}");
        }
    }
}
