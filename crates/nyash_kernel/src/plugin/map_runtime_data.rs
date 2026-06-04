use super::map_key_codec::{map_key_string_from_any, map_key_string_from_i64};
use super::map_probe::map_probe_contains_str;
use super::map_slot_load::map_slot_load_str_with;
use super::map_slot_store::map_slot_store_str_any;
use super::value_codec::{runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller};

#[inline(never)]
pub(super) fn map_runtime_data_get_any_key(handle: i64, key_any: i64) -> i64 {
    let _demand = super::value_demand::MAP_RUNTIME_DATA_KEY_DECODE_ANY;
    let key_str = if key_any <= 0 {
        map_key_string_from_i64(key_any)
    } else {
        map_key_string_from_any(key_any)
    };
    let _value_demand = super::value_demand::MAP_VALUE_LOAD_ENCODE_WITH_CALLER;
    map_slot_load_str_with(handle, &key_str, |value| {
        runtime_i64_from_box_ref_caller(
            value.as_ref(),
            BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey,
        )
    })
}

#[inline(never)]
pub(super) fn map_runtime_data_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let _demand = super::value_demand::MAP_RUNTIME_DATA_KEY_DECODE_ANY;
    let key_str = if key_any <= 0 {
        map_key_string_from_i64(key_any)
    } else {
        map_key_string_from_any(key_any)
    };
    map_slot_store_str_any(handle, key_str, val_any)
}

#[inline(never)]
pub(super) fn map_runtime_data_has_any_key(handle: i64, key_any: i64) -> i64 {
    let _demand = super::value_demand::MAP_RUNTIME_DATA_KEY_DECODE_ANY;
    let key_str = if key_any <= 0 {
        map_key_string_from_i64(key_any)
    } else {
        map_key_string_from_any(key_any)
    };
    map_probe_contains_str(handle, &key_str)
}
