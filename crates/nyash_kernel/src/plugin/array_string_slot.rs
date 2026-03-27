use super::value_codec::{
    string_handle_or_immediate_box_from_obj, try_retarget_borrowed_string_slot,
    try_retarget_borrowed_string_slot_with_source,
};
use crate::nyash_string_indexof_hh_export;
use nyash_rust::boxes::array::ArrayBox;
use nyash_rust::runtime::host_handles as handles;

pub(super) fn array_string_len_by_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    handles::with_handle(handle as u64, |arr_obj| {
        let Some(obj) = arr_obj else {
            return 0;
        };
        let Some(arr) = obj.as_any().downcast_ref::<ArrayBox>() else {
            return 0;
        };
        let idx = idx as usize;
        arr.with_items_read(|items| {
            let Some(item) = items.get(idx) else {
                return 0;
            };
            item.as_str_fast().map(|s| s.len() as i64).unwrap_or(0)
        })
    })
}

#[inline(always)]
pub(super) fn array_string_indexof_by_index(handle: i64, idx: i64, needle_h: i64) -> i64 {
    let item_h = super::array_slot_load::array_slot_load_encoded_i64(handle, idx);
    nyash_string_indexof_hh_export(item_h, needle_h)
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
        arr.with_items_write(|items| {
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
    })
}
