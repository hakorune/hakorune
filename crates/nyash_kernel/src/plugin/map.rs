pub use super::map_compat::*;
pub use super::map_substrate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::boxes::map_box::MapBox;
    use nyash_rust::runtime::host_handles as handles;
    use crate::nyash_runtime_data_has_hh;
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

        assert_eq!(
            nyash_map_slot_store_hhh_alias(handle, key_handle, value_handle),
            1
        );
        assert_eq!(nyash_map_probe_hh_alias(handle, key_handle), 1);
        let got_handle = nyash_map_slot_load_hh_alias(handle, key_handle);
        assert!(got_handle > 0);
        assert_eq!(decode_string_from_handle(got_handle), "slot-value");

        assert_eq!(
            nyash_map_probe_hh_alias(handle, string_handle("missing")),
            0
        );
        assert_eq!(
            nyash_map_slot_load_hh_alias(handle, string_handle("missing")),
            0
        );
    }

    #[test]
    fn slot_probe_raw_aliases_keep_hi_contract() {
        let handle = new_map_handle();
        let value_handle = string_handle("value-hi");

        assert_eq!(
            nyash_map_slot_store_hih_alias(handle, -70001, value_handle),
            1
        );
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

    #[test]
    fn entry_count_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key_a = string_handle("entry-a");
        let key_b = string_handle("entry-b");
        let value = string_handle("entry-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_a, value), 1);
        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_b, value), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 2);
        assert_eq!(nyash_map_entry_count_h(handle), 2);
        assert_eq!(nyash_map_size_h(handle), 2);
        assert_eq!(nyash_map_entry_count_i64(0), 0);
        assert_eq!(nyash_map_entry_count_h(0), 0);
    }

    #[test]
    fn capacity_raw_alias_keeps_observer_contract() {
        let handle = new_map_handle();
        let key_a = string_handle("cap-a");
        let key_b = string_handle("cap-b");
        let value = string_handle("cap-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_a, value), 1);
        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_b, value), 1);
        assert!(nyash_map_cap_h(handle) >= nyash_map_entry_count_h(handle));
        assert_eq!(nyash_map_cap_h(0), 0);
    }

    #[test]
    fn clear_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key = string_handle("clear-key");
        let value = string_handle("clear-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_map_clear_h(handle), 0);
        assert_eq!(nyash_map_entry_count_i64(handle), 0);
        assert_eq!(nyash_map_probe_hh_alias(handle, key), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 0);
        assert_eq!(nyash_map_clear_h(0), 0);
    }

    #[test]
    fn delete_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key = string_handle("delete-key");
        let value = string_handle("delete-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key, value), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 1);
        assert_eq!(nyash_map_delete_hh_alias(handle, key), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 0);
        assert_eq!(nyash_map_probe_hh_alias(handle, key), 0);
        assert_eq!(nyash_map_slot_load_hh_alias(handle, key), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 0);
        assert_eq!(nyash_map_delete_hh_alias(handle, key), 0);
        assert_eq!(nyash_map_delete_hh_alias(0, key), 0);
    }
}
