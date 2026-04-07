use super::array_guard::valid_handle_idx;
use super::handle_cache::{cache_probe_kind, CacheProbeKind as HandleCacheProbeKind};
use super::value_codec::{
    maybe_store_non_string_box_from_verified_source, store_string_box_from_verified_text_source,
    try_retarget_borrowed_string_slot_take_verified_text_source, with_array_store_str_source,
    ArrayStoreStrSource, BorrowedHandleBox, StringHandleSourceKind,
};
use crate::exports::string_view::resolve_string_span_from_handle;
use crate::observe::{self, CacheProbeKind as ObserveCacheProbeKind};
use memchr::{memchr, memmem};
use nyash_rust::runtime::host_handles as handles;
use std::cell::RefCell;

struct CachedNeedle {
    handle: i64,
    drop_epoch: u64,
    value: String,
}

thread_local! {
    static ARRAY_STRING_INDEXOF_NEEDLE_CACHE: RefCell<Option<CachedNeedle>> = const { RefCell::new(None) };
}

#[inline(always)]
fn record_borrowed_alias_string_read_latest_fresh(
    item: &dyn nyash_rust::box_trait::NyashBox,
    indexof: bool,
) {
    if !observe::enabled() {
        return;
    }
    let Some((source_handle, _)) = item.borrowed_handle_source_fast() else {
        return;
    };
    if !observe::len_route_matches_latest_fresh_handle(source_handle) {
        return;
    }
    if indexof {
        observe::record_borrowed_alias_array_indexof_by_index_latest_fresh();
    } else {
        observe::record_borrowed_alias_array_len_by_index_latest_fresh();
    }
}

#[derive(Clone, Copy)]
enum StoreArrayStrPlanSourceKind {
    StringLike,
    OtherObject,
    Missing,
}

#[derive(Clone, Copy)]
enum StoreArrayStrPlanSlotKind {
    BorrowedAlias,
    Other,
}

#[derive(Clone, Copy)]
enum StoreArrayStrPlanAction {
    RetargetAlias,
    StoreFromSource,
    NeedStableObject,
}

#[derive(Clone, Copy)]
struct StoreArrayStrPlan {
    source_kind: StoreArrayStrPlanSourceKind,
    slot_kind: StoreArrayStrPlanSlotKind,
    action: StoreArrayStrPlanAction,
    source_is_string: bool,
    latest_fresh_source: bool,
}

impl StoreArrayStrPlan {
    #[inline(always)]
    fn from_slot(
        items: &[Box<dyn nyash_rust::box_trait::NyashBox>],
        idx: usize,
        value_h: i64,
        source_kind_contract: StringHandleSourceKind,
    ) -> Self {
        let source_is_string = matches!(source_kind_contract, StringHandleSourceKind::StringLike);
        let source_kind = match source_kind_contract {
            StringHandleSourceKind::StringLike => StoreArrayStrPlanSourceKind::StringLike,
            StringHandleSourceKind::OtherObject => StoreArrayStrPlanSourceKind::OtherObject,
            StringHandleSourceKind::Missing => StoreArrayStrPlanSourceKind::Missing,
        };
        let slot_kind = if idx < items.len()
            && items[idx]
                .as_any()
                .downcast_ref::<BorrowedHandleBox>()
                .is_some()
        {
            StoreArrayStrPlanSlotKind::BorrowedAlias
        } else {
            StoreArrayStrPlanSlotKind::Other
        };
        let action = if source_is_string {
            if matches!(slot_kind, StoreArrayStrPlanSlotKind::BorrowedAlias) && idx < items.len() {
                StoreArrayStrPlanAction::RetargetAlias
            } else {
                StoreArrayStrPlanAction::StoreFromSource
            }
        } else {
            StoreArrayStrPlanAction::NeedStableObject
        };
        Self {
            source_kind,
            slot_kind,
            action,
            source_is_string,
            latest_fresh_source: observe::len_route_matches_latest_fresh_handle(value_h),
        }
    }

    #[inline(always)]
    fn record(self) {
        record_store_array_str_plan(self.source_kind, self.slot_kind, self.action);
    }

    #[inline(always)]
    fn can_retarget_alias(self) -> bool {
        self.source_is_string
            && matches!(self.slot_kind, StoreArrayStrPlanSlotKind::BorrowedAlias)
            && matches!(self.action, StoreArrayStrPlanAction::RetargetAlias)
    }
}

#[inline(always)]
fn record_store_array_str_plan(
    source_kind: StoreArrayStrPlanSourceKind,
    slot_kind: StoreArrayStrPlanSlotKind,
    action: StoreArrayStrPlanAction,
) {
    if !observe::enabled() {
        return;
    }
    match source_kind {
        StoreArrayStrPlanSourceKind::StringLike => {
            observe::record_store_array_str_plan_source_kind_string_like();
        }
        StoreArrayStrPlanSourceKind::OtherObject => {
            observe::record_store_array_str_plan_source_kind_other_object();
        }
        StoreArrayStrPlanSourceKind::Missing => {
            observe::record_store_array_str_plan_source_kind_missing();
        }
    }
    match slot_kind {
        StoreArrayStrPlanSlotKind::BorrowedAlias => {
            observe::record_store_array_str_plan_slot_kind_borrowed_alias();
        }
        StoreArrayStrPlanSlotKind::Other => {
            observe::record_store_array_str_plan_slot_kind_other();
        }
    }
    match action {
        StoreArrayStrPlanAction::RetargetAlias => {
            observe::record_store_array_str_plan_action_retarget_alias();
        }
        StoreArrayStrPlanAction::StoreFromSource => {
            observe::record_store_array_str_plan_action_store_from_source();
        }
        StoreArrayStrPlanAction::NeedStableObject => {
            observe::record_store_array_str_plan_action_need_stable_object();
        }
    }
}

pub(super) fn array_string_len_by_index(handle: i64, idx: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            record_borrowed_alias_string_read_latest_fresh(item.as_ref(), false);
            item.as_str_fast().map(|s| s.len() as i64).unwrap_or(0)
        })
    })
    .unwrap_or(0)
}

#[inline(always)]
fn string_indexof_fast_str(hay: &str, needle: &str) -> i64 {
    if needle.is_empty() {
        return 0;
    }
    let hay_b = hay.as_bytes();
    let nee_b = needle.as_bytes();
    match nee_b.len() {
        1 => memchr(nee_b[0], hay_b).map(|pos| pos as i64).unwrap_or(-1),
        2 | 3 | 4 => string_indexof_fast_str_small(hay_b, nee_b),
        _ => memmem::find(hay_b, nee_b)
            .map(|pos| pos as i64)
            .unwrap_or(-1),
    }
}

#[inline(always)]
fn string_indexof_fast_str_small(hay_b: &[u8], nee_b: &[u8]) -> i64 {
    let first = nee_b[0];
    let needle_len = nee_b.len();
    let mut offset = 0usize;
    let mut search = hay_b;

    while let Some(pos) = memchr(first, search) {
        let idx = offset + pos;
        let end = idx + needle_len;
        if end <= hay_b.len() && &hay_b[idx..end] == nee_b {
            return idx as i64;
        }
        offset = idx + 1;
        if offset >= hay_b.len() {
            return -1;
        }
        search = &hay_b[offset..];
    }

    -1
}

#[inline(always)]
fn with_cached_needle_str<R>(needle_h: i64, f: impl FnOnce(&str) -> R) -> R {
    let drop_epoch = handles::drop_epoch();
    ARRAY_STRING_INDEXOF_NEEDLE_CACHE.with(|slot| {
        if let Some(cached) = slot.borrow().as_ref() {
            if cached.handle == needle_h && cached.drop_epoch == drop_epoch {
                return f(cached.value.as_str());
            }
        }
        let value = resolve_string_span_from_handle(needle_h)
            .map(|span| span.as_str().to_owned())
            .unwrap_or_default();
        *slot.borrow_mut() = Some(CachedNeedle {
            handle: needle_h,
            drop_epoch,
            value,
        });
        let borrowed = slot.borrow();
        let cached = borrowed
            .as_ref()
            .expect("[array/string_indexof] needle cache just initialized");
        f(cached.value.as_str())
    })
}

#[inline(always)]
pub(super) fn array_string_indexof_by_index(handle: i64, idx: i64, needle_h: i64) -> i64 {
    with_cached_needle_str(needle_h, |needle| {
        if !valid_handle_idx(handle, idx) {
            return if needle.is_empty() { 0 } else { -1 };
        }
        if needle.is_empty() {
            return 0;
        }
        super::array_handle_cache::with_array_box(handle, |arr| {
            let idx = idx as usize;
            arr.with_items_read(|items| {
                items
                    .get(idx)
                    .and_then(|item| {
                        record_borrowed_alias_string_read_latest_fresh(item.as_ref(), true);
                        item.as_str_fast()
                    })
                    .map(|hay| string_indexof_fast_str(hay, needle))
                    .unwrap_or(-1)
            })
        })
        .unwrap_or(-1)
    })
}

#[inline(always)]
fn execute_store_array_str_slot(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> i64 {
    if idx > items.len() {
        return 0;
    }
    let mut source = source;
    if observe::enabled() {
        if idx < items.len() {
            observe::record_store_array_str_existing_slot();
        } else {
            observe::record_store_array_str_append_slot();
        }
        if matches!(
            source,
            ArrayStoreStrSource::StringLike(_) | ArrayStoreStrSource::OtherObject
        ) {
            observe::record_store_array_str_reason_source_kind_via_object();
        }
        source.record_observe_source_kind();
    }
    let plan = StoreArrayStrPlan::from_slot(items.as_slice(), idx, value_h, source.source_kind());
    plan.record();
    if idx < items.len() {
        if plan.can_retarget_alias() {
            if let ArrayStoreStrSource::StringLike(source_text) = source {
                match try_retarget_borrowed_string_slot_take_verified_text_source(
                    &mut items[idx],
                    value_h,
                    source_text,
                    drop_epoch,
                ) {
                    Ok(()) => {
                        observe::record_store_array_str_retarget_hit();
                        if plan.latest_fresh_source {
                            observe::record_store_array_str_latest_fresh_retarget_hit();
                        }
                        return 1;
                    }
                    Err(source_keep) => {
                        source = ArrayStoreStrSource::StringLike(source_keep);
                    }
                }
            }
        }
    }
    if plan.source_is_string {
        observe::record_store_array_str_source_store();
        if plan.latest_fresh_source {
            observe::record_store_array_str_latest_fresh_source_store();
        }
    } else {
        observe::record_store_array_str_non_string_source();
    }
    let value = match source {
        ArrayStoreStrSource::StringLike(source_text) => {
            store_string_box_from_verified_text_source(value_h, source_text, drop_epoch)
        }
        ArrayStoreStrSource::OtherObject => {
            maybe_store_non_string_box_from_verified_source(value_h, drop_epoch)
        }
        ArrayStoreStrSource::Missing => {
            maybe_store_non_string_box_from_verified_source(value_h, drop_epoch)
        }
    };
    if idx < items.len() {
        items[idx] = value;
    } else {
        items.push(value);
    }
    1
}

#[inline(always)]
fn execute_store_array_str_contract(handle: i64, idx: i64, value_h: i64) -> i64 {
    if !valid_handle_idx(handle, idx) || value_h <= 0 {
        return 0;
    }
    let drop_epoch = handles::drop_epoch();
    observe::record_store_array_str_enter();
    if observe::enabled() {
        let kind = match cache_probe_kind(handle, drop_epoch) {
            HandleCacheProbeKind::Hit => ObserveCacheProbeKind::Hit,
            HandleCacheProbeKind::MissHandle => ObserveCacheProbeKind::MissHandle,
            HandleCacheProbeKind::MissDropEpoch => ObserveCacheProbeKind::MissDropEpoch,
        };
        observe::record_store_array_str_cache_probe(kind);
    }
    super::array_handle_cache::with_array_box_at_epoch(handle, drop_epoch, |arr| {
        let idx = idx as usize;
        let source = with_array_store_str_source(value_h, |source| source);
        arr.with_items_write(|items| {
            execute_store_array_str_slot(items, idx, value_h, source, drop_epoch)
        })
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn array_string_store_handle_at(handle: i64, idx: i64, value_h: i64) -> i64 {
    // phase-150x: keep array-string store semantics owned above this layer and
    // treat the Rust path as the executor for the canonical `store.array.str`
    // reading only.
    execute_store_array_str_contract(handle, idx, value_h)
}
