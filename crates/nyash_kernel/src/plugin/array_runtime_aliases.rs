use super::array_compat::append_integer_raw;
use super::array_slot_append::array_slot_append_any;
use super::array_slot_capacity::{array_slot_cap_i64, array_slot_grow_i64, array_slot_reserve_i64};
use super::array_slot_load::{array_slot_has_index, array_slot_load_encoded_i64};
use super::array_slot_store::{
    array_slot_rmw_add1_i64, array_slot_store_any, array_slot_store_i64,
    array_slot_store_kernel_text_slot, array_slot_store_string_handle,
};
use super::array_string_slot::{
    array_kernel_slot_concat_his, array_kernel_slot_insert_hisi,
    array_string_concat_const_suffix_by_index_store_same_slot_len,
    array_string_concat_const_suffix_by_index_store_same_slot_text,
    array_string_indexof_by_index_const_utf8,
    array_string_insert_const_mid_by_index_store_same_slot_len,
    array_string_insert_const_mid_by_index_store_same_slot_text,
    array_string_insert_const_mid_subrange_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_by_index_store_same_slot_text,
    array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len,
    array_string_insert_const_mid_subrange_len_region_store_len, array_string_len_by_index,
    array_string_len_sum_region, array_string_session_indexof_by_index,
    array_string_session_indexof_by_index_const_utf8, with_cstr_utf8_ptr, with_cstr_utf8_ptr2,
    with_cstr_utf8_ptr3,
};
use super::handle_cache::valid_handle_idx;
use super::runtime_data::{
    array_runtime_get_any_key, array_runtime_has_any_key, array_runtime_set_any_key,
};

// Array ABI alias entry routes.

crate::nyash_export_i64_alias!(nyash_array_get_hh_alias, "nyash.array.get_hh", (handle: i64, key_any: i64), {
    array_runtime_get_any_key(handle, key_any)
});

crate::nyash_export_i64_alias!(nyash_array_set_hhh_alias, "nyash.array.set_hhh", (handle: i64, key_any: i64, val_any: i64), {
    array_runtime_set_any_key(handle, key_any, val_any)
});

crate::nyash_export_i64_alias!(nyash_array_has_hh_alias, "nyash.array.has_hh", (handle: i64, key_any: i64), {
    array_runtime_has_any_key(handle, key_any)
});

crate::nyash_export_i64_alias!(nyash_array_push_hh_alias, "nyash.array.push_hh", (handle: i64, val_any: i64), {
    array_slot_append_any(handle, val_any)
});

crate::nyash_export_i64_alias!(nyash_array_push_hi_alias, "nyash.array.push_hi", (handle: i64, value_i64: i64), {
    append_integer_raw(handle, value_i64)
});

crate::nyash_export_i64_alias!(nyash_array_get_hi_alias, "nyash.array.get_hi", (handle: i64, idx: i64), {
    array_slot_load_encoded_i64(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_set_hih_alias, "nyash.array.set_hih", (handle: i64, idx: i64, val_any: i64), {
    array_slot_store_any(handle, idx, val_any)
});

crate::nyash_export_i64_alias!(nyash_array_set_hii_alias, "nyash.array.set_hii", (handle: i64, idx: i64, value_i64: i64), {
    array_slot_store_i64(handle, idx, value_i64)
});

crate::nyash_export_i64_alias!(nyash_array_set_his_alias, "nyash.array.set_his", (handle: i64, idx: i64, value_h: i64), {
    array_slot_store_string_handle(handle, idx, value_h)
});

crate::nyash_export_i64_alias!(nyash_array_kernel_slot_store_hi_alias, "nyash.array.kernel_slot_store_hi", (handle: i64, idx: i64, slot: *mut super::KernelTextSlot), {
    let Some(slot) = (unsafe { slot.as_mut() }) else {
        return 0;
    };
    array_slot_store_kernel_text_slot(handle, idx, slot)
});

crate::nyash_export_i64_alias!(nyash_array_kernel_slot_concat_his_alias, "nyash.array.kernel_slot_concat_his", (slot: *mut super::KernelTextSlot, handle: i64, idx: i64, suffix_ptr: *const i8), {
    array_kernel_slot_concat_his(slot, handle, idx, suffix_ptr)
});

crate::nyash_export_i64_alias!(nyash_array_string_suffix_store_his_alias, "nyash.array.string_suffix_store_his", (handle: i64, idx: i64, suffix_ptr: *const i8), {
    with_cstr_utf8_ptr(suffix_ptr, |suffix| {
        array_string_concat_const_suffix_by_index_store_same_slot_text(handle, idx, suffix)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_suffix_store_hisi_alias, "nyash.array.string_suffix_store_hisi", (handle: i64, idx: i64, suffix_ptr: *const i8, suffix_len: i64), {
    array_string_concat_const_suffix_by_index_store_same_slot_len(handle, idx, suffix_ptr, suffix_len)
});

crate::nyash_export_i64_alias!(nyash_array_kernel_slot_insert_hisi_alias, "nyash.array.kernel_slot_insert_hisi", (slot: *mut super::KernelTextSlot, handle: i64, idx: i64, middle_ptr: *const i8, split: i64), {
    array_kernel_slot_insert_hisi(slot, handle, idx, middle_ptr, split)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_store_hisi_alias, "nyash.array.string_insert_mid_store_hisi", (handle: i64, idx: i64, middle_ptr: *const i8, split: i64), {
    with_cstr_utf8_ptr(middle_ptr, |middle| {
        array_string_insert_const_mid_by_index_store_same_slot_text(handle, idx, middle, split)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_store_hisii_alias, "nyash.array.string_insert_mid_store_hisii", (handle: i64, idx: i64, middle_ptr: *const i8, middle_len: i64, split: i64), {
    array_string_insert_const_mid_by_index_store_same_slot_len(handle, idx, middle_ptr, middle_len, split)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_lenhalf_store_hisi_alias, "nyash.array.string_insert_mid_lenhalf_store_hisi", (handle: i64, idx: i64, middle_ptr: *const i8, _middle_len: i64), {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_cstr_utf8_ptr(middle_ptr, |middle| {
        let observe_enabled = crate::observe::enabled();
        crate::observe::record_store_array_str_enter();

        let out = super::array_handle_cache::with_array_box(handle, |arr| {
            arr.slot_insert_const_mid_lenhalf_raw(idx, middle)
        })
        .flatten();

        if out.is_some() && observe_enabled {
            crate::observe::record_store_array_str_existing_slot();
            crate::observe::record_store_array_str_source_store();
        }
        out.unwrap_or(0)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_subrange_store_hisiii_alias, "nyash.array.string_insert_mid_subrange_store_hisiii", (handle: i64, idx: i64, middle_ptr: *const i8, split: i64, start: i64, end: i64), {
    if !valid_handle_idx(handle, idx) || middle_ptr.is_null() {
        return 0;
    }
    with_cstr_utf8_ptr(middle_ptr, |middle| {
        array_string_insert_const_mid_subrange_by_index_store_same_slot_text(
            handle, idx, middle, split, start, end,
        )
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_subrange_store_hisiiii_alias, "nyash.array.string_insert_mid_subrange_store_hisiiii", (handle: i64, idx: i64, middle_ptr: *const i8, middle_len: i64, split: i64, start: i64, end: i64), {
    array_string_insert_const_mid_subrange_by_index_store_same_slot_len(handle, idx, middle_ptr, middle_len, split, start, end)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_subrange_len_store_hisi_alias, "nyash.array.string_insert_mid_subrange_len_store_hisi", (handle: i64, idx: i64, middle_ptr: *const i8, middle_len: i64), {
    array_string_insert_const_mid_subrange_len_by_index_store_same_slot_len(handle, idx, middle_ptr, middle_len)
});

crate::nyash_export_i64_alias!(nyash_array_string_insert_mid_subrange_len_store_region_hiisi_alias, "nyash.array.string_insert_mid_subrange_len_store_region_hiisi", (handle: i64, loop_bound: i64, row_modulus: i64, middle_ptr: *const i8, middle_len: i64), {
    array_string_insert_const_mid_subrange_len_region_store_len(handle, loop_bound, row_modulus, middle_ptr, middle_len)
});

crate::nyash_export_i64_alias!(nyash_array_string_indexof_suffix_store_region_hisisi_alias, "nyash.array.string_indexof_suffix_store_region_hisisi", (handle: i64, loop_bound: i64, needle_ptr: *const i8, _needle_len: i64, suffix_ptr: *const i8, _suffix_len: i64), {
    if handle <= 0 || loop_bound < 0 {
        return 0;
    }
    with_cstr_utf8_ptr2(needle_ptr, suffix_ptr, |needle, suffix| {
        super::array_handle_cache::with_array_box(handle, |arr| {
            arr.slot_text_indexof_suffix_store_region_raw(loop_bound, needle, suffix)
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_lenhalf_insert_mid_periodic_indexof_suffix_region_hiisiiisisi_alias, "nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_hiisiiisisi", (handle: i64, loop_bound: i64, row_modulus: i64, middle_ptr: *const i8, _middle_len: i64, observer_period: i64, observer_bound: i64, needle_ptr: *const i8, _needle_len: i64, suffix_ptr: *const i8, _suffix_len: i64), {
    if handle <= 0 || loop_bound < 0 || row_modulus <= 0 || observer_period <= 0 {
        return 0;
    }
    with_cstr_utf8_ptr3(middle_ptr, needle_ptr, suffix_ptr, |middle, needle, suffix| {
        super::array_handle_cache::with_array_box(handle, |arr| {
            arr.slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_raw(
                loop_bound,
                row_modulus,
                middle,
                observer_period,
                observer_bound,
                needle,
                suffix,
            )
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_string_lenhalf_insert_mid_periodic_indexof_suffix_region_ascii_hiisiiisisi_alias, "nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_ascii_hiisiiisisi", (handle: i64, loop_bound: i64, row_modulus: i64, middle_ptr: *const i8, _middle_len: i64, observer_period: i64, observer_bound: i64, needle_ptr: *const i8, _needle_len: i64, suffix_ptr: *const i8, _suffix_len: i64), {
    if handle <= 0 || loop_bound < 0 || row_modulus <= 0 || observer_period <= 0 {
        return 0;
    }
    with_cstr_utf8_ptr3(middle_ptr, needle_ptr, suffix_ptr, |middle, needle, suffix| {
        super::array_handle_cache::with_array_box(handle, |arr| {
            arr.slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_byte_boundary_safe_raw(
                loop_bound,
                row_modulus,
                middle,
                observer_period,
                observer_bound,
                needle,
                suffix,
            )
        })
        .flatten()
        .unwrap_or(0)
    })
    .unwrap_or(0)
});

crate::nyash_export_i64_alias!(nyash_array_has_hi_alias, "nyash.array.has_hi", (handle: i64, idx: i64), {
    array_slot_has_index(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_slot_len_h_alias, "nyash.array.slot_len_h", (handle: i64), {
    super::array_compat::nyash_array_length_h(handle)
});

crate::nyash_export_i64_alias!(nyash_array_slot_cap_h_alias, "nyash.array.slot_cap_h", (handle: i64), {
    array_slot_cap_i64(handle)
});

crate::nyash_export_i64_alias!(nyash_array_slot_load_hi_alias, "nyash.array.slot_load_hi", (handle: i64, idx: i64), {
    array_slot_load_encoded_i64(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_slot_store_hii_alias, "nyash.array.slot_store_hii", (handle: i64, idx: i64, value_i64: i64), {
    array_slot_store_i64(handle, idx, value_i64)
});

crate::nyash_export_i64_alias!(nyash_array_slot_store_hih_alias, "nyash.array.slot_store_hih", (handle: i64, idx: i64, val_any: i64), {
    array_slot_store_any(handle, idx, val_any)
});

crate::nyash_export_i64_alias!(nyash_array_rmw_add1_hi_alias, "nyash.array.rmw_add1_hi", (handle: i64, idx: i64), {
    array_slot_rmw_add1_i64(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_string_len_hi_alias, "nyash.array.string_len_hi", (handle: i64, idx: i64), {
    array_string_len_by_index(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_string_len_sum_region_hiii_alias, "nyash.array.string_len_sum_region_hiii", (handle: i64, loop_bound: i64, row_modulus: i64, initial_accumulator: i64), {
    array_string_len_sum_region(handle, loop_bound, row_modulus, initial_accumulator)
});

crate::nyash_export_i64_alias!(hako_array_text_slot_len_hi_alias, "hako.array_text.slot_len", (handle: i64, idx: i64), {
    array_string_len_by_index(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_string_indexof_hih_alias, "nyash.array.string_indexof_hih", (handle: i64, idx: i64, needle_h: i64), {
    array_string_session_indexof_by_index(handle, idx, needle_h)
});

crate::nyash_export_i64_alias!(nyash_array_string_indexof_hisi_alias, "nyash.array.string_indexof_hisi", (handle: i64, idx: i64, needle_ptr: *const i8, needle_len: i64), {
    array_string_session_indexof_by_index_const_utf8(handle, idx, needle_ptr, needle_len)
});

crate::nyash_export_i64_alias!(hako_array_text_slot_indexof_handle_needle_alias, "hako.array_text.slot_indexof_handle_needle", (handle: i64, idx: i64, needle_h: i64), {
    array_string_session_indexof_by_index(handle, idx, needle_h)
});

crate::nyash_export_i64_alias!(hako_array_text_session_indexof_handle_needle_alias, "hako.array_text.session_indexof_handle_needle", (handle: i64, idx: i64, needle_h: i64), {
    array_string_session_indexof_by_index(handle, idx, needle_h)
});

crate::nyash_export_i64_alias!(hako_array_text_slot_indexof_const_utf8_alias, "hako.array_text.slot_indexof_const_utf8", (handle: i64, idx: i64, needle_ptr: *const i8, needle_len: i64), {
    array_string_indexof_by_index_const_utf8(handle, idx, needle_ptr, needle_len)
});

crate::nyash_export_i64_alias!(hako_array_text_session_indexof_const_utf8_alias, "hako.array_text.session_indexof_const_utf8", (handle: i64, idx: i64, needle_ptr: *const i8, needle_len: i64), {
    array_string_session_indexof_by_index_const_utf8(handle, idx, needle_ptr, needle_len)
});

crate::nyash_export_i64_alias!(hako_array_text_slot_indexof_handle_hih_alias, "hako.array_text.slot_indexof_handle_hih", (handle: i64, idx: i64, needle_h: i64), {
    array_string_session_indexof_by_index(handle, idx, needle_h)
});

crate::nyash_export_i64_alias!(hako_array_text_slot_indexof_const_utf8_hisi_alias, "hako.array_text.slot_indexof_const_utf8_hisi", (handle: i64, idx: i64, needle_ptr: *const i8, needle_len: i64), {
    array_string_session_indexof_by_index_const_utf8(handle, idx, needle_ptr, needle_len)
});

crate::nyash_export_i64_alias!(nyash_array_slot_append_hh_alias, "nyash.array.slot_append_hh", (handle: i64, val_any: i64), {
    array_slot_append_any(handle, val_any)
});

crate::nyash_export_i64_alias!(nyash_array_slot_reserve_hi_alias, "nyash.array.slot_reserve_hi", (handle: i64, additional: i64), {
    array_slot_reserve_i64(handle, additional)
});

crate::nyash_export_i64_alias!(nyash_array_slot_grow_hi_alias, "nyash.array.slot_grow_hi", (handle: i64, target_capacity: i64), {
    array_slot_grow_i64(handle, target_capacity)
});
