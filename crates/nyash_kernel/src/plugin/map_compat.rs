use super::map_debug::map_debug_enabled;
use super::map_runtime_facade::{
    map_runtime_load_any, map_runtime_load_i64, map_runtime_probe_any, map_runtime_probe_i64,
    map_runtime_store_any, map_runtime_store_i64_any,
};

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
    let out = map_runtime_load_i64(handle, key);
    if map_debug_enabled() {
        eprintln!("[MAP] get_h => handle {}", out);
    }
    out
}

// get_hh: (map_handle, key_handle) -> value_handle
#[export_name = "nyash.map.get_hh"]
pub extern "C" fn nyash_map_get_hh(handle: i64, key_any: i64) -> i64 {
    if map_debug_enabled() {
        eprintln!("[MAP] get_hh(handle={}, key_any={})", handle, key_any);
    }
    let out = map_runtime_load_any(handle, key_any);
    if map_debug_enabled() {
        eprintln!("[MAP] get_hh => handle {}", out);
    }
    out
}

// set_h: (map_handle, key_i64, val) -> i64 (ignored/0)
#[export_name = "nyash.map.set_h"]
pub extern "C" fn nyash_map_set_h(handle: i64, key: i64, val: i64) -> i64 {
    if map_debug_enabled() {
        eprintln!("[MAP] set_h(handle={}, key={}, val={})", handle, key, val);
    }
    let applied = map_runtime_store_i64_any(handle, key, val);
    if map_debug_enabled() {
        let size = super::map_substrate::map_entry_count_raw(handle);
        eprintln!("[MAP] set_h applied={} size={}", applied, size);
    }
    0
}

// set_hh: (map_handle, key_any: handle or i64, val_any: handle or i64) -> i64
#[export_name = "nyash.map.set_hh"]
pub extern "C" fn nyash_map_set_hh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let _ = map_runtime_store_any(handle, key_any, val_any);
    0
}

// has_hh: (map_handle, key_any: handle or i64) -> i64 (0/1)
#[export_name = "nyash.map.has_hh"]
pub extern "C" fn nyash_map_has_hh(handle: i64, key_any: i64) -> i64 {
    map_runtime_probe_any(handle, key_any)
}

// has_h: (map_handle, key_i64) -> i64 (0/1)
#[export_name = "nyash.map.has_h"]
pub extern "C" fn nyash_map_has_h(handle: i64, key: i64) -> i64 {
    map_runtime_probe_i64(handle, key)
}
