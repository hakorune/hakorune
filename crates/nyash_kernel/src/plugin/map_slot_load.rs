use super::handle_cache::with_map_box;
use super::value_codec::{
    encode_runtime_value_carrier, RuntimeValueCarrierMode, RuntimeValueCarrierSite,
};
use super::value_demand::MAP_VALUE_LOAD_MATERIALIZE;
use nyash_rust::box_trait::NyashBox;

#[inline(always)]
pub(super) fn map_slot_load_str(handle: i64, key_str: &str) -> i64 {
    let _value_demand = MAP_VALUE_LOAD_MATERIALIZE;
    map_slot_load_str_with(handle, key_str, |value| {
        encode_runtime_value_carrier(
            value.as_ref(),
            RuntimeValueCarrierMode::MixedI64OrHandle,
            RuntimeValueCarrierSite::MapSlotLoad,
        )
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

#[inline(always)]
pub(super) fn map_scalar_load_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        map.get_scalar_i64_key_i64(key_i64).unwrap_or(0)
    })
    .unwrap_or(0)
}
