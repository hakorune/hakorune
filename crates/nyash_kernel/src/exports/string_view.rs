use super::string_birth_placement::{substring_retention_class, RetainedForm};
use super::string_span_cache::{
    string_span_cache_get, string_span_cache_get_pair, string_span_cache_get_triplet,
    string_span_cache_put,
};
use super::string_trace;
use nyash_rust::{
    box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
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
    ViewSpan(StringSpan),
}

#[inline(always)]
pub(crate) fn borrowed_substring_plan_from_live_object(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    obj: &Arc<dyn NyashBox>,
) -> Option<BorrowedSubstringPlan> {
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        let (st_rel, en_rel) = clamp_i64_range(sb.value.len(), start, end);
        if st_rel == 0 && en_rel == sb.value.len() {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_handle",
                "root_full_span",
                "StringBox",
                en_rel.saturating_sub(st_rel),
            );
            return Some(BorrowedSubstringPlan::ReturnHandle);
        }
        if st_rel == en_rel {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_empty",
                "root_empty_span",
                "StringBox",
                0,
            );
            return Some(BorrowedSubstringPlan::ReturnEmpty);
        }
        if sb.value.get(st_rel..en_rel).is_none() {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_empty",
                "root_out_of_range",
                "StringBox",
                en_rel.saturating_sub(st_rel),
            );
            return Some(BorrowedSubstringPlan::ReturnEmpty);
        }
        let placement = substring_retention_class(view_enabled, en_rel - st_rel);
        let span = StringSpan {
            base_handle: handle,
            base_obj: obj.clone(),
            start: st_rel,
            end: en_rel,
        };
        match placement {
            RetainedForm::RetainView => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "view_span",
                    "retain_view",
                    "StringBox",
                    en_rel.saturating_sub(st_rel),
                );
                return Some(BorrowedSubstringPlan::ViewSpan(span));
            }
            RetainedForm::MustFreeze(_) | RetainedForm::KeepTransient => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "freeze_span",
                    "must_freeze_or_keep",
                    "StringBox",
                    en_rel.saturating_sub(st_rel),
                );
                return Some(BorrowedSubstringPlan::FreezeSpan(span));
            }
            RetainedForm::ReturnHandle => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "return_handle",
                    "placement_return_handle",
                    "StringBox",
                    en_rel.saturating_sub(st_rel),
                );
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
        }
    }
    if let Some(view) = obj.as_any().downcast_ref::<StringViewBox>() {
        let Some(base_sb) = view.base_obj.as_any().downcast_ref::<StringBox>() else {
            return None;
        };
        let (parent_st, parent_en) = clamp_usize_range(base_sb.value.len(), view.start, view.end);
        let parent_len = parent_en.saturating_sub(parent_st);
        let (st_rel, en_rel) = clamp_i64_range(parent_len, start, end);
        if st_rel == 0 && en_rel == parent_len {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_handle",
                "view_full_span",
                "StringViewBox",
                en_rel.saturating_sub(st_rel),
            );
            return Some(BorrowedSubstringPlan::ReturnHandle);
        }
        if st_rel == en_rel {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_empty",
                "view_empty_span",
                "StringViewBox",
                0,
            );
            return Some(BorrowedSubstringPlan::ReturnEmpty);
        }
        let abs_st = parent_st.saturating_add(st_rel);
        let abs_en = parent_st.saturating_add(en_rel);
        if base_sb.value.get(abs_st..abs_en).is_none() {
            trace_borrowed_substring_plan(
                handle,
                start,
                end,
                view_enabled,
                "return_empty",
                "view_out_of_range",
                "StringViewBox",
                abs_en.saturating_sub(abs_st),
            );
            return Some(BorrowedSubstringPlan::ReturnEmpty);
        }
        let placement = substring_retention_class(view_enabled, abs_en - abs_st);
        let span = StringSpan {
            base_handle: view.base_handle,
            base_obj: view.base_obj.clone(),
            start: abs_st,
            end: abs_en,
        };
        match placement {
            RetainedForm::RetainView => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "view_span",
                    "retain_view",
                    "StringViewBox",
                    abs_en.saturating_sub(abs_st),
                );
                return Some(BorrowedSubstringPlan::ViewSpan(span));
            }
            RetainedForm::MustFreeze(_) | RetainedForm::KeepTransient => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "freeze_span",
                    "must_freeze_or_keep",
                    "StringViewBox",
                    abs_en.saturating_sub(abs_st),
                );
                return Some(BorrowedSubstringPlan::FreezeSpan(span));
            }
            RetainedForm::ReturnHandle => {
                trace_borrowed_substring_plan(
                    handle,
                    start,
                    end,
                    view_enabled,
                    "return_handle",
                    "placement_return_handle",
                    "StringViewBox",
                    abs_en.saturating_sub(abs_st),
                );
                return Some(BorrowedSubstringPlan::ReturnHandle);
            }
        }
    }
    None
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
        borrowed_substring_plan_from_live_object(handle, start, end, view_enabled, obj)
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

#[cold]
#[inline(never)]
fn trace_borrowed_substring_plan(
    handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    result: &str,
    reason: &str,
    source_kind: &str,
    span_len: usize,
) {
    if !string_trace::enabled() {
        return;
    }
    string_trace::emit(
        "carrier",
        result,
        reason,
        format_args!(
            "handle={} start={} end={} view_enabled={} source_kind={} span_len={}",
            handle, start, end, view_enabled, source_kind, span_len
        ),
    );
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
pub(crate) fn resolve_string_span_from_handle(handle: i64) -> Option<StringSpan> {
    resolve_string_span_from_handle_with_epoch(handle, handles::drop_epoch())
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn resolve_string_span_triplet_from_handles(
    a_h: i64,
    b_h: i64,
    c_h: i64,
) -> Option<(StringSpan, StringSpan, StringSpan)> {
    if a_h <= 0 || b_h <= 0 || c_h <= 0 {
        return None;
    }
    let drop_epoch = handles::drop_epoch();
    let (a_cached, b_cached, c_cached) = string_span_cache_get_triplet(a_h, b_h, c_h, drop_epoch);

    let a_span = match a_cached {
        Some(span) => span,
        None => {
            let span = resolve_string_span_from_handle_uncached(a_h)?;
            string_span_cache_put(a_h, drop_epoch, &span);
            span
        }
    };
    let b_span = if b_h == a_h {
        a_span.clone()
    } else {
        match b_cached {
            Some(span) => span,
            None => {
                let span = resolve_string_span_from_handle_uncached(b_h)?;
                string_span_cache_put(b_h, drop_epoch, &span);
                span
            }
        }
    };
    let c_span = if c_h == a_h {
        a_span.clone()
    } else if c_h == b_h {
        b_span.clone()
    } else {
        match c_cached {
            Some(span) => span,
            None => {
                let span = resolve_string_span_from_handle_uncached(c_h)?;
                string_span_cache_put(c_h, drop_epoch, &span);
                span
            }
        }
    };
    Some((a_span, b_span, c_span))
}

#[inline(always)]
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
