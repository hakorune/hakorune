use super::string_span_cache::{
    string_span_cache_get, string_span_cache_get_pair, string_span_cache_put,
};
use super::string_birth_placement::{substring_retention_class, TextRetentionClass};
use nyash_rust::{
    box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::any::Any;
use std::sync::Arc;

/// StringView(base_handle, [start, end)) keeps substring metadata only.
/// v0 contract:
/// - read-only string helpers resolve via this metadata without eager copy
/// - clone/materialize boundaries convert to StringBox to avoid long-lived base retention
#[derive(Clone)]
pub(crate) struct StringViewBox {
    pub(crate) base_handle: i64,
    pub(crate) base_obj: Arc<dyn NyashBox>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    base: BoxBase,
}

impl std::fmt::Debug for StringViewBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringViewBox")
            .field("base_handle", &self.base_handle)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl StringViewBox {
    pub(crate) fn new(
        base_handle: i64,
        base_obj: Arc<dyn NyashBox>,
        start: usize,
        end: usize,
    ) -> Self {
        let (start, end) = if end < start {
            (end, start)
        } else {
            (start, end)
        };
        Self {
            base_handle,
            base_obj,
            start,
            end,
            base: BoxBase::new(),
        }
    }

    fn materialize_owned(&self) -> String {
        resolve_string_span_from_view(self)
            .map(|span| span.as_str().to_string())
            .unwrap_or_default()
    }
}

impl BoxCore for StringViewBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.materialize_owned())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for StringViewBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(self.materialize_owned())
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        BoolBox::new(self.to_string_box().value == other.to_string_box().value)
    }

    fn type_name(&self) -> &'static str {
        "StringViewBox"
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        // Materialize on clone boundary to avoid leaking long-lived base retention
        // into container/storage paths (map/array persistent store boundary).
        Box::new(self.to_string_box())
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn as_str_fast(&self) -> Option<&str> {
        let base_sb = self.base_obj.as_any().downcast_ref::<StringBox>()?;
        let (st, en) = clamp_usize_range(base_sb.value.len(), self.start, self.end);
        base_sb.value.get(st..en)
    }
}

#[derive(Clone)]
pub(crate) struct StringSpan {
    /// Root base handle (StringBox) this span refers to.
    base_handle: i64,
    /// Root StringBox object.
    base_obj: Arc<dyn NyashBox>,
    /// Absolute byte range on the root string.
    start: usize,
    end: usize,
}

impl StringSpan {
    pub(crate) fn len(&self) -> usize {
        self.as_str().len()
    }

    pub(super) fn span_bytes_len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub(crate) fn as_str(&self) -> &str {
        let Some(sb) = self.base_obj.as_any().downcast_ref::<StringBox>() else {
            return "";
        };
        sb.value.get(self.start..self.end).unwrap_or("")
    }

    pub(crate) fn slice_range(&self, start: usize, end: usize) -> Self {
        let (st, en) = clamp_usize_range(self.span_bytes_len(), start, end);
        Self {
            base_handle: self.base_handle,
            base_obj: self.base_obj.clone(),
            start: self.start.saturating_add(st),
            end: self.start.saturating_add(en),
        }
    }

    pub(crate) fn into_view_box(self) -> StringViewBox {
        StringViewBox::new(self.base_handle, self.base_obj, self.start, self.end)
    }
}

impl std::fmt::Debug for StringSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringSpan")
            .field("base_handle", &self.base_handle)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum TextPiece<'a> {
    Span(StringSpan),
    Inline(&'a str),
}

impl<'a> TextPiece<'a> {
    fn len(&self) -> usize {
        match self {
            Self::Span(span) => span.len(),
            Self::Inline(text) => text.len(),
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn append_to(&self, out: &mut String) {
        match self {
            Self::Span(span) => out.push_str(span.as_str()),
            Self::Inline(text) => out.push_str(text),
        }
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
    pub(crate) fn from_span(span: StringSpan) -> Self {
        Self::View1(span)
    }

    pub(crate) fn from_owned(value: String) -> Self {
        Self::OwnedTmp(value)
    }

    pub(crate) fn from_handle(handle: i64) -> Option<Self> {
        resolve_string_span_from_handle(handle).map(Self::View1)
    }

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

    pub(crate) fn from_two(a: TextPiece<'a>, b: TextPiece<'a>) -> Self {
        Self::from_pair(a, b)
    }

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
                Self::Pieces2 {
                    a,
                    b: c,
                    total_len,
                }
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
            1 => match pieces.pop().expect("normalized piece list should have one element") {
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
                a.append_to(&mut out);
                b.append_to(&mut out);
                c.append_to(&mut out);
                d.append_to(&mut out);
                out.push_str(inline);
                Self::OwnedTmp(out)
            }
            Self::OwnedTmp(mut text) => {
                text.push_str(inline);
                Self::OwnedTmp(text)
            }
        }
    }

    pub(crate) fn insert_inline(self, middle: &'a str, split: usize) -> Self {
        if middle.is_empty() {
            return self;
        }
        match self {
            Self::View1(span) => {
                let source = span.as_str();
                let split = split.min(source.len());
                let prefix = span.slice_range(0, split);
                let suffix = span.slice_range(split, source.len());
                Self::from_three(
                    TextPiece::Span(prefix),
                    TextPiece::Inline(middle),
                    TextPiece::Span(suffix),
                )
            }
            Self::Pieces2 { .. } | Self::Pieces3 { .. } | Self::Pieces4 { .. } | Self::OwnedTmp(_) => {
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

    pub(crate) fn into_owned(self) -> String {
        match self {
            Self::View1(span) => span.as_str().to_string(),
            Self::Pieces2 { a, b, total_len } => {
                let mut out = String::with_capacity(total_len);
                a.append_to(&mut out);
                b.append_to(&mut out);
                out
            }
            Self::Pieces3 { a, b, c, total_len } => {
                let mut out = String::with_capacity(total_len);
                a.append_to(&mut out);
                b.append_to(&mut out);
                c.append_to(&mut out);
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
                a.append_to(&mut out);
                b.append_to(&mut out);
                c.append_to(&mut out);
                d.append_to(&mut out);
                out
            }
            Self::OwnedTmp(text) => text,
        }
    }
}

pub(crate) fn clamp_i64_range(len: usize, start: i64, end: i64) -> (usize, usize) {
    let n = len as i64;
    let mut st = if start < 0 { 0 } else { start };
    let mut en = if end < 0 { 0 } else { end };
    if st > n {
        st = n;
    }
    if en > n {
        en = n;
    }
    if en < st {
        std::mem::swap(&mut st, &mut en);
    }
    (st as usize, en as usize)
}

pub(crate) enum BorrowedSubstringPlan {
    ReturnHandle,
    ReturnEmpty,
    FreezePlan(TextPlan<'static>),
    ViewSpan(StringSpan),
}

pub(crate) fn borrowed_substring_plan_from_handle(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<BorrowedSubstringPlan> {
    if handle <= 0 {
        return None;
    }
    handles::with_handle(handle as u64, |obj| {
        let Some(obj) = obj else {
            return None;
        };
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            let (st_rel, en_rel) = clamp_i64_range(sb.value.len(), start, end);
            if st_rel == 0 && en_rel == sb.value.len() {
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
            if st_rel == en_rel {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            }
            let Some(sub_slice) = sb.value.get(st_rel..en_rel) else {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            };
            let placement = substring_retention_class(view_enabled, sub_slice.len());
            let span = StringSpan {
                base_handle: handle,
                base_obj: obj.clone(),
                start: st_rel,
                end: en_rel,
            };
            match placement {
                TextRetentionClass::RetainView => {
                    return Some(BorrowedSubstringPlan::ViewSpan(span));
                }
                TextRetentionClass::MustFreeze(_) | TextRetentionClass::KeepTransient => {
                    return Some(BorrowedSubstringPlan::FreezePlan(TextPlan::from_span(span)));
                }
                TextRetentionClass::ReturnHandle => {
                    return Some(BorrowedSubstringPlan::ReturnHandle);
                }
            }
        }
        if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
            let Some(base_sb) = view.base_obj.as_any().downcast_ref::<StringBox>() else {
                return None;
            };
            let (parent_st, parent_en) =
                clamp_usize_range(base_sb.value.len(), view.start, view.end);
            let parent_len = parent_en.saturating_sub(parent_st);
            let (st_rel, en_rel) = clamp_i64_range(parent_len, start, end);
            if st_rel == 0 && en_rel == parent_len {
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
            if st_rel == en_rel {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            }
            let abs_st = parent_st.saturating_add(st_rel);
            let abs_en = parent_st.saturating_add(en_rel);
            let Some(sub_slice) = base_sb.value.get(abs_st..abs_en) else {
                return Some(BorrowedSubstringPlan::ReturnEmpty);
            };
            let placement = substring_retention_class(view_enabled, sub_slice.len());
            let span = StringSpan {
                base_handle: view.base_handle,
                base_obj: view.base_obj.clone(),
                start: abs_st,
                end: abs_en,
            };
            match placement {
                TextRetentionClass::RetainView => {
                    return Some(BorrowedSubstringPlan::ViewSpan(span));
                }
                TextRetentionClass::MustFreeze(_) | TextRetentionClass::KeepTransient => {
                    return Some(BorrowedSubstringPlan::FreezePlan(TextPlan::from_span(span)));
                }
                TextRetentionClass::ReturnHandle => {
                    return Some(BorrowedSubstringPlan::ReturnHandle);
                }
            }
        }
        None
    })
}

fn clamp_usize_range(len: usize, start: usize, end: usize) -> (usize, usize) {
    let mut st = start.min(len);
    let mut en = end.min(len);
    if en < st {
        std::mem::swap(&mut st, &mut en);
    }
    (st, en)
}

pub(crate) fn resolve_string_span_from_obj(
    handle: i64,
    obj: Arc<dyn NyashBox>,
) -> Option<StringSpan> {
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        let len = sb.value.len();
        return Some(StringSpan {
            base_handle: handle,
            base_obj: obj,
            start: 0,
            end: len,
        });
    }
    if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
        return resolve_string_span_from_view(view);
    }
    None
}

fn resolve_string_span_from_handle_uncached(handle: i64) -> Option<StringSpan> {
    if handle <= 0 {
        return None;
    }
    let obj = handles::get(handle as u64)?;
    resolve_string_span_from_obj(handle, obj)
}

#[inline(always)]
pub(crate) fn resolve_string_span_from_handle_nocache(handle: i64) -> Option<StringSpan> {
    resolve_string_span_from_handle_uncached(handle)
}

#[inline(always)]
fn resolve_string_span_from_handle_with_epoch(handle: i64, drop_epoch: u64) -> Option<StringSpan> {
    if handle <= 0 {
        return None;
    }
    if let Some(span) = string_span_cache_get(handle, drop_epoch) {
        return Some(span);
    }
    let span = resolve_string_span_from_handle_uncached(handle)?;
    string_span_cache_put(handle, drop_epoch, &span);
    Some(span)
}

pub(crate) fn resolve_string_span_from_handle(handle: i64) -> Option<StringSpan> {
    resolve_string_span_from_handle_with_epoch(handle, handles::drop_epoch())
}

pub(crate) fn resolve_string_span_pair_from_handles(
    a_h: i64,
    b_h: i64,
) -> Option<(StringSpan, StringSpan)> {
    if a_h <= 0 || b_h <= 0 {
        return None;
    }
    if a_h == b_h {
        let span = resolve_string_span_from_handle(a_h)?;
        return Some((span.clone(), span));
    }
    let drop_epoch = handles::drop_epoch();
    let (a_cached, b_cached) = string_span_cache_get_pair(a_h, b_h, drop_epoch);
    match (a_cached, b_cached) {
        (Some(a_span), Some(b_span)) => return Some((a_span, b_span)),
        (Some(a_span), None) => {
            let b_span = resolve_string_span_from_handle_with_epoch(b_h, drop_epoch)?;
            return Some((a_span, b_span));
        }
        (None, Some(b_span)) => {
            let a_span = resolve_string_span_from_handle_with_epoch(a_h, drop_epoch)?;
            return Some((a_span, b_span));
        }
        (None, None) => {}
    }

    let (a_obj, b_obj) = handles::get_pair(a_h as u64, b_h as u64);
    let a_obj = a_obj?;
    let b_obj = b_obj?;
    let a_span = resolve_string_span_from_obj(a_h, a_obj)?;
    let b_span = resolve_string_span_from_obj(b_h, b_obj)?;
    string_span_cache_put(a_h, drop_epoch, &a_span);
    string_span_cache_put(b_h, drop_epoch, &b_span);
    Some((a_span, b_span))
}

fn resolve_string_span_from_view(view: &StringViewBox) -> Option<StringSpan> {
    // Fast path: v0 views keep a strong root reference to avoid repeated
    // registry lookup on every string helper call.
    if let Some(base_sb) = view.base_obj.as_any().downcast_ref::<StringBox>() {
        let (st, en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: view.base_handle,
            base_obj: view.base_obj.clone(),
            start: st,
            end: en,
        });
    }

    if view.base_handle <= 0 {
        return None;
    }
    let base_obj = handles::get(view.base_handle as u64)?;

    if let Some(base_sb) = base_obj.as_any().downcast_ref::<StringBox>() {
        let (st, en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: view.base_handle,
            base_obj,
            start: st,
            end: en,
        });
    }

    // Defensive flattening for malformed/nested view metadata.
    if let Some(parent_view) = base_obj.as_any().downcast_ref::<StringViewBox>() {
        let parent_span = resolve_string_span_from_view(parent_view)?;
        let (st_rel, en_rel) = clamp_usize_range(parent_span.len(), view.start, view.end);
        return Some(StringSpan {
            base_handle: parent_span.base_handle,
            base_obj: parent_span.base_obj.clone(),
            start: parent_span.start + st_rel,
            end: parent_span.start + en_rel,
        });
    }
    None
}

pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    resolve_string_span_from_handle(handle).map(|span| span.len() as i64)
}

pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    resolve_string_span_from_handle(handle).map(|span| span.len() == 0)
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
