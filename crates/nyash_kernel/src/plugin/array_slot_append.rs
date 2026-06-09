use super::array_direct_i64_buffer::direct_array_i64_push_i64;
use super::array_handle_cache::with_array_box;
use super::handle_cache::valid_handle;
use super::value_codec::{
    any_arg_to_box_with_profile, maybe_borrow_string_keep_with_epoch, with_array_store_str_source,
    ArrayStoreStrSource, CodecProfile, StringHandleSourceKind,
};
use super::value_demand::ARRAY_GENERIC_APPEND_ANY;
use nyash_rust::runtime::host_handles as handles;

#[inline(always)]
pub(super) fn array_slot_append_any(handle: i64, val_any: i64) -> i64 {
    let _demand = ARRAY_GENERIC_APPEND_ANY;
    if let Some(new_len) = direct_array_i64_push_i64(handle, val_any) {
        return new_len;
    }
    if !valid_handle(handle) {
        return 0;
    }
    let mut string_lane_result = None;
    with_array_store_str_source(val_any, |source_kind, source| {
        if !matches!(source_kind, StringHandleSourceKind::StringLike) {
            return;
        }
        let Some(value) = (match source {
            ArrayStoreStrSource::StringLike(source_text) => {
                Some(maybe_borrow_string_keep_with_epoch(
                    source_text.into_keep(),
                    val_any,
                    handles::drop_epoch(),
                ))
            }
            ArrayStoreStrSource::OtherObject | ArrayStoreStrSource::Missing => None,
        }) else {
            return;
        };
        string_lane_result = Some(
            with_array_box(handle, |arr| {
                let idx = arr.len() as i64;
                if arr.slot_append_box_raw(value) >= 0 {
                    idx + 1
                } else {
                    0
                }
            })
            .unwrap_or(0),
        );
    });
    if let Some(result) = string_lane_result {
        return result;
    }
    with_array_box(handle, |arr| {
        let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
        if let Some(i64_value) = value.as_i64_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_i64_raw(idx, i64_value) {
                idx + 1
            } else {
                0
            }
        } else if let Some(bool_value) = value.as_bool_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_bool_raw(idx, bool_value) {
                idx + 1
            } else {
                0
            }
        } else if let Some(f64_value) = value.as_f64_fast() {
            let idx = arr.len() as i64;
            if arr.slot_store_f64_raw(idx, f64_value) {
                idx + 1
            } else {
                0
            }
        } else {
            arr.slot_append_box_raw(value).max(0)
        }
    })
    .unwrap_or(0)
}
