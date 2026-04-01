use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};

#[inline(always)]
pub(crate) fn map_key_string_from_i64(key_i64: i64) -> String {
    key_i64.to_string()
}

#[inline(never)]
pub(crate) fn map_key_string_from_any(key_any: i64) -> String {
    any_arg_to_box_with_profile(key_any, CodecProfile::ArrayFastBorrowString)
        .to_string_box()
        .value
}
