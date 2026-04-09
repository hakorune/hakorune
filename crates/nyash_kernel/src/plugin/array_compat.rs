use super::array_guard::valid_handle;
use super::array_handle_cache::with_array_box;
use super::array_slot_load::array_slot_load_encoded_i64;
use super::array_slot_store::array_slot_store_i64;
#[inline(always)]
pub(super) fn cli_verbose_enabled() -> bool {
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
pub(super) fn append_integer_raw(handle: i64, value_i64: i64) -> i64 {
    if !valid_handle(handle) {
        return 0;
    }
    with_array_box(handle, |arr| {
        let idx = arr.len() as i64;
        if arr.slot_store_i64_raw(idx, value_i64) {
            idx + 1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

// Compat-only exports consumed by historical pure/legacy surfaces.
// Manifest truth groups these as compat-only, not mainline substrate.
// Exported as: nyash_array_get_h(i64 handle, i64 idx) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_get_h(handle: i64, idx: i64) -> i64 {
    if cli_verbose_enabled() {
        eprintln!("[ARR] get_h(handle={}, idx={})", handle, idx);
    }
    let out = array_slot_load_encoded_i64(handle, idx);
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
    let applied = array_slot_store_i64(handle, idx, val);
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
    let len = append_integer_raw(handle, val);
    if cli_verbose_enabled() {
        eprintln!("[ARR] push_h -> len {}", len);
    }
    len
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
