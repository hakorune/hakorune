use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayTextSlotSessionMode {
    ResidentOnly,
    Compatible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayTextSlotSessionOutcomeKind {
    ResidentText,
    PromotedTextLane,
    BoxedString,
    BoxedStringLike,
}

struct ArrayTextSlotSession<'a> {
    storage: &'a mut ArrayStorage,
    mode: ArrayTextSlotSessionMode,
}

impl<'a> ArrayTextSlotSession<'a> {
    #[inline(always)]
    fn new(storage: &'a mut ArrayStorage, mode: ArrayTextSlotSessionMode) -> Self {
        Self { storage, mode }
    }

    #[inline(always)]
    fn update<R>(
        &mut self,
        idx: usize,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<(R, ArrayTextSlotSessionOutcomeKind)> {
        if self.mode == ArrayTextSlotSessionMode::Compatible {
            let promoted = match &*self.storage {
                ArrayStorage::Boxed(items) => ArrayBox::try_text_values(items),
                ArrayStorage::Text(_)
                | ArrayStorage::InlineI64(_)
                | ArrayStorage::InlineBool(_)
                | ArrayStorage::InlineF64(_) => None,
            };
            if let Some(values) = promoted {
                *self.storage = ArrayStorage::Text(values);
                if let ArrayStorage::Text(values) = self.storage {
                    return values.get_mut(idx).map(|value| {
                        (
                            f(value.as_mut_string()),
                            ArrayTextSlotSessionOutcomeKind::PromotedTextLane,
                        )
                    });
                }
                unreachable!("boxed text lane promoted to text");
            }
        }

        match self.storage {
            ArrayStorage::Text(values) => values.get_mut(idx).map(|value| {
                (
                    f(value.as_mut_string()),
                    ArrayTextSlotSessionOutcomeKind::ResidentText,
                )
            }),
            ArrayStorage::Boxed(_) if self.mode == ArrayTextSlotSessionMode::ResidentOnly => None,
            ArrayStorage::Boxed(items) => {
                let item = items.get_mut(idx)?;
                if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                    return Some((
                        f(&mut value.value),
                        ArrayTextSlotSessionOutcomeKind::BoxedString,
                    ));
                }
                let mut value = item.as_str_fast().map(str::to_owned)?;
                let out = f(&mut value);
                *item = Box::new(StringBox::new(value));
                Some((out, ArrayTextSlotSessionOutcomeKind::BoxedStringLike))
            }
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }
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
            2 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
            }
            3 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
            }
            4 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
                dst.add(3).write(bytes[3]);
            }
            5 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
                dst.add(3).write(bytes[3]);
                dst.add(4).write(bytes[4]);
            }
            6 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
                dst.add(3).write(bytes[3]);
                dst.add(4).write(bytes[4]);
                dst.add(5).write(bytes[5]);
            }
            7 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
                dst.add(3).write(bytes[3]);
                dst.add(4).write(bytes[4]);
                dst.add(5).write(bytes[5]);
                dst.add(6).write(bytes[6]);
            }
            8 => {
                dst.write(bytes[0]);
                dst.add(1).write(bytes[1]);
                dst.add(2).write(bytes[2]);
                dst.add(3).write(bytes[3]);
                dst.add(4).write(bytes[4]);
                dst.add(5).write(bytes[5]);
                dst.add(6).write(bytes[6]);
                dst.add(7).write(bytes[7]);
            }
            _ => unreachable!("short suffix length is checked above"),
        }
        vec.set_len(base_len + len);
    }
}

#[inline(always)]
fn short_literal_prefix_eq(haystack: &[u8], needle: &[u8]) -> bool {
    let mut index = 0;
    while index < needle.len() {
        if haystack[index] != needle[index] {
            return false;
        }
        index += 1;
    }
    true
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

impl ArrayBox {
    /// Raw text store helper for runtime-private array string lanes.
    /// Public Array semantics stay object-based; mixed arrays degrade to Boxed.
    #[inline(always)]
    pub fn slot_store_text_raw(&self, idx: i64, value: String) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_text(&mut items) {
            if idx < values.len() {
                values[idx] = ArrayTextCell::from(value);
                true
            } else if idx == values.len() {
                values.push(ArrayTextCell::from(value));
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        } else {
            let boxed = Self::ensure_boxed(&mut items);
            let value = Box::new(StringBox::new(value)) as Box<dyn NyashBox>;
            if idx < boxed.len() {
                boxed[idx] = value;
                true
            } else if idx == boxed.len() {
                boxed.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Read a text slot without materializing a public StringBox.
    #[inline(always)]
    pub fn slot_with_text_raw<R>(&self, idx: i64, f: impl FnOnce(&str) -> R) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Text(values) => values.get(idx).map(|value| f(value.as_str())),
            ArrayStorage::Boxed(items) => items.get(idx).and_then(|item| item.as_str_fast().map(f)),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }

    /// Read a text slot length without materializing a public StringBox.
    #[inline(always)]
    pub fn slot_text_len_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Text(values) => values.get(idx).map(|value| value.len() as i64),
            ArrayStorage::Boxed(items) => items
                .get(idx)
                .and_then(|item| item.as_str_fast().map(|value| value.len() as i64)),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }

    /// Mutate a text slot only when storage is already text-resident.
    /// This does not promote boxed arrays; callers that need full ArrayBox compatibility
    /// must fall back to `slot_update_text_raw`.
    #[inline(always)]
    pub fn slot_update_text_resident_raw<R>(
        &self,
        idx: i64,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::ResidentOnly)
            .update(idx, f)
            .map(|(out, _kind)| out)
    }

    /// Mutate a text slot in-place when the array is text-resident.
    /// If the array is mixed but the target slot is string-like, only that slot is materialized.
    #[inline(always)]
    pub fn slot_update_text_raw<R>(&self, idx: i64, f: impl FnOnce(&mut String) -> R) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible)
            .update(idx, f)
            .map(|(out, _kind)| out)
    }

    /// Mutate a text slot through the compatible substrate and report whether
    /// the slot was already text-resident at session entry.
    #[inline(always)]
    pub fn slot_update_text_resident_first_raw<R>(
        &self,
        idx: i64,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<(R, bool)> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible)
            .update(idx, f)
            .map(|(out, kind)| {
                (
                    out,
                    matches!(kind, ArrayTextSlotSessionOutcomeKind::ResidentText),
                )
            })
    }

    /// Runtime-private repeated text-cell update for a MIR-proven loop region.
    /// The write guard stays inside this call; legality and loop shape stay MIR-owned.
    #[inline(always)]
    pub fn slot_text_region_update_sum_raw(
        &self,
        loop_bound: i64,
        row_modulus: i64,
        mut f: impl FnMut(&mut String) -> Option<i64>,
    ) -> Option<i64> {
        if loop_bound < 0 || row_modulus <= 0 {
            return None;
        }
        if loop_bound == 0 {
            return Some(0);
        }
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            let mut total = 0_i64;
            for step in 0..loop_bound {
                let idx = (step % row_modulus) as usize;
                let value = values.get_mut(idx)?;
                total = total.checked_add(f(value.as_mut_string())?)?;
            }
            return Some(total);
        }

        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        let mut total = 0_i64;
        for step in 0..loop_bound {
            let idx = (step % row_modulus) as usize;
            let (delta, _kind) = session.update(idx, |value| f(value))?;
            total = total.checked_add(delta?)?;
        }
        Some(total)
    }

    /// Runtime-private observer/store executor for a MIR-proven text-cell region.
    /// The write guard stays inside this call; legality and publication stay MIR-owned.
    #[inline(always)]
    pub fn slot_text_indexof_suffix_store_region_raw(
        &self,
        loop_bound: i64,
        needle: &str,
        suffix: &str,
    ) -> Option<i64> {
        if loop_bound < 0 {
            return None;
        }
        let loop_bound = usize::try_from(loop_bound).ok()?;
        if loop_bound == 0 {
            return Some(0);
        }
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            if loop_bound > values.len() {
                return None;
            }
            let mut stores = 0_i64;
            for value in values.iter_mut().take(loop_bound) {
                if text_contains_literal(value.as_str(), needle) {
                    append_text_suffix(value.as_mut_string(), suffix);
                    stores += 1;
                }
            }
            return Some(stores);
        }

        let boxed_len = match &*items {
            ArrayStorage::Boxed(boxed) => boxed.len(),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => return None,
            ArrayStorage::Text(_) => unreachable!("text storage returned above"),
        };
        if loop_bound > boxed_len {
            return None;
        }

        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        let mut stores = 0_i64;
        for idx in 0..loop_bound {
            let (hit, _kind) = session.update(idx, |value| {
                if text_contains_literal(value, needle) {
                    append_text_suffix(value, suffix);
                    1_i64
                } else {
                    0_i64
                }
            })?;
            stores += hit;
        }
        Some(stores)
    }
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
