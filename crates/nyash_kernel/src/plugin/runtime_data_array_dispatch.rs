use super::array_slot_append::array_slot_append_any;
use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::array_slot_store::array_slot_store_any;
use super::value_codec::any_arg_to_index;

#[inline(always)]
fn resolve_array_index_key(key_any: i64) -> Option<i64> {
    any_arg_to_index(key_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_get_hh(handle: i64, key_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(key_any) else {
        return 0;
    };
    array_slot_load_encoded_i64(handle, idx)
}

#[inline(always)]
pub(super) fn runtime_data_array_set_hhh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(key_any) else {
        return 0;
    };
    array_slot_store_any(handle, idx, val_any)
}

#[inline(always)]
pub(super) fn runtime_data_array_has_hh(handle: i64, key_any: i64) -> i64 {
    let Some(idx) = resolve_array_index_key(key_any) else {
        return 0;
    };
    array_slot_has_index(handle, idx)
}

#[inline(always)]
pub(super) fn runtime_data_array_push_hh(handle: i64, val_any: i64) -> i64 {
    array_slot_append_any(handle, val_any)
}
