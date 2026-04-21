#[path = "array_string_slot_helpers.rs"]
mod array_string_slot_helpers;
#[path = "array_string_slot_indexof.rs"]
mod array_string_slot_indexof;
#[path = "array_string_slot_store.rs"]
mod array_string_slot_store;
#[path = "array_string_slot_write.rs"]
mod array_string_slot_write;

pub(super) use self::array_string_slot_indexof::{
    array_string_indexof_by_index, array_string_indexof_by_index_const_utf8,
    array_string_len_by_index,
};
pub(super) use self::array_string_slot_store::{
    array_string_store_handle_at, array_string_store_kernel_text_slot_at,
};
pub(super) use self::array_string_slot_write::{
    array_string_concat_const_suffix_by_index_into_slot,
    array_string_concat_const_suffix_by_index_store_same_slot,
    array_string_concat_const_suffix_by_index_store_same_slot_len,
    array_string_insert_const_mid_by_index_into_slot,
    array_string_insert_const_mid_by_index_store_same_slot,
    array_string_insert_const_mid_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_by_index_store_same_slot,
    array_string_insert_const_mid_subrange_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_len_region_store_len,
};
