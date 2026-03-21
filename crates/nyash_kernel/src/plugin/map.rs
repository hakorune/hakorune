// --- AOT ObjectModule dotted-name exports (Map) ---
// Provide dotted symbol names expected by ObjectBuilder lowering for MapBox operations.
use super::handle_helpers::with_map_box;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};
use super::value_codec::{any_arg_to_box, box_to_handle, int_arg_to_box, integer_box_to_i64};

#[inline]
fn map_debug_enabled() -> bool {
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
    with_map_box(handle, |map| {
        let key_box = any_arg_to_box(key_any);
        let value = map.get(key_box);
        box_to_handle(value)
    })
    .unwrap_or(0)
}

// size: (handle) -> i64
#[export_name = "nyash.map.size_h"]
pub extern "C" fn nyash_map_size_h(handle: i64) -> i64 {
    if map_debug_enabled() {
        eprintln!("[MAP] size_h(handle={})", handle);
    }
    with_map_box(handle, |map| {
        if let Some(size) = integer_box_to_i64(map.size().as_ref()) {
            if map_debug_enabled() {
                eprintln!("[MAP] size_h => {}", size);
            }
            size
        } else {
            0
        }
    })
    .unwrap_or(0)
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
        let size = with_map_box(handle, |map| {
            integer_box_to_i64(map.size().as_ref()).unwrap_or(-1)
        })
        .unwrap_or(-1);
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

// Raw map substrate aliases used by collection-owner cutover.
#[export_name = "nyash.map.slot_load_hi"]
pub extern "C" fn nyash_map_slot_load_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_slot_load_i64(handle, key_i64)
}

#[export_name = "nyash.map.slot_load_hh"]
pub extern "C" fn nyash_map_slot_load_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_slot_load_any(handle, key_any)
}

#[export_name = "nyash.map.slot_store_hih"]
pub extern "C" fn nyash_map_slot_store_hih_alias(handle: i64, key_i64: i64, val_any: i64) -> i64 {
    map_slot_store_i64_any(handle, key_i64, val_any)
}

#[export_name = "nyash.map.slot_store_hhh"]
pub extern "C" fn nyash_map_slot_store_hhh_alias(handle: i64, key_any: i64, val_any: i64) -> i64 {
    map_slot_store_any(handle, key_any, val_any)
}

#[export_name = "nyash.map.probe_hi"]
pub extern "C" fn nyash_map_probe_hi_alias(handle: i64, key_i64: i64) -> i64 {
    map_probe_contains_i64(handle, key_i64)
}

#[export_name = "nyash.map.probe_hh"]
pub extern "C" fn nyash_map_probe_hh_alias(handle: i64, key_any: i64) -> i64 {
    map_probe_contains_any(handle, key_any)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::boxes::map_box::MapBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_map_handle() -> i64 {
        let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
        handles::to_handle_arc(map) as i64
    }

    fn string_handle(value: &str) -> i64 {
        let value: Arc<dyn NyashBox> = Arc::new(StringBox::new(value.to_string()));
        handles::to_handle_arc(value) as i64
    }

    fn decode_string_from_handle(handle: i64) -> String {
        let object = handles::get(handle as u64).expect("map raw load handle");
        let string_box = object
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("map raw load must resolve StringBox");
        string_box.value.clone()
    }

    #[test]
    fn slot_probe_raw_aliases_keep_hh_contract() {
        let handle = new_map_handle();
        let key_handle = string_handle("slot-key");
        let value_handle = string_handle("slot-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_handle, value_handle), 1);
        assert_eq!(nyash_map_probe_hh_alias(handle, key_handle), 1);
        let got_handle = nyash_map_slot_load_hh_alias(handle, key_handle);
        assert!(got_handle > 0);
        assert_eq!(decode_string_from_handle(got_handle), "slot-value");

        assert_eq!(nyash_map_probe_hh_alias(handle, string_handle("missing")), 0);
        assert_eq!(nyash_map_slot_load_hh_alias(handle, string_handle("missing")), 0);
    }

    #[test]
    fn slot_probe_raw_aliases_keep_hi_contract() {
        let handle = new_map_handle();
        let value_handle = string_handle("value-hi");

        assert_eq!(nyash_map_slot_store_hih_alias(handle, -70001, value_handle), 1);
        assert_eq!(nyash_map_probe_hi_alias(handle, -70001), 1);
        let got_handle = nyash_map_slot_load_hi_alias(handle, -70001);
        assert!(got_handle > 0);
        assert_eq!(decode_string_from_handle(got_handle), "value-hi");

        assert_eq!(nyash_map_probe_hi_alias(handle, -70002), 0);
        assert_eq!(nyash_map_slot_load_hi_alias(handle, -70002), 0);
    }

    #[test]
    fn raw_aliases_keep_fail_safe_contract() {
        assert_eq!(nyash_map_slot_load_hi_alias(0, 1), 0);
        assert_eq!(nyash_map_slot_load_hh_alias(0, 1), 0);
        assert_eq!(nyash_map_slot_store_hih_alias(0, 1, 2), 0);
        assert_eq!(nyash_map_slot_store_hhh_alias(0, 1, 2), 0);
        assert_eq!(nyash_map_probe_hi_alias(0, 1), 0);
        assert_eq!(nyash_map_probe_hh_alias(0, 1), 0);
    }
}
