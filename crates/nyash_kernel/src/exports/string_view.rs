use super::string_span_cache::{
    string_span_cache_get, string_span_cache_get_pair, string_span_cache_put,
};
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
}

pub(crate) fn string_view_from_span_range(
    span: &StringSpan,
    start_rel: usize,
    end_rel: usize,
) -> StringViewBox {
    StringViewBox::new(
        span.base_handle,
        span.base_obj.clone(),
        span.start.saturating_add(start_rel),
        span.start.saturating_add(end_rel),
    )
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
