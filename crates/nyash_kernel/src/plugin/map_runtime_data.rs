use super::handle_cache::with_map_box;
use super::map_key_codec::map_runtime_data_key_string_from_any;
use super::map_probe::map_probe_contains_str;
use super::value_codec::{
    any_arg_to_box_with_profile, runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller,
    CodecProfile,
};

#[inline(never)]
pub(super) fn map_runtime_data_get_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        map.get_opt_key_str(&key_str)
            .as_ref()
            .map(|value| {
                runtime_i64_from_box_ref_caller(
                    value.as_ref(),
                    BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey,
                )
            })
            .unwrap_or(0)
    })
    .unwrap_or(0)
}

#[inline(never)]
pub(super) fn map_runtime_data_set_any_key(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        let value_box = any_arg_to_box_with_profile(val_any, CodecProfile::MapValueBorrowString);
        map.insert_key_str(key_str, value_box);
        1
    })
    .unwrap_or(0)
}

#[inline(never)]
pub(super) fn map_runtime_data_has_any_key(handle: i64, key_any: i64) -> i64 {
    let key_str = map_runtime_data_key_string_from_any(key_any);
    map_probe_contains_str(handle, &key_str)
}
