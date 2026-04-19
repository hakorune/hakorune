use super::super::array_guard::valid_handle_idx;
use super::super::handle_cache::{cache_probe_kind, CacheProbeKind as HandleCacheProbeKind};
use super::super::value_codec::{
    maybe_store_non_string_box_from_verified_source, store_string_box_from_kernel_text_slot,
    store_string_box_from_verified_text_source,
    store_string_into_existing_string_box_from_kernel_text_slot,
    store_string_keep_from_kernel_text_slot,
    try_retarget_borrowed_string_slot_take_unpublished_keep,
    try_retarget_borrowed_string_slot_take_verified_text_source, with_array_store_str_source,
    ArrayStoreStrSource, BorrowedHandleBox, KernelTextSlot, StringHandleSourceKind,
    StringLikeProof,
};
use super::array_string_slot_helpers::{array_text_owned_cell_demand, StoreArrayStrPlan};
use crate::observe::{self, CacheProbeKind as ObserveCacheProbeKind};
use nyash_rust::runtime::host_handles as handles;

#[inline(always)]
fn execute_store_array_str_slot(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    source_kind: StringHandleSourceKind,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> i64 {
    if idx > items.len() {
        return 0;
    }
    match execute_store_array_str_plan_and_retarget_boundary(
        items,
        idx,
        value_h,
        source_kind,
        source,
        drop_epoch,
    ) {
        StoreArrayStrBoundaryStep::Retargeted => 1,
        StoreArrayStrBoundaryStep::Continue { plan, source } => {
            execute_store_array_str_store_from_source_boundary(
                items, idx, value_h, plan, source, drop_epoch,
            )
        }
    }
}

enum StoreArrayStrBoundaryStep {
    Retargeted,
    Continue {
        plan: StoreArrayStrPlan,
        source: ArrayStoreStrSource,
    },
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_plan_and_retarget_boundary(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    source_kind: StringHandleSourceKind,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> StoreArrayStrBoundaryStep {
    let mut source = source;
    if observe::enabled() {
        if idx < items.len() {
            observe::record_store_array_str_existing_slot();
        } else {
            observe::record_store_array_str_append_slot();
        }
        if matches!(
            source_kind,
            StringHandleSourceKind::StringLike | StringHandleSourceKind::OtherObject
        ) {
            observe::record_store_array_str_reason_source_kind_via_object();
        }
        match &source {
            ArrayStoreStrSource::StringLike(source_text) => match source_text.proof() {
                StringLikeProof::StringBox => {
                    observe::record_store_array_str_source_string_box();
                }
                StringLikeProof::StringView => {
                    observe::record_store_array_str_source_string_view();
                }
            },
            ArrayStoreStrSource::OtherObject => {}
            ArrayStoreStrSource::Missing => observe::record_store_array_str_source_missing(),
        }
    }
    let plan = StoreArrayStrPlan::from_slot(items.as_slice(), idx, value_h, source_kind);
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
                        return StoreArrayStrBoundaryStep::Retargeted;
                    }
                    Err(source_keep) => {
                        source = ArrayStoreStrSource::StringLike(source_keep);
                    }
                }
            }
        }
    }
    StoreArrayStrBoundaryStep::Continue { plan, source }
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_store_from_source_boundary(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    plan: StoreArrayStrPlan,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> i64 {
    if plan.source_is_string {
        observe::record_store_array_str_source_store();
        if plan.latest_fresh_source {
            observe::record_store_array_str_latest_fresh_source_store();
        }
    } else {
        observe::record_store_array_str_non_string_source();
    }
    let value = store_array_str_value_from_source(value_h, source, drop_epoch);
    if idx < items.len() {
        items[idx] = value;
    } else {
        items.push(value);
    }
    1
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn store_array_str_value_from_source(
    value_h: i64,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> Box<dyn nyash_rust::box_trait::NyashBox> {
    match source {
        ArrayStoreStrSource::StringLike(source_text) => {
            store_string_box_from_verified_text_source(value_h, source_text, drop_epoch)
        }
        ArrayStoreStrSource::OtherObject => {
            maybe_store_non_string_box_from_verified_source(value_h, drop_epoch)
        }
        ArrayStoreStrSource::Missing => {
            maybe_store_non_string_box_from_verified_source(value_h, drop_epoch)
        }
    }
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn capture_store_array_str_source(value_h: i64) -> (StringHandleSourceKind, ArrayStoreStrSource) {
    with_array_store_str_source(value_h, |source_kind, source| (source_kind, source))
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_slot_boundary(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    value_h: i64,
    source_kind: StringHandleSourceKind,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> i64 {
    execute_store_array_str_slot(items, idx, value_h, source_kind, source, drop_epoch)
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_kernel_text_slot_boundary(
    items: &mut Vec<Box<dyn nyash_rust::box_trait::NyashBox>>,
    idx: usize,
    slot: &mut KernelTextSlot,
) -> i64 {
    if idx > items.len() {
        return 0;
    }
    if idx < items.len()
        && items[idx]
            .as_any()
            .downcast_ref::<BorrowedHandleBox>()
            .is_some()
    {
        let Some(source_keep) = store_string_keep_from_kernel_text_slot(slot) else {
            return 0;
        };
        if try_retarget_borrowed_string_slot_take_unpublished_keep(
            &mut items[idx],
            source_keep,
            handles::drop_epoch(),
        )
        .is_ok()
        {
            if observe::enabled() {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            observe::record_store_array_str_retarget_hit();
            return 1;
        }
        return 0;
    }
    if idx < items.len() {
        if let Some(value) = items[idx]
            .as_any_mut()
            .downcast_mut::<nyash_rust::box_trait::StringBox>()
        {
            if !store_string_into_existing_string_box_from_kernel_text_slot(slot, value) {
                return 0;
            }
            if observe::enabled() {
                observe::record_store_array_str_existing_slot();
                observe::record_store_array_str_source_store();
            }
            return 1;
        }
    }
    let Some(value) = store_string_box_from_kernel_text_slot(slot) else {
        return 0;
    };
    if observe::enabled() {
        if idx < items.len() {
            observe::record_store_array_str_existing_slot();
        } else {
            observe::record_store_array_str_append_slot();
        }
        observe::record_store_array_str_source_store();
    }
    if idx < items.len() {
        items[idx] = value;
    } else {
        items.push(value);
    }
    1
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
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
    super::super::array_handle_cache::with_array_box_at_epoch(handle, drop_epoch, |arr| {
        let idx = idx as usize;
        let (source_kind, source) = capture_store_array_str_source(value_h);
        arr.with_items_write(|items| {
            execute_store_array_str_slot_boundary(
                items,
                idx,
                value_h,
                source_kind,
                source,
                drop_epoch,
            )
        })
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_store_handle_at(handle: i64, idx: i64, value_h: i64) -> i64 {
    // phase-150x: keep array-string store semantics owned above this layer and
    // treat the Rust path as the executor for the canonical `store.array.str`
    // reading only.
    execute_store_array_str_contract(handle, idx, value_h)
}

#[inline(always)]
pub(in super::super) fn array_string_store_kernel_text_slot_at(
    handle: i64,
    idx: i64,
    slot: &mut KernelTextSlot,
) -> i64 {
    let _demand = array_text_owned_cell_demand();
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    observe::record_store_array_str_enter();
    super::super::array_handle_cache::with_array_box(handle, |arr| {
        let idx = idx as usize;
        arr.with_items_write(|items| {
            execute_store_array_str_kernel_text_slot_boundary(items, idx, slot)
        })
    })
    .unwrap_or(0)
}
