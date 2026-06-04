use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};
use super::value_demand::{MAP_KEY_DECODE_ANY, MAP_KEY_DECODE_I64};

#[inline(always)]
pub(crate) fn map_key_string_from_i64(key_i64: i64) -> String {
    let _demand = MAP_KEY_DECODE_I64;
    key_i64.to_string()
}

#[inline(never)]
pub(crate) fn map_key_string_from_any(key_any: i64) -> String {
    let _demand = MAP_KEY_DECODE_ANY;
    any_arg_to_box_with_profile(key_any, CodecProfile::MapKeyBorrowString)
        .to_string_box()
        .value
}
