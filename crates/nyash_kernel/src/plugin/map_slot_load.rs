use super::handle_cache::with_map_box;
use super::value_codec::box_to_handle;
use super::value_demand::MAP_VALUE_LOAD_MATERIALIZE;
use nyash_rust::box_trait::NyashBox;

#[inline(always)]
pub(super) fn map_slot_load_str(handle: i64, key_str: &str) -> i64 {
    let _value_demand = MAP_VALUE_LOAD_MATERIALIZE;
    map_slot_load_str_with(handle, key_str, |value| {
        if value.borrowed_handle_source_fast().is_some() {
            return box_to_handle(Box::new(value.to_string_box()));
        }
        box_to_handle(value)
    })
}

#[inline(always)]
pub(super) fn map_slot_load_str_with(
    handle: i64,
    key_str: &str,
    f: impl FnOnce(Box<dyn NyashBox>) -> i64,
) -> i64 {
    with_map_box(handle, |map| {
        map.get_opt_key_str(key_str).map(f).unwrap_or(0)
    })
    .unwrap_or(0)
}
