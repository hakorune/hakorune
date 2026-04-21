use std::cmp::Ordering;

const MID_GAP_RIGHT_COMPACT_MIN: usize = 4096;
const MID_GAP_LEFT_OVERSHOOT_LIMIT: usize = 1024;
const MID_GAP_INITIAL_HEADROOM: usize = 64;

#[derive(Clone, Debug)]
pub(super) enum ArrayTextCell {
    Flat(String),
    MidGap {
        left: String,
        right: String,
        right_start: usize,
    },
}

impl ArrayTextCell {
    #[inline(always)]
    pub(super) fn flat(value: String) -> Self {
        Self::Flat(value)
    }

    #[inline(always)]
    pub(super) fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> R {
        match self {
            Self::Flat(value) => f(value.as_str()),
            Self::MidGap {
                left,
                right,
                right_start,
            } => {
                let value = materialize_mid_gap(left, right, *right_start);
                f(value.as_str())
            }
        }
    }

    #[inline(always)]
    pub(super) fn to_visible_string(&self) -> String {
        match self {
            Self::Flat(value) => value.clone(),
            Self::MidGap {
                left,
                right,
                right_start,
            } => materialize_mid_gap(left, right, *right_start),
        }
    }

    #[inline(always)]
    pub(super) fn as_mut_string(&mut self) -> &mut String {
        if !matches!(self, Self::Flat(_)) {
            let value = self.to_visible_string();
            *self = Self::Flat(value);
        }
        match self {
            Self::Flat(value) => value,
            Self::MidGap { .. } => unreachable!("non-flat text cell materialized above"),
        }
    }

    #[inline(always)]
    pub(super) fn into_string(self) -> String {
        match self {
            Self::Flat(value) => value,
            Self::MidGap {
                left,
                right,
                right_start,
            } => materialize_mid_gap(left.as_str(), right.as_str(), right_start),
        }
    }

    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Flat(value) => value.len(),
            Self::MidGap {
                left,
                right,
                right_start,
            } => left.len() + active_mid_gap_right(right, *right_start).len(),
        }
    }

    #[inline(always)]
    pub(super) fn equals_text(&self, needle: &str) -> bool {
        self.with_text(|value| value == needle)
    }

    #[inline(always)]
    pub(super) fn equals_cell(&self, other: &Self) -> bool {
        self.with_text(|lhs| other.with_text(|rhs| lhs == rhs))
    }

    #[inline(always)]
    pub(super) fn cmp_text(&self, other: &Self) -> Ordering {
        self.with_text(|lhs| other.with_text(|rhs| lhs.cmp(rhs)))
    }

    #[inline(always)]
    pub(super) fn contains_literal(&self, needle: &str) -> bool {
        match self {
            Self::Flat(value) => text_contains_literal(value, needle),
            Self::MidGap {
                left,
                right,
                right_start,
            } => mid_gap_contains_literal(left, active_mid_gap_right(right, *right_start), needle),
        }
    }

    #[inline(always)]
    pub(super) fn append_suffix(&mut self, suffix: &str) {
        match self {
            Self::Flat(value) => append_text_suffix(value, suffix),
            Self::MidGap { right, .. } => append_text_suffix(right, suffix),
        }
    }

    #[inline(always)]
    pub(super) fn string_contains_literal(value: &str, needle: &str) -> bool {
        text_contains_literal(value, needle)
    }

    #[inline(always)]
    pub(super) fn four_byte_literal_word(needle: &str) -> Option<u32> {
        if needle.len() == 4 {
            Some(read_u32_unaligned(needle.as_ptr()))
        } else {
            None
        }
    }

    #[inline(always)]
    pub(super) fn contains_four_byte_literal(&self, needle: u32) -> bool {
        match self {
            Self::Flat(value) => text_contains_four_byte_literal(value, needle),
            Self::MidGap {
                left,
                right,
                right_start,
            } => mid_gap_contains_four_byte_literal(
                left,
                active_mid_gap_right(right, *right_start),
                needle,
            ),
        }
    }

    #[inline(always)]
    pub(super) fn string_contains_four_byte_literal(value: &str, needle: u32) -> bool {
        text_contains_four_byte_literal(value, needle)
    }

    #[inline(always)]
    pub(super) fn append_suffix_to_string(value: &mut String, suffix: &str) {
        append_text_suffix(value, suffix)
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf(&mut self, middle: &str) -> i64 {
        match self {
            Self::Flat(value) => {
                if let Some((next, out)) = build_mid_gap_from_flat_lenhalf(value, middle) {
                    *self = next;
                    out
                } else {
                    Self::insert_const_mid_lenhalf_string(value, middle)
                }
            }
            Self::MidGap {
                left,
                right,
                right_start,
            } => match insert_const_mid_lenhalf_mid_gap(left, right, right_start, middle) {
                Some(out) => out,
                None => {
                    let mut value = self.to_visible_string();
                    let out = Self::insert_const_mid_lenhalf_string(&mut value, middle);
                    *self = Self::Flat(value);
                    out
                }
            },
        }
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_byte_boundary_safe(&mut self, middle: &str) -> i64 {
        debug_assert!(middle.is_ascii());
        match self {
            Self::Flat(value) => {
                if let Some((next, out)) =
                    build_mid_gap_from_flat_lenhalf_byte_boundary_safe(value, middle)
                {
                    *self = next;
                    out
                } else {
                    Self::insert_const_mid_lenhalf_string_byte_boundary_safe(value, middle)
                }
            }
            Self::MidGap {
                left,
                right,
                right_start,
            } => match insert_const_mid_lenhalf_mid_gap_byte_boundary_safe(
                left,
                right,
                right_start,
                middle,
            ) {
                Some(out) => out,
                None => {
                    let mut value = self.to_visible_string();
                    let out = Self::insert_const_mid_lenhalf_string(&mut value, middle);
                    *self = Self::Flat(value);
                    out
                }
            },
        }
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string(value: &mut String, middle: &str) -> i64 {
        let split = (value.len() / 2) as i64;
        insert_const_mid_flat(value, middle, split);
        value.len() as i64
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string_byte_boundary_safe(
        value: &mut String,
        middle: &str,
    ) -> i64 {
        debug_assert!(middle.is_ascii());
        let split = value.len() / 2;
        insert_str_byte_boundary_unchecked(value, split, middle);
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
fn active_mid_gap_right(right: &str, right_start: usize) -> &str {
    debug_assert!(right_start <= right.len());
    debug_assert!(right.is_char_boundary(right_start));
    // MidGap updates `right_start` only after char-boundary checks. Keep the
    // hot executor from paying the same UTF-8/range check on every access.
    unsafe { right.get_unchecked(right_start..) }
}

#[inline(always)]
fn mid_gap_right_range(right: &str, start: usize, end: usize) -> &str {
    debug_assert!(start <= end);
    debug_assert!(end <= right.len());
    debug_assert!(right.is_char_boundary(start));
    debug_assert!(right.is_char_boundary(end));
    unsafe { right.get_unchecked(start..end) }
}

#[inline(always)]
fn mid_gap_right_range_byte_boundary_safe(right: &str, start: usize, end: usize) -> &str {
    debug_assert!(start <= end);
    debug_assert!(end <= right.len());
    debug_assert!(right.is_char_boundary(start));
    debug_assert!(right.is_char_boundary(end));
    unsafe { right.get_unchecked(start..end) }
}

#[inline(always)]
fn materialize_mid_gap(left: &str, right: &str, right_start: usize) -> String {
    let active_right = active_mid_gap_right(right, right_start);
    if left.is_empty() {
        return active_right.to_owned();
    }
    if active_right.is_empty() {
        return left.to_owned();
    }
    let mut out = String::with_capacity(left.len() + active_right.len());
    out.push_str(left);
    out.push_str(active_right);
    out
}

#[inline(always)]
fn build_mid_gap_from_flat_lenhalf(value: &str, middle: &str) -> Option<(ArrayTextCell, i64)> {
    if value.is_empty() || middle.is_empty() {
        return None;
    }
    let split = value.len() / 2;
    if !value.is_char_boundary(split) {
        return None;
    }

    let mut left = String::with_capacity(
        split
            .saturating_add(middle.len())
            .saturating_add(MID_GAP_INITIAL_HEADROOM),
    );
    left.push_str(&value[..split]);
    left.push_str(middle);
    let right = value[split..].to_owned();
    let out = left.len() + right.len();
    Some((
        ArrayTextCell::MidGap {
            left,
            right,
            right_start: 0,
        },
        out as i64,
    ))
}

#[inline(always)]
fn build_mid_gap_from_flat_lenhalf_byte_boundary_safe(
    value: &str,
    middle: &str,
) -> Option<(ArrayTextCell, i64)> {
    if value.is_empty() || middle.is_empty() {
        return None;
    }
    debug_assert!(middle.is_ascii());
    let split = value.len() / 2;
    debug_assert!(value.is_char_boundary(split));
    let prefix = unsafe { value.get_unchecked(..split) };
    let suffix = unsafe { value.get_unchecked(split..) };

    let mut left = String::with_capacity(
        split
            .saturating_add(middle.len())
            .saturating_add(MID_GAP_INITIAL_HEADROOM),
    );
    left.push_str(prefix);
    left.push_str(middle);
    let right = suffix.to_owned();
    let out = left.len() + right.len();
    Some((
        ArrayTextCell::MidGap {
            left,
            right,
            right_start: 0,
        },
        out as i64,
    ))
}

#[inline(always)]
fn insert_const_mid_lenhalf_mid_gap(
    left: &mut String,
    right: &mut String,
    right_start: &mut usize,
    middle: &str,
) -> Option<i64> {
    if middle.is_empty() {
        return Some((left.len() + active_mid_gap_right(right, *right_start).len()) as i64);
    }

    let active_right_len = active_mid_gap_right(right, *right_start).len();
    let source_len = left.len() + active_right_len;
    let split = source_len / 2;

    if split <= left.len() {
        if !left.is_char_boundary(split) {
            return None;
        }
        left.insert_str(split, middle);
    } else {
        let delta = split - left.len();
        let start = *right_start;
        let end = start.checked_add(delta)?;
        if end > right.len() || !right.is_char_boundary(end) {
            return None;
        }
        left.push_str(mid_gap_right_range(right, start, end));
        *right_start = end;
        left.push_str(middle);
        compact_mid_gap_right(right, right_start);
    }

    rebalance_mid_gap_left_overshoot(left, right, right_start);
    Some((left.len() + active_mid_gap_right(right, *right_start).len()) as i64)
}

#[inline(always)]
fn insert_const_mid_lenhalf_mid_gap_byte_boundary_safe(
    left: &mut String,
    right: &mut String,
    right_start: &mut usize,
    middle: &str,
) -> Option<i64> {
    debug_assert!(middle.is_ascii());
    if middle.is_empty() {
        return Some((left.len() + active_mid_gap_right(right, *right_start).len()) as i64);
    }

    let active_right_len = active_mid_gap_right(right, *right_start).len();
    let source_len = left.len() + active_right_len;
    let split = source_len / 2;

    if split <= left.len() {
        insert_str_byte_boundary_unchecked(left, split, middle);
    } else {
        let delta = split - left.len();
        let start = *right_start;
        let end = start.checked_add(delta)?;
        if end > right.len() {
            return None;
        }
        left.push_str(mid_gap_right_range_byte_boundary_safe(right, start, end));
        *right_start = end;
        left.push_str(middle);
        compact_mid_gap_right(right, right_start);
    }

    rebalance_mid_gap_left_overshoot_byte_boundary_safe(left, right, right_start);
    Some((left.len() + active_mid_gap_right(right, *right_start).len()) as i64)
}

#[inline(always)]
fn compact_mid_gap_right(right: &mut String, right_start: &mut usize) {
    if *right_start < MID_GAP_RIGHT_COMPACT_MIN {
        return;
    }
    let active_len = right.len().saturating_sub(*right_start);
    if *right_start <= active_len {
        return;
    }
    *right = right[*right_start..].to_owned();
    *right_start = 0;
}

#[inline(always)]
fn rebalance_mid_gap_left_overshoot(
    left: &mut String,
    right: &mut String,
    right_start: &mut usize,
) {
    let active_right = active_mid_gap_right(right, *right_start);
    let total_len = left.len() + active_right.len();
    let target = total_len / 2;
    if left.len() <= target.saturating_add(MID_GAP_LEFT_OVERSHOOT_LIMIT) {
        return;
    }

    let value = materialize_mid_gap(left, right, *right_start);
    if !value.is_char_boundary(target) {
        return;
    }
    let mut new_left = String::with_capacity(target.saturating_add(MID_GAP_INITIAL_HEADROOM));
    new_left.push_str(&value[..target]);
    *left = new_left;
    *right = value[target..].to_owned();
    *right_start = 0;
}

#[inline(always)]
fn rebalance_mid_gap_left_overshoot_byte_boundary_safe(
    left: &mut String,
    right: &mut String,
    right_start: &mut usize,
) {
    let active_right = active_mid_gap_right(right, *right_start);
    let total_len = left.len() + active_right.len();
    let target = total_len / 2;
    if left.len() <= target.saturating_add(MID_GAP_LEFT_OVERSHOOT_LIMIT) {
        return;
    }

    let value = materialize_mid_gap(left, right, *right_start);
    debug_assert!(value.is_char_boundary(target));
    let prefix = unsafe { value.get_unchecked(..target) };
    let suffix = unsafe { value.get_unchecked(target..) };
    let mut new_left = String::with_capacity(target.saturating_add(MID_GAP_INITIAL_HEADROOM));
    new_left.push_str(prefix);
    *left = new_left;
    *right = suffix.to_owned();
    *right_start = 0;
}

#[inline(always)]
fn insert_str_byte_boundary_unchecked(value: &mut String, split: usize, middle: &str) {
    if value.is_empty() {
        value.push_str(middle);
        return;
    }
    if middle.is_empty() {
        return;
    }
    let split = split.min(value.len());
    debug_assert!(value.is_char_boundary(split));
    debug_assert!(middle.is_ascii());
    let len = value.len();
    let add = middle.len();
    value.reserve(add);
    unsafe {
        let bytes = value.as_mut_vec();
        std::ptr::copy(
            bytes.as_ptr().add(split),
            bytes.as_mut_ptr().add(split + add),
            len - split,
        );
        std::ptr::copy_nonoverlapping(middle.as_ptr(), bytes.as_mut_ptr().add(split), add);
        bytes.set_len(len + add);
    }
}

#[inline(always)]
fn mid_gap_contains_literal(left: &str, right: &str, needle: &str) -> bool {
    let needle_bytes = needle.as_bytes();
    if needle_bytes.is_empty() {
        return true;
    }
    if needle_bytes.len() > left.len() + right.len() {
        return false;
    }
    if short_literal_prefix_eq(left.as_bytes(), needle_bytes) {
        return true;
    }
    if text_contains_literal(left, needle) || text_contains_literal(right, needle) {
        return true;
    }
    mid_gap_boundary_contains(left.as_bytes(), right.as_bytes(), needle_bytes)
}

#[inline(always)]
fn text_contains_four_byte_literal(value: &str, needle: u32) -> bool {
    contains_four_byte_word(value.as_bytes(), needle)
}

#[inline(always)]
fn mid_gap_contains_four_byte_literal(left: &str, right: &str, needle: u32) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() + right.len() < 4 {
        return false;
    }
    if contains_four_byte_word(left, needle) || contains_four_byte_word(right, needle) {
        return true;
    }
    mid_gap_boundary_contains_four_byte(left, right, needle)
}

#[inline(always)]
fn mid_gap_boundary_contains(left: &[u8], right: &[u8], needle: &[u8]) -> bool {
    if needle.len() < 2 || left.is_empty() || right.is_empty() {
        return false;
    }
    let max_left_take = needle.len().saturating_sub(1).min(left.len());
    for left_take in 1..=max_left_take {
        let right_take = needle.len() - left_take;
        if right_take > right.len() {
            continue;
        }
        let left_start = left.len() - left_take;
        if left[left_start..] == needle[..left_take] && right[..right_take] == needle[left_take..] {
            return true;
        }
    }
    false
}

#[inline(always)]
fn mid_gap_boundary_contains_four_byte(left: &[u8], right: &[u8], needle: u32) -> bool {
    if left.is_empty() || right.is_empty() {
        return false;
    }
    let needle = needle.to_ne_bytes();
    let max_left_take = 3.min(left.len());
    for left_take in 1..=max_left_take {
        let right_take = 4 - left_take;
        if right_take > right.len() {
            continue;
        }
        let left_start = left.len() - left_take;
        if left[left_start..] == needle[..left_take] && right[..right_take] == needle[left_take..] {
            return true;
        }
    }
    false
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
fn contains_four_byte_word(haystack: &[u8], needle: u32) -> bool {
    if haystack.len() < 4 {
        return false;
    }
    let limit = haystack.len() - 4;
    let mut index = 0;
    while index <= limit {
        // SAFETY: `index <= len - 4`, so the unaligned word read stays in-bounds.
        let word = unsafe { read_u32_unaligned(haystack.as_ptr().add(index)) };
        if word == needle {
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
    use super::{append_text_suffix, text_contains_literal, ArrayTextCell};

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

    #[test]
    fn mid_gap_lenhalf_insert_matches_flat_string() {
        let mut cell = ArrayTextCell::flat("line-seed".to_string());
        let mut expected = "line-seed".to_string();

        for step in 0..128 {
            let out = cell.insert_const_mid_lenhalf("xx");
            let split = expected.len() / 2;
            expected.insert_str(split, "xx");
            assert_eq!(out, expected.len() as i64, "step={step}");

            if step % 8 == 0 {
                cell.append_suffix("ln");
                expected.push_str("ln");
            }

            assert_eq!(cell.len(), expected.len(), "step={step}");
            assert_eq!(cell.to_visible_string(), expected, "step={step}");
        }
    }

    #[test]
    fn byte_boundary_safe_lenhalf_insert_matches_checked_ascii() {
        let mut checked = ArrayTextCell::flat("line-seed".to_string());
        let mut fast = ArrayTextCell::flat("line-seed".to_string());

        for step in 0..128 {
            let checked_len = checked.insert_const_mid_lenhalf("xx");
            let fast_len = fast.insert_const_mid_lenhalf_byte_boundary_safe("xx");
            assert_eq!(fast_len, checked_len, "step={step}");
            assert_eq!(
                fast.to_visible_string(),
                checked.to_visible_string(),
                "step={step}"
            );

            if step % 8 == 0 {
                checked.append_suffix("ln");
                fast.append_suffix("ln");
                assert_eq!(
                    fast.to_visible_string(),
                    checked.to_visible_string(),
                    "append step={step}"
                );
            }
        }
    }

    #[test]
    fn mid_gap_contains_literal_checks_boundary() {
        let mut cell = ArrayTextCell::flat("ab".to_string());
        assert_eq!(cell.insert_const_mid_lenhalf("XY"), 4);

        assert!(cell.contains_literal("aX"));
        assert!(cell.contains_literal("XY"));
        assert!(cell.contains_literal("Yb"));
        assert!(!cell.contains_literal("Ya"));
    }

    #[test]
    fn four_byte_literal_word_matches_generic_contains() {
        let needles = ["line", "seed", "🙂", "none"];
        let cells = [
            ArrayTextCell::flat("line-seed🙂".to_string()),
            ArrayTextCell::MidGap {
                left: "ab".to_string(),
                right: "cd-line".to_string(),
                right_start: 0,
            },
            ArrayTextCell::MidGap {
                left: "prefix-line".to_string(),
                right: "tail".to_string(),
                right_start: 0,
            },
        ];

        for cell in cells {
            for needle in needles {
                let word = ArrayTextCell::four_byte_literal_word(needle).unwrap();
                assert_eq!(
                    cell.contains_four_byte_literal(word),
                    cell.contains_literal(needle),
                    "cell={cell:?} needle={needle:?}"
                );
            }
        }
    }

    #[test]
    fn mid_gap_as_mut_string_materializes_explicitly() {
        let mut cell = ArrayTextCell::flat("abcd".to_string());
        assert_eq!(cell.insert_const_mid_lenhalf("XY"), 6);

        cell.as_mut_string().push_str("!");
        assert_eq!(cell.to_visible_string(), "abXYcd!");
    }
}
