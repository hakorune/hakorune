use super::map_key_codec::map_runtime_data_key_string_from_any;
use super::map_probe::map_probe_contains_str;
use super::map_slot_load::map_slot_load_str_with_caller;
use super::map_slot_store::map_slot_store_str_any;
use super::value_codec::BorrowedAliasEncodeCaller;

#[inline(never)]
pub(super) fn map_runtime_data_get_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    map_slot_load_str_with_caller(
        handle,
        &key_str,
        BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey,
    )
}

#[inline(never)]
pub(super) fn map_runtime_data_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    map_slot_store_str_any(handle, key_str, val_any)
}

#[inline(never)]
pub(super) fn map_runtime_data_has_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    map_probe_contains_str(handle, &key_str)
}
