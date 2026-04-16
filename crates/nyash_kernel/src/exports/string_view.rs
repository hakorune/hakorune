#[path = "string_view/span_resolve.rs"]
mod span_resolve;
#[path = "string_view/substring_plan.rs"]
mod substring_plan;

use nyash_rust::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox};
use std::any::Any;
use std::sync::Arc;

// Lifetime-sensitive string substrate.
// Keep borrowed view/span ownership in Rust even if higher-level String semantics move toward
// `.hako` owner code. This file is native substrate, not a semantic owner.

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
    #[inline(always)]
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

    #[inline(always)]
    fn materialize_owned(&self) -> String {
        span_resolve::resolve_string_span_from_view(self)
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
    pub(crate) fn base_handle(&self) -> i64 {
        self.base_handle
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn end(&self) -> usize {
        self.end
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.span_bytes_len()
    }

    #[inline(always)]
    pub(super) fn span_bytes_len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        let Some(sb) = self.base_obj.as_any().downcast_ref::<StringBox>() else {
            return "";
        };
        sb.value.get(self.start..self.end).unwrap_or("")
    }

    #[inline(always)]
    pub(crate) fn slice_range(&self, start: usize, end: usize) -> Self {
        let (st, en) = clamp_usize_range(self.span_bytes_len(), start, end);
        Self {
            base_handle: self.base_handle,
            base_obj: self.base_obj.clone(),
            start: self.start.saturating_add(st),
            end: self.start.saturating_add(en),
        }
    }

    #[inline(always)]
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
    FreezeSpan(StringSpan),
    ViewSpan {
        span: StringSpan,
        source_box_id: u64,
    },
}

pub(crate) fn borrowed_substring_plan_from_handle(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<BorrowedSubstringPlan> {
    substring_plan::borrowed_substring_plan_from_handle(handle, start, end, view_enabled)
}

fn clamp_usize_range(len: usize, start: usize, end: usize) -> (usize, usize) {
    let mut st = start.min(len);
    let mut en = end.min(len);
    if en < st {
        std::mem::swap(&mut st, &mut en);
    }
    (st, en)
}

#[inline(always)]
pub(crate) fn resolve_string_span_from_handle_nocache(handle: i64) -> Option<StringSpan> {
    span_resolve::resolve_string_span_from_handle_nocache(handle)
}

#[inline(always)]
pub(crate) fn resolve_string_span_from_handle(handle: i64) -> Option<StringSpan> {
    span_resolve::resolve_string_span_from_handle(handle)
}

#[inline(always)]
pub(crate) fn resolve_string_span_pair_from_handles(
    a_h: i64,
    b_h: i64,
) -> Option<(StringSpan, StringSpan)> {
    span_resolve::resolve_string_span_pair_from_handles(a_h, b_h)
}

#[inline(always)]
pub(crate) fn resolve_string_span_triplet_from_handles(
    a_h: i64,
    b_h: i64,
    c_h: i64,
) -> Option<(StringSpan, StringSpan, StringSpan)> {
    span_resolve::resolve_string_span_triplet_from_handles(a_h, b_h, c_h)
}

pub(crate) fn string_len_from_handle(handle: i64) -> Option<i64> {
    span_resolve::string_len_from_handle(handle)
}

pub(crate) fn string_is_empty_from_handle(handle: i64) -> Option<bool> {
    span_resolve::string_is_empty_from_handle(handle)
}
