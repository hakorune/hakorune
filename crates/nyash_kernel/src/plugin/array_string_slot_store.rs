use super::super::array_guard::valid_handle_idx;
use super::super::handle_cache::{cache_probe_kind, CacheProbeKind as HandleCacheProbeKind};
use super::super::value_codec::{
    maybe_store_non_string_box_from_verified_source, with_array_store_str_source,
    ArrayStoreStrSource, KernelTextSlot, StringHandleSourceKind, StringLikeProof,
};
use super::super::value_lane::{
    array_text_cell_store_lane_plan, array_text_degrade_generic_lane_plan, ValueLaneAction,
    ValueLanePlan,
};
use super::array_string_slot_helpers::{
    array_text_owned_cell_demand, record_store_array_str_plan, StoreArrayStrPlanAction,
    StoreArrayStrPlanSlotKind, StoreArrayStrPlanSourceKind,
};
use crate::observe::{self, CacheProbeKind as ObserveCacheProbeKind};
use nyash_rust::{boxes::array::ArrayBox, runtime::host_handles as handles};

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn capture_store_array_str_source(value_h: i64) -> (StringHandleSourceKind, ArrayStoreStrSource) {
    with_array_store_str_source(value_h, |source_kind, source| (source_kind, source))
}

#[inline(always)]
fn store_array_str_plan_source_kind(
    source_kind: StringHandleSourceKind,
) -> StoreArrayStrPlanSourceKind {
    match source_kind {
        StringHandleSourceKind::StringLike => StoreArrayStrPlanSourceKind::StringLike,
        StringHandleSourceKind::OtherObject => StoreArrayStrPlanSourceKind::OtherObject,
        StringHandleSourceKind::Missing => StoreArrayStrPlanSourceKind::Missing,
    }
}

#[inline(always)]
fn store_array_str_plan_action(source_kind: StringHandleSourceKind) -> StoreArrayStrPlanAction {
    if matches!(source_kind, StringHandleSourceKind::StringLike) {
        StoreArrayStrPlanAction::StoreFromSource
    } else {
        StoreArrayStrPlanAction::NeedStableObject
    }
}

#[inline(always)]
fn array_store_str_lane_plan(source_kind: StringHandleSourceKind) -> ValueLanePlan {
    if matches!(source_kind, StringHandleSourceKind::StringLike) {
        array_text_cell_store_lane_plan()
    } else {
        array_text_degrade_generic_lane_plan()
    }
}

#[inline(always)]
fn record_store_array_str_source_shape(source: &ArrayStoreStrSource) {
    if !observe::enabled() {
        return;
    }
    match source {
        ArrayStoreStrSource::StringLike(source_text) => match source_text.proof() {
            StringLikeProof::StringBox => observe::record_store_array_str_source_string_box(),
            StringLikeProof::StringView => observe::record_store_array_str_source_string_view(),
        },
        ArrayStoreStrSource::OtherObject => {}
        ArrayStoreStrSource::Missing => observe::record_store_array_str_source_missing(),
    }
}

#[inline(always)]
fn record_store_array_str_contract(
    idx: usize,
    len: usize,
    value_h: i64,
    source_kind: StringHandleSourceKind,
    source: &ArrayStoreStrSource,
) {
    if observe::enabled() {
        if idx < len {
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
    }
    record_store_array_str_source_shape(source);
    record_store_array_str_plan(
        store_array_str_plan_source_kind(source_kind),
        StoreArrayStrPlanSlotKind::Other,
        store_array_str_plan_action(source_kind),
    );
    if matches!(source_kind, StringHandleSourceKind::StringLike) {
        observe::record_store_array_str_source_store();
        if observe::len_route_matches_latest_fresh_handle(value_h) {
            observe::record_store_array_str_latest_fresh_source_store();
        }
    } else {
        observe::record_store_array_str_non_string_source();
    }
}

#[inline(always)]
fn owned_text_from_array_store_source(source: ArrayStoreStrSource) -> Option<String> {
    match source {
        ArrayStoreStrSource::StringLike(source_text) => Some(
            source_text
                .with_text(|text| text.as_str().to_owned())
                .unwrap_or_else(|| source_text.copy_owned_text_cold()),
        ),
        ArrayStoreStrSource::OtherObject | ArrayStoreStrSource::Missing => None,
    }
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_contract_on_array(
    arr: &ArrayBox,
    idx: i64,
    value_h: i64,
    source_kind: StringHandleSourceKind,
    source: ArrayStoreStrSource,
    drop_epoch: u64,
) -> i64 {
    let idx_usize = idx as usize;
    let len = arr.len();
    if idx_usize > len {
        return 0;
    }
    record_store_array_str_contract(idx_usize, len, value_h, source_kind, &source);
    let lane_plan = array_store_str_lane_plan(source_kind);
    match lane_plan.action {
        ValueLaneAction::TextCellResidence => {
            let Some(value) = owned_text_from_array_store_source(source) else {
                return 0;
            };
            if arr.slot_store_text_raw(idx, value) {
                1
            } else {
                0
            }
        }
        ValueLaneAction::GenericBoxResidence => {
            let value = maybe_store_non_string_box_from_verified_source(value_h, drop_epoch);
            if arr.slot_store_box_raw(idx, value) {
                1
            } else {
                0
            }
        }
    }
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
    let (source_kind, source) = capture_store_array_str_source(value_h);
    super::super::array_handle_cache::with_array_box_at_epoch(handle, drop_epoch, |arr| {
        execute_store_array_str_contract_on_array(
            arr,
            idx,
            value_h,
            source_kind,
            source,
            drop_epoch,
        )
    })
    .unwrap_or(0)
}

#[inline(always)]
pub(in super::super) fn array_string_store_handle_at(handle: i64, idx: i64, value_h: i64) -> i64 {
    // The MIR-level `store.array.str` contract chooses the text source and
    // publication boundary. Runtime only stores text residence or degrades
    // mixed/generic arrays back to Boxed.
    execute_store_array_str_contract(handle, idx, value_h)
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn execute_store_array_str_kernel_text_slot_boundary(
    arr: &ArrayBox,
    idx: i64,
    slot: &mut KernelTextSlot,
) -> i64 {
    let idx_usize = idx as usize;
    let len = arr.len();
    if idx_usize > len {
        return 0;
    }
    let Some(value) = slot
        .take_materialized_owned_bytes()
        .map(|bytes| bytes.into_string())
    else {
        return 0;
    };
    if observe::enabled() {
        if idx_usize < len {
            observe::record_store_array_str_existing_slot();
        } else {
            observe::record_store_array_str_append_slot();
        }
        observe::record_store_array_str_source_store();
    }
    if arr.slot_store_text_raw(idx, value) {
        1
    } else {
        0
    }
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
        let lane_plan = array_text_cell_store_lane_plan();
        debug_assert_eq!(lane_plan.action, ValueLaneAction::TextCellResidence);
        execute_store_array_str_kernel_text_slot_boundary(arr, idx, slot)
    })
    .unwrap_or(0)
}
