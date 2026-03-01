// ---- Array helpers for LLVM lowering (handle-based) ----
use super::handle_helpers::{array_get_index_encoded_i64, with_array_box};
use super::value_codec::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    string_handle_or_immediate_box_from_obj, try_retarget_borrowed_string_slot,
    try_retarget_borrowed_string_slot_with_source, ArrayFastDecodedValue, CodecProfile,
};
use nyash_rust::box_trait::IntegerBox;
use nyash_rust::boxes::array::ArrayBox;
use nyash_rust::runtime::host_handles as handles;

#[inline(always)]
fn cli_verbose_enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1")
    }
    #[cfg(not(test))]
    {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1"))
    }
}

#[inline(always)]
fn array_get_by_index(handle: i64, idx: i64) -> i64 {
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

#[inline(always)]
fn array_set_by_index(handle: i64, idx: i64, val_any: i64) -> i64 {
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

#[inline(always)]
fn array_set_by_index_i64_value(handle: i64, idx: i64, value_i64: i64) -> i64 {
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

#[inline(always)]
fn array_set_by_index_string_handle_value(handle: i64, idx: i64, value_h: i64) -> i64 {
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

fn array_has_by_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 }).unwrap_or(0)
}

#[inline(always)]
fn decode_index_key(key_any: i64) -> Option<i64> {
    any_arg_to_index(key_any)
}

// Exported as: nyash_array_get_h(i64 handle, i64 idx) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_get_h(handle: i64, idx: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] get_h(handle={}, idx={})", handle, idx);
    }
    let out = array_get_by_index(handle, idx);
    if cli_verbose_enabled() {
        eprintln!("[ARR] get_h => {}", out);
    }
    out
}

// Exported as: nyash_array_set_h(i64 handle, i64 idx, i64 val) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_set_h(handle: i64, idx: i64, val: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] set_h(handle={}, idx={}, val={})", handle, idx, val);
    }
    let applied = array_set_by_index_i64_value(handle, idx, val);
    if cli_verbose_enabled() {
        eprintln!("[ARR] set_h applied={} (legacy return=0)", applied);
    }
    // Legacy ABI contract: nyash.array.set_h reports completion with `0`
    // and does not expose applied/non-applied via return code.
    0
}

// Exported as: nyash_array_push_h(i64 handle, i64 val) -> i64 (returns new length)
#[no_mangle]
pub extern "C" fn nyash_array_push_h(handle: i64, val: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] push_h(handle={}, val={})", handle, val);
    }
    if handle <= 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let _ = arr.push(any_arg_to_box(val));
        let len = arr.len() as i64;
        if cli_verbose_enabled() {
            eprintln!("[ARR] push_h -> len {}", len);
        }
        len
    })
    .unwrap_or(0)
}

// Exported as: nyash_array_length_h(i64 handle) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_length_h(handle: i64) -> i64 {
    with_array_box(handle, |arr| arr.len() as i64).unwrap_or(0)
}

// --- AOT ObjectModule dotted-name aliases (Array) ---
// Provide dotted symbol names expected by ObjectBuilder lowering, forwarding to existing underscored exports.
#[export_name = "nyash.array.get_h"]
pub extern "C" fn nyash_array_get_h_alias(handle: i64, idx: i64) -> i64 {
    nyash_array_get_h(handle, idx)
}

#[export_name = "nyash.array.set_h"]
pub extern "C" fn nyash_array_set_h_alias(handle: i64, idx: i64, val: i64) -> i64 {
    nyash_array_set_h(handle, idx, val)
}

#[export_name = "nyash.array.push_h"]
pub extern "C" fn nyash_array_push_h_alias(handle: i64, val: i64) -> i64 {
    nyash_array_push_h(handle, val)
}

#[export_name = "nyash.array.len_h"]
pub extern "C" fn nyash_array_len_h_alias(handle: i64) -> i64 {
    nyash_array_length_h(handle)
}

// RuntimeData mono-route aliases (Array-only semantics).
// Keep contracts aligned with nyash.runtime_data.* when receiver is ArrayBox.
#[export_name = "nyash.array.get_hh"]
pub extern "C" fn nyash_array_get_hh_alias(handle: i64, key_any: i64) -> i64 {
    let Some(idx) = decode_index_key(key_any) else {
        return 0;
    };
    array_get_by_index(handle, idx)
}

#[export_name = "nyash.array.set_hhh"]
pub extern "C" fn nyash_array_set_hhh_alias(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let Some(idx) = decode_index_key(key_any) else {
        return 0;
    };
    array_set_by_index(handle, idx, val_any)
}

#[export_name = "nyash.array.has_hh"]
pub extern "C" fn nyash_array_has_hh_alias(handle: i64, key_any: i64) -> i64 {
    let Some(idx) = decode_index_key(key_any) else {
        return 0;
    };
    array_has_by_index(handle, idx)
}

#[export_name = "nyash.array.push_hh"]
pub extern "C" fn nyash_array_push_hh_alias(handle: i64, val_any: i64) -> i64 {
    if handle <= 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let _ = arr.push(any_arg_to_box_with_profile(
            val_any,
            CodecProfile::ArrayFastBorrowString,
        ));
        arr.len() as i64
    })
    .unwrap_or(0)
}

#[export_name = "nyash.array.push_hi"]
pub extern "C" fn nyash_array_push_hi_alias(handle: i64, value_i64: i64) -> i64 {
    if handle <= 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let _ = arr.push(Box::new(IntegerBox::new(value_i64)));
        arr.len() as i64
    })
    .unwrap_or(0)
}

// RuntimeData mono-route aliases with integer-key contract.
// These routes are selected by lowering when key VID is proven i64/non-negative.
#[export_name = "nyash.array.get_hi"]
pub extern "C" fn nyash_array_get_hi_alias(handle: i64, idx: i64) -> i64 {
    array_get_by_index(handle, idx)
}

#[export_name = "nyash.array.set_hih"]
pub extern "C" fn nyash_array_set_hih_alias(handle: i64, idx: i64, val_any: i64) -> i64 {
    array_set_by_index(handle, idx, val_any)
}

#[export_name = "nyash.array.set_hii"]
pub extern "C" fn nyash_array_set_hii_alias(handle: i64, idx: i64, value_i64: i64) -> i64 {
    array_set_by_index_i64_value(handle, idx, value_i64)
}

#[export_name = "nyash.array.set_his"]
pub extern "C" fn nyash_array_set_his_alias(handle: i64, idx: i64, value_h: i64) -> i64 {
    array_set_by_index_string_handle_value(handle, idx, value_h)
}

#[export_name = "nyash.array.has_hi"]
pub extern "C" fn nyash_array_has_hi_alias(handle: i64, idx: i64) -> i64 {
    array_has_by_index(handle, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::NyashBox;
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_array_handle() -> i64 {
        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        handles::to_handle_arc(arr) as i64
    }

    #[test]
    fn legacy_set_h_returns_zero_but_applies_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 11), 1);

        // Legacy contract keeps return=0 even when applied.
        assert_eq!(nyash_array_set_h(handle, 0, 77), 0);
        assert_eq!(nyash_array_get_h(handle, 0), 77);
    }

    #[test]
    fn hi_hii_aliases_keep_fail_safe_contract() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 10), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 10);

        assert_eq!(nyash_array_set_hii_alias(handle, 0, 33), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 33);
        assert_eq!(nyash_array_has_hi_alias(handle, 0), 1);

        // Out-of-bounds keeps fail-safe return values.
        assert_eq!(nyash_array_get_hi_alias(handle, 3), 0);
        assert_eq!(nyash_array_set_hii_alias(handle, 3, 9), 0);
        assert_eq!(nyash_array_has_hi_alias(handle, 3), 0);
    }

    #[test]
    fn push_hi_alias_appends_integer_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_hi_alias(handle, 7), 1);
        assert_eq!(nyash_array_push_hi_alias(handle, 9), 2);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 7);
        assert_eq!(nyash_array_get_hi_alias(handle, 1), 9);
    }

    #[test]
    fn set_his_alias_sets_string_handle_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 1), 1);
        let string_handle_a = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("ok".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;
        let string_handle_b = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("ng".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;
        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle_a), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), string_handle_a);
        // Re-set same slot keeps alias contract and must expose the latest handle.
        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle_b), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), string_handle_b);
    }
}
