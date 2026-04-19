use super::handle_cache::with_map_box;
use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::value_codec::{
    box_to_handle_materializing_borrowed_string, runtime_i64_from_box_ref_caller,
    BorrowedAliasEncodeCaller,
};
use super::value_demand::{MAP_VALUE_LOAD_ENCODE_WITH_CALLER, MAP_VALUE_LOAD_MATERIALIZE};

#[inline(always)]
pub(super) fn map_slot_load_i64(handle: i64, key_i64: i64) -> i64 {
    let key_str = map_key_string_from_i64(key_i64);
    map_slot_load_str(handle, &key_str)
}

#[inline(always)]
pub(super) fn map_slot_load_any(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    map_slot_load_str(handle, &key_str)
}

#[inline(always)]
pub(super) fn map_slot_load_str(handle: i64, key_str: &str) -> i64 {
    let _value_demand = MAP_VALUE_LOAD_MATERIALIZE;
    with_map_box(handle, |map| {
        let value = map.get_opt_key_str(key_str)?;
        Some(box_to_handle_materializing_borrowed_string(value))
    })
    .flatten()
    .unwrap_or(0)
}

#[inline(always)]
pub(super) fn map_slot_load_str_with_caller(
    handle: i64,
    key_str: &str,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    let _value_demand = MAP_VALUE_LOAD_ENCODE_WITH_CALLER;
    with_map_box(handle, |map| {
        map.get_opt_key_str(key_str)
            .as_ref()
            .map(|value| runtime_i64_from_box_ref_caller(value.as_ref(), caller))
            .unwrap_or(0)
    })
    .unwrap_or(0)
}
