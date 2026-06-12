use std::cmp::Ordering;

const MID_GAP_RIGHT_COMPACT_MIN: usize = 4096;
const MID_GAP_LEFT_OVERSHOOT_LIMIT: usize = 1024;
const MID_GAP_INITIAL_HEADROOM: usize = 64;

#[path = "text_cell_helpers.rs"]
mod helpers;

use self::helpers::*;

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
                    insert_const_mid_lenhalf_string(value, middle)
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
                    let out = insert_const_mid_lenhalf_string(&mut value, middle);
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
                    insert_const_mid_lenhalf_string_byte_boundary_safe(value, middle)
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
                    let out = insert_const_mid_lenhalf_string(&mut value, middle);
                    *self = Self::Flat(value);
                    out
                }
            },
        }
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string(value: &mut String, middle: &str) -> i64 {
        insert_const_mid_lenhalf_string(value, middle)
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string_byte_boundary_safe(
        value: &mut String,
        middle: &str,
    ) -> i64 {
        insert_const_mid_lenhalf_string_byte_boundary_safe(value, middle)
    }
}

impl From<String> for ArrayTextCell {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::flat(value)
    }
}

#[cfg(test)]
#[path = "text_cell_tests.rs"]
mod tests;
