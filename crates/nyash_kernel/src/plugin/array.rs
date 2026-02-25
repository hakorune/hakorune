// ---- Array helpers for LLVM lowering (handle-based) ----
use super::handle_helpers::{array_get_index_encoded_i64, with_array_box};
use super::value_codec::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, int_arg_to_box,
    integer_box_to_i64, CodecProfile,
};

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
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

#[inline(always)]
fn array_set_by_index(handle: i64, idx: i64, val_any: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    let value = any_arg_to_box_with_profile(val_any, CodecProfile::ArrayFastBorrowString);
    with_array_box(handle, |arr| {
        if arr.try_set_index_i64(idx, value) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
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

fn array_has_by_index(handle: i64, idx: i64) -> i64 {
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| if arr.has_index_i64(idx) { 1 } else { 0 }).unwrap_or(0)
}

// Exported as: nyash_array_get_h(i64 handle, i64 idx) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_get_h(handle: i64, idx: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] get_h(handle={}, idx={})", handle, idx);
    }
    if handle <= 0 || idx < 0 {
        return 0;
    }
    with_array_box(handle, |arr| {
        let val = arr.get_index_i64(idx);
        if let Some(iv) = integer_box_to_i64(val.as_ref()) {
            if cli_verbose_enabled() {
                eprintln!("[ARR] get_h => {}", iv);
            }
            iv
        } else {
            0
        }
    })
    .unwrap_or(0)
}

// Exported as: nyash_array_set_h(i64 handle, i64 idx, i64 val) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_set_h(handle: i64, idx: i64, val: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] set_h(handle={}, idx={}, val={})", handle, idx, val);
    }
    if handle <= 0 || idx < 0 {
        return 0;
    }
    let _ = with_array_box(handle, |arr| {
        let _applied = arr.try_set_index_i64(idx, int_arg_to_box(val));
        if cli_verbose_enabled() {
            eprintln!("[ARR] set_h done; size now {}", arr.len());
        }
    });
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
    let Some(idx) = any_arg_to_index(key_any) else {
        return 0;
    };
    array_get_by_index(handle, idx)
}

#[export_name = "nyash.array.set_hhh"]
pub extern "C" fn nyash_array_set_hhh_alias(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let Some(idx) = any_arg_to_index(key_any) else {
        return 0;
    };
    array_set_by_index(handle, idx, val_any)
}

#[export_name = "nyash.array.has_hh"]
pub extern "C" fn nyash_array_has_hh_alias(handle: i64, key_any: i64) -> i64 {
    let Some(idx) = any_arg_to_index(key_any) else {
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

#[export_name = "nyash.array.has_hi"]
pub extern "C" fn nyash_array_has_hi_alias(handle: i64, idx: i64) -> i64 {
    array_has_by_index(handle, idx)
}
