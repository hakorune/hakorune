// Native string plan carrier.
// This file stays below semantic ownership: it packages span/inline pieces for Rust materialize
// and freeze leaves, but it does not decide string route semantics.

use super::string_view::{resolve_string_span_from_handle, StringSpan};
use std::ptr;

#[derive(Clone, Debug)]
pub(crate) enum TextPiece<'a> {
    Span(StringSpan),
    Inline(&'a str),
}

impl<'a> TextPiece<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        match self {
            Self::Span(span) => span.len(),
            Self::Inline(text) => text.len(),
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn append_to_reserved(&self, out: &mut String) {
        match self {
            Self::Span(span) => {
                let text = span.as_text();
                let len = text.len();
                if len == 0 {
                    return;
                }
                let dst_len = out.len();
                unsafe {
                    let dst = out.as_mut_ptr().add(dst_len);
                    ptr::copy_nonoverlapping(text.as_ptr(), dst, len);
                    out.as_mut_vec().set_len(dst_len + len);
                }
            }
            Self::Inline(text) => {
                let len = text.len();
                if len == 0 {
                    return;
                }
                let dst_len = out.len();
                unsafe {
                    let dst = out.as_mut_ptr().add(dst_len);
                    ptr::copy_nonoverlapping(text.as_ptr(), dst, len);
                    out.as_mut_vec().set_len(dst_len + len);
                }
            }
        }
    }

    #[inline(always)]
    fn append_to(&self, out: &mut String) {
        out.reserve(self.len());
        self.append_to_reserved(out);
    }
}

#[derive(Clone, Debug)]
pub(crate) enum TextPlan<'a> {
    View1(StringSpan),
    Pieces2 {
        a: TextPiece<'a>,
        b: TextPiece<'a>,
        total_len: usize,
    },
    Pieces3 {
        a: TextPiece<'a>,
        b: TextPiece<'a>,
        c: TextPiece<'a>,
        total_len: usize,
    },
    Pieces4 {
        a: TextPiece<'a>,
        b: TextPiece<'a>,
        c: TextPiece<'a>,
        d: TextPiece<'a>,
        total_len: usize,
    },
    OwnedTmp(String),
}

impl<'a> TextPlan<'a> {
    #[inline(always)]
    pub(crate) fn from_span(span: StringSpan) -> Self {
        Self::View1(span)
    }

    #[inline(always)]
    pub(crate) fn from_owned(value: String) -> Self {
        Self::OwnedTmp(value)
    }

    #[inline(always)]
    pub(crate) fn from_handle(handle: i64) -> Option<Self> {
        resolve_string_span_from_handle(handle).map(Self::View1)
    }

    #[inline(always)]
    fn from_pair(a: TextPiece<'a>, b: TextPiece<'a>) -> Self {
        match (a.is_empty(), b.is_empty()) {
            (true, true) => Self::OwnedTmp(String::new()),
            (true, false) => match b {
                TextPiece::Span(span) => Self::View1(span),
                TextPiece::Inline(text) => Self::OwnedTmp(text.to_owned()),
            },
            (false, true) => match a {
                TextPiece::Span(span) => Self::View1(span),
                TextPiece::Inline(text) => Self::OwnedTmp(text.to_owned()),
            },
            (false, false) => {
                let total_len = a.len() + b.len();
                Self::Pieces2 { a, b, total_len }
            }
        }
    }

    #[inline(always)]
    pub(crate) fn from_two(a: TextPiece<'a>, b: TextPiece<'a>) -> Self {
        Self::from_pair(a, b)
    }

    #[inline(always)]
    pub(crate) fn from_three(a: TextPiece<'a>, b: TextPiece<'a>, c: TextPiece<'a>) -> Self {
        match (a.is_empty(), b.is_empty(), c.is_empty()) {
            (true, true, true) => Self::OwnedTmp(String::new()),
            (true, true, false) => Self::from_pair(b, c),
            (true, false, true) => Self::from_pair(b, c),
            (false, true, true) => Self::from_pair(a, c),
            (true, false, false) => {
                let total_len = b.len() + c.len();
                Self::Pieces2 {
                    a: b,
                    b: c,
                    total_len,
                }
            }
            (false, true, false) => {
                let total_len = a.len() + c.len();
                Self::Pieces2 { a, b: c, total_len }
            }
            (false, false, true) => {
                let total_len = a.len() + b.len();
                Self::Pieces2 { a, b, total_len }
            }
            (false, false, false) => {
                let total_len = a.len() + b.len() + c.len();
                Self::Pieces3 { a, b, c, total_len }
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn from_pieces(pieces: Vec<TextPiece<'a>>) -> Self {
        let mut pieces = pieces
            .into_iter()
            .filter(|piece| !piece.is_empty())
            .collect::<Vec<_>>();
        match pieces.len() {
            0 => Self::OwnedTmp(String::new()),
            1 => match pieces
                .pop()
                .expect("normalized piece list should have one element")
            {
                TextPiece::Span(span) => Self::View1(span),
                TextPiece::Inline(text) => Self::OwnedTmp(text.to_owned()),
            },
            2 => {
                let mut iter = pieces.into_iter();
                Self::from_two(
                    iter.next()
                        .expect("normalized piece list should have two elements"),
                    iter.next()
                        .expect("normalized piece list should have two elements"),
                )
            }
            3 => {
                let mut iter = pieces.into_iter();
                Self::from_three(
                    iter.next()
                        .expect("normalized piece list should have three elements"),
                    iter.next()
                        .expect("normalized piece list should have three elements"),
                    iter.next()
                        .expect("normalized piece list should have three elements"),
                )
            }
            _ => {
                let mut out = String::new();
                for piece in pieces {
                    piece.append_to(&mut out);
                }
                Self::OwnedTmp(out)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn concat_inline(self, inline: &'a str) -> Self {
        if inline.is_empty() {
            return self;
        }
        match self {
            Self::View1(span) => Self::from_two(TextPiece::Span(span), TextPiece::Inline(inline)),
            Self::Pieces2 { a, b, total_len } => {
                let total_len = total_len.saturating_add(inline.len());
                Self::Pieces3 {
                    a,
                    b,
                    c: TextPiece::Inline(inline),
                    total_len,
                }
            }
            Self::Pieces3 { a, b, c, total_len } => {
                let total_len = total_len.saturating_add(inline.len());
                Self::Pieces4 {
                    a,
                    b,
                    c,
                    d: TextPiece::Inline(inline),
                    total_len,
                }
            }
            Self::Pieces4 {
                a,
                b,
                c,
                d,
                total_len,
            } => {
                let mut out = String::with_capacity(total_len.saturating_add(inline.len()));
                a.append_to_reserved(&mut out);
                b.append_to_reserved(&mut out);
                c.append_to_reserved(&mut out);
                d.append_to_reserved(&mut out);
                out.push_str(inline);
                Self::OwnedTmp(out)
            }
            Self::OwnedTmp(mut text) => {
                text.push_str(inline);
                Self::OwnedTmp(text)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn insert_inline(self, middle: &'a str, split: usize) -> Self {
        if middle.is_empty() {
            return self;
        }
        match self {
            Self::View1(span) => {
                let source = span.as_text();
                let split = split.min(source.len());
                let prefix = span.slice_range(0, split);
                let suffix = span.slice_range(split, source.len());
                Self::from_three(
                    TextPiece::Span(prefix),
                    TextPiece::Inline(middle),
                    TextPiece::Span(suffix),
                )
            }
            Self::Pieces2 { .. }
            | Self::Pieces3 { .. }
            | Self::Pieces4 { .. }
            | Self::OwnedTmp(_) => {
                let source = self.into_owned();
                let split = split.min(source.len());
                let prefix = source.get(0..split).unwrap_or("");
                let suffix = source.get(split..).unwrap_or("");
                let total = prefix.len() + middle.len() + suffix.len();
                let mut out = String::with_capacity(total);
                out.push_str(prefix);
                out.push_str(middle);
                out.push_str(suffix);
                Self::OwnedTmp(out)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn into_owned(self) -> String {
        match self {
            Self::View1(span) => {
                let text = span.as_text();
                if text.is_empty() {
                    String::new()
                } else {
                    text.to_string()
                }
            }
            Self::Pieces2 { a, b, total_len } => {
                let mut out = String::with_capacity(total_len);
                a.append_to_reserved(&mut out);
                b.append_to_reserved(&mut out);
                out
            }
            Self::Pieces3 { a, b, c, total_len } => {
                let mut out = String::with_capacity(total_len);
                a.append_to_reserved(&mut out);
                b.append_to_reserved(&mut out);
                c.append_to_reserved(&mut out);
                out
            }
            Self::Pieces4 {
                a,
                b,
                c,
                d,
                total_len,
            } => {
                let mut out = String::with_capacity(total_len);
                a.append_to_reserved(&mut out);
                b.append_to_reserved(&mut out);
                c.append_to_reserved(&mut out);
                d.append_to_reserved(&mut out);
                out
            }
            Self::OwnedTmp(text) => text,
        }
    }
}

#[inline(always)]
pub(crate) fn concat_const_suffix_plan_from_handle<'a>(a_h: i64, suffix: &'a str) -> TextPlan<'a> {
    if let Some(plan) = TextPlan::from_handle(a_h) {
        return plan.concat_inline(suffix);
    }
    let lhs = super::string::to_owned_string_handle_arg(a_h);
    TextPlan::from_owned(lhs).concat_inline(suffix)
}

#[inline(always)]
pub(crate) fn insert_const_mid_plan_from_handle<'a>(
    source_h: i64,
    middle: &'a str,
    split: i64,
) -> TextPlan<'a> {
    if let Some(source_span) = resolve_string_span_from_handle(source_h) {
        let split = split.clamp(0, source_span.span_bytes_len() as i64) as usize;
        return TextPlan::from_span(source_span).insert_inline(middle, split);
    }

    let source = super::string::to_owned_string_handle_arg(source_h);
    let split = split.clamp(0, source.len() as i64) as usize;
    TextPlan::from_owned(source).insert_inline(middle, split)
}

#[cfg(test)]
mod tests {
    use super::{TextPiece, TextPlan};

    #[test]
    fn normalized_piece_list_flattens_into_owned_text() {
        let plan = TextPlan::from_pieces(vec![TextPiece::Inline("ab"), TextPiece::Inline("cd")]);
        assert_eq!(plan.into_owned(), "abcd");
    }

    #[test]
    fn owned_plan_can_be_extended_and_inserted() {
        let plan = TextPlan::from_owned("abc".to_owned())
            .concat_inline("de")
            .insert_inline("X", 2);
        assert_eq!(plan.into_owned(), "abXcde");
    }
}
