use super::array_compat::nyash_array_length_h;
use super::array_slot_append::array_slot_append_any;
use super::array_slot_capacity::{array_slot_cap_i64, array_slot_grow_i64, array_slot_reserve_i64};
use super::array_slot_load::array_slot_load_encoded_i64;
use super::array_slot_store::{array_slot_rmw_add1_i64, array_slot_store_any, array_slot_store_i64};
use super::array_string_slot::{array_string_indexof_by_index, array_string_len_by_index};

// Mainline substrate aliases used by `.hako` collection owners and adapter defaults.
#[export_name = "nyash.array.slot_len_h"]
pub extern "C" fn nyash_array_slot_len_h_alias(handle: i64) -> i64 {
    nyash_array_length_h(handle)
}

#[export_name = "nyash.array.slot_cap_h"]
pub extern "C" fn nyash_array_slot_cap_h_alias(handle: i64) -> i64 {
    array_slot_cap_i64(handle)
}

#[export_name = "nyash.array.slot_load_hi"]
pub extern "C" fn nyash_array_slot_load_hi_alias(handle: i64, idx: i64) -> i64 {
    array_slot_load_encoded_i64(handle, idx)
}

#[export_name = "nyash.array.slot_store_hii"]
pub extern "C" fn nyash_array_slot_store_hii_alias(handle: i64, idx: i64, value_i64: i64) -> i64 {
    array_slot_store_i64(handle, idx, value_i64)
}

#[export_name = "nyash.array.slot_store_hih"]
pub extern "C" fn nyash_array_slot_store_hih_alias(handle: i64, idx: i64, val_any: i64) -> i64 {
    array_slot_store_any(handle, idx, val_any)
}

#[export_name = "nyash.array.rmw_add1_hi"]
pub extern "C" fn nyash_array_rmw_add1_hi_alias(handle: i64, idx: i64) -> i64 {
    array_slot_rmw_add1_i64(handle, idx)
}

#[export_name = "nyash.array.string_len_hi"]
pub extern "C" fn nyash_array_string_len_hi_alias(handle: i64, idx: i64) -> i64 {
    array_string_len_by_index(handle, idx)
}

#[export_name = "nyash.array.string_indexof_hih"]
pub extern "C" fn nyash_array_string_indexof_hih_alias(
    handle: i64,
    idx: i64,
    needle_h: i64,
) -> i64 {
    array_string_indexof_by_index(handle, idx, needle_h)
}

#[export_name = "nyash.array.slot_append_hh"]
pub extern "C" fn nyash_array_slot_append_hh_alias(handle: i64, val_any: i64) -> i64 {
    array_slot_append_any(handle, val_any)
}

#[export_name = "nyash.array.slot_reserve_hi"]
pub extern "C" fn nyash_array_slot_reserve_hi_alias(handle: i64, additional: i64) -> i64 {
    array_slot_reserve_i64(handle, additional)
}

#[export_name = "nyash.array.slot_grow_hi"]
pub extern "C" fn nyash_array_slot_grow_hi_alias(handle: i64, target_capacity: i64) -> i64 {
    array_slot_grow_i64(handle, target_capacity)
}
