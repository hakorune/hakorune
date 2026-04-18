use super::map_debug::map_debug_enabled;
use super::map_probe::{map_probe_contains_any, map_probe_contains_i64};
use super::map_slot_load::{map_slot_load_any, map_slot_load_i64};
use super::map_slot_store::{map_slot_store_any, map_slot_store_i64_any};

// Compat-only exports consumed by historical pure/legacy surfaces.
// entry_count_h: compatibility alias for historical callers.
#[export_name = "nyash.map.entry_count_h"]
pub extern "C" fn nyash_map_entry_count_h(handle: i64) -> i64 {
    super::map_substrate::map_entry_count_raw(handle)
}

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
    let out = map_slot_load_i64(handle, key);
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
    let out = map_slot_load_any(handle, key_any);
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
    let applied = map_slot_store_i64_any(handle, key, val);
    if map_debug_enabled() {
        let size = super::map_substrate::map_entry_count_raw(handle);
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
        let object = handles::get(handle as u64).expect("map compat get handle");
        let string_box = object
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("map compat value must be StringBox");
        string_box.value.clone()
    }

    #[test]
    fn map_set_h_legacy_completion_code_and_mutation_roundtrip() {
        let map_handle = new_map_handle();
        let key = -70001;
        let value_handle = string_handle("legacy-set-h");

        assert_eq!(nyash_map_set_h(map_handle, key, value_handle), 0);
        assert_eq!(nyash_map_has_hh(map_handle, key), 1);

        let got_handle = nyash_map_get_hh(map_handle, key);
        assert!(got_handle > 0, "map get_hh must return a value handle");
        assert_eq!(decode_string_from_handle(got_handle), "legacy-set-h");
    }

    #[test]
    fn map_set_hh_legacy_completion_code_and_mutation_roundtrip() {
        let map_handle = new_map_handle();
        let key_handle = string_handle("legacy-key-hh");
        let value_handle = string_handle("legacy-value-hh");

        assert_eq!(nyash_map_set_hh(map_handle, key_handle, value_handle), 0);
        assert_eq!(nyash_map_has_hh(map_handle, key_handle), 1);

        let got_handle = nyash_map_get_hh(map_handle, key_handle);
        assert!(got_handle > 0, "map get_hh must return a value handle");
        assert_eq!(decode_string_from_handle(got_handle), "legacy-value-hh");
    }

    #[test]
    fn map_get_h_legacy_reads_integer_key_storage() {
        let map_handle = new_map_handle();
        let value_handle = string_handle("compat-hi");

        assert_eq!(nyash_map_set_h(map_handle, -71001, value_handle), 0);
        let got_handle = nyash_map_get_h(map_handle, -71001);
        assert!(got_handle > 0, "map get_h must return a value handle");
        assert_eq!(decode_string_from_handle(got_handle), "compat-hi");
        assert_eq!(nyash_map_get_h(map_handle, -71002), 0);
    }

    #[test]
    fn map_get_hh_legacy_reads_handle_key_storage() {
        let map_handle = new_map_handle();
        let key_handle = string_handle("compat-key");
        let value_handle = string_handle("compat-value");

        assert_eq!(nyash_map_set_hh(map_handle, key_handle, value_handle), 0);
        let got_handle = nyash_map_get_hh(map_handle, key_handle);
        assert!(got_handle > 0, "map get_hh must return a value handle");
        assert_eq!(decode_string_from_handle(got_handle), "compat-value");
        assert_eq!(nyash_map_get_hh(map_handle, string_handle("missing")), 0);
    }

    #[test]
    fn map_size_h_legacy_alias_reads_entry_count() {
        let map_handle = new_map_handle();
        let key_a = string_handle("size-a");
        let key_b = string_handle("size-b");
        let value_handle = string_handle("size-value");

        assert_eq!(nyash_map_set_hh(map_handle, key_a, value_handle), 0);
        assert_eq!(nyash_map_set_hh(map_handle, key_b, value_handle), 0);
        assert_eq!(nyash_map_size_h(map_handle), 2);
    }

    #[test]
    fn map_entry_count_h_legacy_alias_reads_entry_count() {
        let map_handle = new_map_handle();
        let key_a = string_handle("entry-a");
        let key_b = string_handle("entry-b");
        let value_handle = string_handle("entry-value");

        assert_eq!(nyash_map_set_hh(map_handle, key_a, value_handle), 0);
        assert_eq!(nyash_map_set_hh(map_handle, key_b, value_handle), 0);
        assert_eq!(nyash_map_entry_count_h(map_handle), 2);
        assert_eq!(nyash_map_entry_count_h(0), 0);
    }

    #[test]
    fn map_invalid_handle_fail_safe_contract() {
        assert_eq!(nyash_map_entry_count_h(0), 0);
        assert_eq!(nyash_map_size_h(0), 0);
        assert_eq!(nyash_map_get_h(0, 1), 0);
        assert_eq!(nyash_map_get_hh(0, 1), 0);
        assert_eq!(nyash_map_has_h(0, 1), 0);
        assert_eq!(nyash_map_has_hh(0, 1), 0);
        assert_eq!(nyash_map_set_h(0, 1, 2), 0);
        assert_eq!(nyash_map_set_hh(0, 1, 2), 0);

        assert_eq!(nyash_map_entry_count_h(-1), 0);
        assert_eq!(nyash_map_size_h(-1), 0);
        assert_eq!(nyash_map_get_h(-1, 1), 0);
        assert_eq!(nyash_map_get_hh(-1, 1), 0);
        assert_eq!(nyash_map_has_h(-1, 1), 0);
        assert_eq!(nyash_map_has_hh(-1, 1), 0);
        assert_eq!(nyash_map_set_h(-1, 1, 2), 0);
        assert_eq!(nyash_map_set_hh(-1, 1, 2), 0);
    }
}
