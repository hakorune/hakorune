use super::handle_helpers::{array_get_index_encoded_i64, with_array_box};
use super::value_codec::{
    any_arg_to_index, decode_array_fast_value, string_handle_or_immediate_box_from_obj,
    try_retarget_borrowed_string_slot, try_retarget_borrowed_string_slot_with_source,
    ArrayFastDecodedValue,
};
use nyash_rust::boxes::array::ArrayBox;
use nyash_rust::runtime::host_handles as handles;

pub(super) fn array_get_by_index(handle: i64, idx: i64) -> i64 {
    if let Some(out) = with_array_box(handle, |arr| {
        if idx < 0 {
            return None;
        }
        let items = arr.items.read();
        let item = items.get(idx as usize)?;
        if let Some(iv) = item.as_i64_fast() {
            return Some(iv);
        }
        if let Some(bv) = item.as_bool_fast() {
            return Some(if bv { 1 } else { 0 });
        }
        None
    })
    .flatten()
    {
        return out;
    }
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

pub(super) fn array_set_by_index(handle: i64, idx: i64, val_any: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    match decode_array_fast_value(val_any) {
        ArrayFastDecodedValue::ImmediateI64(v) => array_set_by_index_i64_value(handle, idx, v),
        ArrayFastDecodedValue::Boxed(value) => with_array_box(handle, |arr| {
            if arr.try_set_index_i64(idx, value) {
                1
            } else {
                0
            }
        })
        .unwrap_or(0),
    }
}

pub(super) fn array_set_by_index_i64_value(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.try_set_index_i64_integer(idx, value_i64) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

pub(super) fn array_set_by_index_string_handle_value(handle: i64, idx: i64, value_h: i64) -> i64 {
    if handle <= 0 || idx < 0 || value_h <= 0 {
        return 0;
    }
    let drop_epoch = handles::drop_epoch();
    handles::with_pair(handle as u64, value_h as u64, |arr_obj, value_obj| {
        let Some(obj) = arr_obj else {
            return 0;
        };
        let Some(arr) = obj.as_any().downcast_ref::<ArrayBox>() else {
            return 0;
        };
        let idx = idx as usize;
        let mut items = arr.items.write();
        if idx < items.len() {
            if let Some(value_obj) = value_obj {
                if try_retarget_borrowed_string_slot_with_source(
                    &mut items[idx],
                    value_h,
                    value_obj,
                    drop_epoch,
                ) {
                    return 1;
                }
            } else if try_retarget_borrowed_string_slot(&mut items[idx], value_h) {
                return 1;
            }
            let value = string_handle_or_immediate_box_from_obj(value_obj, value_h, drop_epoch);
            items[idx] = value;
            return 1;
        }
        if idx == items.len() {
            let value = string_handle_or_immediate_box_from_obj(value_obj, value_h, drop_epoch);
            items.push(value);
            return 1;
        }
        0
    })
}

pub(super) fn array_has_by_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 }).unwrap_or(0)
}

pub(super) fn decode_index_key(key_any: i64) -> Option<i64> {
    any_arg_to_index(key_any)
}
