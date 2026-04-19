use super::super::value_codec::{BorrowedHandleBox, StringHandleSourceKind};
use super::super::value_demand::{DemandSet, ARRAY_TEXT_OWNED_CELL, ARRAY_TEXT_READ_REF};
use crate::observe;
use memchr::{memchr, memmem};
use std::cell::RefCell;

pub(super) struct CachedNeedle {
    pub(super) handle: i64,
    pub(super) drop_epoch: u64,
    pub(super) value: String,
}

thread_local! {
    pub(super) static ARRAY_STRING_INDEXOF_NEEDLE_CACHE: RefCell<Option<CachedNeedle>> = const {
        RefCell::new(None)
    };
}

#[inline(always)]
pub(super) fn array_text_read_ref_demand() -> DemandSet {
    ARRAY_TEXT_READ_REF
}

#[inline(always)]
pub(super) fn array_text_owned_cell_demand() -> DemandSet {
    ARRAY_TEXT_OWNED_CELL
}

#[inline(always)]
pub(super) fn with_compiler_const_utf8_ptr_len<R>(
    ptr: *const i8,
    len: i64,
    f: impl FnOnce(&str) -> R,
) -> Option<R> {
    if ptr.is_null() || len < 0 {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    debug_assert!(std::str::from_utf8(bytes).is_ok());
    // Runtime-private direct lowering passes compiler-emitted UTF-8 string
    // constants with explicit length. The CStr public aliases keep validation.
    Some(f(unsafe { std::str::from_utf8_unchecked(bytes) }))
}

#[inline(always)]
pub(super) fn record_borrowed_alias_string_read_latest_fresh(
    observe_enabled: bool,
    item: &dyn nyash_rust::box_trait::NyashBox,
    indexof: bool,
) {
    if !observe_enabled {
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

#[inline(always)]
pub(super) fn string_indexof_fast_str(hay: &str, needle: &str) -> i64 {
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
pub(super) fn string_indexof_fast_str_small(hay_b: &[u8], nee_b: &[u8]) -> i64 {
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

#[derive(Clone, Copy)]
pub(super) enum StoreArrayStrPlanSourceKind {
    StringLike,
    OtherObject,
    Missing,
}

#[derive(Clone, Copy)]
pub(super) enum StoreArrayStrPlanSlotKind {
    BorrowedAlias,
    Other,
}

#[derive(Clone, Copy)]
pub(super) enum StoreArrayStrPlanAction {
    RetargetAlias,
    StoreFromSource,
    NeedStableObject,
}

#[derive(Clone, Copy)]
pub(super) struct StoreArrayStrPlan {
    pub(super) source_kind: StoreArrayStrPlanSourceKind,
    pub(super) slot_kind: StoreArrayStrPlanSlotKind,
    pub(super) action: StoreArrayStrPlanAction,
    pub(super) source_is_string: bool,
    pub(super) latest_fresh_source: bool,
}

impl StoreArrayStrPlan {
    #[inline(always)]
    pub(super) fn from_slot(
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
    pub(super) fn record(self) {
        record_store_array_str_plan(self.source_kind, self.slot_kind, self.action);
    }

    #[inline(always)]
    pub(super) fn can_retarget_alias(self) -> bool {
        self.source_is_string
            && matches!(self.slot_kind, StoreArrayStrPlanSlotKind::BorrowedAlias)
            && matches!(self.action, StoreArrayStrPlanAction::RetargetAlias)
    }
}

#[inline(always)]
pub(super) fn record_store_array_str_plan(
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
