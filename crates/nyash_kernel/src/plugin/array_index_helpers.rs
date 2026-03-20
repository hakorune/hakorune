use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::value_codec::any_arg_to_index;

pub(super) fn array_get_by_index(handle: i64, idx: i64) -> i64 {
    array_slot_load_encoded_i64(handle, idx)
}

pub(super) fn array_has_by_index(handle: i64, idx: i64) -> i64 {
    array_slot_has_index(handle, idx)
}

pub(super) fn decode_index_key(key_any: i64) -> Option<i64> {
    any_arg_to_index(key_any)
}
