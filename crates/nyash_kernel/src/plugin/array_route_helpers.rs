pub(super) use super::array_slot_store::array_slot_store_string_handle as array_set_by_index_string_handle_value;
use super::array_slot_store::{array_slot_store_any, array_slot_store_i64};

pub(super) fn array_set_by_index(handle: i64, idx: i64, val_any: i64) -> i64 {
    array_slot_store_any(handle, idx, val_any)
}

pub(super) fn array_set_by_index_i64_value(handle: i64, idx: i64, value_i64: i64) -> i64 {
    array_slot_store_i64(handle, idx, value_i64)
}
