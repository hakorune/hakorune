use super::handle_cache::with_map_box;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};
use super::map_key_codec::map_key_string_from_any;
use super::value_codec::{box_to_handle, int_arg_to_box};

#[inline]
pub(super) fn map_debug_enabled() -> bool {
    std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1")
}

#[inline]
fn map_get_compat_i64(handle: i64, key_i64: i64) -> i64 {
    with_map_box(handle, |map| {
        let value = map.get(int_arg_to_box(key_i64));
        box_to_handle(value)
    })
    .unwrap_or(0)
}

#[inline]
fn map_get_compat_any(handle: i64, key_any: i64) -> i64 {
    let key_str = map_key_string_from_any(key_any);
    with_map_box(handle, |map| {
        let value = map.get_opt_key_str(&key_str)?;
        Some(box_to_handle(value))
    })
    .flatten()
    .unwrap_or(0)
}

// Compat-only exports consumed by historical pure/legacy surfaces.
// size: compatibility observer (handle) -> i64
#[export_name = "nyash.map.size_h"]
pub extern "C" fn nyash_map_size_h(handle: i64) -> i64 {
    super::map_substrate::map_entry_count_raw(handle)
}

// get_h: (map_handle, key_i64) -> value_handle
#[export_name = "nyash.map.get_h"]
pub extern "C" fn nyash_map_get_h(handle: i64, key: i64) -> i64 {
    if map_debug_enabled() {
        eprintln!("[MAP] get_h(handle={}, key={})", handle, key);
    }
    let out = map_get_compat_i64(handle, key);
    if map_debug_enabled() {
        eprintln!("[MAP] get_h => handle {}", out);
    }
    out
}

// get_hh: (map_handle, key_handle) -> value_handle
#[export_name = "nyash.map.get_hh"]
pub extern "C" fn nyash_map_get_hh(handle: i64, key_any: i64) -> i64 {
    map_get_compat_any(handle, key_any)
}

// set_h: (map_handle, key_i64, val) -> i64 (ignored/0)
#[export_name = "nyash.map.set_h"]
pub extern "C" fn nyash_map_set_h(handle: i64, key: i64, val: i64) -> i64 {
    if map_debug_enabled() {
        eprintln!("[MAP] set_h(handle={}, key={}, val={})", handle, key, val);
    }
    let applied = map_slot_store_i64_any(handle, key, val);
    if map_debug_enabled() {
        let size = with_map_box(handle, |map| map.entry_count_i64()).unwrap_or(-1);
        eprintln!("[MAP] set_h applied={} size={}", applied, size);
    }
    0
}

// set_hh: (map_handle, key_any: handle or i64, val_any: handle or i64) -> i64
#[export_name = "nyash.map.set_hh"]
pub extern "C" fn nyash_map_set_hh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let _ = map_slot_store_any(handle, key_any, val_any);
    0
}

// has_hh: (map_handle, key_any: handle or i64) -> i64 (0/1)
#[export_name = "nyash.map.has_hh"]
pub extern "C" fn nyash_map_has_hh(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}

// has_h: (map_handle, key_i64) -> i64 (0/1)
#[export_name = "nyash.map.has_h"]
pub extern "C" fn nyash_map_has_h(handle: i64, key: i64) -> i64 {
    map_probe_contains_i64(handle, key)
}
