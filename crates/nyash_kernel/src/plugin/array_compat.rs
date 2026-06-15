use super::array_handle_cache::{with_array_box, with_array_box_ready};
use super::array_slot_load::array_slot_load_encoded_i64;
use super::array_slot_store::array_slot_store_i64;
use super::handle_cache::valid_handle;

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

#[inline(always)]
fn cli_verbose_eprintln(args: std::fmt::Arguments<'_>) {
    if crate::env_flags::cli_verbose_enabled() {
        eprintln!("{}", args);
    }
}

// Compatibility exports for the array ABI surface.
// Exported as: nyash_array_get_h(i64 handle, i64 idx) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_get_h(handle: i64, idx: i64) -> i64 {
    cli_verbose_eprintln(format_args!("[ARR] get_h(handle={}, idx={})", handle, idx));
    let out = array_slot_load_encoded_i64(handle, idx);
    cli_verbose_eprintln(format_args!("[ARR] get_h => {}", out));
    out
}

// Exported as: nyash_array_set_h(i64 handle, i64 idx, i64 val) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_set_h(handle: i64, idx: i64, val: i64) -> i64 {
    cli_verbose_eprintln(format_args!(
        "[ARR] set_h(handle={}, idx={}, val={})",
        handle, idx, val
    ));
    let applied = array_slot_store_i64(handle, idx, val);
    cli_verbose_eprintln(format_args!("[ARR] set_h applied={}", applied));
    // ABI contract: nyash.array.set_h reports completion with `0`.
    0
}

// Exported as: nyash_array_push_h(i64 handle, i64 val) -> i64 (returns new length)
#[no_mangle]
pub extern "C" fn nyash_array_push_h(handle: i64, val: i64) -> i64 {
    cli_verbose_eprintln(format_args!("[ARR] push_h(handle={}, val={})", handle, val));
    let len = append_integer_raw(handle, val);
    cli_verbose_eprintln(format_args!("[ARR] push_h -> len {}", len));
    len
}

// Exported as: nyash_array_length_h(i64 handle) -> i64
#[no_mangle]
pub extern "C" fn nyash_array_length_h(handle: i64) -> i64 {
    with_array_box_ready(handle, |arr| arr.len() as i64).unwrap_or(0)
}

// AOT ObjectModule dotted-name aliases (Array).
// Provide dotted symbol names expected by ObjectBuilder lowering.
crate::nyash_export_i64_alias!(nyash_array_get_h_alias, "nyash.array.get_h", (handle: i64, idx: i64), {
    nyash_array_get_h(handle, idx)
});

crate::nyash_export_i64_alias!(nyash_array_set_h_alias, "nyash.array.set_h", (handle: i64, idx: i64, val: i64), {
    nyash_array_set_h(handle, idx, val)
});

crate::nyash_export_i64_alias!(nyash_array_push_h_alias, "nyash.array.push_h", (handle: i64, val: i64), {
    nyash_array_push_h(handle, val)
});

crate::nyash_export_i64_alias!(nyash_array_len_h_alias, "nyash.array.len_h", (handle: i64), {
    nyash_array_length_h(handle)
});
